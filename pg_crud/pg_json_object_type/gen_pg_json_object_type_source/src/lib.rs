use enum_extension_lib::EnumExtension;
use gen_quotes::dq_ts;
use macros_helpers::{
    DeriveSerdeDeserialize, FormatWithCargofmt, ShouldWriteTokenStreamIntoFile,
    StructOrEnumDeriveTsStreamBuilder, SynFieldWrapper, gen_if_write_is_err_curly_braces_ts,
    gen_if_write_is_err_ts, gen_impl_display_ts, gen_impl_from_ts,
    gen_impl_pub_const_new_for_ident_ts, gen_impl_pub_new_for_ident_ts, gen_impl_to_err_string_ts,
    gen_pub_const_new_ts, gen_pub_new_ts, gen_pub_try_new_ts, gen_pub_type_alias_ts,
    gen_simple_syn_punct, get_macro_attr_meta_list_ts, maybe_write_ts_into_file,
};
use naming::{
    AllFieldsAreNoneUcc, ArrayOfUcc, AsRefStrToUccTs, AsUcc, ColumnNameAndMaybeFieldGetterSc,
    ColumnSc, ContainsAllElementsOfArrayUcc, CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc,
    CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc, CreateSc, CreateUpdateDeleteAreEmptyUcc,
    DefaultOptionSomeVecOneElSc, DefaultOptionSomeVecOneElUcc, DeleteSc, DimOneEqualUcc,
    DimOneInUcc, DisplayPlusToTokens, EqualUcc, ErSc, FieldsSc, GenJsonbSetTargetSc, IdSc,
    IdsAreNotUniqueUcc, InUcc, IncrSc, IsNeedToAddLogicalOperatorSc, JsonbObjectUcc,
    JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc, LengthEqualUcc, LengthGreaterThanUcc,
    NotUniqueIdInJsonDeleteArrayUcc, NotUniqueIdInJsonUpdateAndDeleteArraysUcc, OptionUpdateSc,
    OptionVecCreateSc, OverlapsWithArrayUcc, PgJsonTypeTestCasesUcc, PgJsonTypeUcc,
    PgTypeTestCasesUcc, PgTypeUcc, PreviousReadMergedWithOptionUpdateIntoReadSc, QueryPartErUcc,
    QueryPartSc, QuerySc, ReadInnerIntoReadWithNewOrTryNewUnwrapedSc,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc, ReadOnlyIdsIntoOptionValueReadInnerSc,
    ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc, ReadOnlyIdsSc,
    ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc, ReadOnlyIdsToTwoDimalVecReadInnerSc,
    ReadSc, SelectOnlyCreatedIdsQueryBindSc, SelectOnlyCreatedIdsQueryPartSc,
    SelectOnlyIdsQueryPartSc, SelectOnlyUpdatedIdsQueryBindSc, SelectOnlyUpdatedIdsQueryPartSc,
    SelectQueryPartPgTypeSc, SelectQueryPartSc, SelfSc, SelfUcc, StdOptionOptionObjectAccSc,
    ToTokensToUccTs, UpdateQueryBindSc, UpdateQueryPartSc, UpdateSc, UpdateToReadOnlyIdsSc,
    UuidUuidAsNotNullJsonbStringUcc, ValueSc, ValueUcc, VecOfUcc, WithIdUcc,
    parameter::{
        ElementSelfUcc, SelfCreateForQueryUcc, SelfCreateUcc, SelfCurrentSc,
        SelfGenPgJsonObjectTypeModSc, SelfLastSc, SelfReadInnerUcc, SelfReadOnlyIdsHandleUcc,
        SelfReadOnlyIdsUcc, SelfReadTryFromErUcc, SelfReadUcc, SelfSelectElementUcc, SelfSelectSc,
        SelfSelectUcc, SelfTableTypeDeclarationUcc, SelfUpdateElementUcc,
        SelfUpdateForQueryElementUcc, SelfUpdateForQueryUcc, SelfUpdateTryNewErUcc, SelfUpdateUcc,
        SelfWhereUcc,
    },
};
use panic_location::panic_location;
use pg_crud_macros_common::{
    ColumnParameterUnderscore, CreateQueryBindValueUnderscore, CreateQueryPartIncrUnderscore,
    CreateQueryPartValueUnderscore, DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, Dim, ImportPath,
    IncrParameterUnderscore, IsCreateQueryBindMutable, IsNeedToAddLogicalOperatorUnderscore,
    IsNullable, IsPrimaryKeyUnderscore, IsQueryBindMutable, IsSelectOnlyCreatedIdsQueryBindMutable,
    IsSelectOnlyUpdatedIdsQueryBindMutable,
    IsSelectQueryPartColumnNameAndMaybeFieldGetterForErMessageUsed, IsSelectQueryPartIsPgTypeUsed,
    IsSelectQueryPartSelfSelectUsed, IsUpdateQueryBindMutable, IsUpdateQueryPartJsonbSetTargetUsed,
    IsUpdateQueryPartSelfUpdateUsed, PgTypeOrPgJsonType, SelectQueryPartValueUnderscore,
    UpdateQueryPartJsonbSetAccumulatorUnderscore, UpdateQueryPartJsonbSetPathUnderscore,
    UpdateQueryPartJsonbSetTargetUnderscore, UpdateQueryPartValueUnderscore,
    gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts,
    gen_impl_pg_crud_default_option_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_default_option_some_vec_one_el_ts,
    gen_impl_pg_json_type_test_cases_for_ident_ts, gen_impl_pg_json_type_ts,
    gen_impl_pg_type_not_primary_key_for_ident_ts, gen_impl_pg_type_test_cases_for_ident_ts,
    gen_impl_pg_type_ts, gen_impl_serde_deserialize_for_struct_ts,
    gen_impl_sqlx_decode_sqlx_pg_for_ident_ts, gen_impl_sqlx_encode_sqlx_pg_for_ident_ts,
    gen_impl_sqlx_type_for_ident_ts, gen_match_try_new_in_deserialize_ts,
    gen_option_tokens_declaration_ts,
    gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
    gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
    gen_read_only_ids_merged_with_create_into_where_equal_ts,
    gen_return_err_query_part_er_write_into_buffer_ts, gen_sqlx_types_json_type_declaration_ts,
    gen_value_init_ts, gen_vec_tokens_declaration_ts, impl_pg_type_where_filter_for_ident_ts,
    maybe_wrap_into_braces_ts, wrap_content_into_scopes_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::iter::repeat_with;
use strum_macros::{Display, EnumIter};
use syn::{
    Data, DeriveInput, Field, FieldMutability, Fields, Ident, Path, Type, TypePath, Visibility,
    parse2,
    token::{Colon, Pub},
};
use token_patterns::{
    AllowClippyArbitrarySourceItemOrdering, MustUse, PgCrudDefaultOptionSomeVecOneElCall,
    PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall, StringTs,
};
//todo gen authorization rights enum for json fields
//todo bug in update if updating array and creating element in jsonb array without anything - read_only_ids generation logic of vec returns wrong query part
#[must_use]
pub fn gen_pg_json_object_type(input_ts: Ts2) -> Ts2 {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum TraitGen {
        PgJsonType,
        PgTypeAndPgJsonType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumIter, EnumExtension)]
    enum Pattern {
        Standart,
        Array,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct PgJsonObjectTypeRecord {
        is_nullable: IsNullable,
        pattern: Pattern,
        trait_gen: TraitGen,
    }
    #[derive(Debug, Deserialize)]
    struct GenPgJsonTypesConfig {
        pg_table_columns_content_write_into_pg_table_columns_using_pg_json_object_types:
            ShouldWriteTokenStreamIntoFile,
        vrt: PgJsonObjectTypeRecord,
        whole_content_write_into_gen_pg_json_object_type: ShouldWriteTokenStreamIntoFile,
    }
    panic_location();
    let di: DeriveInput = parse2(input_ts).expect("e5f0e27b");
    let import_path = ImportPath::PgCrud;
    let gen_pg_json_object_type_config = from_str::<GenPgJsonTypesConfig>(
        &get_macro_attr_meta_list_ts(
            &di.attrs,
            &format!("{}::pg_json_object_type_config", import_path.sc_str()),
        )
        .to_string(),
    )
    .expect("246de453");
    let pg_json_object_type_record_vec = {
        let pg_json_object_type_record = gen_pg_json_object_type_config.vrt;
        match (&pg_json_object_type_record.is_nullable, &pg_json_object_type_record.pattern) {
            (IsNullable::False, Pattern::Standart) => vec![pg_json_object_type_record],
            (IsNullable::True, Pattern::Standart) |
            (IsNullable::False, Pattern::Array) => vec![
                PgJsonObjectTypeRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Standart,
                    trait_gen: pg_json_object_type_record.trait_gen.clone(),
                },
                pg_json_object_type_record
            ],
            (IsNullable::True, Pattern::Array) => vec![
                PgJsonObjectTypeRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Standart,
                    trait_gen: pg_json_object_type_record.trait_gen.clone(),
                },
                PgJsonObjectTypeRecord {
                    is_nullable: IsNullable::True,
                    pattern: Pattern::Standart,
                    trait_gen: pg_json_object_type_record.trait_gen.clone(),
                },
                PgJsonObjectTypeRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Array,
                    trait_gen: pg_json_object_type_record.trait_gen.clone(),
                },
                pg_json_object_type_record
            ]
        }
    }
    // .into_iter()
    // .filter(|el_2f2d1e6c| {
    //     let is_nullable_filter = match &el_2f2d1e6c.is_nullable {
    //         IsNullable::False => true,
    //         IsNullable::True => true,
    //     };
    //     let pattern_filter = match &el_2f2d1e6c.pattern {
    //         Pattern::Standart => match &el_2f2d1e6c.is_nullable {
    //             IsNullable::False => true,
    //             IsNullable::True => true,
    //         },
    //         Pattern::Array => match &el_2f2d1e6c.is_nullable {
    //             IsNullable::False => true,
    //             IsNullable::True => true,
    //         },
    //     };
    //     let trait_gen_filter = match &el_2f2d1e6c.trait_gen {
    //         TraitGen::PgJsonType => true,
    //         TraitGen::PgTypeAndPgJsonType => true,
    //     };
    //     is_nullable_filter && pattern_filter && trait_gen_filter
    // })
    // .collect::<Vec<PgJsonObjectTypeRecord>>()
    ;
    // write_string_into_file::write_string_into_file(
    //     "GenPgJsonObjectTypeJsonVrts",
    //     &serde_json::to_string(&pg_json_object_type_record_vec).expect("efc7a263"),
    // );
    // element.iter().enumerate().fold(String::new(), |mut acc_1e1c6a6e, (index, element)| {
    //     let el_sc_str = naming_common::AsRefStrToScStr::case(element);
    //     if index == 0 {
    //         acc_1e1c6a6e.push_str(&el_sc_str);
    //     } else {
    //         acc_1e1c6a6e.push_str(&format!("_{el_sc_str}"));
    //     }
    //     acc_1e1c6a6e
    // });
    // let pg_json_object_type_array
    let (fields_ts, pg_json_object_type_array) = pg_json_object_type_record_vec
        .into_iter()
        .enumerate()
        .map(|(index, element)| {
            #[derive(Debug, Display, EnumIter, EnumExtension)]
            enum IsStandartWithId {
                False,
                True,
            }
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum IdentPattern {
                StandartNotNullWithoutId,
                StandartNotNullWithId,
                StandartNullableWithoutId,
                ArrayNotNullWithId,
                ArrayNullableWithIdentifier,//Identifier instead of Id - just to fix clippy lint
            }
            #[allow(clippy::arbitrary_source_item_ordering)]
            #[derive(Debug, Clone, Display)]
            enum PgJsonTypeSubtype {
                TableTypeDeclaration,
                Create,
                CreateForQuery,
                Select,
                Where,
                Read,
                ReadOnlyIds,
                ReadInner,
                Update,
                UpdateForQuery,
            }
            impl ToTokens for PgJsonTypeSubtype {
                fn to_tokens(&self, tokens: &mut Ts2) {
                    self.to_string().parse::<Ts2>().expect("43ac0b62").to_tokens(tokens);
                }
            }
            #[derive(Debug, Clone, Display)]
            enum PgTypeSubtype {
                // TableTypeDeclaration,
                // Create,
                // Select,
                // Where,
                Read,
                // ReadOnlyIds,
                // ReadInner,
                Update,
            }
            impl ToTokens for PgTypeSubtype {
                fn to_tokens(&self, tokens: &mut Ts2) {
                    self.to_string().parse::<Ts2>().expect("5825d4b7").to_tokens(tokens);
                }
            }
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum PgJsonTypeSubtypeTableTypeDeclarationOrCreate {
                TableTypeDeclaration,
                Create,
            }
            impl From<&PgJsonTypeSubtypeTableTypeDeclarationOrCreate> for PgJsonTypeSubtype {
                fn from(v: &PgJsonTypeSubtypeTableTypeDeclarationOrCreate) -> Self {
                    match &v {
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => Self::TableTypeDeclaration,
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Self::Create,
                    }
                }
            }
            enum ReadWithOrWithoutAnnotationOrInner {
                Inner,
                WithSerdeOptionIsNoneAnnotation,
                WithoutSerdeOptionIsNoneAnnotation,
            }
            enum ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation {
                False,
                True,
            }
            enum NewTypeOrStructDeclaration {
                NewType,
                StructDeclaration,
            }
            let is_nullable = &element.is_nullable;
            let pattern = &element.pattern;
            let trait_gen = &element.trait_gen;
            let self_field_vec_ts = quote! {.0.to_vec()};
            let cfg_feature_test_utils = quote! {#[cfg(feature = "test-utils")]};
            let return_err_query_part_er_write_into_buffer_ts = gen_return_err_query_part_er_write_into_buffer_ts(import_path);
            let none_ts = quote!{None};
            let must_use_ts = MustUse;
            let gen_import_path_value_init_ts = |ts: &dyn ToTokens|{
                gen_value_init_ts(
                    &import_path,
                    &ts
                )
            };
            let import_path_query_part_er_ts = quote! {#import_path::#QueryPartErUcc};
            let vec_pg_crud_default_option_some_vec_one_el_call_ts = quote!{vec![#PgCrudDefaultOptionSomeVecOneElCall]};
            let default_but_option_is_some_ts = quote!{
                #import_path::#DefaultOptionSomeVecOneElUcc::#DefaultOptionSomeVecOneElSc
            };
            let default_but_option_is_some_call_ts = quote!{#default_but_option_is_some_ts()};
            let gen_ident_as_default_but_option_is_some_call_ts = |ident_ts: &dyn ToTokens, |{
                quote!{
                    <#ident_ts as #import_path::#DefaultOptionSomeVecOneElUcc>::#DefaultOptionSomeVecOneElSc()
                }
            };
            let gen_ident_as_default_but_option_is_some_ts = |ident_ts_2e6aba01: &dyn ToTokens|quote!{
                <
                    #ident_ts_2e6aba01
                    as
                    #import_path::#DefaultOptionSomeVecOneElUcc
                >::#DefaultOptionSomeVecOneElSc
            };
            let import_path_value_ts = quote!{#import_path::#ValueUcc};
            let wrap_into_value_declaration_ts = |ident_ts: &dyn ToTokens|{
                quote!{#import_path_value_ts<#ident_ts>}
            };
            let wrap_into_value_init_ts = |ts: &dyn ToTokens|{
                quote!{#import_path_value_ts { #ValueSc: #ts }}
            };
            let di_ident = &di.ident;
            let vec_syn_field = if let Data::Struct(data_struct) = &di.data {
                if let Fields::Named(fields_named) = &data_struct.fields {
                    fields_named.named.iter()
                    .collect::<Vec<&Field>>()
                    .iter()
                    .map(|el_f01f3f33|SynFieldWrapper {
                        field_vis: el_f01f3f33.vis.clone(),
                        field_ident: el_f01f3f33.ident.clone().expect("3ac7f263"),
                        field_type: el_f01f3f33.ty.clone(),
                    })
                    .collect::<Vec<SynFieldWrapper>>()
                } else {
                    panic!("4c305996");
                }
            } else {
                panic!("a4fc18a1");
            };
            let is_standart_with_id_false = IsStandartWithId::False;
            let is_standart_with_id_true = IsStandartWithId::True;
            let gen_ident_ucc = |ident_pattern: &IdentPattern| {
                let (rust_part, pg_part, is_nullable_325dc715) = {
                    let di_ident_str = di_ident.to_string();
                    let vec_of_di_ident_with_id = format!("{VecOfUcc}{di_ident}{WithIdUcc}");
                    let jsonb_object_ucc_str = JsonbObjectUcc.to_string();
                    let array_of_not_null_jsonb_object_with_id = format!("{ArrayOfUcc}{}{JsonbObjectUcc}{WithIdUcc}", IsNullable::False.not_null_or_nullable_str());
                    match &ident_pattern {
                        IdentPattern::StandartNotNullWithoutId => (di_ident_str, jsonb_object_ucc_str, IsNullable::False),
                        IdentPattern::StandartNotNullWithId => (format!("{di_ident}{WithIdUcc}"), format!("{JsonbObjectUcc}{WithIdUcc}"), IsNullable::False),
                        IdentPattern::StandartNullableWithoutId => (di_ident_str, jsonb_object_ucc_str, IsNullable::True),
                        IdentPattern::ArrayNotNullWithId => (vec_of_di_ident_with_id, array_of_not_null_jsonb_object_with_id, IsNullable::False),
                        IdentPattern::ArrayNullableWithIdentifier => (vec_of_di_ident_with_id, array_of_not_null_jsonb_object_with_id, IsNullable::True),
                    }
                };
                let is_nullable_325dc715_rust = is_nullable_325dc715.rust();
                let not_null_or_nullable_str = is_nullable_325dc715.not_null_or_nullable_str();
                format!("{is_nullable_325dc715_rust}{rust_part}{AsUcc}{not_null_or_nullable_str}{pg_part}").parse::<Ts2>().expect("43784dd3")
            };
            let ident = &gen_ident_ucc(&match (&is_nullable, &pattern) {
                (IsNullable::False, Pattern::Standart) => IdentPattern::StandartNotNullWithoutId,
                (IsNullable::False, Pattern::Array) => IdentPattern::ArrayNotNullWithId,
                (IsNullable::True, Pattern::Standart) => IdentPattern::StandartNullableWithoutId,
                (IsNullable::True, Pattern::Array) => IdentPattern::ArrayNullableWithIdentifier,
            });
            let ident_standart_not_null_ucc = &gen_ident_ucc(&IdentPattern::StandartNotNullWithoutId);
            let ident_array_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrayNotNullWithId);
            let ident_with_id_standart_not_null_ucc = &gen_ident_ucc(&IdentPattern::StandartNotNullWithId);
            let ident_with_id_array_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrayNotNullWithId);
            let is_standart_not_null = matches!((&is_nullable, pattern), (IsNullable::False, Pattern::Standart));
            let gen_type_as_import_path_ts = |first_type_ts: &dyn ToTokens, second_type_ts: &dyn ToTokens|{
                quote! {<#first_type_ts as #import_path::#second_type_ts>}
            };
            let gen_type_as_pg_json_type_ts = |type_ts: &dyn ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PgJsonTypeUcc)
            };
            let ident_as_import_path_pg_json_type_ts = gen_type_as_pg_json_type_ts(&ident);
            let ident_standart_not_null_as_import_path_pg_json_type_ts = gen_type_as_pg_json_type_ts(&ident_standart_not_null_ucc);
            let ident_array_not_null_as_import_path_pg_json_type_ts = gen_type_as_pg_json_type_ts(&ident_array_not_null_ucc);
            let uuid_uuid_as_not_null_jsonb_string_ts = quote!{#import_path::#UuidUuidAsNotNullJsonbStringUcc};
            let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_update_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_update_ucc = SelfUpdateUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_update_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_where_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_where_ucc = SelfWhereUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_where_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_as_import_path_pg_json_type_ts = gen_type_as_pg_json_type_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_update_ts = quote!{
                #uuid_uuid_as_not_null_jsonb_string_as_import_path_pg_json_type_ts::Update
            };
            let uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts = quote!{
                <#uuid_uuid_as_not_null_jsonb_string_ts as #import_path::PgJsonTypeObjectVecElementId>
            };
            let id_syn_field = {
                let value = Field {
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
                                &[import_path.to_path(), &UuidUuidAsNotNullJsonbStringUcc.to_string()]
                            ),
                        },
                    }),
                };
                SynFieldWrapper {
                    field_vis: value.vis.clone(),
                    field_ident: value.ident.clone().expect("3550d755"),
                    field_type: value.ty,
                }
            };
            let vec_syn_field_with_id: Vec<SynFieldWrapper> = vec_syn_field.clone().into_iter().fold(vec![id_syn_field], |mut acc_9db5e042, el_f01f3f33| {
                acc_9db5e042.push(el_f01f3f33);
                acc_9db5e042
            });
            let get_vec_syn_field = |is_standart_with_id: &IsStandartWithId| -> &Vec<SynFieldWrapper> {
                match &is_standart_with_id {
                    IsStandartWithId::False => &vec_syn_field,
                    IsStandartWithId::True => &vec_syn_field_with_id,
                }
            };
            let gen_type_as_pg_type_ts = |type_ts: &dyn ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PgTypeUcc)
            };
            let gen_type_as_pg_json_type_test_cases_ts = |type_ts: &dyn ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PgJsonTypeTestCasesUcc)
            };
            let gen_type_as_pg_type_test_cases_ts = |type_ts: &dyn ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PgTypeTestCasesUcc)
            };
            let self_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&SelfUcc);
            let ident_standart_not_null_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(
                &ident_standart_not_null_ucc
            );
            let ident_standart_not_null_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&ident_standart_not_null_ucc);
            let import_path_pg_json_type_uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(
                &uuid_uuid_as_not_null_jsonb_string_ts
            );
            let ident_with_id_standart_not_null_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_create_ucc = SelfCreateUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_where_ucc = SelfWhereUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_ts = {
                let gen_struct_ident_ts = |ident_2c1c6b9c: &dyn ToTokens| StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy()
                    .build_struct(
                        &ident_2c1c6b9c,
                        &Ts2::new(),
                        &quote!{;}
                    );
                let ident_ts = gen_struct_ident_ts(&ident);
                let maybe_ident_with_id_standart_not_null_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_ts = gen_struct_ident_ts(&ident_with_id_standart_not_null_ucc);
                    let cfg_feature_test_utils_impl_ident_with_id_standart_not_null_ts = {
                        let read_only_ids_merged_with_create_into_where_equal_ts = gen_read_only_ids_merged_with_create_into_where_equal_ts(
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn ToTokens,
                                    field_type: &dyn ToTokens,
                                    second_argument_ts: &dyn ToTokens,
                                |{
                                    let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                            #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                            #second_argument_ts
                                        )
                                    }
                                };
                                let ident_ts_e06846af = gen_ts(
                                    &IdSc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &PgCrudDefaultOptionSomeVecOneElCall
                                );
                                let ts = vec_syn_field.iter().map(|el_e970b03b| {
                                    let field_ident = &el_e970b03b.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_e970b03b.field_type,
                                        &quote!{#CreateSc.#field_ident}
                                    )
                                });
                                quote!{
                                    #ident_with_id_standart_not_null_where_ucc::#EqualUcc(pg_crud::PgJsonTypeWhereEqual {
                                        logical_operator: pg_crud::LogicalOperator::Or,
                                        #ValueSc: #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                            #ident_ts_e06846af,
                                            #(#ts),*
                                        ),
                                    })
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
                            &import_path,
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn ToTokens,
                                    field_type: &dyn ToTokens,
                                    second_argument_ts: &dyn ToTokens,
                                |{
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #ident_with_id_standart_not_null_where_ucc::#field_ident_ucc(
                                            pg_crud::PgTypeWhere::new(
                                                pg_crud::LogicalOperator::And,
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                                                    #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                    #second_argument_ts
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_ts = gen_ts(
                                    &IdSc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &PgCrudDefaultOptionSomeVecOneElCall
                                );
                                let ts = vec_syn_field.iter().map(|el_4fafbc5e| {
                                    let field_ident = &el_4fafbc5e.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_4fafbc5e.field_type,
                                        &quote!{#CreateSc.#field_ident}
                                    )
                                });
                                quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_ts,
                                        #(#ts),*
                                    ]).expect("5473d8c4")
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
                            import_path,
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn ToTokens,
                                    field_type: &dyn ToTokens,
                                    second_argument_ts: &dyn ToTokens,
                                |{
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #ident_with_id_standart_not_null_where_ucc::#field_ident_ucc(
                                            #import_path::PgTypeWhere::new(
                                                #import_path::LogicalOperator::Or,
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc(
                                                    #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                    #second_argument_ts
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_ts = gen_ts(
                                    &IdSc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &PgCrudDefaultOptionSomeVecOneElCall
                                );
                                let ts = vec_syn_field.iter().map(|el_649e1691| {
                                    let field_ident = &el_649e1691.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_649e1691.field_type,
                                        &quote!{#CreateSc.#field_ident}
                                    )
                                });
                                quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_ts,
                                        #(#ts),*
                                    ]).expect("221a4c55")
                                }
                            },
                        );
                        quote! {
                            #AllowClippyArbitrarySourceItemOrdering
                            #[cfg(feature = "test-utils")]
                            impl #ident_with_id_standart_not_null_ucc {
                                #read_only_ids_merged_with_create_into_where_equal_ts
                                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts
                                #read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts
                            }
                        }
                    };
                    quote! {
                        #ident_with_id_standart_not_null_ts
                        #cfg_feature_test_utils_impl_ident_with_id_standart_not_null_ts
                    }
                }
                else {
                    Ts2::new()
                };
                quote! {
                    #ident_ts
                    #maybe_ident_with_id_standart_not_null_ts
                }
            };
            let ident_array_not_null_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&ident_array_not_null_ucc);
            let ident_with_id_array_not_null_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&ident_with_id_array_not_null_ucc);
            let pg_json_type_subtype_table_type_declaration = PgJsonTypeSubtype::TableTypeDeclaration;
            let pg_json_type_subtype_create = PgJsonTypeSubtype::Create;
            let pg_json_type_subtype_create_for_query = PgJsonTypeSubtype::CreateForQuery;
            let pg_json_type_subtype_select = PgJsonTypeSubtype::Select;
            let pg_json_type_subtype_where = PgJsonTypeSubtype::Where;
            let pg_json_type_subtype_read = PgJsonTypeSubtype::Read;
            let pg_json_type_subtype_read_inner = PgJsonTypeSubtype::ReadInner;
            let pg_json_type_subtype_update = PgJsonTypeSubtype::Update;
            let pg_json_type_subtype_update_for_query = PgJsonTypeSubtype::UpdateForQuery;
            let gen_type_as_pg_json_type_subtype_ts = |type_ts: &dyn ToTokens, pg_json_type_subtype: &PgJsonTypeSubtype| {
                let type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&type_ts);
                quote! {#type_as_pg_json_type_ts::#pg_json_type_subtype}
            };
            let gen_type_as_pg_type_subtype_ts = |type_ts: &dyn ToTokens, pg_type_subtype: &PgTypeSubtype| {
                let type_as_pg_type_ts = gen_type_as_pg_type_ts(&type_ts);
                quote! {#type_as_pg_type_ts::#pg_type_subtype}
            };
            let gen_field_type_as_crud_pg_json_type_from_field_ts = |
                syn_field_wrapper: &SynFieldWrapper
            | gen_type_as_pg_json_type_ts(
                &syn_field_wrapper.field_type
            );
            let gen_gen_impl_location_lib_to_err_string_wrapper_ts = |ident_ts_f6440ce9: &dyn ToTokens| gen_impl_to_err_string_ts(
                &Ts2::new(),
                &ident_ts_f6440ce9,
                &Ts2::new(),
                &quote! {format!("{self:?}")}
            );
            let ident_as_pg_json_type_table_type_declaration_ts = gen_type_as_pg_json_type_subtype_ts(&ident, &pg_json_type_subtype_table_type_declaration);
            let self_value_ts = quote! {Self(#ValueSc)};
            let pg_type_where_filter_query_bind_value_query_ts = quote!{#import_path::PgTypeWhereFilter::query_bind(#ValueSc, #QuerySc)};
            let ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident);
            let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
            let ident_array_not_null_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident_array_not_null_ucc);
            let ident_standart_not_null_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_with_id_standart_not_null_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let wrap_into_scopes_ts = |content: &dyn ToTokens| {
                quote! {(#content);}
            };
            let gen_ident_table_type_declaration_or_ident_create_common_ts = |pg_json_type_subtype_table_type_declaration_or_create: &PgJsonTypeSubtypeTableTypeDeclarationOrCreate| {
                let ident_table_type_declaration_or_ident_create_ucc: &dyn DisplayPlusToTokens = match &pg_json_type_subtype_table_type_declaration_or_create {
                    PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_table_type_declaration_ucc,
                    PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_create_ucc,
                };
                let gen_ident_table_type_declaration_or_create_ts = |
                    attrs_ts: &dyn ToTokens,
                    ident_ts_20954cb5: &dyn ToTokens,
                    ts: &dyn ToTokens
                | {
                    let ts_44f35e48 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
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
                let new_type_or_struct_declaration_struct_declaration = NewTypeOrStructDeclaration::StructDeclaration;
                let new_type_or_struct_declaration_new_type = NewTypeOrStructDeclaration::NewType;
                let gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_ts = |
                    is_standart_with_id: &IsStandartWithId,
                    pg_json_type_subtype_table_type_declaration_or_create_925a7214: &PgJsonTypeSubtypeTableTypeDeclarationOrCreate,
                    new_type_or_struct_declaration: &NewTypeOrStructDeclaration
                | {
                    let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_42f25108| {
                        let field_ident = &el_42f25108.field_ident;
                        let type_as_pg_json_type_subtype_table_type_declaration_ts = gen_type_as_pg_json_type_subtype_ts(
                            &el_42f25108.field_type,
                            &PgJsonTypeSubtype::from(pg_json_type_subtype_table_type_declaration_or_create_925a7214)
                        );
                        quote! {#field_ident: #type_as_pg_json_type_subtype_table_type_declaration_ts}
                    });
                    let fields_ts = quote! {#(#ts),*};
                    match &new_type_or_struct_declaration {
                        NewTypeOrStructDeclaration::StructDeclaration => quote! {{#fields_ts}},
                        NewTypeOrStructDeclaration::NewType => fields_ts,
                    }
                };
                let gen_tokens_table_type_declaration_or_create_ts = |tokens: &dyn ToTokens| {
                    let value: &dyn ToTokens = match &pg_json_type_subtype_table_type_declaration_or_create {
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &SelfTableTypeDeclarationUcc::from_tokens(&tokens),
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &SelfCreateUcc::from_tokens(&tokens),
                    };
                    quote!{#value}
                };
                let ident_table_type_declaration_or_ident_create_ts = gen_ident_table_type_declaration_or_create_ts(
                    &match &pg_json_type_subtype_table_type_declaration_or_create {
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => quote!{#AllowClippyArbitrarySourceItemOrdering},
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Ts2::new(),
                    },
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_ts(&is_standart_with_id_false, pg_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_struct_declaration),
                            IsNullable::True => wrap_into_scopes_ts(&gen_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(ident_standart_not_null_ucc))),
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => wrap_into_scopes_ts(&gen_vec_tokens_declaration_ts(
                                &gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_standart_not_null_ucc)
                            )),
                            IsNullable::True => wrap_into_scopes_ts(&gen_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_array_not_null_ucc))),
                        },
                    }
                );
                let gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_42f25108|&el_42f25108.field_ident);
                    quote! {Self {#(#ts),*}}
                };
                let impl_pub_new_for_ident_table_type_declaration_or_ident_create_ts = {
                    let parameters_ts = {
                        let gen_wrap_into_value_parameter_ts = |type_ts: &dyn ToTokens| {
                            quote! {value: #type_ts}
                        };
                        match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_ts(&is_standart_with_id_false, pg_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_new_type),
                                IsNullable::True => gen_wrap_into_value_parameter_ts(&gen_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(ident_standart_not_null_ucc))),
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => gen_wrap_into_value_parameter_ts(&gen_vec_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_standart_not_null_ucc))),
                                IsNullable::True => gen_wrap_into_value_parameter_ts(&gen_option_tokens_declaration_ts(&gen_vec_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(
                                    &ident_with_id_standart_not_null_ucc,
                                )))),
                            },
                        }
                    };
                    let ts = match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts(&is_standart_with_id_false),
                            IsNullable::True => self_value_ts.clone(),
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => self_value_ts.clone(),
                            IsNullable::True => {
                                let ident_array_not_null_with_id_postfix_ucc = gen_tokens_table_type_declaration_or_create_ts(&gen_ident_ucc(&IdentPattern::ArrayNotNullWithId));
                                quote! {Self(#ValueSc.map(#ident_array_not_null_with_id_postfix_ucc::new))}
                            }
                        },
                    };
                    if matches!(&pattern, Pattern::Array) && matches!(&is_nullable, IsNullable::True) {
                        gen_impl_pub_new_for_ident_ts(
                            &ident_table_type_declaration_or_ident_create_ucc,
                            &must_use_ts,
                            &parameters_ts,
                            &ts,
                        )
                    }
                    else {
                        gen_impl_pub_const_new_for_ident_ts(
                            &ident_table_type_declaration_or_ident_create_ucc,
                            &must_use_ts,
                            &parameters_ts,
                            &ts,
                        )
                    }
                };
                let gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts = |
                    ident_ts_36ce1199: &dyn ToTokens,
                    ts: &dyn ToTokens
                | gen_impl_pg_crud_default_option_some_vec_one_el_ts(
                    &ident_ts_36ce1199,
                    &Ts2::new(),
                    &quote! {Self #ts}
                );
                let gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_6c071492| {
                        let field_ident = &el_6c071492.field_ident;
                        quote! {#field_ident: #PgCrudDefaultOptionSomeVecOneElCall}
                    });
                    quote! {{
                        #(#ts),*
                    }}
                };
                let impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts = gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts(&is_standart_with_id_false);
                let scopes_vec_pg_crud_default_option_some_vec_one_el_call_ts = quote! {(#vec_pg_crud_default_option_some_vec_one_el_call_ts)};
                let scopes_some_pg_crud_default_option_some_vec_one_el_call_ts = quote! {
                    (Some(#PgCrudDefaultOptionSomeVecOneElCall))
                };
                let impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_ident_create_ts = gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    match &is_nullable {
                        IsNullable::False => match &pattern {
                            Pattern::Standart => &impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts,
                            Pattern::Array => &scopes_vec_pg_crud_default_option_some_vec_one_el_call_ts,
                        },
                        IsNullable::True => &scopes_some_pg_crud_default_option_some_vec_one_el_call_ts,
                    },
                );
                let impl_sqlx_encode_sqlx_pg_for_ident_table_type_declaration_or_ident_create_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &quote!{sqlx::types::Json(#SelfSc)}
                );
                let impl_sqlx_type_for_ident_table_type_declaration_or_ident_create_ts = gen_impl_sqlx_type_for_ident_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &quote!{sqlx::types::Json<#SelfUcc>}
                );
                let maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = if is_standart_not_null {
                    let ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc: &dyn DisplayPlusToTokens = match &pg_json_type_subtype_table_type_declaration_or_create {
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_with_id_standart_not_null_table_type_declaration_ucc,
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_with_id_standart_not_null_create_ucc,
                    };
                    let is_standart_with_id_240dfa72 = match &pg_json_type_subtype_table_type_declaration_or_create {
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &is_standart_with_id_true,
                        PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &is_standart_with_id_false,
                    };
                    let ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = gen_ident_table_type_declaration_or_create_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_ts(is_standart_with_id_240dfa72, pg_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_struct_declaration),
                    );
                    let impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &must_use_ts,
                        &gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_ts(is_standart_with_id_240dfa72, pg_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_new_type),
                        &gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts(is_standart_with_id_240dfa72),
                    );
                    let impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &match &pg_json_type_subtype_table_type_declaration_or_create {
                            PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts(&is_standart_with_id_true),
                            PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_ts,
                        },
                    );
                    quote! {
                        #ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                        #impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                        #impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_table_type_declaration_or_ident_create_ts
                    #impl_pub_new_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_pg_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_sqlx_encode_sqlx_pg_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_sqlx_type_for_ident_table_type_declaration_or_ident_create_ts
                    #maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                }
            };
            let ident_table_type_declaration_ts = {
                let ident_table_type_declaration_common_ts = gen_ident_table_type_declaration_or_ident_create_common_ts(&PgJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration);
                quote! {
                    #ident_table_type_declaration_common_ts
                }
            };
            let gen_type_as_pg_json_type_create_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_create);
            let gen_type_as_pg_json_type_create_for_query_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_create_for_query);
            let ident_create_ts = {
                let ident_create_common_ts = gen_ident_table_type_declaration_or_ident_create_common_ts(&PgJsonTypeSubtypeTableTypeDeclarationOrCreate::Create);
                let gen_impl_display_for_ident_create_ts = |ident_ts_39851a31: &dyn ToTokens| gen_impl_display_ts(
                    &Ts2::new(),
                    &ident_ts_39851a31,
                    &Ts2::new(),
                    &quote! {write!(f, "{self:?}")}
                );
                let impl_display_for_ident_create_ts = gen_impl_display_for_ident_create_ts(&ident_create_ucc);
                let impl_location_lib_to_err_string_for_ident_create_ts = gen_gen_impl_location_lib_to_err_string_wrapper_ts(&ident_create_ucc);
                let maybe_ident_with_id_create_standart_not_null_ts = if is_standart_not_null {
                    let impl_display_for_ident_with_id_create_standart_not_null_ts = gen_impl_display_for_ident_create_ts(&ident_with_id_standart_not_null_create_ucc);
                    let impl_location_lib_to_err_string_for_ident_with_id_create_standart_not_null_ts = gen_gen_impl_location_lib_to_err_string_wrapper_ts(&ident_with_id_standart_not_null_create_ucc);
                    quote! {
                        #impl_display_for_ident_with_id_create_standart_not_null_ts
                        #impl_location_lib_to_err_string_for_ident_with_id_create_standart_not_null_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_create_common_ts
                    #impl_display_for_ident_create_ts
                    #impl_location_lib_to_err_string_for_ident_create_ts
                    #maybe_ident_with_id_create_standart_not_null_ts
                }
            };
            let ident_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident);
            let self_as_pg_json_type_create_ts = gen_type_as_pg_json_type_create_ts(&SelfUcc);
            let ident_standart_not_null_as_pg_json_type_create_for_query_ts = gen_type_as_pg_json_type_create_for_query_ts(&ident_standart_not_null_ucc);
            let ident_array_not_null_as_pg_json_type_create_for_query_ts = gen_type_as_pg_json_type_create_for_query_ts(&ident_array_not_null_ucc);
            let ident_array_not_null_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&ident_array_not_null_ucc);
            let pg_crud_path_pg_json_type_uuid_uuid_create_for_query_ts = gen_type_as_pg_json_type_create_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let gen_debug_clone_partialeq_serialize_pub_struct_ts = |
                attrs_ts: &dyn ToTokens,
                ident_ts_d8fa00d3: &dyn ToTokens,
                ts_153ac202: &dyn ToTokens
            | {
                let ts_6ea2da58 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
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
            let ident_create_for_query_ts = {
                let gen_struct_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId|{
                    let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_53c802d8| {
                        let field_ident = &el_53c802d8.field_ident;
                        let type_as_pg_json_type_subtype_crate_for_query_ts = gen_type_as_pg_json_type_subtype_ts(
                            &el_53c802d8.field_type,
                            &PgJsonTypeSubtype::CreateForQuery
                        );
                        quote! {#field_ident: #type_as_pg_json_type_subtype_crate_for_query_ts}
                    });
                    quote! {{#(#ts),*}}
                };
                let impl_from_standart_not_null_without_id_ts = {
                    let ts = vec_syn_field.iter().map(|el_0fc1e145| {
                        let field_ident = &el_0fc1e145.field_ident;
                        let type_as_pg_json_type_subtype_crate_for_query_ts = gen_type_as_pg_json_type_subtype_ts(
                            &el_0fc1e145.field_type,
                            &PgJsonTypeSubtype::CreateForQuery
                        );
                        quote! {#field_ident: #type_as_pg_json_type_subtype_crate_for_query_ts::from(#ValueSc.#field_ident)}
                    });
                    quote! {#(#ts),*}
                };
                let ident_create_for_query_ts = {
                    let ident_create_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        &ident_create_for_query_ucc,
                        &match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => gen_struct_standart_not_null_ts(&is_standart_with_id_false),
                                IsNullable::True => {
                                    wrap_into_scopes_ts(
                                        &gen_option_tokens_declaration_ts(
                                            &gen_type_as_pg_json_type_subtype_ts(
                                                &ident_standart_not_null_ucc,
                                                &pg_json_type_subtype_create_for_query,
                                            )
                                        )
                                    )
                                },
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => wrap_into_scopes_ts(
                                    &gen_vec_tokens_declaration_ts(
                                        &ident_with_id_standart_not_null_create_for_query_ucc
                                    )
                                ),
                                IsNullable::True => wrap_into_scopes_ts(
                                    &gen_option_tokens_declaration_ts(
                                        &gen_type_as_pg_json_type_subtype_ts(
                                            &ident_array_not_null_ucc,
                                            &pg_json_type_subtype_create_for_query,
                                        )
                                    )
                                ),
                            },
                        }
                    );
                    let impl_from_ident_create_for_ident_create_for_query_ts = gen_impl_from_ts(
                        &ident_create_ucc,
                        &ident_create_for_query_ucc,
                        &{
                            let ts = match &is_nullable {
                                IsNullable::False => match &pattern {
                                    Pattern::Standart => quote! {{#impl_from_standart_not_null_without_id_ts}},
                                    Pattern::Array => quote!{(
                                        #ValueSc.0.into_iter().map(#ident_with_id_standart_not_null_create_for_query_ucc::from).collect()
                                    )},
                                },
                                IsNullable::True => {
                                    let ts: &dyn ToTokens = match &pattern {
                                        Pattern::Standart => &ident_standart_not_null_as_pg_json_type_create_for_query_ts,
                                        Pattern::Array => &ident_array_not_null_as_pg_json_type_create_for_query_ts,
                                    };
                                    quote!{(#ValueSc.0.map(#ts::from))}
                                },
                            };
                            quote! {Self #ts}
                        }
                    );
                    let impl_sqlx_encode_sqlx_pg_for_ident_create_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                        &ident_create_for_query_ucc,
                        &quote!{sqlx::types::Json(#SelfSc)}
                    );
                    let impl_sqlx_type_for_ident_create_for_query_ts = gen_impl_sqlx_type_for_ident_ts(
                        &ident_create_for_query_ucc,
                        &quote!{sqlx::types::Json<#SelfUcc>}
                    );
                    quote! {
                        #ident_create_for_query_ts
                        #impl_from_ident_create_for_ident_create_for_query_ts
                        #impl_sqlx_encode_sqlx_pg_for_ident_create_for_query_ts
                        #impl_sqlx_type_for_ident_create_for_query_ts
                    }
                };
                let maybe_ident_with_id_standart_not_null_create_for_query_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_create_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        &ident_with_id_standart_not_null_create_for_query_ucc,
                        &gen_struct_standart_not_null_ts(&is_standart_with_id_true)
                    );
                    let impl_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_ts = gen_impl_from_ts(
                        &ident_with_id_standart_not_null_create_ucc,
                        &ident_with_id_standart_not_null_create_for_query_ucc,
                        &quote! {Self {
                            #IdSc: #pg_crud_path_pg_json_type_uuid_uuid_create_for_query_ts::new(
                                uuid::Uuid::new_v4()
                            ),
                            #impl_from_standart_not_null_without_id_ts
                        }}
                    );
                    quote! {
                        #ident_with_id_standart_not_null_create_for_query_ts
                        #impl_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_create_for_query_ts
                    #maybe_ident_with_id_standart_not_null_create_for_query_ts
                }
            };
            let gen_sqlx_types_json_type_declaration_wrapper_ts = |ident_ts_dff2e4a1: &dyn ToTokens| gen_impl_sqlx_type_for_ident_ts(
                &ident_ts_dff2e4a1,
                &gen_sqlx_types_json_type_declaration_ts(&SelfUcc)
            );
            let gen_impl_sqlx_decode_sqlx_pg_for_ident_wrapper_ts = |ident_ts_65e11453: &dyn ToTokens| gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_ts_65e11453,
                &gen_sqlx_types_json_type_declaration_ts(&SelfUcc),
                &quote! {Ok(v.0)}
            );
            let gen_value_type_ts = |type_ts: &dyn ToTokens| {
                quote! {#ValueSc: #type_ts}
            };
            let gen_pub_const_new_value_type_content_self_value_ts = |ts: &dyn ToTokens|gen_pub_const_new_ts(
                &must_use_ts,
                &gen_value_type_ts(&ts),
                &self_value_ts
            );
            let gen_unique_vec_wrapper_ts = |type_ts: &dyn ToTokens| {
                quote! {#import_path::NotEmptyUniqueVec<#type_ts>}
            };
            let self_some_pg_crud_default_option_some_vec_one_el_call_ts = quote! {
                Self(Some(#PgCrudDefaultOptionSomeVecOneElCall))
            };
            let self_some_pg_crud_default_option_some_vec_one_el_max_page_size_call_ts = quote! {
                Self(Some(#PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall))
            };
            let wrap_content_into_scopes_dot_comma_ts = |ts: &dyn ToTokens| {
                let scopes_ts = wrap_content_into_scopes_ts(&ts);
                quote! {#scopes_ts;}
            };
            let gen_type_as_pg_json_type_update_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_update);
            let gen_type_as_pg_json_type_update_for_query_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_update_for_query);
            let self_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&SelfUcc);
            let self_as_pg_json_type_update_ts = gen_type_as_pg_json_type_update_ts(&SelfUcc);
            let self_as_pg_json_type_create_for_query_ts = gen_type_as_pg_json_type_create_for_query_ts(&SelfUcc);
            let pg_crud_path_pg_json_type_uuid_uuid_update_ts = gen_type_as_pg_json_type_update_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let pg_crud_path_pg_json_type_uuid_uuid_update_for_query_ts = gen_type_as_pg_json_type_update_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let gen_type_as_pg_json_type_select_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_select);
            let ident_standart_not_null_as_pg_json_type_select_ts = gen_type_as_pg_json_type_select_ts(&ident_standart_not_null_ucc);
            let ident_with_id_array_not_null_as_pg_json_type_select_ts = gen_type_as_pg_json_type_select_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_standart_not_null_select_sc = SelfSelectSc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let dim1_pagination_ts = quote! {dim1_pagination};
            let ident_standart_not_null_select_el_ucc = SelfSelectElementUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_with_id_standart_not_null_select_el_ucc = SelfSelectElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let gen_select_query_part_for_loop_ts = |
                acc_ts: &dyn ToTokens,
                is_standart_with_id: &IsStandartWithId,
                in_ts: &dyn ToTokens,
                column_name_and_maybe_field_getter_field_ident_ts: &dyn ToTokens,
                column_name_and_maybe_field_getter_for_er_message_field_ident_ts: &dyn ToTokens,
            |{
                let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_f3a1af0f| {
                    let field_ident_str = el_f3a1af0f.field_ident.to_string();
                    let vrt_name_ts: &dyn ToTokens = &AsRefStrToUccTs::case_or_panic(&field_ident_str);
                    let field_ident_dq_ts: &dyn ToTokens = &dq_ts(&field_ident_str);
                    let field_type_as_crud_pg_json_type_from_field_ts = gen_type_as_pg_json_type_ts(&el_f3a1af0f.field_type);
                    let ident_or_ident_with_id_standart_not_null_select_el_ucc: &dyn ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_standart_not_null_select_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_el_ucc,
                    };
                    quote! {
                        #ident_or_ident_with_id_standart_not_null_select_el_ucc::#vrt_name_ts(v_3c8acf6a) => match #field_type_as_crud_pg_json_type_from_field_ts::#SelectQueryPartSc(
                            v_3c8acf6a,
                            #field_ident_dq_ts,
                            #column_name_and_maybe_field_getter_field_ident_ts,
                            #column_name_and_maybe_field_getter_for_er_message_field_ident_ts,
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
                        match el_0127bf54 {
                            #(#ts),*
                        }
                    },
                    &return_err_query_part_er_write_into_buffer_ts
                );
                quote!{
                    for el_0127bf54 in #in_ts #self_field_vec_ts {
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
                    let ts_83d3ad18 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
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
                let gen_ident_select_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_standart_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_standart_not_null_ucc);
                    gen_pub_struct_ident_select_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        match &is_standart_with_id {
                            IsStandartWithId::False => &ident_standart_not_null_select_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_select_ucc,
                        },
                        &wrap_content_into_scopes_dot_comma_ts(&gen_unique_vec_wrapper_ts(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_standart_not_null_select_el_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_select_el_ucc,
                        })),
                    )
                };
                let import_path_pagination_ts = quote! {#import_path::PaginationStartsWithZero};
                let ident_select_ts = match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Standart => gen_ident_select_standart_not_null_ts(&is_standart_with_id_false),
                        Pattern::Array => gen_pub_struct_ident_select_ts(
                            &AllowClippyArbitrarySourceItemOrdering,
                            &ident_select_ucc,
                            &quote! {{
                                #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_ucc,
                                #dim1_pagination_ts: #import_path_pagination_ts
                            }},
                        ),
                    },
                    IsNullable::True => gen_pub_struct_ident_select_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        &ident_select_ucc,
                        &wrap_content_into_scopes_dot_comma_ts(&gen_option_tokens_declaration_ts(&match &pattern {
                            Pattern::Standart => &ident_standart_not_null_as_pg_json_type_select_ts,
                            Pattern::Array => &ident_with_id_array_not_null_as_pg_json_type_select_ts,
                        })),
                    ),
                };
                let impl_ident_select_ts = {
                    let pub_new_ts = {
                        let parameters_ts = {
                            let unique_vec_ident_select_el_standart_not_null_ts = gen_unique_vec_wrapper_ts(&ident_standart_not_null_select_el_ucc);
                            match &pattern {
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => gen_value_type_ts(&unique_vec_ident_select_el_standart_not_null_ts),
                                    IsNullable::True => gen_value_type_ts(&gen_option_tokens_declaration_ts(&unique_vec_ident_select_el_standart_not_null_ts)),
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => quote! {
                                        #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_ucc,
                                        #dim1_pagination_ts: #import_path_pagination_ts
                                    },
                                    IsNullable::True => gen_value_type_ts(&gen_option_tokens_declaration_ts(&ident_with_id_array_not_null_as_pg_json_type_select_ts)),
                                },
                            }
                        };
                        let ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => self_value_ts.clone(),
                                IsNullable::True => quote! {
                                    Self(#ValueSc.map(#ident_standart_not_null_as_pg_json_type_select_ts::new))
                                },
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    quote! {Self {
                                        #ident_with_id_standart_not_null_select_sc,
                                        #dim1_pagination_ts,
                                    }}
                                }
                                IsNullable::True => self_value_ts.clone(),
                            },
                        };
                        if matches!(&pattern, Pattern::Standart) && matches!(&is_nullable, IsNullable::True) {
                            gen_pub_new_ts(
                                &must_use_ts,
                                &parameters_ts,
                                &ts
                            )
                        }
                        else {
                             gen_pub_const_new_ts(
                                &must_use_ts,
                                &parameters_ts,
                                &ts
                            )
                        }
                    };
                    let maybe_select_query_part_ts = if matches!(&pattern, Pattern::Standart) &&
                    matches!(&is_nullable, IsNullable::False) {
                        let acc_ac57d097_ts = quote!{acc_ac57d097};
                        let select_query_part_for_loop_ts = gen_select_query_part_for_loop_ts(
                            &acc_ac57d097_ts,
                            &is_standart_with_id_false,
                            &SelfSc,
                            &quote!{column_name_and_maybe_field_getter},
                            &quote!{column_name_and_maybe_field_getter_for_er_message},
                        );
                        quote! {
                            fn #SelectQueryPartSc(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                column_name_and_maybe_field_getter_for_er_message: &str,
                            ) -> Result<#StringTs, #import_path_query_part_er_ts> {
                                let mut #acc_ac57d097_ts = #StringTs::default();
                                #select_query_part_for_loop_ts
                                let _: Option<char> = #acc_ac57d097_ts.pop();
                                let _: Option<char> = #acc_ac57d097_ts.pop();
                                Ok(#acc_ac57d097_ts)
                            }
                        }
                    }
                    else {
                        Ts2::new()
                    };
                    let select_query_part_pg_type_ts = {
                        let ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => quote! {
                                    #SelfSc.#SelectQueryPartSc(
                                        #ColumnSc,
                                        #ColumnSc,
                                    )
                                },
                                IsNullable::True => {
                                    let ident_740c9034 = match &pattern {
                                        Pattern::Standart => &ident_standart_not_null_as_pg_json_type_select_ts,
                                        Pattern::Array => &ident_with_id_array_not_null_as_pg_json_type_select_ts,
                                    };
                                    quote! {
                                        let #ValueSc = self.0.as_ref().map_or_else(
                                            <#ident_740c9034 as pg_crud::DefaultOptionSomeVecOneEl>::default_option_some_vec_one_el,
                                            Clone::clone
                                        );
                                        match #ValueSc.#SelectQueryPartPgTypeSc(#ColumnSc) {
                                            Ok(v_c69f1ffe) => Ok(format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({v_c69f1ffe}) end")),
                                            Err(#ErSc) => Err(#ErSc)
                                        }
                                    }
                                },
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let acc_399d9786_ts = quote!{acc_399d9786};
                                    let select_query_part_for_loop_ts = gen_select_query_part_for_loop_ts(
                                        &acc_399d9786_ts,
                                        &is_standart_with_id_true,
                                        &quote!{#SelfSc.#ident_with_id_standart_not_null_select_sc},
                                        &dq_ts(&ValueSc),
                                        &ColumnSc
                                    );
                                    let format_handle_ts = dq_ts(&format!(
                                        "(case when (jsonb_array_length({{column}}) = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_sc}}})) from jsonb_array_elements((select {{column}})) with ordinality where ordinality between {{dim1_start}} and {{dim1_end}}) end)"
                                    ));
                                    quote! {
                                        let #ident_with_id_standart_not_null_select_sc = {
                                            let mut #acc_399d9786_ts = #StringTs::default();
                                            #select_query_part_for_loop_ts
                                            let _: Option<char> = #acc_399d9786_ts.pop();
                                            let _: Option<char> = #acc_399d9786_ts.pop();
                                            #acc_399d9786_ts
                                        };
                                        let dim1_start = self.#dim1_pagination_ts.start();
                                        let dim1_end = self.#dim1_pagination_ts.end();
                                        Ok(format!(#format_handle_ts))
                                    }
                                }
                                IsNullable::True => {
                                    let format_handle_ts = dq_ts(&"case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({v_c2ca032e}) end");
                                    let ident_with_id_array_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                        &ident_with_id_array_not_null_as_pg_json_type_select_ts
                                    );
                                    quote! {
                                        let #ValueSc = self.0.as_ref().map_or_else(
                                            #ident_with_id_array_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts,
                                            Clone::clone
                                        );
                                        match #ValueSc.#SelectQueryPartPgTypeSc(column) {
                                            Ok(v_c2ca032e) => Ok(format!(#format_handle_ts)),
                                            Err(#ErSc) => Err(#ErSc)
                                        }
                                    }
                                },
                            },
                        };
                        quote! {
                            fn #SelectQueryPartPgTypeSc(
                                &self,
                                #ColumnSc: &str,
                            ) -> Result<#StringTs, #import_path_query_part_er_ts> {
                                #ts
                            }
                        }
                    };
                    quote! {
                        impl #ident_select_ucc {
                            #pub_new_ts
                            #maybe_select_query_part_ts
                            #select_query_part_pg_type_ts
                        }
                    }
                };
                let impl_sqlx_type_for_ident_select_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_select_ucc);
                let impl_sqlx_decode_sqlx_pg_for_ident_select_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_wrapper_ts(&ident_select_ucc);
                let impl_pg_crud_default_option_some_vec_one_el_standart_not_null_ts = quote! {
                    Self(#PgCrudDefaultOptionSomeVecOneElCall)
                };
                let impl_pg_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_ts = quote! {
                    Self(#PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall)
                };
                let (
                    impl_pg_crud_default_option_some_vec_one_el_for_ident_select_ts,
                    impl_pg_crud_default_option_some_vec_one_el_max_page_size_for_ident_select_ts
                ) = {
                    let gen_default_some_one_ts = |default_some_one_or_default_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                        match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => match &default_some_one_or_default_some_one_with_max_page_size {
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => quote! {#impl_pg_crud_default_option_some_vec_one_el_standart_not_null_ts},
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => quote! {#impl_pg_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_ts},
                                },
                                IsNullable::True => match &default_some_one_or_default_some_one_with_max_page_size {
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_pg_crud_default_option_some_vec_one_el_call_ts.clone(),
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_pg_crud_default_option_some_vec_one_el_max_page_size_call_ts.clone(),
                                },
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let (
                                        ident_with_id_standart_not_null_select_ts,
                                        dim1_pagination_ts_364caa54
                                    ): (
                                        &dyn ToTokens,
                                        &dyn ToTokens
                                    ) = match &default_some_one_or_default_some_one_with_max_page_size {
                                        DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => (
                                            &PgCrudDefaultOptionSomeVecOneElCall,
                                            &PgCrudDefaultOptionSomeVecOneElCall
                                        ),
                                        DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => (
                                            &PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall,
                                            &PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall
                                        ),
                                    };
                                    quote! {
                                        Self {
                                            #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_ts,
                                            #dim1_pagination_ts: #dim1_pagination_ts_364caa54,
                                        }
                                    }
                                },
                                IsNullable::True => match &default_some_one_or_default_some_one_with_max_page_size {
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_pg_crud_default_option_some_vec_one_el_call_ts.clone(),
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_pg_crud_default_option_some_vec_one_el_max_page_size_call_ts.clone(),
                                },
                            },
                        }
                    };
                    (
                        gen_impl_pg_crud_default_option_some_vec_one_el_ts(
                            &ident_select_ucc,
                            &Ts2::new(),
                            &gen_default_some_one_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                        ),
                        gen_impl_pg_crud_default_option_some_vec_one_el_max_page_size_ts(
                            &ident_select_ucc,
                            &Ts2::new(),
                            &gen_default_some_one_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                        )
                    )
                };
                let gen_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_select_el_or_ident_with_id_select_el_ucc: &dyn ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_standart_not_null_select_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_el_ucc,
                    };
                    let ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = {
                        let ts_bf3bd19e = StructOrEnumDeriveTsStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_enum(
                            &ident_select_el_or_ident_with_id_select_el_ucc,
                            &Ts2::new(),
                            &{
                                let ts_ecc4a666 = get_vec_syn_field(is_standart_with_id).iter().map(|el_840c2253| {
                                    let field_ident = &el_840c2253.field_ident;
                                    let serde_field_ident_dq_ts = dq_ts(&field_ident);
                                    let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_json_type_select_ts = gen_type_as_pg_json_type_select_ts(&el_840c2253.field_type);
                                    quote! {
                                        #[serde(rename(serialize = #serde_field_ident_dq_ts, deserialize = #serde_field_ident_dq_ts))]
                                        #vrt_ident_ucc_ts(#field_type_as_json_type_select_ts)
                                    }
                                });
                                quote!{{#(#ts_ecc4a666),*}}
                            }
                        );
                        quote!{
                            #AllowClippyArbitrarySourceItemOrdering
                            #ts_bf3bd19e
                        }
                    };
                    let impl_location_lib_to_err_string_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = gen_gen_impl_location_lib_to_err_string_wrapper_ts(&ident_select_el_or_ident_with_id_select_el_ucc);
                    let (
                        impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts,
                        impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_with_max_page_size_ts
                    ) = {
                        let gen_default_some_one_ts = |default_some_one_or_default_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                            let vec_ts = {
                                let ts: &dyn ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudDefaultOptionSomeVecOneElCall,
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall,
                                };
                                let elements_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_20fc16d2| {
                                    let field_ident = &el_20fc16d2.field_ident;
                                    let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    quote! {#SelfUcc::#field_ident_ucc_ts(#ts)}
                                });
                                quote! {#(#elements_ts),*}
                            };
                            quote! {vec![
                                #vec_ts
                            ]}
                        };
                        (
                            gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts(
                                &ident_select_el_or_ident_with_id_select_el_ucc,
                                &gen_default_some_one_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                            ),
                            gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_max_page_size_ts(
                                &ident_select_el_or_ident_with_id_select_el_ucc,
                                &gen_default_some_one_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                            )
                        )
                    };
                    quote! {
                        #ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_location_lib_to_err_string_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_with_max_page_size_ts
                    }
                };
                let maybe_ident_select_el_ts = if is_standart_not_null { gen_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts(&is_standart_with_id_false) } else { Ts2::new() };
                let maybe_ident_with_id_standart_not_null_select_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_select_ts = gen_ident_select_standart_not_null_ts(&is_standart_with_id_true);
                    let impl_ident_with_id_standart_not_null_select_ts = {
                        let impl_new_for_ident_with_id_standart_not_null_select_ts = gen_pub_const_new_value_type_content_self_value_ts(
                            &gen_unique_vec_wrapper_ts(
                                &ident_with_id_standart_not_null_select_el_ucc
                            )
                        );
                        quote!{
                            impl #ident_with_id_standart_not_null_select_ucc {
                                #impl_new_for_ident_with_id_standart_not_null_select_ts
                            }
                        }
                    };
                    let impl_sqlx_type_for_ident_with_id_standart_not_null_select_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_with_id_standart_not_null_select_ucc);
                    let impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_select_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_wrapper_ts(&ident_with_id_standart_not_null_select_ucc);
                    let impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_select_ts = gen_impl_pg_crud_default_option_some_vec_one_el_ts(
                        &ident_with_id_standart_not_null_select_ucc,
                        &Ts2::new(),
                        &impl_pg_crud_default_option_some_vec_one_el_standart_not_null_ts
                    );
                    let impl_pg_crud_default_option_some_vec_one_el_max_page_size_for_ident_with_id_standart_not_null_select_ts = gen_impl_pg_crud_default_option_some_vec_one_el_max_page_size_ts(
                        &ident_with_id_standart_not_null_select_ucc,
                        &Ts2::new(),
                        &impl_pg_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_ts
                    );
                    let ident_with_id_select_el_ts = gen_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts(&is_standart_with_id_true);
                    quote! {
                        #ident_with_id_standart_not_null_select_ts
                        #impl_ident_with_id_standart_not_null_select_ts
                        #impl_sqlx_type_for_ident_with_id_standart_not_null_select_ts
                        #impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_select_ts
                        #impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_select_ts
                        #impl_pg_crud_default_option_some_vec_one_el_max_page_size_for_ident_with_id_standart_not_null_select_ts
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
                    #impl_pg_crud_default_option_some_vec_one_el_for_ident_select_ts
                    #impl_pg_crud_default_option_some_vec_one_el_max_page_size_for_ident_select_ts
                    #maybe_ident_select_el_ts
                    #maybe_ident_with_id_standart_not_null_select_ts
                }
            };
            let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
            let ident_where_ts = match &is_nullable {
                IsNullable::False => {
                    let gen_ident_where_field_vrts_ts = |is_standart_with_id: &IsStandartWithId| {
                        let vrts_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_622d1e96| {
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_622d1e96.field_ident.to_string());
                            let field_type_as_json_type_where_ts = gen_type_as_pg_json_type_subtype_ts(
                                &el_622d1e96.field_type,
                                &pg_json_type_subtype_where
                            );
                            quote! {
                                #field_ident_ucc_ts(#import_path::PgTypeWhere<
                                    #field_type_as_json_type_where_ts
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
                        let ts_60d5d187 = StructOrEnumDeriveTsStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
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
                    let equal_vrt_ident_ts = quote! {#EqualUcc(#import_path::PgJsonTypeWhereEqual<#ident_as_pg_json_type_table_type_declaration_ts>)};
                    let equal_vrt_query_part_ts = quote!{
                        #SelfUcc::#EqualUcc(#ValueSc) => #import_path::PgTypeWhereFilter::query_part(
                            #ValueSc,
                            #IncrSc,
                            &#ColumnSc,
                            is_need_to_add_logical_operator
                        )
                    };
                    let equal_vrt_query_bind_ts = quote!{
                        #SelfUcc::#EqualUcc(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts
                    };
                    let maybe_ident_where_ts = {
                        let gen_ident_where_wrapper_ts = |ts: &dyn ToTokens| gen_ident_where_ts(
                            &AllowClippyArbitrarySourceItemOrdering,
                            &ident_where_ucc,
                            &ts
                        );
                        match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Standart => gen_ident_where_wrapper_ts(&{
                                    let ident_where_field_vrts_ts = gen_ident_where_field_vrts_ts(&is_standart_with_id_false);
                                    quote!{
                                        #ident_where_field_vrts_ts,
                                        #equal_vrt_ident_ts,
                                    }
                                }),
                                Pattern::Array => gen_ident_where_wrapper_ts(&{
                                    let dim_one_equal_ts = quote! {
                                        DimOneEqual(#import_path::PgJsonTypeWhereDimOneEqual<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let length_equal_ts = quote! {
                                        LengthEqual(#import_path::PgJsonTypeWhereLengthEqual),
                                    };
                                    let length_greater_than_ts = quote! {
                                        LengthGreaterThan(#import_path::PgJsonTypeWhereLengthGreaterThan),
                                    };
                                    let in_ts = quote! {
                                        In(#import_path::PgJsonTypeWhereIn<#ident_as_pg_json_type_table_type_declaration_ts>),
                                    };
                                    let dim_one_in_ts = quote! {
                                        DimOneIn(#import_path::PgJsonTypeWhereDimOneIn<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let contains_all_elements_of_array_ts = quote! {
                                        ContainsAllElementsOfArray(#import_path::PgJsonTypeWhereContainsAllElementsOfArray<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let overlaps_with_array_ts = quote! {
                                        OverlapsWithArray(#import_path::PgJsonTypeWhereOverlapsWithArray<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_3e7f45d9| {
                                        let field_ident = &el_3e7f45d9.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let el_type_as_pg_json_type_where_ts = gen_type_as_pg_json_type_subtype_ts(
                                            &el_3e7f45d9.field_type,
                                            &PgJsonTypeSubtype::Where
                                        );
                                        quote! {
                                            #el_field_ident_ucc(#import_path::PgTypeWhere<
                                                #el_type_as_pg_json_type_where_ts
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
                                        #contains_all_elements_of_array_ts
                                        #overlaps_with_array_ts
                                        #(#el_filters_ts),*
                                    }
                                }),
                            },
                            IsNullable::True => Ts2::new(),
                        }
                    };
                    let gen_where_filter_query_part_fields_content_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                        let query_part_vrts_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_32d414b1| {
                            let field_ident_str = el_32d414b1.field_ident.to_string();
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&field_ident_str);
                            let format_handle_ts = dq_ts(&format!("{{column}}->'{field_ident_str}'"));
                            quote! {
                                Self::#field_ident_ucc_ts(value) => #import_path::PgTypeWhereFilter::#QueryPartSc(
                                    value,
                                    incr,
                                    &format!(#format_handle_ts),
                                    is_need_to_add_logical_operator,
                                )
                            }
                        });
                        quote! {#(#query_part_vrts_ts),*}
                    };
                    let gen_where_filter_query_bind_fields_content_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                        let query_bind_vrts_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_ee6a2665| {
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_ee6a2665.field_ident.to_string());
                            quote! {
                                Self::#field_ident_ucc_ts(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts
                            }
                        });
                        quote! {#(#query_bind_vrts_ts),*}
                    };
                    let gen_impl_pg_type_where_filter_ts = |
                        ident_ts_e0f20014: &dyn ToTokens,
                        query_part_ts: &dyn ToTokens,
                        is_query_bind_mutable: IsQueryBindMutable,
                        query_bind_ts: &dyn ToTokens
                    | {
                        impl_pg_type_where_filter_for_ident_ts(
                            &quote! {<'lifetime>},
                            &ident_ts_e0f20014,
                            &Ts2::new(),
                            &IncrParameterUnderscore::False,
                            &ColumnParameterUnderscore::False,
                            &IsNeedToAddLogicalOperatorUnderscore::False,
                            &query_part_ts,
                            &is_query_bind_mutable,
                            &query_bind_ts,
                            &ImportPath::PgCrud,
                        )
                    };
                    let maybe_impl_pg_crud_pg_type_pg_type_where_filter_for_ident_where_ts = {
                        let gen_impl_pg_type_where_filter_for_ident_ts = |query_part_ts: &dyn ToTokens, is_query_bind_mutable: IsQueryBindMutable, query_bind_ts: &dyn ToTokens| gen_impl_pg_type_where_filter_ts(&ident_where_ucc, &query_part_ts, is_query_bind_mutable, &query_bind_ts);
                        match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => gen_impl_pg_type_where_filter_for_ident_ts(
                                    &{
                                        let fields_ts = gen_where_filter_query_part_fields_content_standart_not_null_ts(&is_standart_with_id_false);
                                        quote!{
                                            match &self {
                                                #fields_ts,
                                                #equal_vrt_query_part_ts,
                                            }
                                        }
                                    },
                                    IsQueryBindMutable::False,
                                    &{
                                        let fields_ts = gen_where_filter_query_bind_fields_content_standart_not_null_ts(&is_standart_with_id_false);
                                        quote!{
                                            match self {
                                                #fields_ts,
                                                #equal_vrt_query_bind_ts,
                                            }
                                        }
                                    }
                                ),
                                IsNullable::True => Ts2::new(),
                            },
                            Pattern::Array => gen_impl_pg_type_where_filter_for_ident_ts(
                                &{
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_7845d48a| {
                                        let field_ident = &el_7845d48a.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let field_ident_dq_ts = dq_ts(&field_ident);
                                        quote! {
                                            Self::#el_field_ident_ucc(#ValueSc) => gen_el_query(
                                                #ValueSc.get_logical_operator(),
                                                #ValueSc,
                                                #field_ident_dq_ts
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
                                        quote!{#ContainsAllElementsOfArrayUcc},
                                        quote!{#OverlapsWithArrayUcc}
                                    ].into_iter().map(|el_ts|quote!{
                                        Self::#el_ts(#ValueSc) => #import_path::PgTypeWhereFilter::#QueryPartSc(
                                            #ValueSc,
                                            #IncrSc,
                                            #ColumnSc,
                                            #IsNeedToAddLogicalOperatorSc
                                        ),
                                    });
                                    quote! {
                                        let mut gen_el_query = |
                                            logical_operator: &#import_path::LogicalOperator,
                                            #ValueSc: &dyn #import_path::PgTypeWhereFilter<'_>,
                                            field: &str
                                        | -> Result<#StringTs, #import_path_query_part_er_ts> {
                                            let logical_operator_query_part = logical_operator.to_query_part(is_need_to_add_logical_operator);
                                            let elem = "elem";
                                            let v_9696ee60 = match #import_path::PgTypeWhereFilter::#QueryPartSc(
                                                #ValueSc,
                                                #IncrSc,
                                                &format!("{elem}->'{field}'"),
                                                false
                                            ) {
                                                Ok(v_c7ec4e53) => v_c7ec4e53,
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            };
                                            Ok(format!("{logical_operator_query_part}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {v_9696ee60}))"))
                                        };
                                        match &self {
                                            #(#concrete_filters_ts)*
                                            #(#el_filters_ts),*
                                        }
                                    }
                                },
                                IsQueryBindMutable::False,
                                &{
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_9956277c| {
                                        let field_ident = &el_9956277c.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        quote! {Self::#el_field_ident_ucc(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts}
                                    });
                                    quote! {
                                        match self {
                                            Self::Equal(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::DimOneEqual(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::LengthEqual(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::LengthGreaterThan(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::In(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::DimOneIn(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::ContainsAllElementsOfArray(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            Self::OverlapsWithArray(#ValueSc) => #pg_type_where_filter_query_bind_value_query_ts,
                                            #(#el_filters_ts),*
                                        }
                                    }
                                },
                            ),
                        }
                    };
                    let maybe_impl_location_lib_to_err_string_for_ident_where_ts = if matches!((&pattern, &is_nullable), (Pattern::Standart, IsNullable::True)) {
                        Ts2::new()
                    } else {
                        gen_gen_impl_location_lib_to_err_string_wrapper_ts(&ident_where_ucc)
                    };
                    let gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_content_standart_not_null_where = |is_standart_with_id: &IsStandartWithId| {
                        let gen_self_vrt_default_some_one_ts = |ts: &dyn ToTokens|quote!{
                            Self::#ts(#PgCrudDefaultOptionSomeVecOneElCall)
                        };
                        let vrts_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_7f3524bc| {
                            gen_self_vrt_default_some_one_ts(&AsRefStrToUccTs::case_or_panic(&el_7f3524bc.field_ident.to_string()))
                        });
                        let self_equal_default_some_one_ts = gen_self_vrt_default_some_one_ts(&EqualUcc);
                        quote! {vec![
                            #(#vrts_ts),*,
                            #self_equal_default_some_one_ts
                        ]}
                    };
                    let maybe_impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_where_ts = match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts(&ident_where_ucc, &gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_content_standart_not_null_where(&is_standart_with_id_false)),
                            IsNullable::True => Ts2::new(),
                        },
                        Pattern::Array => gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts(&ident_where_ucc, &{
                            let el_filters_ts = vec_syn_field_with_id.iter().map(|el_a3184731| {
                                let field_ident = &el_a3184731.field_ident;
                                let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                quote! {Self::#el_field_ident_ucc(#default_but_option_is_some_call_ts)}
                            });
                            quote! {
                                vec![
                                    Self::Equal(#default_but_option_is_some_call_ts),
                                    Self::DimOneEqual(#default_but_option_is_some_call_ts),
                                    Self::LengthEqual(#default_but_option_is_some_call_ts),
                                    Self::LengthGreaterThan(#default_but_option_is_some_call_ts),
                                    Self::In(#default_but_option_is_some_call_ts),
                                    Self::DimOneIn(#default_but_option_is_some_call_ts),
                                    Self::ContainsAllElementsOfArray(#default_but_option_is_some_call_ts),
                                    Self::OverlapsWithArray(#default_but_option_is_some_call_ts),
                                    #(#el_filters_ts),*
                                ]
                            }
                        }),
                    };
                    let maybe_ident_with_id_standart_not_null_where_ts = if is_standart_not_null {
                        let ident_with_id_standart_not_null_where_ts = gen_ident_where_ts(
                            &AllowClippyArbitrarySourceItemOrdering,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let ident_where_field_vrts_ts = gen_ident_where_field_vrts_ts(&is_standart_with_id_true);
                                quote!{
                                    #ident_where_field_vrts_ts,
                                    #EqualUcc(#import_path::PgJsonTypeWhereEqual<#ident_with_id_standart_not_null_table_type_declaration_ucc>),//todo maybe reuse? vrt generation
                                }
                            }
                        );
                        let impl_pg_crud_pg_type_pg_type_where_filter_for_ident_with_id_standart_not_null_where_ts = gen_impl_pg_type_where_filter_ts(
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let fields_ts = gen_where_filter_query_part_fields_content_standart_not_null_ts(&is_standart_with_id_true);
                                quote!{
                                    match &self {
                                        #fields_ts,
                                        Self::#EqualUcc(#ValueSc) => pg_crud::PgTypeWhereFilter::query_part(
                                            #ValueSc,
                                            #IncrSc,
                                            &#ColumnSc,
                                            is_need_to_add_logical_operator
                                        ),//todo maybe reuse? vrt generation
                                    }
                                }
                            },
                            IsQueryBindMutable::False,
                            &{
                                let fields_ts = gen_where_filter_query_bind_fields_content_standart_not_null_ts(&is_standart_with_id_true);
                                quote!{
                                    match self {
                                        #fields_ts,
                                        Self::#EqualUcc(#ValueSc) => pg_crud::PgTypeWhereFilter::query_bind(#ValueSc, #QuerySc),//todo maybe reuse? vrt generation
                                    }
                                }
                            },
                        );
                        let impl_location_lib_to_err_string_for_ident_with_id_standart_not_null_where_ts = gen_gen_impl_location_lib_to_err_string_wrapper_ts(&ident_with_id_standart_not_null_where_ucc);
                        let impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_where_ts = gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts(
                            &ident_with_id_standart_not_null_where_ucc,
                            &gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_content_standart_not_null_where(&is_standart_with_id_true)
                        );
                        quote! {
                            #ident_with_id_standart_not_null_where_ts
                            #impl_pg_crud_pg_type_pg_type_where_filter_for_ident_with_id_standart_not_null_where_ts
                            #impl_location_lib_to_err_string_for_ident_with_id_standart_not_null_where_ts
                            #impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_where_ts
                        }
                    } else {
                        Ts2::new()
                    };
                    quote! {
                        #maybe_ident_where_ts
                        #maybe_impl_pg_crud_pg_type_pg_type_where_filter_for_ident_where_ts
                        #maybe_impl_location_lib_to_err_string_for_ident_where_ts
                        #maybe_impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_where_ts
                        #maybe_ident_with_id_standart_not_null_where_ts
                    }
                }
                IsNullable::True => {
                    let ident_standart_or_ident_with_id_array_as_pg_json_type_where_ts = gen_type_as_pg_json_type_subtype_ts(
                        &match &pattern {
                            Pattern::Standart => &ident_standart_not_null_ucc,
                            Pattern::Array => &ident_with_id_array_not_null_ucc,
                        },
                        &pg_json_type_subtype_where
                    );
                    quote! {
                        pub type #ident_where_ucc = #import_path::NullableJsonObjectPgTypeWhereFilter<
                            #ident_standart_or_ident_with_id_array_as_pg_json_type_where_ts
                        >;
                    }
                }
            };
            let gen_field_ident_dq_ts = |v: &SynFieldWrapper| {
                dq_ts(&v.field_ident)
            };
            let gen_type_as_pg_json_type_read_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_read);
            let gen_type_as_pg_json_type_read_inner_ts = |type_ts: &dyn ToTokens| gen_type_as_pg_json_type_subtype_ts(&type_ts, &pg_json_type_subtype_read_inner);
            let gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts = |
                is_standart_with_id: &IsStandartWithId,
                read_with_or_without_annotation_or_inner: &ReadWithOrWithoutAnnotationOrInner
            | {
                let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_274293a0| {
                    let maybe_serde_skip_serializing_if_option_is_none_ts = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation => quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::Inner => Ts2::new(),
                    };
                    let field_ident = &el_274293a0.field_ident;
                    let field_type_as_json_type_read_ts = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::Inner => gen_type_as_pg_json_type_read_inner_ts(
                            &el_274293a0.field_type
                        ),
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation => gen_type_as_pg_json_type_read_ts(
                            &el_274293a0.field_type
                        ),
                    };
                    let option_value_field_type_as_json_type_read_ts = gen_option_tokens_declaration_ts(
                        &wrap_into_value_declaration_ts(&field_type_as_json_type_read_ts)
                    );
                    quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_ts
                        #field_ident: #option_value_field_type_as_json_type_read_ts
                    }
                });
                quote! {#(#ts),*}
            };
            let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_read_ts = {
                let ident_read_try_from_er_ucc = SelfReadTryFromErUcc::from_tokens(&ident);
                let ident_with_id_standart_not_null_read_try_from_er_ucc = SelfReadTryFromErUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
                let ident_standart_not_null_as_pg_json_type_read_ts = gen_type_as_pg_json_type_read_ts(&ident_standart_not_null_ucc);
                let ident_with_id_array_not_null_as_pg_json_type_read_ts = gen_type_as_pg_json_type_read_ts(&ident_with_id_array_not_null_ucc);
                let gen_ident_read_ts = |
                    ident_ts_fc625e96: &dyn ToTokens,
                    ts_1c85ea2c: &dyn ToTokens,
                    derive_serde_deserialize: DeriveSerdeDeserialize
                | {
                    let ts_3a67b41f = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize_if(derive_serde_deserialize)
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &ident_ts_fc625e96,
                        &Ts2::new(),
                        &ts_1c85ea2c
                    );
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        #ts_3a67b41f
                    }
                };
                let ident_read_ts = {
                    let (ts, derive_serde_deserialize) = match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => (
                                {
                                    let ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                        &is_standart_with_id_false,
                                        &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                                    );
                                    quote! {{#ts}}
                                },
                                DeriveSerdeDeserialize::False,
                            ),
                            IsNullable::True => (wrap_content_into_scopes_dot_comma_ts(&gen_option_tokens_declaration_ts(&ident_standart_not_null_as_pg_json_type_read_ts)), DeriveSerdeDeserialize::True),
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => (wrap_content_into_scopes_dot_comma_ts(&gen_vec_tokens_declaration_ts(&ident_with_id_standart_not_null_read_ucc)), DeriveSerdeDeserialize::True),
                            IsNullable::True => (wrap_content_into_scopes_dot_comma_ts(&gen_option_tokens_declaration_ts(&ident_with_id_array_not_null_as_pg_json_type_read_ts)), DeriveSerdeDeserialize::True),
                        },
                    };
                    gen_ident_read_ts(&ident_read_ucc, &ts, derive_serde_deserialize)
                };
                let gen_ident_read_try_from_er_ts = |ident_ts_df27c0c4: &dyn ToTokens|StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_location_lib_location()
                    .build_enum(
                        &ident_ts_df27c0c4,
                        &Ts2::new(),
                        &quote!{{
                            #AllFieldsAreNoneUcc {
                                loc: location_lib::loc::Loc,
                            },
                        }}
                    );
                let maybe_ident_read_try_from_er_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => gen_ident_read_try_from_er_ts(&ident_read_try_from_er_ucc),
                        IsNullable::True => Ts2::new(),
                    },
                    Pattern::Array => Ts2::new(),
                };
                let gen_ident_read_or_ident_with_id_standart_not_null_read_ucc = |is_standart_with_id: &IsStandartWithId| match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_ucc,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_ucc,
                };
                let gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_read_try_from_er_or_ident_with_id_standart_not_null_read_try_from_er_ucc: &dyn ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_try_from_er_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_read_try_from_er_ucc,
                    };
                    gen_pub_try_new_ts(
                        &gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                            is_standart_with_id,
                            &ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation
                        ),
                        &ident_read_try_from_er_or_ident_with_id_standart_not_null_read_try_from_er_ucc,
                        &{
                            let vec_syn_field_19e98ce1 = get_vec_syn_field(is_standart_with_id);
                            let (fields_reference_ts, fields_ts) = {
                                enum WithReference {
                                    False,
                                    True,
                                }
                                let gen_fields_ts = |with_reference: &WithReference| {
                                    let maybe_reference_symbol_ts = match &with_reference {
                                        WithReference::False => Ts2::new(),
                                        WithReference::True => quote! {&},
                                    };
                                    let fields_ts = vec_syn_field_19e98ce1.iter().map(|el_a6b6e788| {
                                        let field_ident = &el_a6b6e788.field_ident;
                                        quote! {#maybe_reference_symbol_ts #field_ident}
                                    });
                                    quote! {
                                        #(#fields_ts),*
                                    }
                                };
                                (gen_fields_ts(&WithReference::True), gen_fields_ts(&WithReference::False))
                            };
                            let check_if_all_fields_are_none_ts = {
                                let vec_syn_field_19e98ce1_len = vec_syn_field_19e98ce1.len();
                                let maybe_wrap_into_braces_handle_ts = |ts: &dyn ToTokens| maybe_wrap_into_braces_ts(
                                    ts,
                                    vec_syn_field_19e98ce1_len > 1
                                );
                                let left_ts = maybe_wrap_into_braces_handle_ts(&fields_reference_ts);
                                let right_ts = maybe_wrap_into_braces_handle_ts(&{
                                    let nones_ts = repeat_with(||quote!{None}).take(vec_syn_field_19e98ce1_len);
                                    quote! {#(#nones_ts),*}
                                });
                                let ts = if vec_syn_field_19e98ce1_len == 1 {
                                    let ts = maybe_wrap_into_braces_handle_ts(&fields_ts);
                                    quote! {#ts.is_none()}
                                }
                                else {
                                    quote! {matches!(#left_ts, #right_ts)}
                                };
                                quote! {
                                    if #ts {
                                        return Err(#ident_read_try_from_er_or_ident_with_id_standart_not_null_read_try_from_er_ucc::#AllFieldsAreNoneUcc {
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
                        let vec_ident_with_id_standart_not_null_read_ts = gen_vec_tokens_declaration_ts(&ident_with_id_standart_not_null_read_ucc);
                        match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                                IsNullable::True => gen_pub_const_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &gen_option_tokens_declaration_ts(
                                            &ident_standart_not_null_as_pg_json_type_read_ts
                                        )
                                    ),
                                    &self_value_ts
                                ),
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => gen_pub_const_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &vec_ident_with_id_standart_not_null_read_ts
                                    ),
                                    &self_value_ts
                                ),
                                IsNullable::True => gen_pub_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &gen_option_tokens_declaration_ts(
                                            &vec_ident_with_id_standart_not_null_read_ts
                                        )
                                    ),
                                    &quote! {Self(#ValueSc.map(#ident_with_id_array_not_null_as_pg_json_type_read_ts::new))},
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
                let gen_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                    let vec_syn_field_13a9b1ed = get_vec_syn_field(is_standart_with_id);
                    gen_impl_serde_deserialize_for_struct_ts(
                        &gen_ident_read_or_ident_with_id_standart_not_null_read_ucc(is_standart_with_id),
                        &vec_syn_field_13a9b1ed.iter().map(|el_00a75629|
                            (&el_00a75629.field_ident, &el_00a75629.field_type)
                        ).collect::<Vec<(&Ident, &Type)>>(),
                        vec_syn_field_13a9b1ed.len(),
                        &|_: &Ident, syn_type: &Type| {
                            let type_read_ts = gen_type_as_pg_json_type_read_ts(&syn_type);
                            gen_option_tokens_declaration_ts(
                                &wrap_into_value_declaration_ts(&type_read_ts)
                            )
                        }
                    )
                };
                let maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => gen_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                        IsNullable::True => Ts2::new(),
                    },
                    Pattern::Array => Ts2::new(),
                };
                let gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                    gen_impl_pg_crud_default_option_some_vec_one_el_ts(&gen_ident_read_or_ident_with_id_standart_not_null_read_ucc(is_standart_with_id), &Ts2::new(), &{
                        let fields_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_6a2035df| {
                            let field_ident = &el_6a2035df.field_ident;
                            let value_ts = wrap_into_value_init_ts(
                                &PgCrudDefaultOptionSomeVecOneElCall
                            );
                            quote! {#field_ident: Some(#value_ts)}
                        });
                        quote! {Self{#(#fields_ts),*}}
                    })
                };
                let impl_pg_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                        IsNullable::True => gen_impl_pg_crud_default_option_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_pg_crud_default_option_some_vec_one_el_call_ts),
                    },
                    Pattern::Array => match &is_nullable {
                        IsNullable::False => gen_impl_pg_crud_default_option_some_vec_one_el_ts(
                            &ident_read_ucc,
                            &Ts2::new(),
                            &quote! {
                                Self(#vec_pg_crud_default_option_some_vec_one_el_call_ts)
                            },
                        ),
                        IsNullable::True => gen_impl_pg_crud_default_option_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_pg_crud_default_option_some_vec_one_el_call_ts),
                    },
                };
                let impl_sqlx_type_for_ident_read_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_read_ucc);
                let impl_sqlx_encode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                    &ident_read_ucc,
                    &quote!{sqlx::types::Json(#SelfSc)}
                );
                let impl_sqlx_decode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_wrapper_ts(&ident_read_ucc);
                let maybe_ident_with_id_read_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_ts = gen_ident_read_ts(
                        &ident_with_id_standart_not_null_read_ucc,
                        &{
                            let ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                &is_standart_with_id_true,
                                &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                            );
                            quote! {{#ts}}
                        },
                        DeriveSerdeDeserialize::False,
                    );
                    let ident_with_id_standart_not_null_read_try_from_er_ts = gen_ident_read_try_from_er_ts(&ident_with_id_standart_not_null_read_try_from_er_ucc);
                    let impl_ident_with_id_standart_not_null_read_ts = {
                        let pub_try_new_ts = gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                        quote!{
                            impl #ident_with_id_standart_not_null_read_ucc {
                                #pub_try_new_ts
                            }
                        }
                    };
                    let impl_serde_deserialize_for_ident_with_id_standart_not_null_read_ts = gen_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                    let impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_read_ts = gen_impl_pg_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                    let impl_sqlx_type_for_ident_with_id_standart_not_null_read_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_with_id_standart_not_null_read_ucc);
                    let impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_read_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_wrapper_ts(&ident_with_id_standart_not_null_read_ucc);
                    quote! {
                        #ident_with_id_standart_not_null_read_ts
                        #ident_with_id_standart_not_null_read_try_from_er_ts
                        #impl_ident_with_id_standart_not_null_read_ts
                        #impl_serde_deserialize_for_ident_with_id_standart_not_null_read_ts
                        #impl_pg_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_read_ts
                        #impl_sqlx_type_for_ident_with_id_standart_not_null_read_ts
                        #impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_read_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_read_ts
                    #maybe_ident_read_try_from_er_ts
                    #impl_ident_read_ts
                    #maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts
                    #impl_pg_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts
                    #impl_sqlx_type_for_ident_read_ts
                    #impl_sqlx_encode_sqlx_pg_for_ident_read_ts
                    #impl_sqlx_decode_sqlx_pg_for_ident_read_ts
                    #maybe_ident_with_id_read_ts
                }
            };
            let ident_with_id_standart_not_null_read_only_ids_handle_ucc = SelfReadOnlyIdsHandleUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_standart_not_null_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
            let ident_read_only_ids_handle_ucc = SelfReadOnlyIdsHandleUcc::from_tokens(&ident);
            let gen_ident_read_only_ids_or_ident_with_id_read_only_ids_ts = |is_standart_with_id: &IsStandartWithId| {
                let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_5f9102af| {
                    let field_ident = &el_5f9102af.field_ident;
                    let field_type_as_pg_json_type_read_only_ids_ts = gen_type_as_pg_json_type_subtype_ts(
                        &el_5f9102af.field_type,
                        &PgJsonTypeSubtype::ReadOnlyIds
                    );
                    quote! {#field_ident: #field_type_as_pg_json_type_read_only_ids_ts}
                });
                quote! {{#(#ts),*}}
            };
            let gen_impl_sqlx_decode_ts = |ident_ts_caa12b2f: &dyn ToTokens|{
                gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                    &ident_ts_caa12b2f,
                    &quote!{sqlx::types::Json<Self>},
                    &quote!{Ok(v.0)}
                )
            };
            let gen_impl_sqlx_type_ts = |ident_ts_58d92fbf: &dyn ToTokens|{
                gen_impl_sqlx_type_for_ident_ts(
                    &ident_ts_58d92fbf,
                    &quote!{sqlx::types::Json<Self>}
                )
            };
            let gen_fields_read_only_ids_into_option_value_read_inner_ts = |is_standart_with_id: &IsStandartWithId, parameters_ts: &dyn ToTokens|{
                let ident_ts_6ad36393: &dyn ToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_standart_not_null_read_inner_ucc,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                };
                let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_278a1e1d| {
                    let field_ident = &el_278a1e1d.field_ident;
                    let field_type = &el_278a1e1d.field_type;
                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&field_type);
                    let field_type_as_pg_json_type_read_ts = gen_type_as_pg_json_type_subtype_ts(&field_type, &PgJsonTypeSubtype::Read);
                    let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                    let value_ts = wrap_into_value_init_ts(&{
                        let default_but_option_is_some_call_ts_f378bbab = gen_ident_as_default_but_option_is_some_call_ts(
                            &field_type_as_pg_json_type_read_ts
                        );
                        quote!{#field_type_as_pg_json_type_ts::into_inner(#default_but_option_is_some_call_ts_f378bbab)}
                    });
                    quote! {
                        #field_ident: #field_type_as_pg_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                            #parameters_ts.0.#ValueSc.#field_ident
                        ).map_or_else(
                            || Some(#value_ts),
                            Some
                        )
                    }
                });
                let value_ts = wrap_into_value_init_ts(&quote!{
                    #ident_ts_6ad36393 {
                        #(#ts),*
                    }
                });
                quote!{Some(#value_ts)}
            };
            let ident_read_only_ids_ts = {
                let maybe_ident_read_only_ids_handle_ts = if is_standart_not_null {
                    let ts_1e087f4d = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_handle_ucc,
                        &Ts2::new(),
                        &gen_ident_read_only_ids_or_ident_with_id_read_only_ids_ts(&IsStandartWithId::False)
                    );
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        #ts_1e087f4d
                    }
                }
                else {
                    Ts2::new()
                };
                let ident_read_only_ids_ts = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_ucc,
                        &Ts2::new(),
                        &match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let value_ident_read_only_ids_handle_ucc_ts = wrap_into_value_declaration_ts(&ident_read_only_ids_handle_ucc);
                                    quote! {(#value_ident_read_only_ids_handle_ucc_ts);}
                                },
                                IsNullable::True => {
                                    let value_option_ident_read_only_ids_standart_not_null_ts = wrap_into_value_declaration_ts(&gen_option_tokens_declaration_ts(
                                        &ident_standart_not_null_read_only_ids_ucc
                                    ));
                                    quote! {
                                        (#value_option_ident_read_only_ids_standart_not_null_ts);
                                    }
                                }
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let value_vec_ident_with_id_standart_not_null_read_only_ids_ts = wrap_into_value_declaration_ts(&gen_vec_tokens_declaration_ts(
                                        &ident_with_id_standart_not_null_read_only_ids_ucc
                                    ));
                                    quote! {
                                        (#value_vec_ident_with_id_standart_not_null_read_only_ids_ts);
                                    }
                                },
                                IsNullable::True => {
                                    let value_option_ident_with_id_read_only_ids_array_not_null_ts = wrap_into_value_declaration_ts(&gen_option_tokens_declaration_ts(
                                        &SelfReadOnlyIdsUcc::from_tokens(&gen_ident_ucc(&IdentPattern::ArrayNotNullWithId))
                                    ));
                                    quote! {(#value_option_ident_with_id_read_only_ids_array_not_null_ts);}
                                }
                            },
                        }
                    );
                let impl_sqlx_decode_sqlx_pg_for_ident_read_only_ids_ts = gen_impl_sqlx_decode_ts(&ident_read_only_ids_ucc);
                let impl_sqlx_type_for_ident_read_only_ids_ts = gen_impl_sqlx_type_ts(&ident_read_only_ids_ucc);
                let maybe_ident_with_id_standart_not_null_read_only_ids_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_only_ids_ts = {
                        let ident_with_id_standart_not_null_read_only_ids_handle_ts = {
                            let ts_fe644945 = StructOrEnumDeriveTsStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_handle_ucc,
                                &Ts2::new(),
                                &gen_ident_read_only_ids_or_ident_with_id_read_only_ids_ts(&IsStandartWithId::True)
                            );
                            quote!{
                                #AllowClippyArbitrarySourceItemOrdering
                                #ts_fe644945
                            }
                        };
                        let ident_with_id_standart_not_null_read_only_ids_ts = StructOrEnumDeriveTsStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_ucc,
                                &Ts2::new(),
                                &{
                                    let value_ident_with_id_standart_not_null_read_only_ids_handle_ts = wrap_into_value_declaration_ts(
                                        &ident_with_id_standart_not_null_read_only_ids_handle_ucc
                                    );
                                    quote!{(pub #value_ident_with_id_standart_not_null_read_only_ids_handle_ts);}
                                }
                            );
                        quote! {
                            #ident_with_id_standart_not_null_read_only_ids_handle_ts
                            #ident_with_id_standart_not_null_read_only_ids_ts
                        }
                    };
                    let impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_read_only_ids_ts = gen_impl_sqlx_decode_ts(&ident_with_id_standart_not_null_read_only_ids_ucc);
                    let impl_sqlx_type_for_ident_with_id_standart_not_null_read_only_ids_ts = gen_impl_sqlx_type_ts(&ident_with_id_standart_not_null_read_only_ids_ucc);
                    quote! {
                        #ident_with_id_standart_not_null_read_only_ids_ts
                        #impl_sqlx_decode_sqlx_pg_for_ident_with_id_standart_not_null_read_only_ids_ts
                        #impl_sqlx_type_for_ident_with_id_standart_not_null_read_only_ids_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #maybe_ident_read_only_ids_handle_ts
                    #ident_read_only_ids_ts
                    #impl_sqlx_decode_sqlx_pg_for_ident_read_only_ids_ts
                    #impl_sqlx_type_for_ident_read_only_ids_ts
                    #maybe_ident_with_id_standart_not_null_read_only_ids_ts
                }
            };
            let ident_read_inner_ts = {
                let gen_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ts_3d7e760e = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .build_struct(
                        match &is_standart_with_id {
                            IsStandartWithId::False => &ident_read_inner_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                        },
                        &Ts2::new(),
                        &{
                            let ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                is_standart_with_id,
                                &ReadWithOrWithoutAnnotationOrInner::Inner
                            );
                            quote!{{#ts}}
                        }
                    );
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        #ts_3d7e760e
                    }
                };
                let ident_read_inner_ts = {
                    let gen_pub_type_ident_read_inner_alias_ts = |ts: &dyn ToTokens| gen_pub_type_alias_ts(&ident_read_inner_ucc, &ts);
                    match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => gen_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_ts(&IsStandartWithId::False),
                            IsNullable::True => gen_pub_type_ident_read_inner_alias_ts(&gen_option_tokens_declaration_ts(&gen_type_as_pg_json_type_read_inner_ts(&ident_standart_not_null_ucc))),
                        },
                        Pattern::Array => gen_pub_type_ident_read_inner_alias_ts(&match &is_nullable {
                            IsNullable::False => gen_vec_tokens_declaration_ts(
                                &ident_with_id_standart_not_null_read_inner_ucc
                            ),
                            IsNullable::True => gen_option_tokens_declaration_ts(&gen_type_as_pg_json_type_read_inner_ts(&ident_with_id_array_not_null_ucc)),
                        }),
                    }
                };
                let maybe_ident_with_id_read_inner_ts = if is_standart_not_null {
                    let ident_with_id_read_inner_ts = gen_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_ts(&IsStandartWithId::True);
                    quote! {
                        #ident_with_id_read_inner_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_read_inner_ts
                    #maybe_ident_with_id_read_inner_ts
                }
            };
            let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
            let ident_standart_not_null_update_el_ucc = &SelfUpdateElementUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_standart_not_null_update_for_query_el_ucc = &SelfUpdateForQueryElementUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_update_el_ucc = &SelfUpdateElementUcc::from_tokens(&ident);
            let ident_update_for_query_el_ucc = &SelfUpdateForQueryElementUcc::from_tokens(&ident);
            let ident_standart_not_null_as_pg_json_type_update_ts = gen_type_as_pg_json_type_update_ts(&ident_standart_not_null_ucc);
            let ident_standart_not_null_as_pg_json_type_update_for_query_ts = gen_type_as_pg_json_type_update_for_query_ts(&ident_standart_not_null_ucc);
            let ident_with_id_array_not_null_as_pg_json_type_update_ts = gen_type_as_pg_json_type_update_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_array_not_null_as_pg_json_type_update_for_query_ts = gen_type_as_pg_json_type_update_for_query_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_standart_not_null_update_el_ucc = &SelfUpdateElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_update_for_query_el_ucc = &SelfUpdateForQueryElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let (gen_jsonb_set_target_sc, gen_jsonb_set_target_ts) = {
                let gen_jsonb_set_target_ts = {
                    let format_handle_ts = dq_ts(&format!("{{{JsonbSetTargetSc}}}->'{{v_12d082b5}}'"));
                    quote! {
                        let #GenJsonbSetTargetSc = |v_12d082b5: &str|{
                            format!(#format_handle_ts)
                        };
                    }
                };
                (GenJsonbSetTargetSc, gen_jsonb_set_target_ts)
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts = quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_el_ucc>
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_el_ts = quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_for_query_el_ucc>
            };
            let gen_create_update_delete_fields_ts_043c4057 = |
                should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation,
                create_ts: &dyn ToTokens,
                update_ts: &dyn ToTokens,
                delete_ts: &dyn ToTokens
            | {
                let maybe_serde_skip_serializing_if_vec_is_empty_ts = match &should_add_serde_skip_serializing_if_vec_is_empty_annotation {
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False => Ts2::new(),
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True => quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
                };
                quote! {
                    #maybe_serde_skip_serializing_if_vec_is_empty_ts
                    #CreateSc: #create_ts,
                    #UpdateSc: #update_ts,
                    #maybe_serde_skip_serializing_if_vec_is_empty_ts
                    #DeleteSc: #delete_ts,
                }
            };
            let ident_update_ts = {
                let gen_ident_update_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                    gen_unique_vec_wrapper_ts(match &is_standart_with_id {
                        IsStandartWithId::False => &ident_update_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_update_el_ucc,
                    })
                };
                let vec_ident_with_id_standart_not_null_create_ts = gen_vec_tokens_declaration_ts(
                    &ident_with_id_standart_not_null_create_ucc
                );
                let vec_pg_crud_path_pg_json_type_uuid_uuid_update_ts = gen_vec_tokens_declaration_ts(
                    &pg_crud_path_pg_json_type_uuid_uuid_update_ts
                );
                let gen_create_update_delete_fields_ts_ffcbdaf0 = |should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation| {
                    gen_create_update_delete_fields_ts_043c4057(
                        should_add_serde_skip_serializing_if_vec_is_empty_annotation,
                        &vec_ident_with_id_standart_not_null_create_ts,
                        &import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts,
                        &vec_pg_crud_path_pg_json_type_uuid_uuid_update_ts
                    )
                };
                let ident_update_ts = {
                    let gen_option_ident_type_ts = |ident_ts_dee7d090: &dyn ToTokens| wrap_content_into_scopes_dot_comma_ts(
                        &gen_option_tokens_declaration_ts(&ident_ts_dee7d090)
                    );
                    let (
                        derive_serde_deserialize,
                        ts_975df5c5
                    ) = match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => (
                                DeriveSerdeDeserialize::True,
                                &wrap_content_into_scopes_dot_comma_ts(
                                    &gen_ident_update_standart_not_null_ts(&is_standart_with_id_false)
                                )
                            ),
                            IsNullable::True => (
                                DeriveSerdeDeserialize::True,
                                &gen_option_ident_type_ts(&ident_standart_not_null_as_pg_json_type_update_ts)
                            ),
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => (
                                DeriveSerdeDeserialize::False,
                                &{
                                    let fields_ts = gen_create_update_delete_fields_ts_ffcbdaf0(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                    quote! {{#fields_ts}}
                                }
                            ),
                            IsNullable::True => (
                                DeriveSerdeDeserialize::True,
                                &gen_option_ident_type_ts(&ident_with_id_array_not_null_as_pg_json_type_update_ts)
                            ),
                        },
                    };
                    let ts_c9a843aa = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize_if(derive_serde_deserialize)
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &ident_update_ucc,
                        &Ts2::new(),
                        &ts_975df5c5
                    );
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        #ts_c9a843aa
                    }
                };
                let ident_update_try_new_er_ucc = &SelfUpdateTryNewErUcc::from_tokens(&ident);
                let maybe_ident_update_try_new_er_ts = match &pattern {
                    Pattern::Standart => Ts2::new(),
                    Pattern::Array => match &is_nullable {
                        IsNullable::False => StructOrEnumDeriveTsStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_location_lib_location()
                            .build_enum(
                                &ident_update_try_new_er_ucc,
                                &Ts2::new(),
                                &quote!{{
                                    #CreateUpdateDeleteAreEmptyUcc {
                                        loc: location_lib::loc::Loc,
                                    },
                                    #IdsAreNotUniqueUcc {
                                        #[eo_to_err_string_serde]
                                        duplicate: #StringTs,
                                        loc: location_lib::loc::Loc,
                                    },
                                    #NotUniqueIdInJsonDeleteArrayUcc {
                                        #[eo_to_err_string_serde]
                                        er: #StringTs,
                                        loc: location_lib::loc::Loc,
                                    },
                                    #NotUniqueIdInJsonUpdateAndDeleteArraysUcc {
                                        #[eo_to_err_string_serde]
                                        er: #StringTs,
                                        loc: location_lib::loc::Loc,
                                    },
                                }}
                            ),
                        IsNullable::True => Ts2::new(),
                    },
                };
                let impl_ident_update_ts = {
                    let maybe_pub_new_or_try_new_for_ident_update_ts = match &pattern {
                        Pattern::Standart => gen_pub_const_new_ts(
                            &must_use_ts,
                            &gen_value_type_ts(&match &is_nullable {
                                IsNullable::False => gen_unique_vec_wrapper_ts(&ident_standart_not_null_update_el_ucc),
                                IsNullable::True => gen_option_tokens_declaration_ts(&ident_standart_not_null_as_pg_json_type_update_ts)
                            }),
                            &self_value_ts
                        ),
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => gen_pub_try_new_ts(
                                &gen_create_update_delete_fields_ts_ffcbdaf0(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False),
                                &ident_update_try_new_er_ucc,
                                &{
                                    let custom_serde_er_deserializing_ident_update_str = format!("custom serde er deserializing {ident_update_ucc}");
                                    let check_if_all_empty_ts = {
                                        quote! {
                                            if create.is_empty() && update.is_empty() && delete.is_empty() {
                                                return Err(#ident_update_try_new_er_ucc::#CreateUpdateDeleteAreEmptyUcc {
                                                    loc: location_lib::loc!()
                                                });
                                            }
                                        }
                                    };
                                    let check_if_ids_are_unique_ts = {
                                        let (
                                            uuid_as_pg_json_type_update_to_err_string_el_id_ts,
                                            uuid_as_pg_json_type_update_to_err_string_el_ts,
                                        ) = {
                                            #[allow(clippy::arbitrary_source_item_ordering)]
                                            enum UpdateOrDelete {
                                                Update,
                                                Delete
                                            }
                                            let gen_uuid_as_pg_json_type_update_to_err_string_ts = |
                                                update_or_delete: &UpdateOrDelete,
                                                el_ts: &dyn ToTokens,
                                            |{
                                                let ts: &dyn ToTokens = match &update_or_delete {
                                                    UpdateOrDelete::Update => &quote!{&#el_ts.#IdSc},
                                                    UpdateOrDelete::Delete => &el_ts
                                                };
                                                quote!{
                                                    <#uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_update_ts as location_lib::ToErrString>::to_err_string(
                                                        #ts
                                                    )
                                                }
                                            };
                                            (
                                                gen_uuid_as_pg_json_type_update_to_err_string_ts(
                                                    &UpdateOrDelete::Update,
                                                    &quote!{el_dff7634c}
                                                ),
                                                gen_uuid_as_pg_json_type_update_to_err_string_ts(
                                                    &UpdateOrDelete::Delete,
                                                    &quote!{el_2b0181e6}
                                                )
                                            )
                                        };
                                        quote!{{
                                            let mut acc_2bf4e098 = Vec::new();
                                            for el_dff7634c in update.to_vec() {
                                                if acc_2bf4e098.contains(&&el_dff7634c.#IdSc) {
                                                    return Err(#ident_update_try_new_er_ucc::#IdsAreNotUniqueUcc {
                                                        duplicate: #uuid_as_pg_json_type_update_to_err_string_el_id_ts,
                                                        loc: location_lib::loc!()
                                                    });
                                                }
                                                acc_2bf4e098.push(&el_dff7634c.#IdSc);
                                            }
                                            for el_2b0181e6 in &delete {
                                                if acc_2bf4e098.contains(&el_2b0181e6) {
                                                    return Err(#ident_update_try_new_er_ucc::#IdsAreNotUniqueUcc {
                                                        duplicate: #uuid_as_pg_json_type_update_to_err_string_el_ts,
                                                        loc: location_lib::loc!()
                                                    });
                                                }
                                                acc_2bf4e098.push(el_2b0181e6);
                                            }
                                        }}
                                    };
                                    let check_not_unique_id_ts = {
                                        let check_not_unique_id_in_update_array_ts = quote! {
                                            let update_acc = #UpdateSc.to_vec().iter()
                                            .map(|el_b6af219f|&el_b6af219f.#IdSc)
                                            .collect::<Vec<&#uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_update_ts>>();
                                        };
                                        let check_not_unique_id_in_delete_aray_ts = {
                                            let not_unique_id_in_json_delete_array_dq_ts = dq_ts(&format!("{custom_serde_er_deserializing_ident_update_str}: not unique {IdSc} in json delete array: {{}}"));
                                            quote! {
                                                let delete_acc = {
                                                    let mut delete_acc = Vec::new();
                                                    for el_2ecc509c in &delete {
                                                        if delete_acc.contains(&el_2ecc509c) {
                                                            return Err(#ident_update_try_new_er_ucc::#NotUniqueIdInJsonDeleteArrayUcc {
                                                                er: format!(
                                                                    #not_unique_id_in_json_delete_array_dq_ts,
                                                                    #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::get_inner(
                                                                        &el_2ecc509c.clone().into()
                                                                    )
                                                                ),
                                                                loc: location_lib::loc!()
                                                            });
                                                        }
                                                        delete_acc.push(el_2ecc509c);
                                                    }
                                                    delete_acc
                                                };
                                            }
                                        };
                                        let check_not_unique_id_in_update_and_delete_arrays_ts = {
                                            let not_unique_id_in_json_update_and_delete_arrays_dq_ts = dq_ts(&format!("{custom_serde_er_deserializing_ident_update_str}: not unique {IdSc} in json update and delete arrays: {{}}"));
                                            quote! {
                                                for el_fefe9816 in update_acc {
                                                    if delete_acc.contains(&el_fefe9816) {
                                                        return Err(#ident_update_try_new_er_ucc::#NotUniqueIdInJsonUpdateAndDeleteArraysUcc {
                                                            er: format!(
                                                                #not_unique_id_in_json_update_and_delete_arrays_dq_ts,
                                                                #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::get_inner(
                                                                    &el_fefe9816.clone().into()
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
                                                #check_not_unique_id_in_update_array_ts
                                                #check_not_unique_id_in_delete_aray_ts
                                                #check_not_unique_id_in_update_and_delete_arrays_ts
                                            }
                                        }
                                    };
                                    quote!{
                                        #check_if_all_empty_ts
                                        #check_if_ids_are_unique_ts
                                        #check_not_unique_id_ts
                                        Ok(Self {
                                            #CreateSc,
                                            #UpdateSc,
                                            #DeleteSc
                                        })
                                    }
                                }
                            ),
                            IsNullable::True => gen_pub_const_new_value_type_content_self_value_ts(
                                &gen_option_tokens_declaration_ts(
                                    &ident_with_id_array_not_null_as_pg_json_type_update_ts
                                )
                            )
                        },
                    };
                    quote!{
                        impl #ident_update_ucc {
                            #maybe_pub_new_or_try_new_for_ident_update_ts
                        }
                    }
                };
                let maybe_impl_serde_deserialize_for_ident_update_ts = match &pattern {
                    Pattern::Standart => Ts2::new(),
                    Pattern::Array => match &is_nullable {
                        IsNullable::False => {
                            //todo maybe reuse?
                            let tuple_struct_ident_update_dq_ts = dq_ts(&format!("tuple struct {ident_update_ucc}"));
                            let ident_update_dq_ts = dq_ts(&ident_update_ucc);
                            let match_try_new_in_deserialize_ts = gen_match_try_new_in_deserialize_ts(
                                &ident_update_ucc,
                                &quote! {f0_value, f1_value, f2_value}
                            );
                            quote! {
                                #[allow(clippy::absolute_paths)]
                                #AllowClippyArbitrarySourceItemOrdering
                                impl<'de> serde::Deserialize<'de> for #ident_update_ucc {
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
                                                    "create" => Ok(__Field::f0),
                                                    "update" => Ok(__Field::f1),
                                                    "delete" => Ok(__Field::f2),
                                                    _ => Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match v {
                                                    b"create" => Ok(__Field::f0),
                                                    b"update" => Ok(__Field::f1),
                                                    b"delete" => Ok(__Field::f2),
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
                                            marker: serde::__private228::PhantomData<#ident_update_ucc>,
                                            lifetime: serde::__private228::PhantomData<&'de ()>,
                                        }
                                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #ident_update_ucc;
                                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                                serde::__private228::Formatter::write_str(__f, #tuple_struct_ident_update_dq_ts)
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let f0_value = serde::de::SeqAccess::next_element::<#vec_ident_with_id_standart_not_null_create_ts>(&mut __seq)?.unwrap_or_default();
                                                let f1_value = serde::de::SeqAccess::next_element::<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts>(&mut __seq)?.unwrap_or_default();
                                                let f2_value = serde::de::SeqAccess::next_element::<#vec_pg_crud_path_pg_json_type_uuid_uuid_update_ts>(&mut __seq)?.unwrap_or_default();
                                                #match_try_new_in_deserialize_ts
                                            }
                                            #[inline]
                                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::MapAccess<'de>,
                                            {
                                                let mut f0: Option<#vec_ident_with_id_standart_not_null_create_ts> = None;
                                                let mut f1: Option<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts> = None;
                                                let mut f2: Option<#vec_pg_crud_path_pg_json_type_uuid_uuid_update_ts> = None;
                                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                    match __key {
                                                        __Field::f0 => {
                                                            if Option::is_some(&f0) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                            }
                                                            f0 = Some(serde::de::MapAccess::next_value::<#vec_ident_with_id_standart_not_null_create_ts>(&mut __map)?);
                                                        }
                                                        __Field::f1 => {
                                                            if Option::is_some(&f1) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                            }
                                                            f1 = Some(serde::de::MapAccess::next_value::<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts>(&mut __map)?);
                                                        }
                                                        __Field::f2 => {
                                                            if Option::is_some(&f2) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                            }
                                                            f2 = Some(serde::de::MapAccess::next_value::<#vec_pg_crud_path_pg_json_type_uuid_uuid_update_ts>(&mut __map)?);
                                                        }
                                                        __Field::__ignore => {
                                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                let f0_value = f0.unwrap_or_default();
                                                let f1_value = f1.unwrap_or_default();
                                                let f2_value = f2.unwrap_or_default();
                                                #match_try_new_in_deserialize_ts
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &[&str] = &["create", "update", "delete"];
                                        serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_update_dq_ts,
                                            FIELDS,
                                            __Visitor {
                                                marker: serde::__private228::PhantomData::<#SelfUcc>,
                                                lifetime: serde::__private228::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        }
                        IsNullable::True => Ts2::new(),
                    },
                };
                let impl_pg_crud_default_option_some_vec_one_el_for_ident_update_ts = gen_impl_pg_crud_default_option_some_vec_one_el_ts(&ident_update_ucc, &Ts2::new(), &{
                    let value = match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => quote! {(#PgCrudDefaultOptionSomeVecOneElCall)},
                            IsNullable::True => quote! {(Some(#PgCrudDefaultOptionSomeVecOneElCall))},
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => quote! {{
                                #CreateSc: #vec_pg_crud_default_option_some_vec_one_el_call_ts,
                                #UpdateSc: #PgCrudDefaultOptionSomeVecOneElCall,
                                #DeleteSc: #vec_pg_crud_default_option_some_vec_one_el_call_ts,
                            }},
                            IsNullable::True => quote! {
                                (Some(#PgCrudDefaultOptionSomeVecOneElCall))
                            },
                        },
                    };
                    quote! {Self #value}
                });
                let maybe_ident_update_el_ts = if is_standart_not_null {
                    let ident_update_el_ts = {
                        let ts_b258e2eb = StructOrEnumDeriveTsStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_enum(
                            &ident_standart_not_null_update_el_ucc,
                            &Ts2::new(),
                            &{
                                let vrts_ts = vec_syn_field.iter().map(|el_092057f6| {
                                    let field_ident = &el_092057f6.field_ident;
                                    let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_ident_dq_ts = gen_field_ident_dq_ts(el_092057f6);
                                    let value_field_type_as_json_type_update_ts = wrap_into_value_declaration_ts(
                                        &gen_type_as_pg_json_type_update_ts(&el_092057f6.field_type)
                                    );
                                    quote! {
                                        #[serde(rename(serialize = #field_ident_dq_ts, deserialize = #field_ident_dq_ts))]
                                        #vrt_ident_ucc_ts(#value_field_type_as_json_type_update_ts)
                                    }
                                });
                                quote!{{#(#vrts_ts),*}}
                            }
                        );
                        quote!{
                            #AllowClippyArbitrarySourceItemOrdering
                            #ts_b258e2eb
                        }
                    };
                    let impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_update_el_ts = gen_impl_pg_crud_all_vrts_default_option_some_vec_one_el_ts(&ident_standart_not_null_update_el_ucc, &{
                        let elements_ts = vec_syn_field.iter().map(|el_2080bd7e| {
                            let field_ident = &el_2080bd7e.field_ident;
                            let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                            let value_ts = wrap_into_value_init_ts(
                                &PgCrudDefaultOptionSomeVecOneElCall
                            );
                            quote! {#SelfUcc::#vrt_ident_ucc_ts(#value_ts)}
                        });
                        quote! {vec![#(#elements_ts),*]}
                    });
                    quote! {
                        #ident_update_el_ts
                        #impl_pg_crud_all_vrts_default_option_some_vec_one_el_for_ident_update_el_ts
                    }
                } else {
                    Ts2::new()
                };
                let maybe_ident_with_id_standart_not_null_update_el_ts = if is_standart_not_null {
                    //thought it can be reused as struct with generic parameter, but turns out its too painfull
                    // pub trait MyTrait {
                    //     type AdditionalType: PartialEq;
                    // }
                    // pub struct MyStruct;
                    // #[derive(PartialEq)]
                    // pub struct MyStructAdditionalType(String);
                    // impl MyTrait for MyStruct {
                    //     type AdditionalType = MyStructAdditionalType;
                    // }
                    // #[derive(PartialEq)]
                    // pub struct WrapperOfMyTrait<T: MyTrait>(<T as MyTrait>::AdditionalType);
                    // pub type WrapperOfMyTraitAlias = WrapperOfMyTrait<MyStruct>;
                    // #[derive(PartialEq)]
                    // pub struct WrapperOfWrapperOfMyTraitAlias(WrapperOfMyTraitAlias);
                    // // er[E0369]: binary operation `==` cannot be applied to type `WrapperOfMyTrait<MyStruct>`
                    let ident_with_id_standart_not_null_update_el_fields_declaration_ts = quote! {
                        #IdSc: #pg_crud_path_pg_json_type_uuid_uuid_update_ts,
                        #FieldsSc: #ident_standart_not_null_as_pg_json_type_update_ts
                    };
                    let ident_with_id_standart_not_null_update_el_ts = {
                        let ts_d18600a2 = StructOrEnumDeriveTsStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_struct(
                            &ident_with_id_standart_not_null_update_el_ucc,
                            &Ts2::new(),
                            &quote!{{#ident_with_id_standart_not_null_update_el_fields_declaration_ts}}
                        );
                        quote!{
                            #AllowClippyArbitrarySourceItemOrdering
                            #ts_d18600a2
                        }
                    };
                    let impl_pub_new_for_ident_with_id_standart_not_null_update_el_ts = gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &must_use_ts,
                        &ident_with_id_standart_not_null_update_el_fields_declaration_ts,
                        &quote! {Self {
                            #IdSc,
                            #FieldsSc
                        }},
                    );
                    let impl_pg_crud_default_option_some_vec_one_el_for_ident_with_standart_not_null_update_el_ts = gen_impl_pg_crud_default_option_some_vec_one_el_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &Ts2::new(),
                        &quote! {Self {
                            #IdSc: #PgCrudDefaultOptionSomeVecOneElCall,
                            #FieldsSc: #PgCrudDefaultOptionSomeVecOneElCall,
                        }},
                    );
                    quote! {
                        #ident_with_id_standart_not_null_update_el_ts
                        #impl_pub_new_for_ident_with_id_standart_not_null_update_el_ts
                        #impl_pg_crud_default_option_some_vec_one_el_for_ident_with_standart_not_null_update_el_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_update_ts
                    #maybe_ident_update_try_new_er_ts
                    #impl_ident_update_ts
                    #maybe_impl_serde_deserialize_for_ident_update_ts
                    #impl_pg_crud_default_option_some_vec_one_el_for_ident_update_ts
                    #maybe_ident_update_el_ts
                    #maybe_ident_with_id_standart_not_null_update_el_ts
                }
            };
            let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
            let ident_update_for_query_ts = {
                let ident_update_for_query_ts = {
                    let gen_ident_update_for_query_ts = |ts: &dyn ToTokens| {
                        gen_debug_clone_partialeq_serialize_pub_struct_ts(
                            &AllowClippyArbitrarySourceItemOrdering,
                            &ident_update_for_query_ucc,
                            &ts
                        )
                    };
                    let gen_option_ident_type_ts = |ident_ts_5a4646ae: &dyn ToTokens| wrap_content_into_scopes_dot_comma_ts(
                        &gen_option_tokens_declaration_ts(&ident_ts_5a4646ae)
                    );
                    let gen_ident_update_for_query_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                        gen_unique_vec_wrapper_ts(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_update_for_query_el_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        })
                    };
                    let vec_ident_with_id_standart_not_null_create_for_query_ts = gen_vec_tokens_declaration_ts(
                        &ident_with_id_standart_not_null_create_for_query_ucc
                    );
                    let vec_pg_crud_path_pg_json_type_uuid_uuid_update_for_query_ts = gen_vec_tokens_declaration_ts(
                        &pg_crud_path_pg_json_type_uuid_uuid_update_for_query_ts
                    );
                    match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => gen_ident_update_for_query_ts(
                                &wrap_content_into_scopes_dot_comma_ts(
                                    &gen_ident_update_for_query_standart_not_null_ts(
                                        &is_standart_with_id_false
                                    )
                                )
                            ),
                            IsNullable::True => gen_ident_update_for_query_ts(
                                &gen_option_ident_type_ts(
                                    &ident_standart_not_null_as_pg_json_type_update_for_query_ts
                                )
                            ),
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => gen_ident_update_for_query_ts(
                                &{
                                    let fields_ts = gen_create_update_delete_fields_ts_043c4057(
                                        &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True,
                                        &vec_ident_with_id_standart_not_null_create_for_query_ts,
                                        &import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_el_ts,
                                        &vec_pg_crud_path_pg_json_type_uuid_uuid_update_for_query_ts,//todo maybe expand logic with where cases
                                    );
                                    quote! {{#fields_ts}}
                                }
                            ),
                            IsNullable::True => gen_ident_update_for_query_ts(
                                &gen_option_ident_type_ts(&ident_with_id_array_not_null_as_pg_json_type_update_for_query_ts)
                            ),
                        },
                    }
                };
                let impl_ident_update_for_query_ts = {
                    let select_only_updated_ids_query_part_ts = {
                        let ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let match_vrts_ts = vec_syn_field.iter().map(|el_bca06812| {
                                        let field_ident = &el_bca06812.field_ident;
                                        let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_dq_ts = dq_ts(&field_ident);
                                        let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_bca06812.field_type);
                                        let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_8e628eaf, "jsonb_build_object({v_c3ae3be4})||"},
                                            &return_err_query_part_er_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(#ValueSc) => {
                                                match #field_type_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryPartSc(
                                                    &#ValueSc.#ValueSc,
                                                    #field_ident_dq_ts,
                                                    column_name_and_maybe_field_getter,
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
                                        for el_0963b7df in self.0.to_vec() {
                                            match &el_0963b7df {
                                                #(#match_vrts_ts),*
                                            }
                                        }
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        Ok(acc_8e628eaf)
                                    }
                                },
                                IsNullable::True => {
                                    let match_ts = vec_syn_field.iter().map(|el_a8f45572| {
                                        let field_ident = &el_a8f45572.field_ident;
                                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_dq_ts = dq_ts(&field_ident);
                                        let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_a8f45572.field_type);
                                        let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_f7537df2, "jsonb_build_object({value})||"},
                                            &return_err_query_part_er_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc_ts(
                                                v_92d002a5
                                            ) => match #field_type_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryPartSc(
                                                &v_92d002a5.#ValueSc,
                                                #field_ident_dq_ts,
                                                column_name_and_maybe_field_getter,
                                                #IncrSc
                                            ) {
                                                Ok(mut #ValueSc) => {
                                                    let _: Option<char> = #ValueSc.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                }
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        }
                                    });
                                    quote!{
                                        Ok(match &self.0 {
                                            Some(v_9570957e) => {
                                                let mut acc_f7537df2 = #StringTs::default();
                                                for el_97687be3 in v_9570957e.0.to_vec() {
                                                    match &el_97687be3 {
                                                        #(#match_ts),*
                                                    }
                                                }
                                                let _: Option<char> = acc_f7537df2.pop();
                                                let _: Option<char> = acc_f7537df2.pop();
                                                format!("jsonb_build_object('value',{acc_f7537df2})")
                                            },
                                            None => "'null'::jsonb".to_owned()//todo maybe reuse
                                        })
                                    }
                                },
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let match_vrts_ts = vec_syn_field.iter().map(|el_74643094| {
                                        let field_ident = &el_74643094.field_ident;
                                        let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_dq_ts = dq_ts(&field_ident);
                                        let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_74643094.field_type);
                                        let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_892857b1, "jsonb_build_object({v_33d3b52e})||"},
                                            &return_err_query_part_er_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(#ValueSc) => match #field_type_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryPartSc(
                                                &#ValueSc.#ValueSc,
                                                #field_ident_dq_ts,
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
                                    let select_only_created_ids_query_part_ts = vec_syn_field_with_id.iter().map(|el_e6d6df84| {
                                        let field_ident = &el_e6d6df84.field_ident;
                                        let field_ident_dq_ts = dq_ts(&field_ident);
                                        let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_e6d6df84.field_type);
                                        let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_57cd0744, "jsonb_build_object({value})||"},
                                            &return_err_query_part_er_write_into_buffer_ts
                                        );
                                        quote! {
                                            match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryPartSc(
                                                &el_b1359d90.#field_ident,
                                                #field_ident_dq_ts,
                                                "elem",
                                                #IncrSc
                                            ) {
                                                Ok(mut #ValueSc) => {
                                                    let _: Option<char> = #ValueSc.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                },
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        }
                                    });
                                    let if_write_is_err_curly_braces_0_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_892857b1, "jsonb_build_object({value})||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    let if_write_is_err_curly_braces_1_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_57cd0744, "{acc_892857b1}||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    let if_write_is_err_0_ts = gen_if_write_is_err_ts(
                                        &quote!{acc_d497e8a5, "${v_c31cb081},"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    let if_write_is_err_1_ts = gen_if_write_is_err_ts(
                                        &quote!{acc_d497e8a5, "${v_b52c3fe1},"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    quote!{
                                        Ok(format!(
                                            "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                                            {
                                                let mut acc_57cd0744 = #StringTs::new();
                                                for el_d7561f40 in self.#UpdateSc.to_vec() {
                                                    //todo maybe wrong for multiple updates by id?
                                                    let mut acc_892857b1 = #StringTs::new();
                                                    match #import_path_pg_json_type_uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_ts ::select_only_updated_ids_query_part(
                                                        &el_d7561f40.id,
                                                        "id",
                                                        "elem",
                                                        #IncrSc
                                                    ) {
                                                        Ok(mut #ValueSc) => {
                                                            let _: Option<char> = #ValueSc.pop();
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
                                                for el_b1359d90 in &self.create {
                                                    #(#select_only_created_ids_query_part_ts)*
                                                }
                                                let _: Option<char> = acc_57cd0744.pop();
                                                let _: Option<char> = acc_57cd0744.pop();
                                                format!("jsonb_build_object('value',{acc_57cd0744})")
                                            },
                                            column_name_and_maybe_field_getter,
                                            {
                                                let mut acc_d497e8a5 = #StringTs::new();
                                                for _ in self.#UpdateSc.to_vec() {
                                                    match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                                                        Ok(v_c31cb081) => {
                                                            #if_write_is_err_0_ts
                                                        },
                                                        Err(#ErSc) => {
                                                            return Err(#ErSc);
                                                        },
                                                    }
                                                }
                                                for _ in &self.#CreateSc {
                                                    match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
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
                                IsNullable::True => quote!{
                                    Ok(match &self.0 {
                                        Some(v_bc509c9a) => format!(
                                            "jsonb_build_object('value',{})",
                                            match #ident_array_not_null_update_for_query_ucc::#SelectOnlyUpdatedIdsQueryPartSc(
                                                v_bc509c9a,
                                                column_name_and_maybe_field_getter,
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
                                },
                            },
                        };
                        quote!{
                            #[allow(clippy::single_call_fn)]//for some reason lint ignoring this function call in other struct trait methonds(array not null)
                            fn #SelectOnlyUpdatedIdsQueryPartSc(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                #IncrSc: &mut u64
                            ) -> Result<#StringTs, #import_path_query_part_er_ts> {
                                #ts
                            }
                        }
                    };
                    quote!{
                        impl #ident_update_for_query_ucc {
                            #select_only_updated_ids_query_part_ts
                        }
                    }
                };
                let impl_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_ts = gen_impl_from_ts(
                    &quote!{#ident_as_import_path_pg_json_type_ts::Update},
                    &quote!{#ident_as_import_path_pg_json_type_ts::UpdateForQuery},
                    &match &is_nullable {
                        IsNullable::False => match &pattern {
                            Pattern::Standart => quote!{
                                Self(#import_path::NotEmptyUniqueVec::from_t1_impl_from_t2(#ValueSc.0))
                            },
                            Pattern::Array => quote!{
                                Self {
                                    #CreateSc: #ValueSc.#CreateSc.into_iter().map(#ident_with_id_standart_not_null_create_for_query_ucc::from).collect(),
                                    #UpdateSc: #import_path::UniqueVec::from_t1_impl_from_t2(#ValueSc.#UpdateSc),
                                    #DeleteSc: #ValueSc.#DeleteSc.into_iter().map(Into::into).collect(),
                                }
                            }
                        },
                        IsNullable::True => {
                            let ts: &dyn ToTokens = match &pattern {
                                Pattern::Standart => &ident_standart_not_null_as_import_path_pg_json_type_ts,
                                Pattern::Array => &ident_array_not_null_as_import_path_pg_json_type_ts
                            };
                            quote!{Self(#ValueSc.0.map(#ts::UpdateForQuery::from))}
                        }
                    }
                );
                let maybe_ident_update_for_query_el_ts = if is_standart_not_null {
                    let ident_standart_not_null_update_for_query_el_ts = StructOrEnumDeriveTsStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .build_enum(
                            &ident_standart_not_null_update_for_query_el_ucc,
                            &Ts2::new(),
                            &{
                                let vrts_ts = vec_syn_field.iter().map(|el_9d8af887| {
                                    let field_ident = &el_9d8af887.field_ident;
                                    let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_ident_dq_ts = gen_field_ident_dq_ts(el_9d8af887);
                                    let value_field_type_as_json_type_update_for_query_ts = wrap_into_value_declaration_ts(&gen_type_as_pg_json_type_update_for_query_ts(&el_9d8af887.field_type));
                                    quote! {
                                        #[serde(rename(serialize = #field_ident_dq_ts, deserialize = #field_ident_dq_ts))]
                                        #vrt_ident_ucc_ts(#value_field_type_as_json_type_update_for_query_ts)
                                    }
                                });
                                quote!{{#(#vrts_ts),*}}
                            }
                        );
                    let impl_from_ident_standart_not_null_update_el_for_ident_standart_not_null_update_for_query_el_ts = gen_impl_from_ts(
                        &ident_standart_not_null_update_el_ucc,
                        &ident_standart_not_null_update_for_query_el_ucc,
                        &{
                            let vrts_ts = vec_syn_field.iter().map(|el_2a5d6ff3| {
                                let field_ident = &el_2a5d6ff3.field_ident;
                                let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                let value_init_ts = gen_import_path_value_init_ts(&{
                                    let field_type_as_json_type_update_for_query_ts = gen_type_as_pg_json_type_update_for_query_ts(
                                        &el_2a5d6ff3.field_type
                                    );
                                    quote!{
                                        #field_type_as_json_type_update_for_query_ts::from(v_121f1c54.#ValueSc)
                                    }
                                });
                                quote! {
                                    #ident_standart_not_null_update_el_ucc::#vrt_ident_ucc_ts(v_121f1c54) => #SelfUcc::#vrt_ident_ucc_ts(#value_init_ts)
                                }
                            });
                            quote!{
                                match #ValueSc {
                                    #(#vrts_ts),*
                                }
                            }
                        }
                    );
                    quote! {
                        #ident_standart_not_null_update_for_query_el_ts
                        #impl_from_ident_standart_not_null_update_el_for_ident_standart_not_null_update_for_query_el_ts
                    }
                } else {
                    Ts2::new()
                };
                let maybe_ident_with_id_standart_not_null_update_for_query_el_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts = quote! {
                        #IdSc: #pg_crud_path_pg_json_type_uuid_uuid_update_for_query_ts,
                        #FieldsSc: #ident_standart_not_null_as_pg_json_type_update_for_query_ts
                    };
                    let ident_with_id_standart_not_null_update_for_query_el_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &AllowClippyArbitrarySourceItemOrdering,
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &quote!{{#ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts}}
                    );
                    let impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_el_ts = gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &must_use_ts,
                        &ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts,
                        &quote! {Self {
                            #IdSc,
                            #FieldsSc
                        }},
                    );
                    let impl_from_ident_with_id_standart_not_null_update_el_for_ident_with_id_standart_not_null_update_for_query_el_ts = gen_impl_from_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &quote! {Self {
                            #IdSc: #uuid_uuid_as_not_null_jsonb_string_as_import_path_pg_json_type_ts::UpdateForQuery::from(
                                #ValueSc.#IdSc
                            ),
                            fields: #ident_standart_not_null_as_import_path_pg_json_type_ts::UpdateForQuery::from(
                                #ValueSc.fields
                            ),
                        }}
                    );
                    quote! {
                        #ident_with_id_standart_not_null_update_for_query_el_ts
                        #impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_el_ts
                        #impl_from_ident_with_id_standart_not_null_update_el_for_ident_with_id_standart_not_null_update_for_query_el_ts
                    }
                } else {
                    Ts2::new()
                };
                quote!{
                    #ident_update_for_query_ts
                    #impl_ident_update_for_query_ts
                    #impl_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_ts
                    #maybe_ident_update_for_query_el_ts
                    #maybe_ident_with_id_standart_not_null_update_for_query_el_ts
                }
            };
            let (impl_pg_crud_pg_json_type_for_ident_ts, maybe_impl_pg_crud_pg_types_pg_type_pg_type_ts) = {
                let pg_type_or_pg_json_type_pg_type = PgTypeOrPgJsonType::PgType;
                let pg_type_or_pg_json_type_pg_json_type = PgTypeOrPgJsonType::PgJsonType;
                let gen_update_query_part_standart_nullable_ts = |pg_type_or_pg_json_type: &PgTypeOrPgJsonType|{
                    let format_handle_ts = dq_ts(&match &pg_type_or_pg_json_type {
                        PgTypeOrPgJsonType::PgType => format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{{JsonbSetPathSc}}}}}}}',${{v_27b8537f}})"),
                        PgTypeOrPgJsonType::PgJsonType => "${v_27b8537f}".to_owned(),
                    });
                    quote! {
                        match &#ValueSc.0 {
                            Some(v_92f34435) => #ident_standart_not_null_as_pg_json_type_ts::#UpdateQueryPartSc(
                                v_92f34435,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                incr,
                            ),
                            None => match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                                Ok(v_27b8537f) => Ok(format!(#format_handle_ts)),
                                Err(#ErSc) => Err(#ErSc),
                            }
                        }
                    }
                };
                let gen_update_delete_create_array_ts = |format_handle_ts: &dyn ToTokens|{
                    let if_write_is_err_ts = gen_if_write_is_err_ts(
                        &quote!{acc_2e2ad041, "{v_8333f8f4}"},
                        &return_err_query_part_er_write_into_buffer_ts
                    );
                    let if_write_is_err_curly_braces_0_ts = gen_if_write_is_err_curly_braces_ts(
                        &quote!{acc_5b4cd920, "{maybe_space_and_space}elem->>'id' <> ${incr_cb6ba4a7}"},
                        &return_err_query_part_er_write_into_buffer_ts
                    );
                    let if_write_is_err_curly_braces_1_ts = gen_if_write_is_err_curly_braces_ts(
                        &quote!{acc_8554f572, "${incr_5bbc4961},"},
                        &return_err_query_part_er_write_into_buffer_ts
                    );
                    quote! {
                        let update_query_part_acc = {
                            if v_58d685d3.#UpdateSc.is_empty() {
                                #StringTs::from("elem")
                            } else {
                                let mut acc_2e2ad041 = #StringTs::default();
                                for el_a0a61823 in v_58d685d3.#UpdateSc.to_vec() {
                                    let ident_with_id_handle = {
                                        let id_incr = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
                                            Ok(v_15b44b54) => v_15b44b54,
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        };
                                        match #ident_standart_not_null_as_pg_json_type_ts::#UpdateQueryPartSc(
                                            &el_a0a61823.fields,
                                            "",
                                            "elem",
                                            "",
                                            #IncrSc
                                        ) {
                                            Ok(v_56c44461) => Ok(format!("when elem->>'id' = ${id_incr} then {v_56c44461} ")),
                                            Err(#ErSc) => Err(#ErSc)
                                        }
                                    };
                                    match ident_with_id_handle {
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
                        let delete_query_part_acc = {
                            let mut acc_5b4cd920 = #StringTs::default();
                            for _ in &v_58d685d3.#DeleteSc {
                                let incr_cb6ba4a7 = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
                                    Ok(v_110650cc) => v_110650cc,
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                };
                                let maybe_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                                #if_write_is_err_curly_braces_0_ts
                            }
                            acc_5b4cd920
                        };
                        let create_query_part_acc = {
                            let mut acc_8554f572 = #StringTs::default();
                            for _ in &v_58d685d3.#CreateSc {
                                let incr_5bbc4961 = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
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
                        let maybe_where = if v_58d685d3.#DeleteSc.is_empty() {
                            #StringTs::default()
                        } else {
                            format!(" where {delete_query_part_acc}")
                        };
                        let maybe_jsonb_build_array = if v_58d685d3.#CreateSc.is_empty() {
                            #StringTs::default()
                        } else {
                            format!(" || jsonb_build_array({create_query_part_acc})")
                        };
                        Ok (format!(#format_handle_ts))
                    }
                };
                let gen_update_query_part_array_not_null_ts = |pg_type_or_pg_json_type: &PgTypeOrPgJsonType|{
                    let ts_c75c3cd1 = gen_update_delete_create_array_ts(&dq_ts(&match &pg_type_or_pg_json_type {
                        PgTypeOrPgJsonType::PgType => "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})",
                        PgTypeOrPgJsonType::PgJsonType => "((select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) {maybe_jsonb_build_array})",
                    }));
                    quote!{
                        let v_58d685d3 = #ValueSc;
                        #ts_c75c3cd1
                    }
                };
                let impl_pg_crud_pg_json_type_for_ident_ts = gen_impl_pg_json_type_ts(
                    &ImportPath::PgCrud,
                    &ident,
                    &ident_table_type_declaration_ucc,
                    &ident_create_ucc,
                    &ident_create_for_query_ucc,
                    &ident_select_ucc,
                    &IsSelectQueryPartSelfSelectUsed::True,
                    &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErMessageUsed::True,
                    &IsSelectQueryPartIsPgTypeUsed::True,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => quote! {
                                match #ValueSc.#SelectQueryPartSc(
                                    &if is_pg_type {
                                        column_name_and_maybe_field_getter.to_owned()
                                    } else {
                                        format!("{column_name_and_maybe_field_getter}->'{field_ident}'")
                                    },
                                    &format!("{column_name_and_maybe_field_getter_for_er_message}.{field_ident}"),
                                ) {
                                    Ok(v_156121ad) => Ok(
                                        if is_pg_type {
                                            v_156121ad
                                        } else {
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',{v_156121ad}))")
                                        }
                                    ),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            },
                            IsNullable::True => {
                                let ident_standart_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                    &ident_standart_not_null_as_pg_json_type_select_ts
                                );
                                quote! {
                                    let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
                                    let v_46039f0e = value.0.as_ref().map_or_else(
                                        #ident_standart_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts,
                                        Clone::clone
                                    );
                                    match #ident_standart_not_null_as_pg_json_type_ts::#SelectQueryPartSc(
                                        &v_46039f0e,
                                        field_ident,
                                        &column_name_and_maybe_field_getter_field_ident,
                                        column_name_and_maybe_field_getter_for_er_message,
                                        true
                                    ) {
                                        Ok(v_1f8de96a) => Ok(
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({v_1f8de96a}) end))")
                                        ),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            },
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => {
                                let acc_41dea548_ts = quote!{acc_41dea548};
                                let select_query_part_for_loop_ts = {
                                    let value_dq_ts = dq_ts(&ValueSc);
                                    gen_select_query_part_for_loop_ts(
                                        &acc_41dea548_ts,
                                        &is_standart_with_id_true,
                                        &quote!{#ValueSc.#ident_with_id_standart_not_null_select_sc},
                                        &value_dq_ts,
                                        &value_dq_ts,
                                    )
                                };
                                let format_handle_ts = dq_ts(&format!(
                                    "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',case when (jsonb_array_length({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_sc}}})) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{dim1_start}} and {{dim1_end}}) end ))"
                                ));
                                quote! {
                                    let #ident_with_id_standart_not_null_select_sc = {
                                        let mut #acc_41dea548_ts = #StringTs::default();
                                        #select_query_part_for_loop_ts
                                        let _: Option<char> = #acc_41dea548_ts.pop();
                                        let _: Option<char> = #acc_41dea548_ts.pop();
                                        #acc_41dea548_ts
                                    };
                                    let dim1_start = #ValueSc.#dim1_pagination_ts.start();
                                    let dim1_end = #ValueSc.#dim1_pagination_ts.end();
                                    Ok(format!(#format_handle_ts))
                                }
                            }
                            IsNullable::True => {
                                let format_handle_ts = dq_ts(
                                    &"case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({v_d7bbd03c}) end"
                                );
                                let ident_with_id_array_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                    &ident_with_id_array_not_null_as_pg_json_type_select_ts
                                );
                                quote! {
                                    let v_174d33cd = #ValueSc.0.as_ref().map_or_else(
                                        #ident_with_id_array_not_null_as_pg_json_type_select_as_default_but_option_is_some_ts,
                                        Clone::clone
                                    );
                                    match #ident_with_id_array_not_null_as_pg_json_type_ts::#SelectQueryPartSc(
                                        &v_174d33cd,
                                        field_ident,
                                        column_name_and_maybe_field_getter,
                                        column_name_and_maybe_field_getter_for_er_message,
                                        true
                                    ) {
                                        Ok(v_d7bbd03c) => Ok(format!(#format_handle_ts)),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            }
                        },
                    },
                    &ident_where_ucc,
                    &ident_read_ucc,
                    &ident_read_only_ids_ucc,
                    &match &is_nullable {
                        IsNullable::False => {
                            let ts = {
                                let ts = {
                                    let acc_push_ts = get_vec_syn_field(match &pattern {
                                        Pattern::Standart => &is_standart_with_id_false,
                                        Pattern::Array => &is_standart_with_id_true
                                    }).iter().map(|el_a6a15738| {
                                        let field_ident = &el_a6a15738.field_ident;
                                        let format_handle_ts = dq_ts(&format!("jsonb_build_object('{field_ident}',{{}})||"));
                                        let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_a6a15738.field_type);
                                        let ts = match &pattern {
                                            Pattern::Standart => {
                                                let format_ts = dq_ts(&format!("{{column_name_and_maybe_field_getter}}->'{field_ident}'"));
                                                quote! {&format!(#format_ts)}
                                            },
                                            Pattern::Array => dq_ts(&format!("elem->'{field_ident}'"))
                                        };
                                        gen_if_write_is_err_curly_braces_ts(
                                            &quote!{
                                                acc_2912b128,
                                                #format_handle_ts,
                                                match #field_type_as_pg_json_type_ts::#SelectOnlyIdsQueryPartSc(#ts) {
                                                    Ok(v_2317e0af) => v_2317e0af,
                                                    Err(#ErSc) => {
                                                        return Err(#ErSc);
                                                    }
                                                }
                                            },
                                            &return_err_query_part_er_write_into_buffer_ts
                                        )
                                    });
                                    quote! {{
                                        let mut acc_2912b128 = #StringTs::default();
                                        #(#acc_push_ts)*
                                        let _: Option<char> = acc_2912b128.pop();
                                        let _: Option<char> = acc_2912b128.pop();
                                        format!("jsonb_build_object('value',{acc_2912b128})")
                                    }}
                                };
                                match &pattern {
                                    Pattern::Standart => ts,
                                    Pattern::Array => {
                                        let format_handle_ts = dq_ts(
                                            &format!("jsonb_build_object('value',(select jsonb_agg({{}}) from jsonb_array_elements({{{ColumnNameAndMaybeFieldGetterSc}}}) as elem))")
                                        );
                                        quote! {format!(#format_handle_ts, #ts)}
                                    },
                                }
                            };
                            quote! {Ok(#ts)}
                        },
                        IsNullable::True => {
                            let ts: &dyn ToTokens = match &pattern {
                                Pattern::Standart => &ident_standart_not_null_as_pg_json_type_ts,
                                Pattern::Array => &ident_with_id_array_not_null_as_pg_json_type_ts,
                            };
                            let case_null_format_handle_ts = dq_ts(
                                &format!("jsonb_build_object('value',case when jsonb_typeof({{{ColumnNameAndMaybeFieldGetterSc}}})='null' then 'null'::jsonb else {{v_21000130}} end)")
                            );
                            quote! {
                                match #ts::#SelectOnlyIdsQueryPartSc(#ColumnNameAndMaybeFieldGetterSc) {
                                    Ok(v_21000130) => Ok(format!(#case_null_format_handle_ts)),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            }
                        }
                    },
                    &ident_read_inner_ucc,
                    &{
                        let gen_into_inner_ts = |ident_ts_c89f002f: &dyn ToTokens, parameters_ts: &dyn ToTokens|{
                            quote!{#ident_ts_c89f002f::into_inner(#parameters_ts)}
                        };
                        let gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                            let ident_ts_df0c096c: &dyn ToTokens = match &is_standart_with_id {
                                IsStandartWithId::False => &ident_read_inner_ucc,
                                IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                            };
                            let ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_d2c28655| {
                                let field_ident = &el_d2c28655.field_ident;
                                let ts = wrap_into_value_init_ts(&gen_into_inner_ts(
                                    &gen_type_as_pg_json_type_ts(&el_d2c28655.field_type),
                                    &quote!{v_6e5af985.#ValueSc},
                                ));
                                let parameter_ts: &dyn ToTokens = match &is_standart_with_id {
                                    IsStandartWithId::False => &ValueSc,
                                    IsStandartWithId::True => &quote!{el_34d57236},
                                };
                                quote! {#field_ident: #parameter_ts.#field_ident.map(|v_6e5af985| #ts)}
                            });
                            quote! {
                                #ident_ts_df0c096c {
                                    #(#ts),*
                                }
                            }
                        };
                        match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Standart => gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&IsStandartWithId::False),
                                Pattern::Array => {
                                    let ts = gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&IsStandartWithId::True);
                                    quote! {#ValueSc.0.into_iter().map(|el_34d57236|#ts).collect()}
                                },
                            },
                            IsNullable::True => {
                                let ident_ddcdad63 = gen_type_as_pg_json_type_ts(&match &pattern {
                                    Pattern::Standart => ident_standart_not_null_ucc,
                                    Pattern::Array => ident_array_not_null_ucc,
                                });
                                quote! {#ValueSc.0.map(#ident_ddcdad63::into_inner)}
                            }
                        }
                    },
                    &ident_update_ucc,
                    &ident_update_for_query_ucc,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let format_handle_ts = dq_ts(&format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{{JsonbSetPathSc}}}}}}}',{{{StdOptionOptionObjectAccSc}}})"));
                                let query_part_vrts_ts = vec_syn_field.iter().map(|el_ebd92dbf| {
                                    let vrt_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_ebd92dbf.field_ident.to_string());
                                    let field_ident_dq_ts = gen_field_ident_dq_ts(el_ebd92dbf);
                                    let field_type_as_crud_pg_json_type_from_field_ts = gen_field_type_as_crud_pg_json_type_from_field_ts(el_ebd92dbf);
                                    quote! {
                                        #ident_update_for_query_el_ucc::#vrt_ident_ucc_ts(v_3b3fae4c) => {
                                            match #field_type_as_crud_pg_json_type_from_field_ts::#UpdateQueryPartSc(
                                                &v_3b3fae4c.#ValueSc,
                                                &#StdOptionOptionObjectAccSc,
                                                &#gen_jsonb_set_target_sc(#field_ident_dq_ts),
                                                #field_ident_dq_ts,
                                                #IncrSc,
                                            ) {
                                                Ok(v_5edc1648) => {
                                                    #StdOptionOptionObjectAccSc = v_5edc1648;
                                                }
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        }
                                    }
                                });
                                let some_format_handle_ts = dq_ts(&format!("case when jsonb_typeof({{{JsonbSetTargetSc}}}) = 'object' then ({{{JsonbSetTargetSc}}})::jsonb else '{{{{}}}}'::jsonb end"));
                                quote! {
                                    let mut #StdOptionOptionObjectAccSc = format!(#some_format_handle_ts);
                                    #gen_jsonb_set_target_ts
                                    for el_157f4b80 in #ValueSc.0.to_vec() {
                                        match el_157f4b80 {
                                            #(#query_part_vrts_ts),*
                                        }
                                    }
                                    if #JsonbSetPathSc.is_empty() {
                                        Ok(#StdOptionOptionObjectAccSc)
                                    }
                                    else {
                                        Ok(format!(#format_handle_ts))
                                    }
                                }
                            },
                            IsNullable::True => gen_update_query_part_standart_nullable_ts(
                                &pg_type_or_pg_json_type_pg_type
                            )
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => gen_update_query_part_array_not_null_ts(
                                &pg_type_or_pg_json_type_pg_type
                            ),
                            IsNullable::True => quote! {
                                match &#ValueSc.0 {
                                    Some(v_3245b79f) => #ident_array_not_null_as_pg_json_type_ts::#UpdateQueryPartSc(
                                        v_3245b79f,
                                        jsonb_set_accumulator,
                                        jsonb_set_target,
                                        jsonb_set_path,
                                        #IncrSc,
                                    ),
                                    None => match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                                        Ok(v_87e08bec) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${v_87e08bec})")),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            },
                        },
                    },
                    &IsUpdateQueryPartSelfUpdateUsed::True,
                    &IsUpdateQueryPartJsonbSetTargetUsed::True,
                    &IsUpdateQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let update_query_bind_vrts_ts = vec_syn_field.iter().map(|el_9968a29b| {
                                    let vrt_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_9968a29b.field_ident.to_string());
                                    let field_type_as_crud_pg_json_type_from_field_ts = gen_field_type_as_crud_pg_json_type_from_field_ts(
                                        el_9968a29b
                                    );
                                    quote! {
                                        #ident_update_for_query_el_ucc::#vrt_ident_ucc_ts(v_b27f5b09) => {
                                            match #field_type_as_crud_pg_json_type_from_field_ts::#UpdateQueryBindSc(
                                                v_b27f5b09.#ValueSc,
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
                                    for el_f14dcf6b in #ValueSc.0.into_vec() {
                                        match el_f14dcf6b {
                                            #(#update_query_bind_vrts_ts),*
                                        }
                                    }
                                    Ok(#QuerySc)
                                }
                            },
                            IsNullable::True => quote! {
                                match #ValueSc.0 {
                                    Some(v_269a0d34) => #ident_standart_not_null_as_pg_json_type_ts::update_query_bind(
                                        v_269a0d34,
                                        #QuerySc
                                    ),
                                    None => if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(#self_as_pg_json_type_update_ts::new(None))) {
                                        Err(#ErSc.to_string())
                                    }
                                    else {
                                        Ok(#QuerySc)
                                    },
                                }
                            }
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => quote! {
                                for el_30541f69 in #ValueSc.#UpdateSc.into_vec() {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::query_bind_string_as_pg_text_update_for_query(
                                        el_30541f69.#IdSc,
                                        #QuerySc
                                    ) {
                                        Ok(v_7633dc9b) => {
                                            #QuerySc = v_7633dc9b;
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                    match #ident_standart_not_null_as_pg_json_type_ts::update_query_bind(
                                        el_30541f69.#FieldsSc,
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
                                for el_4b6f8c01 in #ValueSc.delete {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::query_bind_string_as_pg_text_update_for_query(
                                        el_4b6f8c01,
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
                                for el_a44eb132 in #ValueSc.#CreateSc {
                                    if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(el_a44eb132)) {
                                        return Err(#ErSc.to_string());
                                    }
                                }
                                Ok(#QuerySc)
                            },
                            IsNullable::True => quote! {
                                match #ValueSc.0 {
                                    Some(v_a2156b3e) => #ident_array_not_null_as_pg_json_type_ts::update_query_bind(
                                        v_a2156b3e,
                                        #QuerySc
                                    ),
                                    None => if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(#self_as_pg_json_type_update_ts::new(None))) {
                                        Err(#ErSc.to_string())
                                    }
                                    else {
                                        Ok(#QuerySc)
                                    },
                                }
                            },
                        },
                    },
                    &quote!{
                        match #ValueSc.#SelectOnlyUpdatedIdsQueryPartSc(
                            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                            #IncrSc
                        ) {
                            Ok(v_e137951b) => Ok(format!("'{field_ident}',jsonb_build_object('value',{v_e137951b}),")),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    },
                    &IsSelectOnlyUpdatedIdsQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let match_ts = vec_syn_field.iter().map(|el_e3bd5731| {
                                    let field_ident = &el_e3bd5731.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_e3bd5731.field_type);
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    quote! {
                                        #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(v_b79c2851) => {
                                            match #field_type_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(
                                                &v_b79c2851.#ValueSc,
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
                                    for el_31dd08ee in #ValueSc.0.to_vec() {
                                        match el_31dd08ee {
                                            #(#match_ts),*
                                        }
                                    }
                                    Ok(#QuerySc)
                                }
                            },
                            IsNullable::True => quote!{
                                if let Some(v_6334d77d) = &#ValueSc.0 {
                                    match #ident_standart_not_null_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(v_6334d77d, #QuerySc) {
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
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => {
                                let select_only_created_ids_query_bind_ts = vec_syn_field_with_id.iter().map(|el_27e0de74| {
                                    let field_ident = &el_27e0de74.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_27e0de74.field_type);
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryBindSc(
                                            &el_5fba4c1f.#field_ident,
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
                                    for el_e5af9b26 in #ValueSc.#UpdateSc.to_vec() {
                                        match #import_path_pg_json_type_uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(
                                            &el_e5af9b26.#IdSc,
                                            #QuerySc
                                        ) {
                                            Ok(v_0fd735de) => {
                                                #QuerySc = v_0fd735de;
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                        match #ident_standart_not_null_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(
                                            &el_e5af9b26.fields,
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
                                    for el_5fba4c1f in &#ValueSc.create {
                                        #(#select_only_created_ids_query_bind_ts)*
                                    }
                                    for el_d9eff5ca in #ValueSc.#UpdateSc.to_vec() {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::query_bind_string_as_pg_text_update_for_query(
                                            el_d9eff5ca.#IdSc.clone(),
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
                                    for el_ae971756 in &#ValueSc.#CreateSc {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::query_bind_string_as_pg_text_create_for_query(
                                            el_ae971756.#IdSc.clone(),
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
                                if let Some(v_107e6639) = &#ValueSc.0 {
                                    match #ident_array_not_null_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(v_107e6639, #QuerySc) {
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
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field.iter().map(|el_6bcf3221| {
                                    let field_ident = &el_6bcf3221.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_6bcf3221.field_type);
                                    let field_ident_dq_ts = &dq_ts(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_dq_ts = &dq_ts(
                                        &format!("{{{ColumnNameAndMaybeFieldGetterSc}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0fe559fa, "jsonb_build_object({v_cddc8a0a})||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryPartSc(
                                            &#ValueSc.#field_ident,
                                            #field_ident_dq_ts,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_dq_ts),
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
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
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
                                let ts = vec_syn_field.iter().map(|el_88c65ca5| {
                                    let field_ident = &el_88c65ca5.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_88c65ca5.field_type);
                                    let field_ident_dq_ts = &dq_ts(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_dq_ts = &dq_ts(
                                        &format!("{{{ColumnNameAndMaybeFieldGetterSc}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0e9170a3, "jsonb_build_object({v_93015133})||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryPartSc(
                                            &v_90219286.#field_ident,
                                            #field_ident_dq_ts,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_dq_ts),
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
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#ValueSc.0 {
                                            Some(v_90219286) => format!(
                                                "jsonb_build_object('value',{})",
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
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field_with_id.iter().map(|el_bfecacfd| {
                                    let field_ident = &el_bfecacfd.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_bfecacfd.field_type);
                                    let field_ident_dq_ts = &dq_ts(&field_ident);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0f2b92d0, "jsonb_build_object({v_6d76c065})||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryPartSc(
                                            &el_3c1dab62.#field_ident,
                                            #field_ident_dq_ts,
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
                                    &return_err_query_part_er_write_into_buffer_ts
                                );
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
                                        {
                                            let mut acc_0f2b92d0 = #StringTs::new();
                                            for el_3c1dab62 in &#ValueSc.0 {
                                                #(#ts)*
                                            }
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            format!("jsonb_build_object('value',{acc_0f2b92d0})")
                                        },
                                        &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                        {
                                            let mut acc_44b1f772 = #StringTs::new();
                                            for _ in &#ValueSc.0 {
                                                match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
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
                                let ts = vec_syn_field_with_id.iter().map(|el_76f33159| {
                                    let field_ident = &el_76f33159.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_76f33159.field_type);
                                    let field_ident_dq_ts = &dq_ts(&field_ident);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_1a91bdc7, "jsonb_build_object({v_d49fe9d8})||"},
                                        &return_err_query_part_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryPartSc(
                                            &el_9bdcd847.#field_ident,
                                            #field_ident_dq_ts,
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
                                    &return_err_query_part_er_write_into_buffer_ts
                                );
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#ValueSc.0 {
                                            Some(v_3c415c92) => format!(
                                                "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                                                {
                                                    let mut acc_1a91bdc7 = #StringTs::new();
                                                    for el_9bdcd847 in &v_3c415c92.0 {
                                                        #(#ts)*
                                                    }
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    format!("jsonb_build_object('value',{acc_1a91bdc7})")
                                                },
                                                &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                                {
                                                    let mut acc_857ce631 = #StringTs::new();
                                                    for _ in &v_3c415c92.0 {
                                                        match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
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
                    &IsSelectOnlyCreatedIdsQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field.iter().map(|el_9d72fe6e| {
                                    let field_ident = &el_9d72fe6e.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_9d72fe6e.field_type);
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryBindSc(
                                            &#ValueSc.#field_ident,
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
                                    if let Some(v_a1ccd526) = &#ValueSc.0 {
                                        match #ident_standart_not_null_as_import_path_pg_json_type_ts::#SelectOnlyCreatedIdsQueryBindSc(
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
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field_with_id.iter().map(|el_43b720bb| {
                                    let field_ident = &el_43b720bb.field_ident;
                                    let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&el_43b720bb.field_type);
                                    quote! {
                                        match #field_type_as_pg_json_type_ts::#SelectOnlyCreatedIdsQueryBindSc(&el_9bdcd847.#field_ident, #QuerySc) {
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
                                    for el_9bdcd847 in &#ValueSc.0 {
                                        #(#ts)*
                                    }
                                    for el_b191a891 in &#ValueSc.0 {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::query_bind_string_as_pg_text_create_for_query(
                                            el_b191a891.#IdSc.clone(),
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
                                    if let Some(v_0b55a65a) = &#ValueSc.0 {
                                        match #ident_array_not_null_as_import_path_pg_json_type_ts::#SelectOnlyCreatedIdsQueryBindSc(v_0b55a65a, #QuerySc) {
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
                    &ImportPath::PgCrud,
                    &ident,
                    &ident_table_type_declaration_ucc,
                    &IsPrimaryKeyUnderscore::True,
                    &{
                        let format_handle_ts = dq_ts(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_owned());
                        quote! {
                            format!(#format_handle_ts, serde_json::to_string(&schemars::schema_for!(#ident_table_type_declaration_ucc)).expect("59a1654b"))
                        }
                    },
                    &ident_create_ucc,
                    &CreateQueryPartValueUnderscore::True,
                    &CreateQueryPartIncrUnderscore::False,
                    &quote!{
                        match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    },
                    &CreateQueryBindValueUnderscore::False,
                    &IsCreateQueryBindMutable::True,
                    &quote!{
                        if let Err(#ErSc) = #QuerySc.try_bind(
                            #self_as_pg_json_type_create_for_query_ts::from(#ValueSc)
                        ) {
                            return Err(#ErSc.to_string());
                        }
                        Ok(#QuerySc)
                    },
                    &ident_select_ucc,
                    &SelectQueryPartValueUnderscore::False,
                    &quote! {
                        match #ValueSc.#SelectQueryPartPgTypeSc(#ColumnSc) {
                            Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {column}")),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    },
                    &ident_where_ucc,
                    &ident_read_ucc,
                    &ValueSc,
                    &ident_read_only_ids_ucc,
                    &quote! {
                        match #self_as_pg_json_type_ts::#SelectOnlyIdsQueryPartSc(#ColumnSc) {
                            Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {column},")),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    },
                    &ident_read_inner_ucc,
                    &quote!{#self_as_pg_json_type_ts::into_inner(#ValueSc)},
                    &ident_update_ucc,
                    &ident_update_for_query_ucc,
                    &UpdateQueryPartValueUnderscore::False,
                    &UpdateQueryPartJsonbSetAccumulatorUnderscore::False,
                    &UpdateQueryPartJsonbSetTargetUnderscore::False,
                    &UpdateQueryPartJsonbSetPathUnderscore::False,
                    &match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => quote!{#self_as_pg_json_type_ts::#UpdateQueryPartSc(
                                value,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                incr
                            )},
                            IsNullable::True => gen_update_query_part_standart_nullable_ts(
                                &pg_type_or_pg_json_type_pg_json_type
                            )
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => gen_update_query_part_array_not_null_ts(
                                &pg_type_or_pg_json_type_pg_json_type
                            ),
                            IsNullable::True => {
                                let ts = gen_update_delete_create_array_ts(&quote!{
                                    "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
                                });
                                quote! {
                                    match &#ValueSc.0 {
                                        Some(v_58d685d3) => {
                                            #ts
                                        },
                                        None => match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                                            Ok(v_d31ab6f0) => Ok(format!("${v_d31ab6f0}")),
                                            Err(#ErSc) => Err(#ErSc)
                                        }
                                    }
                                }
                            },
                        },
                    },
                    &IsUpdateQueryBindMutable::False,
                    &quote!{#self_as_pg_json_type_ts::#UpdateQueryBindSc(
                        #ValueSc,
                        #QuerySc
                    )},
                    &quote!{
                        match #ValueSc.#SelectOnlyUpdatedIdsQueryPartSc(
                            #ColumnSc,
                            #IncrSc
                        ) {
                            Ok(v_f0787243) => Ok(format!("jsonb_build_object('value',{v_f0787243}) as {column},")),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    },
                    &IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                    &quote!{#self_as_pg_json_type_ts::#SelectOnlyUpdatedIdsQueryBindSc(
                        #ValueSc,
                        #QuerySc
                    )},
                );
                match &trait_gen {
                    TraitGen::PgTypeAndPgJsonType => (impl_pg_crud_pg_json_type_for_ident_ts, impl_pg_crud_pg_types_pg_type_pg_type_ts),
                    TraitGen::PgJsonType => (impl_pg_crud_pg_json_type_for_ident_ts, Ts2::new()),
                }
            };
            let self_pg_json_type_ts = quote!{#SelfUcc::#PgJsonTypeUcc};
            let (impl_pg_json_type_test_cases_for_ident_ts, impl_pg_type_test_cases_for_ident_ts) = {
                let gen_dim_equal_ts = |dim: &Dim|{
                    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc = dim.read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc();
                    let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                        match #import_path::NotEmptyUniqueVec::try_new(
                            match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                (Some(read_only_ids_cdcb6239), Some(create_fdd53941)) => match <
                                    #ts
                                    as
                                    #import_path::PgJsonTypeTestCases
                                >::#read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
                                    read_only_ids_cdcb6239,
                                    create_fdd53941
                                ) {
                                    Some(v_d6124e21) => {
                                        let mut acc_bd78dc08 = Vec::new();
                                        for el_6739e82f in v_d6124e21.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![el_6739e82f]
                                            ) {
                                                Ok(v_7ed84f3b) => {
                                                    acc_bd78dc08.push(
                                                        #import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_7ed84f3b))
                                                    );
                                                },
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("23dca12f")
                                                }
                                            }
                                        }
                                        let v_e48110ec = #import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_d6124e21));
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
                                (None, None) => vec![#import_path::NullableJsonObjectPgTypeWhereFilter(None)]
                            }
                        ) {
                            Ok(v_55f2dc3d) => Some(v_55f2dc3d),
                            Err(er) => match er {
                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("88912e24")
                            }
                        }
                    };
                    match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field.iter().map(|el_3a1eac56| {
                                    let field_ident = &el_3a1eac56.field_ident;
                                    let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_3a1eac56.field_type);
                                    quote! {
                                        if let Some(v_2bbd2c96) = #field_type_as_pg_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
                                            #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                            #CreateSc.#field_ident
                                        ) {
                                            for el_84537322 in v_2bbd2c96.clone().into_vec() {
                                                acc_2fe1cca8.push(
                                                    #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PgTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![el_84537322]
                                                        ).expect("9a25e058")
                                                    )
                                                );
                                            }
                                            let v_c45bab0d = #ident_where_ucc::#field_ident_ucc(
                                                #import_path::PgTypeWhere::new(
                                                    #import_path::LogicalOperator::And,
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
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_2fe1cca8 = Vec::new();
                                        #(#ts)*
                                        acc_2fe1cca8
                                    }) {
                                        Ok(v_a5fa471d) => Some(v_a5fa471d),
                                        Err(er) => match er {
                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("89e719cf")
                                        }
                                    }
                                }
                            }
                            IsNullable::True => gen_nullable_ts(&ident_standart_not_null_ucc)
                        },
                        Pattern::Array => match &is_nullable {
                            IsNullable::False => {
                                let ts_f0710cd9 = {
                                    let ts_57d244f8 = vec_syn_field.iter().map(|el_18682ae5| {
                                        let field_ident = &el_18682ae5.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_18682ae5.field_type);
                                        quote! {
                                            if let Some(v_bf84026e) = #field_type_as_pg_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
                                                read_only_ids_420d38ca.0.#ValueSc.#field_ident.clone(),
                                                create_76f032c1.#field_ident.clone()
                                            ) {
                                                for el_f6be06df in v_bf84026e.clone().into_vec() {
                                                    let v_592e6b5f = #ident_where_ucc::#el_field_ident_ucc(
                                                        #import_path::PgTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![el_f6be06df]
                                                        ).expect("1f7ae335")
                                                    );
                                                    if !acc_dd377eb1.contains(&v_592e6b5f) {
                                                        acc_dd377eb1.push(v_592e6b5f);
                                                    }
                                                }
                                                let v_03205172 = #ident_where_ucc::#el_field_ident_ucc(
                                                    #import_path::PgTypeWhere::new(
                                                        #import_path::LogicalOperator::And,
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
                                            let ts_91a09fe2 = vec_syn_field.iter().map(|el_a83927c8| {
                                                let field_ident = &el_a83927c8.field_ident;
                                                let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_a83927c8.field_type);
                                                quote! {
                                                    #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                        read_only_ids_420d38ca.0.#ValueSc.#field_ident,
                                                        create_76f032c1.#field_ident
                                                    )
                                                }
                                            });
                                            quote!{
                                                acc_dd377eb1.push(#ident_where_ucc::DimOneEqual(#import_path::PgJsonTypeWhereDimOneEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    dims: #import_path::BoundedStdVecVec::try_from(
                                                        vec![
                                                            #import_path::UnsignedPartOfI32::try_from(
                                                                i32::try_from(index_47620dcf).expect("5341936f")
                                                            ).expect("76906f3c")
                                                        ]
                                                    ).expect("8a624c70"),
                                                    #ValueSc: #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                        <#uuid_uuid_as_not_null_jsonb_string_ts as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                            read_only_ids_420d38ca.0.#ValueSc.#IdSc,
                                                            #PgCrudDefaultOptionSomeVecOneElCall
                                                        ),
                                                        #(#ts_91a09fe2),*
                                                    ),
                                                }));
                                            }
                                        };
                                        quote!{
                                            for (index_47620dcf, (read_only_ids_420d38ca, create_76f032c1)) in #ReadOnlyIdsSc.0.#ValueSc.into_iter()
                                                .zip(#CreateSc.0.into_iter())
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
                                        for (read_only_ids_420d38ca, create_76f032c1) in #ReadOnlyIdsSc.0.#ValueSc.into_iter()
                                            .zip(#CreateSc.0.into_iter())
                                        {
                                            #ts_f0710cd9
                                        }
                                    },
                                };
                                quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_dd377eb1 = Vec::new();
                                        #ts_2cc4a40e
                                        acc_dd377eb1
                                    }) {
                                        Ok(v_dfac36e4) => Some(v_dfac36e4),
                                        Err(er) => match er {
                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("93390f1a")
                                        },
                                    }
                                }
                            }
                            IsNullable::True => gen_nullable_ts(&ident_array_not_null_ucc)
                        },
                    }
                };
                (
                    {
                        let option_vec_create_ts = {
                            let ts = match &is_nullable {
                                IsNullable::False => match &pattern {
                                    Pattern::Standart => {
                                        let ts = vec_syn_field.iter().map(|el_4f2f70d2| {
                                            let field_ident = &el_4f2f70d2.field_ident;
                                            let field_type_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_4f2f70d2.field_type);
                                            let parameters_ts = vec_syn_field.iter().map(|el_value| {
                                                let field_ident_58f0ef7c = &el_value.field_ident;
                                                if field_ident == field_ident_58f0ef7c {
                                                    quote! {el_37154498}
                                                } else {
                                                    quote! {
                                                        #PgCrudDefaultOptionSomeVecOneElCall
                                                    }
                                                }
                                            });
                                            quote! {
                                                if let Some(v_0296b347) = #field_type_type_as_pg_json_type_test_cases_ts::#OptionVecCreateSc() {
                                                    for el_37154498 in v_0296b347 {
                                                        let #ValueSc = #self_as_pg_json_type_create_ts::new(
                                                            #(#parameters_ts),*
                                                        );
                                                        if !acc_ccd79a32.contains(&#ValueSc) {
                                                            acc_ccd79a32.push(#ValueSc);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {#(#ts)*}
                                    },
                                    Pattern::Array => {
                                        let ts = vec_syn_field.iter().map(|el_ddefdb90| {
                                            let field_ident = &el_ddefdb90.field_ident;
                                            let field_type_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_ddefdb90.field_type);
                                            let (
                                                option_additional_parameters_ts,
                                                parameters_ts
                                            ) = {
                                                #[derive(Clone)]
                                                enum ShouldAddDotClone {
                                                    False,
                                                    True,
                                                }
                                                let gen_parameters_ts = |
                                                    should_add_dot_clone: ShouldAddDotClone,
                                                    el_ts: &dyn ToTokens,
                                                |{
                                                    vec_syn_field.iter().map(|el_value| {
                                                        let field_ident_4fd46df4 = &el_value.field_ident;
                                                        if field_ident == field_ident_4fd46df4 {
                                                            let maybe_dot_clone_ts = match should_add_dot_clone.clone() {
                                                                ShouldAddDotClone::False => Ts2::new(),
                                                                ShouldAddDotClone::True => quote! { .clone() },
                                                            };
                                                            quote! {#el_ts #maybe_dot_clone_ts}
                                                        } else {
                                                            quote! {#PgCrudDefaultOptionSomeVecOneElCall}
                                                        }
                                                    }).collect::<Vec<Ts2>>()
                                                };
                                                (
                                                    gen_parameters_ts(
                                                        ShouldAddDotClone::True,
                                                        &quote!{el_37154498}
                                                    ),
                                                    gen_parameters_ts(
                                                        ShouldAddDotClone::False,
                                                        &quote!{el_37154498}
                                                    )
                                                )
                                            };
                                            quote! {
                                                if let Some(vec_create) = #field_type_type_as_pg_json_type_test_cases_ts::#OptionVecCreateSc() {
                                                    let mut acc_6a886d56 = Vec::new();
                                                    let option_additional = {
                                                        let mut option_additional = None;
                                                        for el_37154498 in &vec_create {
                                                            if option_additional.is_none() {
                                                                let #ValueSc = #ident_with_id_standart_not_null_create_ucc::new(
                                                                    #(#option_additional_parameters_ts),*
                                                                );
                                                                option_additional = Some((
                                                                    #ident_create_ucc::new(vec![#ValueSc.clone()]),
                                                                    #ident_create_ucc::new(vec![#ValueSc.clone(), #ValueSc])
                                                                ));
                                                            }
                                                            else {
                                                                break;
                                                            }
                                                        }
                                                        option_additional
                                                    };
                                                    let has_len_greater_than_one = vec_create.len() > 1;
                                                    for el_37154498 in vec_create {
                                                        acc_6a886d56.push(#ident_with_id_standart_not_null_create_ucc::new(
                                                            #(#parameters_ts),*
                                                        ));
                                                    }
                                                    {
                                                        let v_07c0c08c = #ident_create_ucc::new(acc_6a886d56);
                                                        if !acc_ccd79a32.contains(&v_07c0c08c) {
                                                            acc_ccd79a32.push(v_07c0c08c);
                                                        }
                                                    }
                                                    if let Some(v_f6686d5d) = option_additional {
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
                                        ident_not_null_as_pg_json_type_test_cases_ts_e690553a,
                                        ts
                                    ): (
                                        &dyn ToTokens,
                                        &dyn ToTokens
                                    ) = match &pattern {
                                        Pattern::Standart => (
                                            &ident_standart_not_null_as_pg_json_type_test_cases_ts,
                                            &Ts2::new()
                                        ),
                                        Pattern::Array => (
                                            &ident_array_not_null_as_pg_json_type_test_cases_ts,
                                            &quote!{.0}
                                        ),
                                    };
                                    quote! {
                                        if let Some(v_399e6a50) = #ident_not_null_as_pg_json_type_test_cases_ts_e690553a::#OptionVecCreateSc() {
                                            for el_e2767811 in v_399e6a50 {
                                                let #ValueSc = #self_as_pg_json_type_ts::Create::new(Some(el_e2767811 #ts));
                                                if !acc_ccd79a32.contains(&#ValueSc) {
                                                    acc_ccd79a32.push(#ValueSc);
                                                }
                                            }
                                        }
                                        {
                                            let #ValueSc = #self_as_pg_json_type_ts::Create::new(None);
                                            if !acc_ccd79a32.contains(&#ValueSc) {
                                                acc_ccd79a32.push(#ValueSc);
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
                        let read_only_ids_to_two_dimal_vec_read_inner_ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let fields_last_init_ts = {
                                        if vec_syn_field.len() == 1 {
                                            Ts2::new()
                                        }
                                        else {
                                            let ts = vec_syn_field.iter().map(|el_43e09b9b| {
                                                let field_ident = &el_43e09b9b.field_ident;
                                                let field_ident_last_sc = SelfLastSc::from_display(&field_ident);
                                                let field_type_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_43e09b9b.field_type);
                                                quote! {
                                                    let mut #field_ident_last_sc = #field_type_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsIntoOptionValueReadInnerSc(
                                                        read_only_ids.0.value.#field_ident.clone()
                                                    );
                                                }
                                            });
                                            quote!{#(#ts)*}
                                        }
                                    };
                                    let ts = vec_syn_field.iter().map(|el_9b199f7f| {
                                        let field_ident = &el_9b199f7f.field_ident;
                                        let field_ident_current_sc = SelfCurrentSc::from_display(&field_ident);
                                        let field_ident_last_sc = SelfLastSc::from_display(&field_ident);
                                        let maybe_field_ident_last_clone_from_field_ident_current = if vec_syn_field.len() == 1 {
                                            Ts2::new()
                                        }
                                        else {
                                            quote!{#field_ident_last_sc.clone_from(&#field_ident_current_sc);}
                                        };
                                        let fields_ts = vec_syn_field.iter().map(|el_value| {
                                            let field_ident_84b52e71 = &el_value.field_ident;
                                            let field_ident_84b52e71_current_sc = SelfCurrentSc::from_display(&field_ident_84b52e71);
                                            let field_ident_84b52e71_last_sc = SelfLastSc::from_display(&field_ident_84b52e71);
                                            let ts: &dyn ToTokens = if field_ident == field_ident_84b52e71 {
                                                &field_ident_84b52e71_current_sc
                                            } else {
                                                &field_ident_84b52e71_last_sc
                                            };
                                            quote! {#field_ident_84b52e71: #ts.clone()}
                                        });
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_9b199f7f.field_type);
                                        let value_ts = wrap_into_value_init_ts(&quote!{el_2720df8a});
                                        quote! {
                                            for el_7bf83754 in #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(&#ReadOnlyIdsSc.0.value.#field_ident) {
                                                for el_2720df8a in el_7bf83754 {
                                                    let #field_ident_current_sc = Some(#value_ts);
                                                    #maybe_field_ident_last_clone_from_field_ident_current
                                                    acc_ef081dc3.push(
                                                        vec![
                                                            #ident_standart_not_null_read_inner_ucc {
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
                                        #ReadOnlyIdsSc.0.#ValueSc.as_ref().into_iter().flat_map(|v_5fa0668c| {
                                            #ident_standart_not_null_as_pg_json_type_test_cases_ts::
                                                #ReadOnlyIdsToTwoDimalVecReadInnerSc(v_5fa0668c)
                                                .into_iter()
                                                .flat_map(|element0| {
                                                    element0.into_iter().map(|element1| vec![Some(element1)])
                                                })
                                        })
                                        .chain(std::iter::once(vec![None]))
                                        .collect()
                                    }
                                }
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let ts = vec_syn_field.iter().map(|el_bb247316| {
                                        let field_ident = &el_bb247316.field_ident;
                                        let fields_ts = vec_syn_field.iter().map(|el_value| {
                                            let field_ident_dd46e0cb = &el_value.field_ident;
                                            if field_ident == field_ident_dd46e0cb {
                                                let value_ts = wrap_into_value_init_ts(&quote!{el_18d1f553});
                                                quote! {
                                                    #field_ident_dd46e0cb: Some(#value_ts)
                                                }
                                            } else {
                                                let field_type_as_pg_json_type_test_cases_ts_64dc25bd = gen_type_as_pg_json_type_test_cases_ts(&el_value.field_type);
                                                quote! {
                                                    #field_ident_dd46e0cb: #field_type_as_pg_json_type_test_cases_ts_64dc25bd::#ReadOnlyIdsIntoOptionValueReadInnerSc(
                                                        el_49a5bb62.0.#ValueSc.#field_ident_dd46e0cb.clone()
                                                    )
                                                }
                                            }
                                        });
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_bb247316.field_type);
                                        let value_ts = wrap_into_value_init_ts(&quote!{el_49a5bb62.0.#ValueSc.#IdSc.0.#ValueSc});
                                        quote! {
                                            for el_4b4da5aa in #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(
                                                &el_49a5bb62.0.#ValueSc.#field_ident.clone()
                                            ) {
                                                for el_18d1f553 in el_4b4da5aa {
                                                    acc_00b3df88.push(
                                                        vec![
                                                            #ident_with_id_standart_not_null_read_inner_ucc {
                                                                #IdSc: Some(#value_ts),
                                                                #(#fields_ts),*
                                                            }
                                                        ]
                                                    );
                                                }
                                            }
                                        }
                                    });
                                    quote! {
                                        #ReadOnlyIdsSc.0.#ValueSc.iter().map(|el_49a5bb62|{
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
                                        if let Some(v_6ee5750e) = &#ReadOnlyIdsSc.0.#ValueSc {
                                            for el_4a5a4b09 in #ident_array_not_null_as_pg_json_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(v_6ee5750e) {
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
                        let read_inner_into_read_with_new_or_try_new_unwraped_ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_type_ts, &PgTypeSubtype::Read);
                                    let parameters_ts = vec_syn_field.iter().map(|el_13640e7f| {
                                        let field_ident = &el_13640e7f.field_ident;
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_13640e7f.field_type);
                                        let value_ts = wrap_into_value_init_ts(&quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(v_8ff65e09.#ValueSc)
                                        });
                                        quote! {#ValueSc.#field_ident.map(|v_8ff65e09|#value_ts)}
                                    });
                                    quote! {#self_el_as_pg_type_read_ts::try_new(#(#parameters_ts),*).expect("3aeeabba")}
                                }
                                IsNullable::True => {
                                    let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_type_ts, &PgTypeSubtype::Read);
                                    quote! {
                                        #self_el_as_pg_type_read_ts::new(
                                            #ValueSc.map(#ident_standart_not_null_as_pg_json_type_test_cases_ts::#ReadInnerIntoReadWithNewOrTryNewUnwrapedSc)
                                        )
                                    }
                                }
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let ts = vec_syn_field_with_id.iter().map(|el_96f7b50a| {
                                        let field_ident = &el_96f7b50a.field_ident;
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_96f7b50a.field_type);
                                        let value_ts = wrap_into_value_init_ts(&quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(v_3ac52220.#ValueSc)
                                        });
                                        quote! {#field_ident: el_ffed1bfc.#field_ident.map(|v_3ac52220|#value_ts)}
                                    });
                                    quote!{
                                        #ident_read_ucc::new(
                                            #ValueSc.into_iter().map(|el_ffed1bfc| #ident_with_id_standart_not_null_read_ucc {
                                                #(#ts),*
                                            }).collect()
                                        )
                                    }
                                }
                                IsNullable::True => {
                                    let ts = vec_syn_field_with_id.iter().map(|el_e47b9709| {
                                        let field_ident = &el_e47b9709.field_ident;
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_e47b9709.field_type);
                                        // let maybe_dot_clone_ts = if vec_syn_field.len() == 1 {
                                        //     Ts2::new()
                                        // }
                                        // else {
                                        //     quote!{.clone()}
                                        // };
                                        let value_ts = wrap_into_value_init_ts(&quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(
                                                el_5c1f7f63.#ValueSc
                                                // #maybe_dot_clone_ts
                                                .clone()
                                            )
                                        });
                                        quote! {
                                            #field_ident: el_ffed1bfc.#field_ident.as_ref().map(|el_5c1f7f63| #value_ts)
                                        }
                                    });
                                    let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_type_ts, &PgTypeSubtype::Read);
                                    quote! {
                                        #self_el_as_pg_type_read_ts::new(
                                            #ValueSc.map(|v_189e3c07|
                                                v_189e3c07
                                                .into_iter()
                                                .map(|el_ffed1bfc|#ident_with_id_standart_not_null_read_ucc {
                                                    #(#ts),*
                                                }).collect()
                                            )
                                        )
                                    }
                                }
                            },
                        };
                        let read_inner_into_update_with_new_or_try_new_unwraped_ts = match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Standart => {
                                    let self_el_as_pg_type_update_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_type_ts, &PgTypeSubtype::Update);
                                    let parameters_ts = vec_syn_field.iter().map(|el_cefffeeb| {
                                        let field_ident = &el_cefffeeb.field_ident;
                                        let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_cefffeeb.field_type);
                                        let value_ts = wrap_into_value_init_ts(&quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc(el_23bdfe1e.#ValueSc)
                                        });
                                        quote! {
                                            acc_ebea163e.extend(#ValueSc.#field_ident.map(|el_23bdfe1e| {
                                                #ident_standart_not_null_update_el_ucc::#field_ident_ucc(#value_ts)
                                            }));
                                        }
                                    });
                                    quote! {
                                        #self_el_as_pg_type_update_ts::new(
                                            #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_ebea163e = Vec::new();
                                                #(#parameters_ts)*
                                                acc_ebea163e
                                            }).expect("a06dbdc5")
                                        )
                                    }
                                },
                                Pattern::Array => {
                                    let fields_ts = vec_syn_field.iter().map(|el_d13faa4c| {
                                        let field_ident = &el_d13faa4c.field_ident;
                                        quote! {#field_ident: el_ffed1bfc.#field_ident}
                                    });
                                    quote! {
                                        #ident_update_ucc::try_new(
                                            Vec::new(),
                                            #import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts::try_new(
                                                #ValueSc.into_iter().map(|el_ffed1bfc| #ident_with_id_standart_not_null_update_el_ucc {
                                                    #IdSc: #uuid_uuid_as_not_null_jsonb_string_update_ucc::new(el_ffed1bfc.#IdSc.clone().expect("f04a2c6d").#ValueSc),
                                                    fields: #ident_standart_not_null_as_pg_json_type_test_cases_ts::#ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc(
                                                        #ident_standart_not_null_read_inner_ucc {
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
                                    Pattern::Standart => &ident_standart_not_null_ucc,
                                    Pattern::Array => &ident_with_id_array_not_null_ucc,
                                });
                                let self_el_as_pg_type_update_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_type_ts, &PgTypeSubtype::Update);
                                quote! {
                                    #self_el_as_pg_type_update_ts::new(
                                        #ValueSc.map(#ts::#ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc)
                                    )
                                }
                            },
                        };
                        let read_only_ids_into_option_value_read_inner_ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => gen_fields_read_only_ids_into_option_value_read_inner_ts(&is_standart_with_id_false, &ValueSc),
                                IsNullable::True => {
                                    let value_ts = wrap_into_value_init_ts(&quote!{
                                        #ValueSc.0.#ValueSc.and_then(|v_5d7e3961| match #ident_standart_not_null_as_pg_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                                            v_5d7e3961
                                        ) {
                                            Some(v_cfca0099) => Some(v_cfca0099.#ValueSc),
                                            None => None,
                                        })
                                    });
                                    quote! {Some(#value_ts)}
                                }
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let value_ts = wrap_into_value_init_ts(&{
                                        let ts = vec_syn_field_with_id.iter().map(|el_3ebdd830| {
                                            let field_ident = &el_3ebdd830.field_ident;
                                            let field_type = &el_3ebdd830.field_type;
                                            let field_type_as_pg_json_type_ts = gen_type_as_pg_json_type_ts(&field_type);
                                            let field_type_as_pg_json_type_read_ts = gen_type_as_pg_json_type_subtype_ts(&field_type, &PgJsonTypeSubtype::Read);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            let value_ts = wrap_into_value_init_ts(&{
                                                let default_but_option_is_some_call_ts_a3f714b3 = gen_ident_as_default_but_option_is_some_call_ts(
                                                    &field_type_as_pg_json_type_read_ts
                                                );
                                                quote!{#field_type_as_pg_json_type_ts::into_inner(#default_but_option_is_some_call_ts_a3f714b3)}
                                            });
                                            quote! {
                                                #field_ident: #field_type_as_pg_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                                                    el_6603f209.0.#ValueSc.#field_ident
                                                ).map_or_else(|| Some(#value_ts), Some)
                                            }
                                        });
                                        quote!{
                                            #ValueSc.0.#ValueSc.into_iter().fold(Vec::new(), |mut acc_cf4743b1, el_6603f209| {
                                                acc_cf4743b1.push(#ident_with_id_standart_not_null_read_inner_ucc {
                                                    #(#ts),*
                                                });
                                                acc_cf4743b1
                                            })
                                        }
                                    });
                                    quote! {Some(#value_ts)}
                                }
                                IsNullable::True => {
                                    let value_ts = wrap_into_value_init_ts(&quote!{
                                        #ValueSc.0.#ValueSc.and_then(|v_f816032d| match #ident_array_not_null_as_pg_json_type_test_cases_ts::#ReadOnlyIdsIntoOptionValueReadInnerSc(
                                            v_f816032d
                                        ) {
                                            Some(v_d0549423) => Some(v_d0549423.#ValueSc),
                                            None => None,
                                        })
                                    });
                                    quote! {Some(#value_ts)}
                                }
                            },
                        };
                        let update_to_read_only_ids_ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let fields_init_ts = vec_syn_field.iter().map(|el_3f07f901| {
                                        let field_ident = &el_3f07f901.field_ident;
                                        quote! {let mut #field_ident = None;}
                                    });
                                    let match_ts = vec_syn_field.iter().map(|el_4fb1f3d0| {
                                        let field_ident = &el_4fb1f3d0.field_ident;
                                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_4fb1f3d0.field_type);
                                        quote! {
                                            #ident_update_el_ucc::#field_ident_ucc_ts(v_0f4d677e) => {
                                                #field_ident = Some(#field_type_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc(&v_0f4d677e.#ValueSc));
                                            }
                                        }
                                    });
                                    let struct_fields_ts = vec_syn_field.iter().map(|el_af4d3d80| {
                                        let field_ident = &el_af4d3d80.field_ident;
                                        quote! {#field_ident: #field_ident.expect("106f16f2")}
                                    });
                                    let value_ts = wrap_into_value_init_ts(&quote!{
                                        #ident_read_only_ids_handle_ucc{
                                            #(#struct_fields_ts),*
                                        }
                                    });
                                    quote! {
                                        #(#fields_init_ts)*
                                        for el_b3974846 in #ValueSc.0.to_vec() {
                                            match el_b3974846 {
                                                #(#match_ts),*
                                            }
                                        }
                                        #ident_read_only_ids_ucc(#value_ts)
                                    }
                                }
                                IsNullable::True => {
                                    let value_ts = wrap_into_value_init_ts(&{
                                        quote!{
                                            #ValueSc.0.as_ref().map(#ident_standart_not_null_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc)
                                        }
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_ts)}
                                }
                            },
                            Pattern::Array => match &is_nullable {
                                IsNullable::False => {
                                    let value_ts = wrap_into_value_init_ts(&{
                                        let init_ts = vec_syn_field.iter().map(|el_09cee28c| {
                                            let field_ident = &el_09cee28c.field_ident;
                                            quote! {let mut #field_ident = None;}
                                        });
                                        let for_loop_ts = vec_syn_field.iter().map(|el_cf4923ce| {
                                            let field_ident = &el_cf4923ce.field_ident;
                                            let field_ident_ts = {
                                                let field_ident_ucc_ts_fe80e842 = ToTokensToUccTs::case_or_panic(&field_ident);
                                                quote!{
                                                    #ident_standart_not_null_update_el_ucc::#field_ident_ucc_ts_fe80e842(v_d2a6daf8) => {
                                                        #field_ident = Some(v_d2a6daf8.#ValueSc.clone());
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
                                                .filter(|el_a1502880| el_a1502880.field_ident != *field_ident)
                                                .map(|el_2908bd7a| {
                                                    let el_2908bd7a_field_ident = &el_2908bd7a.field_ident;
                                                    let el_2908bd7a_field_ident_ucc_ts =
                                                        ToTokensToUccTs::case_or_panic(
                                                            &el_2908bd7a_field_ident,
                                                        );
                                                    quote! {
                                                        #ident_standart_not_null_update_el_ucc::#el_2908bd7a_field_ident_ucc_ts(_)
                                                    }
                                                })
                                                .fold(None, |acc_bbf653f7, el_2be3f4ee| Some(match acc_bbf653f7 {
                                                    None => el_2be3f4ee,
                                                    Some(v_5b375af0) => quote! { #v_5b375af0 | #el_2be3f4ee },
                                                }));
                                                ts_e0bf4e67.map_or_else(
                                                    Ts2::new,
                                                    |v_5c826b8c| quote!{#v_5c826b8c => (),}
                                                )
                                            };
                                            quote! {
                                                for el_da177c5a in el_4634bb8a.fields.0.to_vec() {
                                                    match &el_da177c5a {
                                                        #field_ident_ts
                                                        #fields_without_ident_ts_11eab22a
                                                    }
                                                }
                                            }
                                        });
                                        let value_ts = wrap_into_value_init_ts(&{
                                            let uuid_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
                                            let fields_ts = vec_syn_field.iter().map(|el_134779da| {
                                                let field_ident = &el_134779da.field_ident;
                                                let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_134779da.field_type);
                                                quote! {
                                                    #field_ident: #field_type_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc(&#field_ident.expect("a3ec7cae"))
                                                }
                                            });
                                            quote!{
                                                #ident_with_id_standart_not_null_read_only_ids_handle_ucc {
                                                    #IdSc: #uuid_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc(&el_4634bb8a.#IdSc),
                                                    #(#fields_ts),*
                                                }
                                            }
                                        });
                                        quote!{
                                            #ValueSc.#UpdateSc.to_vec().iter().map(|el_4634bb8a|{
                                                #(#init_ts)*
                                                #(#for_loop_ts)*
                                                #ident_with_id_standart_not_null_read_only_ids_ucc(#value_ts)
                                            }).collect()
                                        }
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_ts)}
                                }
                                IsNullable::True => {
                                    let value_ts = wrap_into_value_init_ts(&quote!{
                                        #ValueSc.0.as_ref().map(#ident_array_not_null_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc)
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_ts)}
                                }
                            },
                        };
                        let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = {
                            let value_init_ts = gen_import_path_value_init_ts(&match &pattern {
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_2b018c89| {
                                            let field_ident = &el_2b018c89.field_ident;
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_2b018c89.field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                    &#ValueSc.0.#ValueSc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::try_new(
                                                #(#parameters_ts),*
                                            ).expect("57820868")
                                        }
                                    }
                                    IsNullable::True => quote! {
                                        #ident_read_ucc::new(
                                            #ValueSc.0.#ValueSc.as_ref().and_then(|v_dfa7815e| match #ident_standart_not_null_as_pg_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                v_dfa7815e
                                            ) {
                                                Some(v_02cef266) => Some(v_02cef266.#ValueSc),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => {
                                        let parameters_ts = vec_syn_field_with_id.iter().map(|el_f4599b96| {
                                            let field_ident = &el_f4599b96.field_ident;
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_f4599b96.field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                    &el_629b1544.0.#ValueSc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::new({
                                                let mut acc_5f587d35 = #ValueSc.0.#ValueSc.clone().into_iter().map(|el_629b1544|{
                                                    #ident_with_id_standart_not_null_read_ucc::try_new(
                                                        #(#parameters_ts),*
                                                    ).expect("8f6fb6b6")
                                                }).collect::<Vec<#ident_with_id_standart_not_null_read_ucc>>();
                                                acc_5f587d35.sort_by(|first, second| {
                                                    if let (Some(v_first), Some(v_second)) = (&first.id, &second.id) {
                                                        //maybe remove .clone - add .get by ref method
                                                        #import_path_pg_json_type_uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_ts::into_inner(
                                                            v_first.#ValueSc.clone()
                                                        )
                                                        .cmp(&#import_path_pg_json_type_uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_ts::into_inner(
                                                            v_second.#ValueSc.clone()
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
                                            #ValueSc.0.#ValueSc.as_ref().and_then(|v_16ab4136| match #ident_array_not_null_as_pg_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                v_16ab4136
                                            ) {
                                                Some(v_71a66429) => Some(v_71a66429.#ValueSc.0),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                            });
                            quote!{Some(#value_init_ts)}
                        };
                        let previous_read_merged_with_option_update_into_read_ts = {
                            let fields_init_ts = vec_syn_field.iter().map(|el_8caae90c| {
                                let field_ident = &el_8caae90c.field_ident;
                                quote! {let mut #field_ident = None;}
                            });
                            let match_ts = vec_syn_field.iter().map(|el_b625a4b0| {
                                let field_ident = &el_b625a4b0.field_ident;
                                let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                quote! {
                                    #ident_standart_not_null_update_el_ucc::#field_ident_ucc_ts(#ValueSc) => {
                                        #field_ident = Some(#ValueSc.#ValueSc);
                                    }
                                }
                            });
                            let gen_struct_init_ts = |function: &dyn Fn(&dyn ToTokens) -> Ts2|{//ts: &dyn ToTokens
                                let ts = vec_syn_field.iter().map(|el_96e0a086| {
                                    let field_ident = &el_96e0a086.field_ident;
                                    let value_init_ts = gen_import_path_value_init_ts(&{
                                        let ts = function(&field_ident);
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_96e0a086.field_type);
                                        quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::previous_read_merged_with_option_update_into_read(
                                                #ts,
                                                #field_ident
                                            )
                                        }
                                    });
                                    quote! {#field_ident: Some(#value_init_ts)}
                                });
                                quote!{#(#ts),*}
                            };
                            let gen_option_ts = |pattern_53bdff8d: &Pattern|{
                                let ident_as_pg_json_type_test_cases_ts_53bdff8d = gen_type_as_pg_json_type_test_cases_ts(match &pattern_53bdff8d {
                                    Pattern::Standart => &ident_standart_not_null_ucc,
                                    Pattern::Array => &ident_array_not_null_ucc
                                });
                                quote! {
                                    match #OptionUpdateSc {
                                        Some(v_fca601b5) => #ident_read_ucc(
                                            match v_fca601b5.0 {
                                                Some(v_8d7747f1) => Some(
                                                    #ident_as_pg_json_type_test_cases_ts_53bdff8d::previous_read_merged_with_option_update_into_read(
                                                        #ReadSc.0.unwrap_or_else(#default_but_option_is_some_ts),
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
                                    Pattern::Standart => {
                                        let struct_init_ts = gen_struct_init_ts(&|ts: &dyn ToTokens|{
                                            quote!{
                                                #ReadSc.#ts.expect("a2d26e36").#ValueSc
                                            }
                                        });
                                        quote!{
                                            match #OptionUpdateSc {
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
                                    Pattern::Array => {
                                        let struct_init_ts = gen_struct_init_ts(&|ts: &dyn ToTokens|{
                                            quote!{
                                                found_read_element.#ts.expect("2e8229f7").#ValueSc
                                            }
                                        });
                                        quote! {
                                            match #OptionUpdateSc {
                                                Some(v_47f5a20b) => #ident_read_ucc({
                                                    let mut acc_04a67ef2 = Vec::new();
                                                    for el_377d1bb4 in v_47f5a20b.#UpdateSc.into_vec() {
                                                        let mut option_read_element = None;
                                                        for el_d2daee30 in &#ReadSc.0 {
                                                            if *#uuid_uuid_as_not_null_jsonb_string_as_pg_json_type_object_vec_el_id_ts::get_inner(&el_377d1bb4.#IdSc.clone().into())
                                                            ==
                                                            #uuid_uuid_as_not_null_jsonb_string_as_import_path_pg_json_type_ts::into_inner(
                                                                el_d2daee30.#IdSc.clone().expect("df2413fe").#ValueSc
                                                            )
                                                            {
                                                                option_read_element = Some(el_d2daee30.clone());
                                                                break;
                                                            }
                                                        }
                                                        let found_read_element = option_read_element.expect("139882b9");
                                                        #(#fields_init_ts)*
                                                        for el_629b1544 in el_377d1bb4.fields.0.into_vec() {
                                                            match el_629b1544 {
                                                                #(#match_ts),*
                                                            }
                                                        }
                                                        acc_04a67ef2.push(#ident_with_id_standart_not_null_read_ucc {
                                                            #IdSc: found_read_element.#IdSc,
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
                                IsNullable::True => gen_option_ts(pattern)
                            }
                        };
                        let read_only_ids_merged_with_create_into_read_ts = {
                            let gen_nullable_ts = |ident_ts_3eb17505: &dyn ToTokens, ts: &dyn ToTokens|{
                                let ident_as_pg_json_type_test_cases_ts_3eb17505 = gen_type_as_pg_json_type_test_cases_ts(&ident_ts_3eb17505);
                                quote! {
                                    #ident_read_ucc::new(
                                        match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                            (Some(read_only_ids_2b2ab8a1), Some(create_4a1adaa3)) => {
                                                Some(
                                                    #ident_as_pg_json_type_test_cases_ts_3eb17505::#ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc(
                                                        read_only_ids_2b2ab8a1,
                                                        create_4a1adaa3
                                                    ).expect("56ac4450").#ValueSc #ts
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
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_9bdce5ca| {
                                            let field_ident = &el_9bdce5ca.field_ident;
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_9bdce5ca.field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc(
                                                    #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                    #CreateSc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::try_new(
                                                #(#parameters_ts),*
                                            ).expect("52ad3994")
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(
                                        &ident_standart_not_null_ucc,
                                        &Ts2::new()
                                    )
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => {
                                        let gen_parameter_ts = |field_type: &dyn ToTokens, field_ident: &dyn ToTokens, ts: &dyn ToTokens|{
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc(
                                                    read_only_ids_225e2b76.0.#ValueSc.#field_ident,
                                                    #ts,
                                                )
                                            }
                                        };
                                        let id_parameter_ts = gen_parameter_ts(
                                            &uuid_uuid_as_not_null_jsonb_string_ts,
                                            &IdSc,
                                            &default_but_option_is_some_call_ts
                                        );
                                        let parameters_ts = vec_syn_field.iter().map(|el_2a1148f0|{
                                            let field_ident = &el_2a1148f0.field_ident;
                                            gen_parameter_ts(
                                                &el_2a1148f0.field_type,
                                                &field_ident,
                                                &quote! {create_3c660445.#field_ident}
                                            )
                                        });
                                        quote! {
                                            #ident_read_ucc::new({
                                                assert_eq!(#ReadOnlyIdsSc.0.#ValueSc.len(), #CreateSc.0.len(), "90d33ddd");
                                                let mut acc_37909420 = Vec::new();
                                                for (read_only_ids_225e2b76, create_3c660445) in #ReadOnlyIdsSc.0.#ValueSc.into_iter().zip(#CreateSc.0.into_iter()) {
                                                    acc_37909420.push(#ident_with_id_standart_not_null_read_ucc::try_new(
                                                        #id_parameter_ts,
                                                        #(#parameters_ts),*
                                                    ).expect("1330ac8d"));
                                                }
                                                acc_37909420
                                            })
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(
                                        &ident_array_not_null_ucc,
                                        &quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_option_value_read_ts = {
                            let value_init_ts = gen_import_path_value_init_ts(&quote!{
                                <#SelfUcc as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                                    #ReadOnlyIdsSc,
                                    #CreateSc
                                )
                            });
                            quote!{Some(#value_init_ts)}
                        };
                        let read_only_ids_merged_with_create_into_table_type_declaration_ts = {
                            let gen_nullable_ts = |ident_ts_971139d7: &dyn ToTokens, ts: &dyn ToTokens|{
                                let ident_as_pg_json_type_test_cases_ts_971139d7 = gen_type_as_pg_json_type_test_cases_ts(&ident_ts_971139d7);
                                quote! {
                                    #ident_table_type_declaration_ucc::new(
                                        match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                            (Some(read_only_ids_fb2ec2e4), Some(create_2f615d4f)) => {
                                                Some(
                                                    #ident_as_pg_json_type_test_cases_ts_971139d7::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                        read_only_ids_fb2ec2e4,
                                                        create_2f615d4f
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
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_ca3a96db| {
                                            let field_ident = &el_ca3a96db.field_ident;
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_ca3a96db.field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                    #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                    #CreateSc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_table_type_declaration_ucc::new(
                                                #(#parameters_ts),*
                                            )
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(
                                        &ident_standart_not_null_ucc,
                                        &Ts2::new()
                                    )
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => {
                                        let gen_parameter_ts = |field_type: &dyn ToTokens, field_ident: &dyn ToTokens, ts: &dyn ToTokens|{
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                    read_only_ids_94b49496.0.#ValueSc.#field_ident,
                                                    #ts,
                                                )
                                            }
                                        };
                                        let id_parameter_ts = gen_parameter_ts(
                                            &uuid_uuid_as_not_null_jsonb_string_ts,
                                            &IdSc,
                                            &default_but_option_is_some_call_ts
                                        );
                                        let parameters_ts = vec_syn_field.iter().map(|el_d5acef17|{
                                            let field_ident = &el_d5acef17.field_ident;
                                            gen_parameter_ts(
                                                &el_d5acef17.field_type,
                                                &field_ident,
                                                &quote! {create_24629087.#field_ident}
                                            )
                                        });
                                        quote! {
                                            #ident_table_type_declaration_ucc::new({
                                                assert_eq!(#ReadOnlyIdsSc.0.#ValueSc.len(), #CreateSc.0.len(), "7776a146");
                                                let mut acc_319e1fb1 = Vec::new();
                                                for (read_only_ids_94b49496, create_24629087) in #ReadOnlyIdsSc.0.#ValueSc.into_iter().zip(#CreateSc.0.into_iter()) {
                                                    acc_319e1fb1.push(#ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                        #id_parameter_ts,
                                                        #(#parameters_ts),*
                                                    ));
                                                }
                                                acc_319e1fb1
                                            })
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(
                                        &ident_array_not_null_ucc,
                                        &quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_where_equal_ts = match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Standart => {
                                    let parameters_ts = vec_syn_field.iter().map(|el_9990b32d| {
                                        let field_ident = &el_9990b32d.field_ident;
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_9990b32d.field_type);
                                        quote! {
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                #CreateSc.#field_ident
                                            )
                                        }
                                    });
                                    quote! {
                                        #ident_where_ucc::#EqualUcc(
                                            #import_path::PgJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                #ValueSc: #ident_table_type_declaration_ucc::new(
                                                    #(#parameters_ts),*
                                                )
                                            }
                                        )
                                    }
                                },
                                Pattern::Array => {
                                    let gen_read_only_ids_merged_with_create_into_table_type_declaration_ts = |
                                        field_ident: &dyn ToTokens,
                                        field_type: &dyn ToTokens,
                                        ts: &dyn ToTokens
                                    |{
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                        quote!{
                                            #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                read_only_ids_ea32954c.0.#ValueSc.#field_ident,
                                                #ts
                                            )
                                        }
                                    };
                                    let ident_ts_978daa48 = gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                        &IdSc,
                                        &uuid_uuid_as_not_null_jsonb_string_ts,
                                        &default_but_option_is_some_call_ts
                                    );
                                    let parameters_ts = vec_syn_field.iter().map(|el_fc74a022| {
                                        let field_ident = &el_fc74a022.field_ident;
                                        gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                            &field_ident,
                                            &el_fc74a022.field_type,
                                            &quote!{create_3cbe8967.#field_ident}
                                        )
                                    });
                                    quote! {
                                        #ident_where_ucc::#EqualUcc(
                                            #import_path::PgJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::And,
                                                #ValueSc: #ident_table_type_declaration_ucc::new({
                                                    let mut acc_321b3fcd = Vec::new();
                                                    for (read_only_ids_ea32954c, create_3cbe8967) in #ReadOnlyIdsSc.0.#ValueSc.into_iter().zip(#CreateSc.0.into_iter()) {
                                                        acc_321b3fcd.push(
                                                            #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                                #ident_ts_978daa48,
                                                                #(#parameters_ts),*
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
                                    let ident_ts_bdc5fdf7 = gen_type_as_pg_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                        Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote!{
                                        vec![
                                            #ident_ts_bdc5fdf7::#ReadOnlyIdsMergedWithCreateIntoWhereEqualSc(
                                                read_only_ids_ce30c0fe,
                                                create_8fd81ed8
                                            )
                                        ]
                                    }
                                };
                                quote! {
                                    #import_path::NullableJsonObjectPgTypeWhereFilter(
                                        match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                            (Some(read_only_ids_ce30c0fe), Some(create_8fd81ed8)) => match #import_path::NotEmptyUniqueVec::try_new(#ts) {
                                                Ok(v_7a9cd49b) => Some(v_7a9cd49b),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("463769fc")
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
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = {
                            let ts = match &is_nullable {
                                IsNullable::False => match &pattern {
                                    Pattern::Standart => {
                                        let elements_ts = vec_syn_field.iter().map(|el_23a78055| {
                                            let field_ident = &el_23a78055.field_ident;
                                            let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_23a78055.field_type);
                                            quote! {
                                                #ident_where_ucc::#field_ident_ucc_ts(
                                                    #import_path::PgTypeWhere::new(
                                                        #import_path::LogicalOperator::And,
                                                        #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                                                            #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                            #CreateSc.#field_ident
                                                        )
                                                    )
                                                )
                                            }
                                        });
                                        quote! {#(#elements_ts),*}
                                    },
                                    Pattern::Array => {
                                        let gen_read_only_ids_merged_with_create_into_table_type_declaration_ts = |
                                            field_ident: &dyn ToTokens,
                                            field_type: &dyn ToTokens,
                                            ts: &dyn ToTokens
                                        |{
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            quote!{
                                                #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                                                    read_only_ids_319c9e78.0.#ValueSc.#field_ident,
                                                    #ts
                                                )
                                            }
                                        };
                                        let ident_ts_76fd9d28 = gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                            &IdSc,
                                            &uuid_uuid_as_not_null_jsonb_string_ts,
                                            &default_but_option_is_some_call_ts
                                        );
                                        let parameters_ts = vec_syn_field.iter().map(|el_a8f4df4f| {
                                            let field_ident = &el_a8f4df4f.field_ident;
                                            gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                                &field_ident,
                                                &el_a8f4df4f.field_type,
                                                &quote!{create_00ae06d2.#field_ident}
                                            )
                                        });
                                        quote! {
                                            #ident_where_ucc::#EqualUcc(
                                                #import_path::PgJsonTypeWhereEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    #ValueSc: #ident_table_type_declaration_ucc::new({
                                                        let mut acc_97ebf7d6 = Vec::new();
                                                        for (read_only_ids_319c9e78, create_00ae06d2) in #ReadOnlyIdsSc.0.#ValueSc.into_iter().zip(#CreateSc.0.into_iter()) {
                                                            acc_97ebf7d6.push(
                                                                #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                                    #ident_ts_76fd9d28,
                                                                    #(#parameters_ts),*
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
                                        let ident_ts_b9e61412 = gen_type_as_pg_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                            Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                            Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                        }));
                                        quote! {
                                            #ident_ts_b9e61412::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                                                read_only_ids_2898c440,
                                                create_f1c4667c
                                            )
                                        }
                                    };
                                    quote! {
                                        #import_path::NullableJsonObjectPgTypeWhereFilter(
                                            match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                                (Some(read_only_ids_2898c440), Some(create_f1c4667c)) => Some(#ts),
                                                (Some(_), None) => panic!("49e4c289"),
                                                (None, Some(_)) => panic!("ad71caa2"),
                                                (None, None) => None,
                                            }
                                        )
                                    }
                                },
                            };
                            quote!{
                                #import_path::NotEmptyUniqueVec::try_new(vec![
                                    #ts
                                ]).expect("ba9c52c1")
                            }
                        };
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = match &pattern {
                            Pattern::Standart => match &is_nullable {
                                IsNullable::False => {
                                    let ts = vec_syn_field.iter().map(|el_459c3da8| {
                                        let field_ident = &el_459c3da8.field_ident;
                                        let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_459c3da8.field_type);
                                        quote! {
                                            for el_d830c061 in #field_type_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc(
                                                #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                #CreateSc.#field_ident
                                            ).into_vec() {
                                                acc_89ec072c.push(
                                                    #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PgTypeWhere::try_new(
                                                            #import_path::LogicalOperator::Or,
                                                            vec![el_d830c061],
                                                        )
                                                        .expect("0c6ccad1"),
                                                    )
                                                );
                                            }
                                        }
                                    });
                                    quote!{
                                        #import_path::NotEmptyUniqueVec::try_new({
                                            let mut acc_89ec072c = Vec::new();
                                            #(#ts)*
                                            acc_89ec072c
                                        }).expect("9c50391c")
                                    }
                                },
                                IsNullable::True => quote!{
                                    #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_12b6f16d = Vec::new();
                                        match (#ReadOnlyIdsSc.0.#ValueSc, #CreateSc.0) {
                                            (Some(read_only_ids_2f024927), Some(create_120c1dad)) => {
                                                for el_a8b181a0 in #ident_standart_not_null_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc(
                                                    read_only_ids_2f024927,
                                                    create_120c1dad
                                                ).into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                                        Ok(v_8e72cfd7) => {
                                                            acc_12b6f16d.push(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_8e72cfd7)));
                                                        },
                                                        Err(er) => match er {
                                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("2a88b17f")
                                                        }
                                                    }
                                                }
                                            },
                                            (Some(_), None) => panic!("b4507b4c"),
                                            (None, Some(_)) => panic!("8f458c1d"),
                                            (None, None) => {
                                                acc_12b6f16d.push(#import_path::NullableJsonObjectPgTypeWhereFilter(None));
                                            },
                                        }
                                        acc_12b6f16d
                                    }).expect("7efc9aae")
                                }
                            },
                            Pattern::Array => quote!{
                                #self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                                    #ReadOnlyIdsSc,
                                    #CreateSc
                                )
                            }
                        };
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts = gen_dim_equal_ts(&Dim::One);
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts = gen_dim_equal_ts(&Dim::Two);
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts = gen_dim_equal_ts(&Dim::Three);
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts = gen_dim_equal_ts(&Dim::Four);
                        let create_into_pg_json_type_option_vec_where_length_equal_ts = {
                            let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                                match #import_path::NotEmptyUniqueVec::try_new(
                                    match #CreateSc.0 {
                                        Some(create_09a81dae) => match <
                                            #ts
                                            as
                                            #import_path::PgJsonTypeTestCases
                                        >::#CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(create_09a81dae) {
                                            Some(v_3680a4c9) => {
                                                let mut acc_5c441d3a = Vec::new();
                                                for el_a8b181a0 in v_3680a4c9.clone().into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                                        Ok(v_15097b27) => {
                                                            acc_5c441d3a.push(
                                                                #import_path::NullableJsonObjectPgTypeWhereFilter(
                                                                    Some(v_15097b27)
                                                                )
                                                            );
                                                        },
                                                        Err(er) => match er {
                                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("6c4da72e")
                                                        }
                                                    }
                                                }
                                                let v_84ea8e4c = #import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_3680a4c9));
                                                if !acc_5c441d3a.contains(&v_84ea8e4c) {
                                                    acc_5c441d3a.push(v_84ea8e4c);
                                                }
                                                acc_5c441d3a
                                            },
                                            None => {
                                                return None;
                                            }
                                        },
                                        None => vec![#import_path::NullableJsonObjectPgTypeWhereFilter(None)],
                                    }
                                ) {
                                    Ok(v_72dbefbc) => Some(v_72dbefbc),
                                    Err(er) => match er {
                                        #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("d41bcbca")
                                    }
                                }
                            };
                            match &pattern {
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => {
                                        let ts = vec_syn_field.iter().map(|el_d41dce84| {
                                            let field_ident = &el_d41dce84.field_ident;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_d41dce84.field_type);
                                            quote! {
                                                if let Some(v_927601a4) = #field_type_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(
                                                    #CreateSc.#field_ident
                                                ) {
                                                    for el_194a660a in v_927601a4.clone().into_vec() {
                                                        acc_587bf907.push(
                                                            #ident_where_ucc::#field_ident_ucc(
                                                                #import_path::PgTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_194a660a]
                                                                ).expect("2f437949")
                                                            )
                                                        );
                                                    }
                                                    let v_84ea8e4c = #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PgTypeWhere::new(
                                                            #import_path::LogicalOperator::And,
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
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_587bf907 = Vec::new();
                                                #(#ts)*
                                                acc_587bf907
                                            }) {
                                                Ok(v_ea661a62) => Some(v_ea661a62),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("7786dfd4")
                                                },
                                            }
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(&ident_standart_not_null_ucc)
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => {
                                        let ts = vec_syn_field.iter().map(|el_c776b608| {
                                            let field_ident = &el_c776b608.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_c776b608.field_type);
                                            quote! {
                                                for create_e06a9fe2 in #CreateSc.0.clone() {
                                                    if let Some(v_ee015fcc) = #field_type_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(
                                                        create_e06a9fe2.#field_ident
                                                    ) {
                                                        for el_63008daa in v_ee015fcc.clone().into_vec() {
                                                            let v_0ae29f5f = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PgTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_63008daa]
                                                                )
                                                                .expect("38ca88dc"),
                                                            );
                                                            if !acc_480d72e5.contains(&v_0ae29f5f) {
                                                                acc_480d72e5.push(v_0ae29f5f);
                                                            }
                                                        }
                                                        let v_4e4cfda3 = #ident_where_ucc::#el_field_ident_ucc(
                                                            #import_path::PgTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
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
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_480d72e5 = Vec::new();
                                                #(#ts)*
                                                acc_480d72e5.push(#ident_where_ucc::LengthEqual(
                                                    #import_path::PgJsonTypeWhereLengthEqual {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #ValueSc: #import_path::UnsignedPartOfI32::try_from(
                                                            i32::try_from(#CreateSc.0.len()).expect("1811faf7")
                                                        ).expect("a590f39b"),
                                                    }
                                                ));
                                                acc_480d72e5
                                            }) {
                                                Ok(v_cc01db9a) => Some(v_cc01db9a),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("bad01dd0")
                                                },
                                            }
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(&ident_array_not_null_ucc)
                                },
                            }
                        };
                        let create_into_pg_json_type_option_vec_where_length_greater_than_ts = {
                            let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                                #CreateSc.0.map_or_else(|| None, |create_612f2a61| <
                                    #ts
                                    as
                                    #import_path::PgJsonTypeTestCases
                                >::create_into_pg_json_type_option_vec_where_length_greater_than(create_612f2a61).map_or_else(
                                    || None,
                                    |v_1ea95b5d| match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_87f84b5c = Vec::new();
                                        for el_9bbf8527 in v_1ea95b5d.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(vec![el_9bbf8527]) {
                                                Ok(v_1d0202fc) => {
                                                    acc_87f84b5c.push(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_1d0202fc)));
                                                }
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => (),
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("bdb0a112"),
                                                },
                                            }
                                        }
                                        let v_4e4cfda3 = #import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_1ea95b5d));
                                        if !acc_87f84b5c.contains(&v_4e4cfda3) {
                                            acc_87f84b5c.push(v_4e4cfda3);
                                        }
                                        acc_87f84b5c
                                    }) {
                                        Ok(v_ea4ca151) => Some(v_ea4ca151),
                                        Err(er) => match er {
                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => None,
                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("c7ecc36f"),
                                        },
                                    },
                                ))
                            };
                            match &pattern {
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => {
                                        let ts = vec_syn_field.iter().map(|el_9d0245f1| {
                                            let field_ident = &el_9d0245f1.field_ident;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_9d0245f1.field_type);
                                            quote! {
                                                if let Some(v_3432b965) = #field_type_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc(
                                                    #CreateSc.#field_ident
                                                ) {
                                                    for el_9bbf8527 in v_3432b965.clone().into_vec() {
                                                        acc_f5866fb6.push(
                                                            #ident_where_ucc::#field_ident_ucc(
                                                                #import_path::PgTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_9bbf8527]
                                                                ).expect("479db858")
                                                            )
                                                        );
                                                    }
                                                    let el_4a00ab02 = #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PgTypeWhere::new(
                                                            #import_path::LogicalOperator::And,
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
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_f5866fb6 = Vec::new();
                                                #(#ts)*
                                                acc_f5866fb6
                                            }) {
                                                Ok(v_c4c01cd9) => Some(v_c4c01cd9),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("91d713b5")
                                                },
                                            }
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(&ident_standart_not_null_ucc)
                                },
                                Pattern::Array => match &is_nullable {
                                    IsNullable::False => {
                                        let ts = vec_syn_field.iter().map(|el_47c8f26c| {
                                            let field_ident = &el_47c8f26c.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&el_47c8f26c.field_type);
                                            quote! {
                                                for create_34a1e540 in #CreateSc.0.clone() {
                                                    if let Some(v_51fe384b) = #field_type_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc(
                                                        create_34a1e540.#field_ident
                                                    ) {
                                                        for el_4a00ab02 in v_51fe384b.clone().into_vec() {
                                                            let el_938f8b34 = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PgTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_4a00ab02]
                                                                )
                                                                .expect("955c6c27"),
                                                            );
                                                            if !acc_acceb7eb.contains(&el_938f8b34) {
                                                                acc_acceb7eb.push(el_938f8b34);
                                                            }
                                                        }
                                                        let el_e17d9fba = #ident_where_ucc::#el_field_ident_ucc(
                                                            #import_path::PgTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
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
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_acceb7eb = Vec::new();
                                                #(#ts)*
                                                acc_acceb7eb.push(#ident_where_ucc::LengthGreaterThan(
                                                    #import_path::PgJsonTypeWhereLengthGreaterThan {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #ValueSc: #import_path::UnsignedPartOfI32::try_from(
                                                            i32::try_from(
                                                                #CreateSc.0.len().checked_sub(1).unwrap_or_else(|| {
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
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("a9e99f81")
                                                },
                                            }
                                        }
                                    }
                                    IsNullable::True => gen_nullable_ts(&ident_array_not_null_ucc)
                                },
                            }
                        };
                        let (
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts
                        ) = {
                            let gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts = |method_name_ts: &dyn ToTokens|match &is_nullable {
                                IsNullable::False => match &pattern {
                                    Pattern::Standart => {
                                        let ts = vec_syn_field.iter().map(|el_59346ba9| {
                                            let field_ident = &el_59346ba9.field_ident;
                                            let field_type = &el_59346ba9.field_type;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                if let Some(v_a2900ac9) = #field_type_as_pg_json_type_test_cases_ts::#method_name_ts(
                                                    #ReadOnlyIdsSc.0.#ValueSc.#field_ident,
                                                    #CreateSc.#field_ident
                                                ) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    for el_3e86d33d in v_a2900ac9.clone().into_vec() {
                                                        match el_3e86d33d {
                                                            #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PgTypeWhere::new(
                                                                            and,
                                                                            multiple
                                                                        ))
                                                                    )
                                                                );
                                                            },
                                                            #import_path::SingleOrMultiple::Single(single) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PgTypeWhere::try_new(
                                                                            and,
                                                                            vec![single]
                                                                        ).expect("2635ede5"))
                                                                    )
                                                                );
                                                            },
                                                        }
                                                    }
                                                    let v_3e75a2f2 = #import_path::SingleOrMultiple::Single(
                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PgTypeWhere::try_new(
                                                            and,
                                                            v_a2900ac9.into_vec().into_iter().flat_map(|el_9efefcdc| match el_9efefcdc {
                                                                #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                                #import_path::SingleOrMultiple::Single(single) => {
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
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_a94bd7fb = Vec::new();
                                                #(#ts)*
                                                acc_a94bd7fb
                                            }) {
                                                Ok(v_ebe930f0) => Some(v_ebe930f0),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("b877e9c0")
                                                }
                                            }
                                        }
                                    },
                                    Pattern::Array => {
                                        let init_ts = vec_syn_field.iter().map(|el_3fde3bb4| {
                                            let field_ident = &el_3fde3bb4.field_ident;
                                            let field_type = &el_3fde3bb4.field_type;
                                            let field_type_as_pg_json_type_test_cases_ts = gen_type_as_pg_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                let #field_ident = #field_type_as_pg_json_type_test_cases_ts::#method_name_ts(
                                                    read_only_ids_629675e2.0.#ValueSc.#field_ident,
                                                    create_82796400.#field_ident
                                                );
                                            }
                                        });
                                        let if_some_ts = {
                                            let (last, rest) = vec_syn_field.split_last().expect("a8e7b6d6");
                                            let gen_field_ident_is_some_ts = |field_ident: &Ident|quote!{#field_ident.is_some()};
                                            let rest_ts = rest.iter().map(|el_cd54f3c6| {
                                                let field_ident_is_some_ts = gen_field_ident_is_some_ts(&el_cd54f3c6.field_ident);
                                                quote!{#field_ident_is_some_ts || }
                                            });
                                            let last_ts = gen_field_ident_is_some_ts(&last.field_ident);
                                            quote! {#(#rest_ts)* #last_ts}
                                        };
                                        let ts = vec_syn_field.iter().map(|el_dbdd7930| {
                                            let field_ident = &el_dbdd7930.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            quote! {
                                                if let Some(v_f190793e) = #field_ident {
                                                    for el_22ac4087 in v_f190793e.clone().into_vec() {
                                                        let where_f8a4319c = #ident_where_ucc::#el_field_ident_ucc(
                                                            match el_22ac4087 {
                                                                #import_path::SingleOrMultiple::Multiple(multiple) => #import_path::PgTypeWhere::new(
                                                                    and,
                                                                    multiple.clone()
                                                                ),
                                                                #import_path::SingleOrMultiple::Single(single) => #import_path::PgTypeWhere::try_new(
                                                                    and,
                                                                    vec![single]
                                                                ).expect("2ed4dc5e"),
                                                            }
                                                        );
                                                        all_fields_acc.push(where_f8a4319c.clone());
                                                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                                                            #IdSc.clone(),
                                                            where_f8a4319c
                                                        ]) {
                                                            Ok(v_fdd1b3eb) => {
                                                                let multiple_where_with_id_f8a4319c = #import_path::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                                                if !acc_359c0b3f.contains(&multiple_where_with_id_f8a4319c) {
                                                                    acc_359c0b3f.push(multiple_where_with_id_f8a4319c);
                                                                }
                                                            },
                                                            Err(er) => match er {
                                                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("f0e3d01b")
                                                            }
                                                        }
                                                    }
                                                    match #import_path::NotEmptyUniqueVec::try_new(
                                                        v_f190793e.into_vec().into_iter().flat_map(|el_6df4f0be| match el_6df4f0be {
                                                            #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                            #import_path::SingleOrMultiple::Single(single) => {
                                                                std::iter::once(single).collect()
                                                            }
                                                        })
                                                        .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                                            if !acc_01265629.contains(&el_9a7c960d) {
                                                                acc_01265629.push(el_9a7c960d);
                                                            }
                                                            acc_01265629
                                                        })
                                                    ) {
                                                        Ok(v_a4000d70) => {
                                                            let v_d6218307 = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PgTypeWhere::new(
                                                                    and,
                                                                    v_a4000d70
                                                                )
                                                            );
                                                            if !all_fields_acc.contains(&v_d6218307) {
                                                                all_fields_acc.push(v_d6218307);
                                                            }
                                                        },
                                                        Err(er) => match er {
                                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("f8fcc434")
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_359c0b3f = Vec::new();
                                                for (read_only_ids_629675e2, create_82796400) in #ReadOnlyIdsSc.0.#ValueSc.into_iter().zip(#CreateSc.0.into_iter()) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    let #IdSc = #ident_where_ucc::ElementId(
                                                        #import_path::PgTypeWhere::try_new(
                                                            and,
                                                            vec![
                                                                #uuid_uuid_as_not_null_jsonb_string_where_ucc::Equal(#import_path::PgJsonTypeWhereEqual {
                                                                    logical_operator: #import_path::LogicalOperator::Or,
                                                                    #ValueSc: #uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc::new(
                                                                        read_only_ids_629675e2.0.#ValueSc.#IdSc.0.#ValueSc
                                                                    ),
                                                                })
                                                            ],
                                                        )
                                                        .expect("31db8e1e"),
                                                    );
                                                    #(#init_ts)*
                                                    if #if_some_ts {
                                                        let mut all_fields_acc = vec![];
                                                        #(#ts)*
                                                        match #import_path::NotEmptyUniqueVec::try_new({
                                                            all_fields_acc.push(#IdSc);
                                                            all_fields_acc
                                                        }) {
                                                            Ok(v_80199720) => {
                                                                acc_359c0b3f.push(#import_path::SingleOrMultiple::Multiple(v_80199720));
                                                            },
                                                            Err(er) => match er {
                                                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("32a3da97")
                                                            }
                                                        }
                                                    }
                                                }
                                                acc_359c0b3f
                                            }) {
                                                Ok(v_752f0e8d) => Some(v_752f0e8d),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("76542a11")
                                                }
                                            }
                                        }
                                    }
                                },
                                IsNullable::True => {
                                    let ident_ts_a8bc30fc = gen_type_as_pg_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                        Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote! {
                                        match (#ReadOnlyIdsSc.0.value, #CreateSc.0) {
                                            (Some(read_only_ids_3e2e30c8), Some(create_79039a2f)) => #ident_ts_a8bc30fc::#method_name_ts(
                                                read_only_ids_3e2e30c8,
                                                create_79039a2f
                                            ).map_or_else(|| None, |v_35662b3a| match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_e0d72451 = vec![];
                                                for el_4632f100 in v_35662b3a.into_vec() {
                                                    match el_4632f100 {
                                                        #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                            acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(multiple))));
                                                        },
                                                        #import_path::SingleOrMultiple::Single(single) => match #import_path::NotEmptyUniqueVec::try_new(vec![single]) {
                                                            Ok(v_4ce6ecd3) => {
                                                                acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_4ce6ecd3))));
                                                            }
                                                            Err(er) => match er {
                                                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => (),
                                                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("626ffa77"),
                                                            },
                                                        },
                                                    }
                                                }
                                                acc_e0d72451
                                            }) {
                                                Ok(v_5d381053) => Some(v_5d381053),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => None,
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("23a17416"),
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
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc
                                ),
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc
                                ),
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc
                                ),
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc
                                ),
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc
                                ),
                                gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                                    &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc
                                )
                            )
                        };
                        gen_impl_pg_json_type_test_cases_for_ident_ts(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_ucc,
                            &ident,
                            &option_vec_create_ts,
                            &read_only_ids_to_two_dimal_vec_read_inner_ts,
                            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                            &read_only_ids_into_option_value_read_inner_ts,
                            &update_to_read_only_ids_ts,
                            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
                            &previous_read_merged_with_option_update_into_read_ts,
                            &read_only_ids_merged_with_create_into_read_ts,
                            &read_only_ids_merged_with_create_into_option_value_read_ts,
                            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
                            &read_only_ids_merged_with_create_into_where_equal_ts,
                            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
                            &create_into_pg_json_type_option_vec_where_length_equal_ts,
                            &create_into_pg_json_type_option_vec_where_length_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
                        )
                    },
                    {
                        let option_vec_create_ts = quote! {#self_as_pg_json_type_test_cases_ts::#OptionVecCreateSc()};
                        let read_only_ids_to_two_dimal_vec_read_inner_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(#ReadOnlyIdsSc)};
                        let read_inner_into_read_with_new_or_try_new_unwraped_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(#ValueSc)};
                        let read_inner_into_update_with_new_or_try_new_unwraped_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc(#ValueSc)};
                        let update_to_read_only_ids_ts = quote! {#self_as_pg_json_type_test_cases_ts::#UpdateToReadOnlyIdsSc(#ValueSc)};
                        let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc(#ValueSc)};
                        let previous_read_merged_with_option_update_into_read_ts = quote! {#self_as_pg_json_type_test_cases_ts::#PreviousReadMergedWithOptionUpdateIntoReadSc(#ReadSc, #OptionUpdateSc)};
                        let read_only_ids_merged_with_create_into_read_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_option_value_read_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_table_type_declaration_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_where_equal_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoWhereEqualSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts = quote!{Some(#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        ))};
                        let create_into_pg_type_option_vec_where_dim_one_equal_ts = &none_ts;
                        let pg_type_option_vec_where_greater_than_test_ts = &none_ts;
                        let read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts = &none_ts;
                        let (
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
                            read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts
                        ) = {
                            let gen_dim_equal_handle_ts = |dim: &Dim|{
                                let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc = dim.read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc();
                                quote!{#self_as_pg_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
                                    #ReadOnlyIdsSc,
                                    #CreateSc
                                )}
                            };
                            (
                                gen_dim_equal_handle_ts(&Dim::One),
                                gen_dim_equal_handle_ts(&Dim::Two),
                                gen_dim_equal_handle_ts(&Dim::Three),
                                gen_dim_equal_handle_ts(&Dim::Four)
                            )
                        };
                        let create_into_pg_json_type_option_vec_where_length_equal_ts = quote!{#self_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(
                            #CreateSc
                        )};
                        let create_into_pg_json_type_option_vec_where_length_greater_than_ts = quote!{#self_as_pg_json_type_test_cases_ts::#CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc(
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts = quote!{#self_as_pg_json_type_test_cases_ts::#ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc(
                            #ReadOnlyIdsSc,
                            #CreateSc
                        )};
                        gen_impl_pg_type_test_cases_for_ident_ts(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_ucc,
                            &ident,
                            &option_vec_create_ts,
                            &read_only_ids_to_two_dimal_vec_read_inner_ts,
                            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                            &update_to_read_only_ids_ts,
                            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
                            &previous_read_merged_with_option_update_into_read_ts,
                            &read_only_ids_merged_with_create_into_read_ts,
                            &read_only_ids_merged_with_create_into_option_value_read_ts,
                            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
                            &read_only_ids_merged_with_create_into_where_equal_ts,
                            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                            &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts,
                            &create_into_pg_type_option_vec_where_dim_one_equal_ts,
                            &pg_type_option_vec_where_greater_than_test_ts,
                            &read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
                            &create_into_pg_json_type_option_vec_where_length_equal_ts,
                            &create_into_pg_json_type_option_vec_where_length_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
                            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
                        )
                    },
                )
            };
            let impl_pg_type_not_primary_key_for_ident_ts = gen_impl_pg_type_not_primary_key_for_ident_ts(&import_path, &ident);
            let generated = quote! {
                #ident_ts
                #ident_table_type_declaration_ts
                #ident_create_ts
                #ident_create_for_query_ts
                #ident_select_ts
                #ident_where_ts
                #ident_read_ts
                #ident_read_only_ids_ts
                #ident_read_inner_ts
                #ident_update_ts
                #ident_update_for_query_ts
                #impl_pg_crud_pg_json_type_for_ident_ts
                #maybe_impl_pg_crud_pg_types_pg_type_pg_type_ts
                #impl_pg_json_type_test_cases_for_ident_ts
                #impl_pg_type_test_cases_for_ident_ts
                #impl_pg_type_not_primary_key_for_ident_ts
            };
            (
                {
                    let field_ident = format!("field_{index}").parse::<Ts2>().expect("7f9a06a5");
                    quote! {
                        pub #field_ident: #ident,
                    }
                },
                generated,
            )
        })
        .collect::<(Vec<Ts2>, Vec<Ts2>)>();
    maybe_write_ts_into_file(
        gen_pg_json_object_type_config
            .pg_table_columns_content_write_into_pg_table_columns_using_pg_json_object_types,
        "pg_table_columns_using_pg_json_object_types",
        &quote! {
            pub struct PgTableColumnsContentWriteIntoPgTableColumnsUsingPgJsonObjectTypes {
                #(#fields_ts)*
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated: Ts2 = {
        let ident_gen_pg_json_object_type_mod =
            SelfGenPgJsonObjectTypeModSc::from_tokens(&di.ident);
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #ident_gen_pg_json_object_type_mod {
                #(#pg_json_object_type_array)*
            }
            pub use #ident_gen_pg_json_object_type_mod::*;
        }
    };
    maybe_write_ts_into_file(
        gen_pg_json_object_type_config.whole_content_write_into_gen_pg_json_object_type,
        "gen_pg_json_object_type",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
