use naming::{
    AllFieldsAreNoneUcc, ArrayOfUcc, AsRefStrToUccTs, AsUcc, ColumnNameAndMaybeFieldGetterSc,
    ColumnSc, CreateIntoPostgresJsonTypeOptionVecWhereLengthEqualSc,
    CreateIntoPostgresJsonTypeOptionVecWhereLengthGreaterThanSc, CreateSc,
    CreateUpdateDeleteAreEmptyUcc, DefaultOptionSomeVecOneElSc, DefaultOptionSomeVecOneElUcc,
    DeleteSc, EqualUcc, ErrorSc, FieldsSc, GenJsonbSetTargetSc, IdSc, IdsAreNotUniqueUcc,
    IncrementSc, IsNeedToAddLogicalOperatorSc, JsonbObjectUcc, JsonbSetAccumulatorSc,
    JsonbSetPathSc, JsonbSetTargetSc, NotUniqueIdInJsonDeleteArrayUcc,
    NotUniqueIdInJsonUpdateAndDeleteArraysUcc, OptionUpdateSc, OptionVecCreateSc,
    PostgresJsonTypeTestCasesUcc, PostgresJsonTypeUcc, PostgresTypeTestCasesUcc, PostgresTypeUcc,
    PreviousReadMergedWithOptionUpdateIntoReadSc, QueryPartErrorNamedUcc, QueryPartSc, QuerySc,
    ReadInnerIntoReadWithNewOrTryNewUnwrapedSc, ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc,
    ReadOnlyIdsIntoOptionValueReadInnerSc, ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereContainsElRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc, ReadOnlyIdsSc,
    ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc,
    ReadOnlyIdsToTwoDimensionalVecReadInnerSc, ReadSc, SelectOnlyCreatedIdsQueryBindSc,
    SelectOnlyCreatedIdsQueryPartSc, SelectOnlyIdsQueryPartSc, SelectOnlyUpdatedIdsQueryBindSc,
    SelectOnlyUpdatedIdsQueryPartSc, SelectQueryPartPostgresTypeSc, SelectQueryPartSc, SelfSc,
    SelfUcc, StdFmtDisplayPlusQuoteToTokens, StdOptionOptionObjectAccSc, ToTokensToUccTs,
    UpdateQueryBindSc, UpdateQueryPartSc, UpdateSc, UpdateToReadOnlyIdsSc,
    UuidUuidAsNotNullJsonbStringUcc, ValueSc, ValueUcc, VecOfUcc, WithIdUcc,
    parameter::{
        ElementSelfUcc, SelfCreateForQueryUcc, SelfCreateUcc, SelfCurrentSc,
        SelfGenPostgresJsonObjectTypeModSc, SelfLastSc, SelfReadInnerUcc, SelfReadOnlyIdsHandleUcc,
        SelfReadOnlyIdsUcc, SelfReadTryFromErrorNamedUcc, SelfReadUcc, SelfSelectElementUcc,
        SelfSelectSc, SelfSelectUcc, SelfTableTypeDeclarationUcc, SelfUpdateElementUcc,
        SelfUpdateForQueryElementUcc, SelfUpdateForQueryUcc, SelfUpdateTryNewErrorNamedUcc,
        SelfUpdateUcc, SelfWhereUcc,
    },
};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::iter::repeat_with;
use syn::token::{Colon, Pub};
//todo gen authorization rights enum for json fields
//todo bug in update if updating array and creating element in jsonb array without anything - read_only_ids generation logic of vec returns wrong query part
#[must_use]
pub fn gen_postgres_json_object_type(input_ts: Ts2) -> Ts2 {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    enum TraitGen {
        PostgresJsonType,
        PostgresTypeAndPostgresJsonType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum Pattern {
        Standart,
        Array,
    }
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct PostgresJsonObjectTypeRecord {
        not_null_or_nullable: postgres_crud_macros_common::NotNullOrNullable,
        pattern: Pattern,
        trait_gen: TraitGen,
    }
    #[derive(Debug, serde::Deserialize)]
    struct GenPostgresJsonTypesConfig {
        postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_json_object_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        variant: PostgresJsonObjectTypeRecord,
        whole_content_write_into_gen_postgres_json_object_type:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
    }
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse2(input_ts).expect("e5f0e27b-e1fc-4d4d-b1f6-dbd56501ad66");
    let import_path = postgres_crud_macros_common::ImportPath::PostgresCrud;
    let gen_postgres_json_object_type_config = serde_json::from_str::<GenPostgresJsonTypesConfig>(
        &macros_helpers::get_macro_attribute_meta_list_ts(
            &syn_derive_input.attrs,
            &format!(
                "{}::postgres_json_object_type_config",
                import_path.sc_std_primitive_str()
            ),
        )
        .to_string(),
    )
    .expect("246de453-00ee-420a-a502-c3db0c1b984d");
    let postgres_json_object_type_record_vec = {
        let postgres_json_object_type_record = gen_postgres_json_object_type_config.variant;
        match (&postgres_json_object_type_record.not_null_or_nullable, &postgres_json_object_type_record.pattern) {
            (postgres_crud_macros_common::NotNullOrNullable::NotNull, Pattern::Standart) => vec![postgres_json_object_type_record],
            (postgres_crud_macros_common::NotNullOrNullable::Nullable, Pattern::Standart) |
            (postgres_crud_macros_common::NotNullOrNullable::NotNull, Pattern::Array) => vec![
                PostgresJsonObjectTypeRecord {
                    not_null_or_nullable: postgres_crud_macros_common::NotNullOrNullable::NotNull,
                    pattern: Pattern::Standart,
                    trait_gen: postgres_json_object_type_record.trait_gen.clone(),
                },
                postgres_json_object_type_record
            ],
            (postgres_crud_macros_common::NotNullOrNullable::Nullable, Pattern::Array) => vec![
                PostgresJsonObjectTypeRecord {
                    not_null_or_nullable: postgres_crud_macros_common::NotNullOrNullable::NotNull,
                    pattern: Pattern::Standart,
                    trait_gen: postgres_json_object_type_record.trait_gen.clone(),
                },
                PostgresJsonObjectTypeRecord {
                    not_null_or_nullable: postgres_crud_macros_common::NotNullOrNullable::Nullable,
                    pattern: Pattern::Standart,
                    trait_gen: postgres_json_object_type_record.trait_gen.clone(),
                },
                PostgresJsonObjectTypeRecord {
                    not_null_or_nullable: postgres_crud_macros_common::NotNullOrNullable::NotNull,
                    pattern: Pattern::Array,
                    trait_gen: postgres_json_object_type_record.trait_gen.clone(),
                },
                postgres_json_object_type_record
            ]
        }
    }
    // .into_iter()
    // .filter(|el_2f2d1e6c| {
    //     use postgres_crud_macros_common::NotNullOrNullable;
    //     let not_null_or_nullable_filter = match &el_2f2d1e6c.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => true,
    //         NotNullOrNullable::Nullable => true,
    //     };
    //     let pattern_filter = match &el_2f2d1e6c.pattern {
    //         Pattern::Standart => match &el_2f2d1e6c.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //         Pattern::Array => match &el_2f2d1e6c.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //     };
    //     let trait_gen_filter = match &el_2f2d1e6c.trait_gen {
    //         TraitGen::PostgresJsonType => true,
    //         TraitGen::PostgresTypeAndPostgresJsonType => true,
    //     };
    //     not_null_or_nullable_filter && pattern_filter && trait_gen_filter
    // })
    // .collect::<Vec<PostgresJsonObjectTypeRecord>>()
    ;
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GenPostgresJsonObjectTypeJsonVariants",
    //     &serde_json::to_string(&postgres_json_object_type_record_vec).expect("efc7a263-f6cd-44ca-aacf-470a37971f7f"),
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
    // let postgres_json_object_type_array
    let (fields_ts, postgres_json_object_type_array) = postgres_json_object_type_record_vec
        .into_iter()
        .enumerate()
        .map(|(index, element)| {
            #[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresJsonTypeSubtype {
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
            impl quote::ToTokens for PostgresJsonTypeSubtype {
                fn to_tokens(&self, tokens: &mut Ts2) {
                    self.to_string().parse::<Ts2>().expect("43ac0b62-551a-421e-aee0-9bf3dfffa3cc").to_tokens(tokens);
                }
            }
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresTypeSubtype {
                // TableTypeDeclaration,
                // Create,
                // Select,
                // Where,
                Read,
                // ReadOnlyIds,
                // ReadInner,
                Update,
            }
            impl quote::ToTokens for PostgresTypeSubtype {
                fn to_tokens(&self, tokens: &mut Ts2) {
                    self.to_string().parse::<Ts2>().expect("5825d4b7-dd55-41e4-b54e-7b31557181b6").to_tokens(tokens);
                }
            }
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate {
                TableTypeDeclaration,
                Create,
            }
            impl From<&PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate> for PostgresJsonTypeSubtype {
                fn from(value: &PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate) -> Self {
                    match &value {
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => Self::TableTypeDeclaration,
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Self::Create,
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
            let not_null_or_nullable = &element.not_null_or_nullable;
            let pattern = &element.pattern;
            let trait_gen = &element.trait_gen;

            let create_sc = CreateSc;
            let update_sc = UpdateSc;
            let delete_sc = DeleteSc;
            let value_ucc = ValueUcc;
            let value_sc = ValueSc;
            let as_ucc = AsUcc;
            let select_query_part_postgres_type_sc = SelectQueryPartPostgresTypeSc;
            let increment_sc = IncrementSc;
            let query_sc = QuerySc;
            let id_sc = IdSc;
            let error_sc = ErrorSc;
            let fields_sc = FieldsSc;
            let self_ucc = SelfUcc;
            let update_query_part_sc = UpdateQueryPartSc;
            let update_query_bind_sc = UpdateQueryBindSc;
            let jsonb_set_accumulator_sc = JsonbSetAccumulatorSc;
            let jsonb_set_target_sc = JsonbSetTargetSc;
            let jsonb_set_path_sc = JsonbSetPathSc;
            let column_name_and_maybe_field_getter_sc = ColumnNameAndMaybeFieldGetterSc;
            let select_query_part_sc = SelectQueryPartSc;
            let column_sc = ColumnSc;
            let self_sc = SelfSc;
            let read_sc = ReadSc;
            let equal_ucc = EqualUcc;
            let option_update_sc = OptionUpdateSc;
            let query_part_sc = QueryPartSc;
            let read_only_ids_sc = ReadOnlyIdsSc;
            let read_only_ids_to_two_dimensional_vec_read_inner_sc = ReadOnlyIdsToTwoDimensionalVecReadInnerSc;
            let select_only_ids_query_part_sc = SelectOnlyIdsQueryPartSc;
            let read_inner_into_read_with_new_or_try_new_unwraped_sc = ReadInnerIntoReadWithNewOrTryNewUnwrapedSc;
            let read_only_ids_into_option_value_read_inner_sc = ReadOnlyIdsIntoOptionValueReadInnerSc;
            let update_to_read_only_ids_sc = UpdateToReadOnlyIdsSc;
            let select_only_updated_ids_query_part_sc = SelectOnlyUpdatedIdsQueryPartSc;
            let select_only_created_ids_query_part_sc = SelectOnlyCreatedIdsQueryPartSc;
            let is_need_to_add_logical_operator_sc = IsNeedToAddLogicalOperatorSc;
            let select_only_updated_ids_query_bind_sc = SelectOnlyUpdatedIdsQueryBindSc;
            let select_only_created_ids_query_bind_sc = SelectOnlyCreatedIdsQueryBindSc;
            let read_inner_into_update_with_new_or_try_new_unwraped_sc = ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc;
            let option_vec_create_sc = OptionVecCreateSc;
            let postgres_json_type_ucc = PostgresJsonTypeUcc;
            let read_only_ids_merged_with_create_into_read_sc = ReadOnlyIdsMergedWithCreateIntoReadSc;
            let read_only_ids_to_option_value_read_default_option_some_vec_one_el_sc = ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc;
            let previous_read_merged_with_option_update_into_read_sc = PreviousReadMergedWithOptionUpdateIntoReadSc;
            let read_only_ids_merged_with_create_into_option_value_read_sc = ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc;
            let read_only_ids_merged_with_create_into_where_equal_sc = ReadOnlyIdsMergedWithCreateIntoWhereEqualSc;
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc = ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc;
            let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_sc = ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc;
            let read_only_ids_merged_with_create_into_table_type_declaration_sc = ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc;
            let create_into_postgres_json_type_option_vec_where_length_equal_sc = CreateIntoPostgresJsonTypeOptionVecWhereLengthEqualSc;
            let create_into_postgres_json_type_option_vec_where_length_greater_than_sc = CreateIntoPostgresJsonTypeOptionVecWhereLengthGreaterThanSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereGreaterThanSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereBetweenSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereInSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereRegularExpressionSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereContainsElGreaterThanSc;
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_sc = ReadOnlyIdsMergedWithCreateIntoPostgresJsonTypeOptionVecWhereContainsElRegularExpressionSc;
            let default_option_some_vec_one_el_ucc = DefaultOptionSomeVecOneElUcc;
            let default_option_some_vec_one_el_sc = DefaultOptionSomeVecOneElSc;

            let std_string_string_ts = token_patterns::StdStringString;
            let self_field_vec_ts = quote! {.0.to_vec()};
            let cfg_feature_test_utils = quote! {#[cfg(feature = "test-utils")]};
            let return_err_query_part_error_named_write_into_buffer_ts = postgres_crud_macros_common::gen_return_err_query_part_error_named_write_into_buffer_ts(import_path);
            let none_ts = quote!{None};
            let must_use_ts = token_patterns::MustUse;
            let allow_clippy_arbitrary_source_item_ordering_ts = token_patterns::AllowClippyArbitrarySourceItemOrdering;

            let gen_import_path_value_initialization_ts = |content_ts: &dyn quote::ToTokens|{
                postgres_crud_macros_common::gen_value_initialization_ts(
                    &import_path,
                    &content_ts
                )
            };

            let import_path_query_part_error_named_ts = {
                let query_part_error_named_ucc = QueryPartErrorNamedUcc;
                quote! {#import_path::#query_part_error_named_ucc}
            };
            let postgres_crud_default_option_some_vec_one_el_call_ts = token_patterns::PostgresCrudDefaultOptionSomeVecOneElCall;
            let postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts = token_patterns::PostgresCrudDefaultOptionSomeVecOneElMaxPageSizeCall;
            let vec_postgres_crud_default_option_some_vec_one_el_call_ts = quote!{vec![#postgres_crud_default_option_some_vec_one_el_call_ts]};
            let default_but_option_is_some_ts = quote!{
                #import_path::#default_option_some_vec_one_el_ucc::#default_option_some_vec_one_el_sc
            };
            let default_but_option_is_some_call_ts = quote!{#default_but_option_is_some_ts()};
            let gen_ident_as_default_but_option_is_some_call_ts = |ident_ts: &dyn quote::ToTokens, |{
                quote!{
                    <#ident_ts as #import_path::#default_option_some_vec_one_el_ucc>::#default_option_some_vec_one_el_sc()
                }
            };
            let gen_ident_as_default_but_option_is_some_ts = |ident_ts_2e6aba01: &dyn quote::ToTokens|quote!{
                <
                    #ident_ts_2e6aba01
                    as
                    #import_path::#default_option_some_vec_one_el_ucc
                >::#default_option_some_vec_one_el_sc
            };
            let import_path_value_ts = quote!{#import_path::#value_ucc};
            let wrap_into_value_declaration_ts = |ident_ts: &dyn quote::ToTokens|{
                quote!{#import_path_value_ts<#ident_ts>}
            };
            let wrap_into_value_initialization_ts = |content_ts: &dyn quote::ToTokens|{
                quote!{#import_path_value_ts { #value_sc: #content_ts }}
            };

            let syn_derive_input_ident = &syn_derive_input.ident;
            let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
                if let syn::Fields::Named(fields_named) = &data_struct.fields {
                    fields_named.named.iter()
                    .collect::<Vec<&syn::Field>>()
                    .iter()
                    .map(|el_f01f3f33|macros_helpers::SynFieldWrapper {
                        field_visibility: el_f01f3f33.vis.clone(),
                        field_ident: el_f01f3f33.ident.clone().expect("3ac7f263-e0bf-4c5b-9d44-57edf50f79b5"),
                        field_type: el_f01f3f33.ty.clone(),
                    })
                    .collect::<Vec<macros_helpers::SynFieldWrapper>>()
                } else {
                    panic!("4c305996-2508-4ea4-ab34-22c8c20c51f9");
                }
            } else {
                panic!("a4fc18a1-9fa0-40c6-a27f-91883a3ba6a1");
            };
            let is_standart_with_id_false = IsStandartWithId::False;
            let is_standart_with_id_true = IsStandartWithId::True;
            let gen_ident_ucc = |ident_pattern: &IdentPattern| {
                let vec_of_ucc = VecOfUcc;
                let array_of_ucc = ArrayOfUcc;
                let jsonb_object_ucc = JsonbObjectUcc;
                let with_id_ucc = WithIdUcc;
                let syn_derive_input_ident_str = syn_derive_input_ident.to_string();
                let jsonb_object_ucc_str = jsonb_object_ucc.to_string();
                let vec_of_syn_derive_input_ident_with_id = format!("{vec_of_ucc}{syn_derive_input_ident}{with_id_ucc}");
                let array_of_not_null_jsonb_object_with_id = format!("{array_of_ucc}{}{jsonb_object_ucc}{with_id_ucc}", postgres_crud_macros_common::NotNullOrNullable::NotNull);
                let (rust_part, postgres_part, current_not_null_or_nullable) = match &ident_pattern {
                    IdentPattern::StandartNotNullWithoutId => (syn_derive_input_ident_str, jsonb_object_ucc_str, postgres_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::StandartNotNullWithId => (format!("{syn_derive_input_ident}{with_id_ucc}"), format!("{jsonb_object_ucc}{with_id_ucc}"), postgres_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::StandartNullableWithoutId => (syn_derive_input_ident_str, jsonb_object_ucc_str, postgres_crud_macros_common::NotNullOrNullable::Nullable),
                    IdentPattern::ArrayNotNullWithId => (vec_of_syn_derive_input_ident_with_id, array_of_not_null_jsonb_object_with_id, postgres_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::ArrayNullableWithIdentifier => (vec_of_syn_derive_input_ident_with_id, array_of_not_null_jsonb_object_with_id, postgres_crud_macros_common::NotNullOrNullable::Nullable),
                };
                let current_not_null_or_nullable_rust = current_not_null_or_nullable.rust();
                format!("{current_not_null_or_nullable_rust}{rust_part}{as_ucc}{current_not_null_or_nullable}{postgres_part}").parse::<Ts2>().expect("43784dd3-f37a-438d-8bc8-d17f63feac66")
            };

            let ident = &gen_ident_ucc(&match (&not_null_or_nullable, &pattern) {
                (postgres_crud_macros_common::NotNullOrNullable::NotNull, Pattern::Standart) => IdentPattern::StandartNotNullWithoutId,
                (postgres_crud_macros_common::NotNullOrNullable::NotNull, Pattern::Array) => IdentPattern::ArrayNotNullWithId,
                (postgres_crud_macros_common::NotNullOrNullable::Nullable, Pattern::Standart) => IdentPattern::StandartNullableWithoutId,
                (postgres_crud_macros_common::NotNullOrNullable::Nullable, Pattern::Array) => IdentPattern::ArrayNullableWithIdentifier,
            });
            let ident_standart_not_null_ucc = &gen_ident_ucc(&IdentPattern::StandartNotNullWithoutId);
            let ident_array_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrayNotNullWithId);
            let ident_with_id_standart_not_null_ucc = &gen_ident_ucc(&IdentPattern::StandartNotNullWithId);
            let ident_with_id_array_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrayNotNullWithId);
            let is_standart_not_null = matches!((&not_null_or_nullable, pattern), (postgres_crud_macros_common::NotNullOrNullable::NotNull, Pattern::Standart));
            let gen_type_as_import_path_ts = |first_type_ts: &dyn quote::ToTokens, second_type_ts: &dyn quote::ToTokens|{
                quote! {<#first_type_ts as #import_path::#second_type_ts>}
            };
            let gen_type_as_postgres_json_type_ts = |type_ts: &dyn quote::ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &postgres_json_type_ucc)
            };
            let ident_as_import_path_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&ident);
            let ident_standart_not_null_as_import_path_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&ident_standart_not_null_ucc);
            let ident_array_not_null_as_import_path_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&ident_array_not_null_ucc);

            let uuid_uuid_as_not_null_jsonb_string_ucc = UuidUuidAsNotNullJsonbStringUcc;
            let uuid_uuid_as_not_null_jsonb_string_ts = quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_ucc};
            let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_display(&uuid_uuid_as_not_null_jsonb_string_ucc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_update_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_update_ucc = SelfUpdateUcc::from_display(&uuid_uuid_as_not_null_jsonb_string_ucc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_update_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_where_ucc = {
                let uuid_uuid_as_not_null_jsonb_string_where_ucc = SelfWhereUcc::from_display(&uuid_uuid_as_not_null_jsonb_string_ucc);
                quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_where_ucc}
            };
            let uuid_uuid_as_not_null_jsonb_string_as_import_path_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_update_ts = quote!{
                #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgres_json_type_ts::Update
            };
            let uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts = quote!{
                <#uuid_uuid_as_not_null_jsonb_string_ts as #import_path::PostgresJsonTypeObjectVecElementId>
            };
            let id_syn_field = {
                let value = syn::Field {
                    attrs: Vec::new(),
                    vis: syn::Visibility::Public(Pub { span: proc_macro2::Span::call_site() }),
                    mutability: syn::FieldMutability::None,
                    ident: Some(syn::Ident::new(&id_sc.to_string(), proc_macro2::Span::call_site())),
                    colon_token: Some(Colon { spans: [proc_macro2::Span::call_site()] }),
                    ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: macros_helpers::gen_simple_syn_punctuated_punctuated(
                                &[import_path.to_path(), &uuid_uuid_as_not_null_jsonb_string_ucc.to_string()]
                            ),
                        },
                    }),
                };
                macros_helpers::SynFieldWrapper {
                    field_visibility: value.vis.clone(),
                    field_ident: value.ident.clone().expect("3550d755-3fda-484f-8693-2c8e1f577b17"),
                    field_type: value.ty,
                }
            };
            let vec_syn_field_with_id: Vec<macros_helpers::SynFieldWrapper> = vec_syn_field.clone().into_iter().fold(vec![id_syn_field], |mut acc_9db5e042, el_f01f3f33| {
                acc_9db5e042.push(el_f01f3f33);
                acc_9db5e042
            });
            let get_vec_syn_field = |is_standart_with_id: &IsStandartWithId| -> &Vec<macros_helpers::SynFieldWrapper> {
                match &is_standart_with_id {
                    IsStandartWithId::False => &vec_syn_field,
                    IsStandartWithId::True => &vec_syn_field_with_id,
                }
            };
            let gen_type_as_postgres_type_ts = |type_ts: &dyn quote::ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PostgresTypeUcc)
            };
            let gen_type_as_postgres_json_type_test_cases_ts = |type_ts: &dyn quote::ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PostgresJsonTypeTestCasesUcc)
            };
            let gen_type_as_postgres_type_test_cases_ts = |type_ts: &dyn quote::ToTokens| {
                gen_type_as_import_path_ts(&type_ts, &PostgresTypeTestCasesUcc)
            };
            let self_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&self_ucc);
            let ident_standart_not_null_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(
                &ident_standart_not_null_ucc
            );
            let ident_standart_not_null_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&ident_standart_not_null_ucc);
            let import_path_postgres_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(
                &uuid_uuid_as_not_null_jsonb_string_ts
            );
            let ident_with_id_standart_not_null_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_create_ucc = SelfCreateUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_where_ucc = SelfWhereUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_ts = {
                let gen_struct_ident_ts = |current_ident: &dyn quote::ToTokens| macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy()
                    .build_struct(
                        &current_ident,
                        &quote!{;}
                    );
                let ident_ts = gen_struct_ident_ts(&ident);
                let maybe_ident_with_id_standart_not_null_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_ts = gen_struct_ident_ts(&ident_with_id_standart_not_null_ucc);
                    let cfg_feature_test_utils_impl_ident_with_id_standart_not_null_ts = {
                        let read_only_ids_merged_with_create_into_where_equal_ts = postgres_crud_macros_common::gen_read_only_ids_merged_with_create_into_where_equal_ts(
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_ts: &dyn quote::ToTokens,
                                |{
                                    let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                            #read_only_ids_sc.0.#value_sc.#field_ident,
                                            #second_argument_ts
                                        )
                                    }
                                };
                                let current_ident_ts = gen_ts(
                                    &id_sc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &postgres_crud_default_option_some_vec_one_el_call_ts
                                );
                                let content_ts = vec_syn_field.iter().map(|el_e970b03b| {
                                    let field_ident = &el_e970b03b.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_e970b03b.field_type,
                                        &quote!{#create_sc.#field_ident}
                                    )
                                });
                                quote!{
                                    #ident_with_id_standart_not_null_where_ucc::#equal_ucc(postgres_crud::PostgresJsonTypeWhereEqual {
                                        logical_operator: postgres_crud::LogicalOperator::Or,
                                        #value_sc: #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                            #current_ident_ts,
                                            #(#content_ts),*
                                        ),
                                    })
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = postgres_crud_macros_common::gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
                            &import_path,
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_ts: &dyn quote::ToTokens,
                                |{
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #ident_with_id_standart_not_null_where_ucc::#field_ident_ucc(
                                            postgres_crud::PostgresTypeWhere::new(
                                                postgres_crud::LogicalOperator::And,
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                                                    #read_only_ids_sc.0.#value_sc.#field_ident,
                                                    #second_argument_ts
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_ts = gen_ts(
                                    &id_sc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &postgres_crud_default_option_some_vec_one_el_call_ts
                                );
                                let content_ts = vec_syn_field.iter().map(|el_4fafbc5e| {
                                    let field_ident = &el_4fafbc5e.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_4fafbc5e.field_type,
                                        &quote!{#create_sc.#field_ident}
                                    )
                                });
                                quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_ts,
                                        #(#content_ts),*
                                    ]).expect("5473d8c4-2b10-4ba8-8a4a-704fde84f6ff")
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = postgres_crud_macros_common::gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
                            import_path,
                            &ident_with_id_standart_not_null_read_only_ids_ucc,
                            &ident_with_id_standart_not_null_create_ucc,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let gen_ts = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_ts: &dyn quote::ToTokens,
                                |{
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                    quote!{
                                        #ident_with_id_standart_not_null_where_ucc::#field_ident_ucc(
                                            #import_path::PostgresTypeWhere::new(
                                                #import_path::LogicalOperator::Or,
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_sc(
                                                    #read_only_ids_sc.0.#value_sc.#field_ident,
                                                    #second_argument_ts
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_ts = gen_ts(
                                    &id_sc,
                                    &uuid_uuid_as_not_null_jsonb_string_ts,
                                    &postgres_crud_default_option_some_vec_one_el_call_ts
                                );
                                let content_ts = vec_syn_field.iter().map(|el_649e1691| {
                                    let field_ident = &el_649e1691.field_ident;
                                    gen_ts(
                                        &field_ident,
                                        &el_649e1691.field_type,
                                        &quote!{#create_sc.#field_ident}
                                    )
                                });
                                quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_ts,
                                        #(#content_ts),*
                                    ]).expect("221a4c55-5467-44f1-94bb-b748a92cfada")
                                }
                            },
                        );
                        quote! {
                            #allow_clippy_arbitrary_source_item_ordering_ts
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
            let ident_array_not_null_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&ident_array_not_null_ucc);
            let ident_with_id_array_not_null_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&ident_with_id_array_not_null_ucc);
            let postgres_json_type_subtype_table_type_declaration = PostgresJsonTypeSubtype::TableTypeDeclaration;
            let postgres_json_type_subtype_create = PostgresJsonTypeSubtype::Create;
            let postgres_json_type_subtype_create_for_query = PostgresJsonTypeSubtype::CreateForQuery;
            let postgres_json_type_subtype_select = PostgresJsonTypeSubtype::Select;
            let postgres_json_type_subtype_where = PostgresJsonTypeSubtype::Where;
            let postgres_json_type_subtype_read = PostgresJsonTypeSubtype::Read;
            let postgres_json_type_subtype_read_inner = PostgresJsonTypeSubtype::ReadInner;
            let postgres_json_type_subtype_update = PostgresJsonTypeSubtype::Update;
            let postgres_json_type_subtype_update_for_query = PostgresJsonTypeSubtype::UpdateForQuery;
            let gen_type_as_postgres_json_type_subtype_ts = |type_ts: &dyn quote::ToTokens, postgres_json_type_subtype: &PostgresJsonTypeSubtype| {
                let type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&type_ts);
                quote! {#type_as_postgres_json_type_ts::#postgres_json_type_subtype}
            };
            let gen_type_as_postgres_type_subtype_ts = |type_ts: &dyn quote::ToTokens, postgres_type_subtype: &PostgresTypeSubtype| {
                let type_as_postgres_type_ts = gen_type_as_postgres_type_ts(&type_ts);
                quote! {#type_as_postgres_type_ts::#postgres_type_subtype}
            };
            let gen_field_type_as_crud_postgres_json_type_from_field_ts = |
                syn_field_wrapper: &macros_helpers::SynFieldWrapper
            | gen_type_as_postgres_json_type_ts(
                &syn_field_wrapper.field_type
            );
            let gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts = |current_ident_ts: &dyn quote::ToTokens| macros_helpers::gen_impl_error_occurence_lib_to_std_string_string_ts(
                &Ts2::new(),
                &current_ident_ts,
                &Ts2::new(),
                &quote! {format!("{self:?}")}
            );
            let ident_as_postgres_json_type_table_type_declaration_ts = gen_type_as_postgres_json_type_subtype_ts(&ident, &postgres_json_type_subtype_table_type_declaration);
            let self_value_ts = quote! {Self(#value_sc)};
            let postgres_type_where_filter_query_bind_value_query_ts = quote!{#import_path::PostgresTypeWhereFilter::query_bind(#value_sc, #query_sc)};

            let ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident);
            let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
            let ident_array_not_null_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident_array_not_null_ucc);
            let ident_standart_not_null_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_with_id_standart_not_null_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let wrap_into_scopes_ts = |content: &dyn quote::ToTokens| {
                quote! {(#content);}
            };
            let gen_ident_table_type_declaration_or_ident_create_common_ts = |postgres_json_type_subtype_table_type_declaration_or_create: &PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate| {
                let ident_table_type_declaration_or_ident_create_ucc: &dyn StdFmtDisplayPlusQuoteToTokens = match &postgres_json_type_subtype_table_type_declaration_or_create {
                    PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_table_type_declaration_ucc,
                    PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_create_ucc,
                };
                let gen_ident_table_type_declaration_or_create_ts = |
                    attributes_ts: &dyn quote::ToTokens,
                    current_ident_ts: &dyn quote::ToTokens,
                    content_ts: &dyn quote::ToTokens
                | {
                    let content_ts_44f35e48 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &current_ident_ts,
                        &content_ts
                    );
                    quote!{
                        #attributes_ts
                        #content_ts_44f35e48
                    }
                };
                let new_type_or_struct_declaration_struct_declaration = NewTypeOrStructDeclaration::StructDeclaration;
                let new_type_or_struct_declaration_new_type = NewTypeOrStructDeclaration::NewType;
                let gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_ts = |
                    is_standart_with_id: &IsStandartWithId,
                    current_postgres_json_type_subtype_table_type_declaration_or_create: &PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate,
                    new_type_or_struct_declaration: &NewTypeOrStructDeclaration
                | {
                    let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_42f25108| {
                        let field_ident = &el_42f25108.field_ident;
                        let type_as_postgres_json_type_subtype_table_type_declaration_ts = gen_type_as_postgres_json_type_subtype_ts(
                            &el_42f25108.field_type,
                            &PostgresJsonTypeSubtype::from(current_postgres_json_type_subtype_table_type_declaration_or_create)
                        );
                        quote! {#field_ident: #type_as_postgres_json_type_subtype_table_type_declaration_ts}
                    });
                    let fields_content_ts = quote! {#(#content_ts),*};
                    match &new_type_or_struct_declaration {
                        NewTypeOrStructDeclaration::StructDeclaration => quote! {{#fields_content_ts}},
                        NewTypeOrStructDeclaration::NewType => fields_content_ts,
                    }
                };
                let gen_tokens_table_type_declaration_or_create_ts = |tokens: &dyn quote::ToTokens| {
                    let value: &dyn quote::ToTokens = match &postgres_json_type_subtype_table_type_declaration_or_create {
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &SelfTableTypeDeclarationUcc::from_tokens(&tokens),
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &SelfCreateUcc::from_tokens(&tokens),
                    };
                    quote!{#value}
                };
                let ident_table_type_declaration_or_ident_create_ts = gen_ident_table_type_declaration_or_create_ts(
                    &match &postgres_json_type_subtype_table_type_declaration_or_create {
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => quote!{#allow_clippy_arbitrary_source_item_ordering_ts},
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Ts2::new(),
                    },
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_ts(&is_standart_with_id_false, postgres_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_struct_declaration),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(ident_standart_not_null_ucc))),
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => wrap_into_scopes_ts(&postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                                &gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_standart_not_null_ucc)
                            )),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_array_not_null_ucc))),
                        },
                    }
                );
                let gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts = |is_standart_with_id: &IsStandartWithId| {
                    let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_42f25108|&el_42f25108.field_ident);
                    quote! {Self {#(#content_ts),*}}
                };
                let impl_pub_new_for_ident_table_type_declaration_or_ident_create_ts = {
                    let parameters_ts = {
                        let gen_wrap_into_value_parameter_ts = |type_ts: &dyn quote::ToTokens| {
                            quote! {value: #type_ts}
                        };
                        match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_ts(&is_standart_with_id_false, postgres_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_new_type),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_wrap_into_value_parameter_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(ident_standart_not_null_ucc))),
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_wrap_into_value_parameter_ts(&postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(&ident_with_id_standart_not_null_ucc))),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_wrap_into_value_parameter_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&gen_tokens_table_type_declaration_or_create_ts(
                                    &ident_with_id_standart_not_null_ucc,
                                )))),
                            },
                        }
                    };
                    let content_ts = match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts(&is_standart_with_id_false),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => self_value_ts.clone(),
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => self_value_ts.clone(),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_array_not_null_with_id_postfix_ucc = gen_tokens_table_type_declaration_or_create_ts(&gen_ident_ucc(&IdentPattern::ArrayNotNullWithId));
                                quote! {Self(#value_sc.map(#ident_array_not_null_with_id_postfix_ucc::new))}
                            }
                        },
                    };
                    if matches!(&pattern, Pattern::Array) && matches!(&not_null_or_nullable, postgres_crud_macros_common::NotNullOrNullable::Nullable) {
                        macros_helpers::gen_impl_pub_new_for_ident_ts(
                            &ident_table_type_declaration_or_ident_create_ucc,
                            &must_use_ts,
                            &parameters_ts,
                            &content_ts,
                        )
                    }
                    else {
                        macros_helpers::gen_impl_pub_const_new_for_ident_ts(
                            &ident_table_type_declaration_or_ident_create_ucc,
                            &must_use_ts,
                            &parameters_ts,
                            &content_ts,
                        )
                    }
                };
                let gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts = |
                    current_ident_ts: &dyn quote::ToTokens,
                    content_ts: &dyn quote::ToTokens
                | postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(
                    &current_ident_ts,
                    &Ts2::new(),
                    &quote! {Self #content_ts}
                );
                let gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts = |is_standart_with_id: &IsStandartWithId| {
                    let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_6c071492| {
                        let field_ident = &el_6c071492.field_ident;
                        quote! {#field_ident: #postgres_crud_default_option_some_vec_one_el_call_ts}
                    });
                    quote! {{
                        #(#content_ts),*
                    }}
                };
                let impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts = gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts(&is_standart_with_id_false);
                let scopes_vec_postgres_crud_default_option_some_vec_one_el_call_ts = quote! {(#vec_postgres_crud_default_option_some_vec_one_el_call_ts)};
                let scopes_some_postgres_crud_default_option_some_vec_one_el_call_ts = quote! {
                    (Some(#postgres_crud_default_option_some_vec_one_el_call_ts))
                };
                let impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_ident_create_ts = gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                            Pattern::Standart => &impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts,
                            Pattern::Array => &scopes_vec_postgres_crud_default_option_some_vec_one_el_call_ts,
                        },
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => &scopes_some_postgres_crud_default_option_some_vec_one_el_call_ts,
                    },
                );
                let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &quote!{sqlx::types::Json(#self_sc)}
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(
                    &ident_table_type_declaration_or_ident_create_ucc,
                    &quote!{sqlx::types::Json<#self_ucc>}
                );
                let maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = if is_standart_not_null {
                    let ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc: &dyn StdFmtDisplayPlusQuoteToTokens = match &postgres_json_type_subtype_table_type_declaration_or_create {
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_with_id_standart_not_null_table_type_declaration_ucc,
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_with_id_standart_not_null_create_ucc,
                    };
                    let current_is_standart_with_id = match &postgres_json_type_subtype_table_type_declaration_or_create {
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &is_standart_with_id_true,
                        PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &is_standart_with_id_false,
                    };
                    let ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = gen_ident_table_type_declaration_or_create_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_ts(current_is_standart_with_id, postgres_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_struct_declaration),
                    );
                    let impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = macros_helpers::gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &must_use_ts,
                        &gen_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_ts(current_is_standart_with_id, postgres_json_type_subtype_table_type_declaration_or_create, &new_type_or_struct_declaration_new_type),
                        &gen_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_ts(current_is_standart_with_id),
                    );
                    let impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts = gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_ts(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_ucc,
                        &match &postgres_json_type_subtype_table_type_declaration_or_create {
                            PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts(&is_standart_with_id_true),
                            PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_create_standart_not_null_content_ts,
                        },
                    );
                    quote! {
                        #ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                        #impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                        #impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_table_type_declaration_or_ident_create_ts
                    #impl_pub_new_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_postgres_crud_default_option_some_vec_one_el_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_ts
                    #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_ts
                    #maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_ts
                }
            };

            let ident_table_type_declaration_ts = {
                let ident_table_type_declaration_common_ts = gen_ident_table_type_declaration_or_ident_create_common_ts(&PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration);
                quote! {
                    #ident_table_type_declaration_common_ts
                }
            };
            let gen_type_as_postgres_json_type_create_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_create);
            let gen_type_as_postgres_json_type_create_for_query_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_create_for_query);
            let ident_create_ts = {
                let ident_create_common_ts = gen_ident_table_type_declaration_or_ident_create_common_ts(&PostgresJsonTypeSubtypeTableTypeDeclarationOrCreate::Create);
                let gen_impl_std_fmt_display_for_ident_create_ts = |current_ident_ts: &dyn quote::ToTokens| macros_helpers::gen_impl_std_fmt_display_ts(
                    &Ts2::new(),
                    &current_ident_ts, &Ts2::new(),
                    &quote! {write!(f, "{self:?}")}
                );
                let impl_std_fmt_display_for_ident_create_ts = gen_impl_std_fmt_display_for_ident_create_ts(&ident_create_ucc);
                let impl_error_occurence_lib_to_std_string_string_for_ident_create_ts = gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts(&ident_create_ucc);
                let maybe_ident_with_id_create_standart_not_null_ts = if is_standart_not_null {
                    let impl_std_fmt_display_for_ident_with_id_create_standart_not_null_ts = gen_impl_std_fmt_display_for_ident_create_ts(&ident_with_id_standart_not_null_create_ucc);
                    let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_ts = gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts(&ident_with_id_standart_not_null_create_ucc);
                    quote! {
                        #impl_std_fmt_display_for_ident_with_id_create_standart_not_null_ts
                        #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_create_common_ts
                    #impl_std_fmt_display_for_ident_create_ts
                    #impl_error_occurence_lib_to_std_string_string_for_ident_create_ts
                    #maybe_ident_with_id_create_standart_not_null_ts
                }
            };
            let ident_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident);
            let self_as_postgres_json_type_create_ts = gen_type_as_postgres_json_type_create_ts(&self_ucc);
            let ident_standart_not_null_as_postgres_json_type_create_for_query_ts = gen_type_as_postgres_json_type_create_for_query_ts(&ident_standart_not_null_ucc);
            let ident_array_not_null_as_postgres_json_type_create_for_query_ts = gen_type_as_postgres_json_type_create_for_query_ts(&ident_array_not_null_ucc);
            let ident_array_not_null_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&ident_array_not_null_ucc);
            let postgres_crud_path_postgres_json_type_uuid_uuid_create_for_query_ts = gen_type_as_postgres_json_type_create_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let gen_debug_clone_partialeq_serialize_pub_struct_ts = |
                attributes_ts: &dyn quote::ToTokens,
                current_ident_ts: &dyn quote::ToTokens,
                content_ts_153ac202: &dyn quote::ToTokens
            | {
                let content_ts_6ea2da58 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .build_struct(
                        &current_ident_ts,
                        &content_ts_153ac202
                    );
                quote!{
                    #attributes_ts
                    #content_ts_6ea2da58
                }
            };
            let ident_create_for_query_ts = {
                let gen_struct_standart_not_null_content_ts = |is_standart_with_id: &IsStandartWithId|{
                    let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_53c802d8| {
                        let field_ident = &el_53c802d8.field_ident;
                        let type_as_postgres_json_type_subtype_crate_for_query_ts = gen_type_as_postgres_json_type_subtype_ts(
                            &el_53c802d8.field_type,
                            &PostgresJsonTypeSubtype::CreateForQuery
                        );
                        quote! {#field_ident: #type_as_postgres_json_type_subtype_crate_for_query_ts}
                    });
                    quote! {{#(#content_ts),*}}
                };
                let impl_std_convert_from_standart_not_null_without_id_content_ts = {
                    let content_ts = vec_syn_field.iter().map(|el_0fc1e145| {
                        let field_ident = &el_0fc1e145.field_ident;
                        let type_as_postgres_json_type_subtype_crate_for_query_ts = gen_type_as_postgres_json_type_subtype_ts(
                            &el_0fc1e145.field_type,
                            &PostgresJsonTypeSubtype::CreateForQuery
                        );
                        quote! {#field_ident: #type_as_postgres_json_type_subtype_crate_for_query_ts::from(#value_sc.#field_ident)}
                    });
                    quote! {#(#content_ts),*}
                };
                let ident_create_for_query_ts = {
                    let ident_create_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
                        &ident_create_for_query_ucc,
                        &match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_struct_standart_not_null_content_ts(&is_standart_with_id_false),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    wrap_into_scopes_ts(
                                        &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                            &gen_type_as_postgres_json_type_subtype_ts(
                                                &ident_standart_not_null_ucc,
                                                &postgres_json_type_subtype_create_for_query,
                                            )
                                        )
                                    )
                                },
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => wrap_into_scopes_ts(
                                    &postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                                        &ident_with_id_standart_not_null_create_for_query_ucc
                                    )
                                ),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_ts(
                                    &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                        &gen_type_as_postgres_json_type_subtype_ts(
                                            &ident_array_not_null_ucc,
                                            &postgres_json_type_subtype_create_for_query,
                                        )
                                    )
                                ),
                            },
                        }
                    );
                    let impl_std_convert_from_ident_create_for_ident_create_for_query_ts = macros_helpers::gen_impl_std_convert_from_ts(
                        &ident_create_ucc,
                        &ident_create_for_query_ucc,
                        &{
                            let content_ts = match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                    Pattern::Standart => quote! {{#impl_std_convert_from_standart_not_null_without_id_content_ts}},
                                    Pattern::Array => quote!{(
                                        #value_sc.0.into_iter().map(#ident_with_id_standart_not_null_create_for_query_ucc::from).collect()
                                    )},
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_ts: &dyn quote::ToTokens = match &pattern {
                                        Pattern::Standart => &ident_standart_not_null_as_postgres_json_type_create_for_query_ts,
                                        Pattern::Array => &ident_array_not_null_as_postgres_json_type_create_for_query_ts,
                                    };
                                    quote!{(#value_sc.0.map(#content_ts::from))}
                                },
                            };
                            quote! {Self #content_ts}
                        }
                    );
                    let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(
                        &ident_create_for_query_ucc,
                        &quote!{sqlx::types::Json(#self_sc)}
                    );
                    let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(
                        &ident_create_for_query_ucc,
                        &quote!{sqlx::types::Json<#self_ucc>}
                    );
                    quote! {
                        #ident_create_for_query_ts
                        #impl_std_convert_from_ident_create_for_ident_create_for_query_ts
                        #impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_ts
                        #impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_ts
                    }
                };
                let maybe_ident_with_id_standart_not_null_create_for_query_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_create_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
                        &ident_with_id_standart_not_null_create_for_query_ucc,
                        &gen_struct_standart_not_null_content_ts(&is_standart_with_id_true)
                    );
                    let impl_std_convert_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_ts = macros_helpers::gen_impl_std_convert_from_ts(
                        &ident_with_id_standart_not_null_create_ucc,
                        &ident_with_id_standart_not_null_create_for_query_ucc,
                        &quote! {Self {
                            #id_sc: #postgres_crud_path_postgres_json_type_uuid_uuid_create_for_query_ts::new(
                                uuid::Uuid::new_v4()
                            ),
                            #impl_std_convert_from_standart_not_null_without_id_content_ts
                        }}
                    );
                    quote! {
                        #ident_with_id_standart_not_null_create_for_query_ts
                        #impl_std_convert_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_create_for_query_ts
                    #maybe_ident_with_id_standart_not_null_create_for_query_ts
                }
            };
            let gen_sqlx_types_json_type_declaration_wrapper_ts = |current_ident_ts: &dyn quote::ToTokens| postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(
                &current_ident_ts,
                &postgres_crud_macros_common::gen_sqlx_types_json_type_declaration_ts(&self_ucc)
            );
            let gen_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_ts = |current_ident_ts: &dyn quote::ToTokens| postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(
                &current_ident_ts,
                &postgres_crud_macros_common::gen_sqlx_types_json_type_declaration_ts(&self_ucc),
                &quote! {Ok(value_147c3532.0)}
            );
            let gen_value_type_ts = |type_ts: &dyn quote::ToTokens| {
                quote! {#value_sc: #type_ts}
            };
            let gen_pub_const_new_value_type_content_self_value_ts = |content_ts: &dyn quote::ToTokens|macros_helpers::gen_pub_const_new_ts(
                &must_use_ts,
                &gen_value_type_ts(&content_ts),
                &self_value_ts
            );
            let gen_unique_vec_wrapper_ts = |type_ts: &dyn quote::ToTokens| {
                quote! {#import_path::NotEmptyUniqueVec<#type_ts>}
            };
            let self_some_postgres_crud_default_option_some_vec_one_el_call_ts = quote! {
                Self(Some(#postgres_crud_default_option_some_vec_one_el_call_ts))
            };
            let self_some_postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts = quote! {
                Self(Some(#postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts))
            };
            let wrap_content_into_scopes_dot_comma_ts = |content_ts: &dyn quote::ToTokens| {
                let scopes_content_ts = postgres_crud_macros_common::wrap_content_into_scopes_ts(&content_ts);
                quote! {#scopes_content_ts;}
            };
            let gen_type_as_postgres_json_type_update_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_update);
            let gen_type_as_postgres_json_type_update_for_query_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_update_for_query);
            let self_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&self_ucc);
            let self_as_postgres_json_type_update_ts = gen_type_as_postgres_json_type_update_ts(&self_ucc);
            let self_as_postgres_json_type_create_for_query_ts = gen_type_as_postgres_json_type_create_for_query_ts(&self_ucc);
            let postgres_crud_path_postgres_json_type_uuid_uuid_update_ts = gen_type_as_postgres_json_type_update_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let postgres_crud_path_postgres_json_type_uuid_uuid_update_for_query_ts = gen_type_as_postgres_json_type_update_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
            let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let gen_type_as_postgres_json_type_select_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_select);
            let ident_standart_not_null_as_postgres_json_type_select_ts = gen_type_as_postgres_json_type_select_ts(&ident_standart_not_null_ucc);
            let ident_with_id_array_not_null_as_postgres_json_type_select_ts = gen_type_as_postgres_json_type_select_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_standart_not_null_select_sc = SelfSelectSc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let dimension1_pagination_ts = quote! {dimension1_pagination};
            let ident_standart_not_null_select_el_ucc = SelfSelectElementUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_with_id_standart_not_null_select_el_ucc = SelfSelectElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let gen_select_query_part_for_loop_ts = |
                acc_ts: &dyn quote::ToTokens,
                is_standart_with_id: &IsStandartWithId,
                in_ts: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_field_ident_ts: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_for_error_message_field_ident_ts: &dyn quote::ToTokens,
            |{
                let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_f3a1af0f| {
                    let field_ident_str = el_f3a1af0f.field_ident.to_string();
                    let variant_name_ts: &dyn quote::ToTokens = &AsRefStrToUccTs::case_or_panic(&field_ident_str);
                    let field_ident_double_quotes_ts: &dyn quote::ToTokens = &gen_quotes::double_quotes_ts(&field_ident_str);
                    let field_type_as_crud_postgres_json_type_from_field_ts = gen_type_as_postgres_json_type_ts(&el_f3a1af0f.field_type);
                    let ident_or_ident_with_id_standart_not_null_select_el_ucc: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_standart_not_null_select_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_el_ucc,
                    };
                    quote! {
                        #ident_or_ident_with_id_standart_not_null_select_el_ucc::#variant_name_ts(value_3c8acf6a) => match #field_type_as_crud_postgres_json_type_from_field_ts::#select_query_part_sc(
                            value_3c8acf6a,
                            #field_ident_double_quotes_ts,
                            #column_name_and_maybe_field_getter_field_ident_ts,
                            #column_name_and_maybe_field_getter_for_error_message_field_ident_ts,
                            false,
                        ) {
                            Ok(value_d54cf786) => value_d54cf786,
                            Err(#error_sc) => {
                                return Err(#error_sc);
                            }
                        }
                    }
                });
                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                    &quote!{
                        #acc_ts,
                        "{}||",
                        match el_0127bf54 {
                            #(#content_ts),*
                        }
                    },
                    &return_err_query_part_error_named_write_into_buffer_ts
                );
                quote!{
                    for el_0127bf54 in #in_ts #self_field_vec_ts {
                        #if_write_is_err_ts
                    }
                }
            };
            let ident_select_ts = {
                let gen_pub_struct_ident_select_ts = |
                    attributes_ts: &dyn quote::ToTokens,
                    current_ident_ts: &dyn quote::ToTokens,
                    content_ts_fc7ad384: &dyn quote::ToTokens
                | {
                    let content_ts_83d3ad18 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &current_ident_ts,
                        &content_ts_fc7ad384
                    );
                    quote!{
                        #attributes_ts
                        #content_ts_83d3ad18
                    }
                };
                let gen_ident_select_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_standart_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_standart_not_null_ucc);
                    gen_pub_struct_ident_select_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
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
                let ident_select_ts = match &not_null_or_nullable {
                    postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                        Pattern::Standart => gen_ident_select_standart_not_null_ts(&is_standart_with_id_false),
                        Pattern::Array => gen_pub_struct_ident_select_ts(
                            &allow_clippy_arbitrary_source_item_ordering_ts,
                            &ident_select_ucc,
                            &quote! {{
                                #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_ucc,
                                #dimension1_pagination_ts: #import_path_pagination_ts
                            }},
                        ),
                    },
                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_pub_struct_ident_select_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
                        &ident_select_ucc,
                        &wrap_content_into_scopes_dot_comma_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&match &pattern {
                            Pattern::Standart => &ident_standart_not_null_as_postgres_json_type_select_ts,
                            Pattern::Array => &ident_with_id_array_not_null_as_postgres_json_type_select_ts,
                        })),
                    ),
                };
                let impl_ident_select_ts = {
                    let pub_new_ts = {
                        let parameters_ts = {
                            let unique_vec_ident_select_el_standart_not_null_ts = gen_unique_vec_wrapper_ts(&ident_standart_not_null_select_el_ucc);
                            match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_value_type_ts(&unique_vec_ident_select_el_standart_not_null_ts),
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_value_type_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&unique_vec_ident_select_el_standart_not_null_ts)),
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {
                                        #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_ucc,
                                        #dimension1_pagination_ts: #import_path_pagination_ts
                                    },
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_value_type_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&ident_with_id_array_not_null_as_postgres_json_type_select_ts)),
                                },
                            }
                        };
                        let content_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => self_value_ts.clone(),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                    Self(#value_sc.map(#ident_standart_not_null_as_postgres_json_type_select_ts::new))
                                },
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    quote! {Self {
                                        #ident_with_id_standart_not_null_select_sc,
                                        #dimension1_pagination_ts,
                                    }}
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => self_value_ts.clone(),
                            },
                        };
                        if matches!(&pattern, Pattern::Standart) && matches!(&not_null_or_nullable, postgres_crud_macros_common::NotNullOrNullable::Nullable) {
                            macros_helpers::gen_pub_new_ts(
                                &must_use_ts,
                                &parameters_ts,
                                &content_ts
                            )
                        }
                        else {
                             macros_helpers::gen_pub_const_new_ts(
                                &must_use_ts,
                                &parameters_ts,
                                &content_ts
                            )
                        }
                    };
                    let maybe_select_query_part_ts = if matches!(&pattern, Pattern::Standart) &&
                    matches!(&not_null_or_nullable, postgres_crud_macros_common::NotNullOrNullable::NotNull) {
                        let acc_ac57d097_ts = quote!{acc_ac57d097};
                        let select_query_part_for_loop_ts = gen_select_query_part_for_loop_ts(
                            &acc_ac57d097_ts,
                            &is_standart_with_id_false,
                            &self_sc,
                            &quote!{column_name_and_maybe_field_getter},
                            &quote!{column_name_and_maybe_field_getter_for_error_message},
                        );
                        quote! {
                            fn #select_query_part_sc(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                column_name_and_maybe_field_getter_for_error_message: &str,
                            ) -> Result<#std_string_string_ts, #import_path_query_part_error_named_ts> {
                                let mut #acc_ac57d097_ts = #std_string_string_ts::default();
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
                    let select_query_part_postgres_type_ts = {
                        let content_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {
                                    #self_sc.#select_query_part_sc(
                                        #column_sc,
                                        #column_sc,
                                    )
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let ident_740c9034 = match &pattern {
                                        Pattern::Standart => &ident_standart_not_null_as_postgres_json_type_select_ts,
                                        Pattern::Array => &ident_with_id_array_not_null_as_postgres_json_type_select_ts,
                                    };
                                    quote! {
                                        let #value_sc = self.0.as_ref().map_or_else(
                                            <#ident_740c9034 as postgres_crud::DefaultOptionSomeVecOneEl>::default_option_some_vec_one_el,
                                            Clone::clone
                                        );
                                        match #value_sc.#select_query_part_postgres_type_sc(#column_sc) {
                                            Ok(value_c69f1ffe) => Ok(format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c69f1ffe}) end")),
                                            Err(#error_sc) => Err(#error_sc)
                                        }
                                    }
                                },
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let acc_399d9786_ts = quote!{acc_399d9786};
                                    let select_query_part_for_loop_ts = gen_select_query_part_for_loop_ts(
                                        &acc_399d9786_ts,
                                        &is_standart_with_id_true,
                                        &quote!{#self_sc.#ident_with_id_standart_not_null_select_sc},
                                        &gen_quotes::double_quotes_ts(&value_sc),
                                        &column_sc
                                    );
                                    let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                        "(case when (jsonb_array_length({{column}}) = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_sc}}})) from jsonb_array_elements((select {{column}})) with ordinality where ordinality between {{dimension1_start}} and {{dimension1_end}}) end)"
                                    ));
                                    quote! {
                                        let #ident_with_id_standart_not_null_select_sc = {
                                            let mut #acc_399d9786_ts = #std_string_string_ts::default();
                                            #select_query_part_for_loop_ts
                                            let _: Option<char> = #acc_399d9786_ts.pop();
                                            let _: Option<char> = #acc_399d9786_ts.pop();
                                            #acc_399d9786_ts
                                        };
                                        let dimension1_start = self.#dimension1_pagination_ts.start();
                                        let dimension1_end = self.#dimension1_pagination_ts.end();
                                        Ok(format!(#format_handle_ts))
                                    }
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let format_handle_ts = gen_quotes::double_quotes_ts(&"case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c2ca032e}) end");
                                    let ident_with_id_array_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                        &ident_with_id_array_not_null_as_postgres_json_type_select_ts
                                    );
                                    quote! {
                                        let #value_sc = self.0.as_ref().map_or_else(
                                            #ident_with_id_array_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts,
                                            Clone::clone
                                        );
                                        match #value_sc.#select_query_part_postgres_type_sc(column) {
                                            Ok(value_c2ca032e) => Ok(format!(#format_handle_ts)),
                                            Err(#error_sc) => Err(#error_sc)
                                        }
                                    }
                                },
                            },
                        };
                        quote! {
                            fn #select_query_part_postgres_type_sc(
                                &self,
                                #column_sc: &str,
                            ) -> Result<#std_string_string_ts, #import_path_query_part_error_named_ts> {
                                #content_ts
                            }
                        }
                    };
                    quote! {
                        impl #ident_select_ucc {
                            #pub_new_ts
                            #maybe_select_query_part_ts
                            #select_query_part_postgres_type_ts
                        }
                    }
                };
                let impl_sqlx_type_sqlx_postgres_for_ident_select_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_select_ucc);
                let impl_sqlx_decode_sqlx_postgres_for_ident_select_ts = gen_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_ts(&ident_select_ucc);
                let impl_postgres_crud_default_option_some_vec_one_el_standart_not_null_content_ts = quote! {
                    Self(#postgres_crud_default_option_some_vec_one_el_call_ts)
                };
                let impl_postgres_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_content_ts = quote! {
                    Self(#postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts)
                };
                let (
                    impl_postgres_crud_default_option_some_vec_one_el_for_ident_select_ts,
                    impl_postgres_crud_default_option_some_vec_one_el_max_page_size_for_ident_select_ts
                ) = {
                    let gen_default_some_one_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                        match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => quote! {#impl_postgres_crud_default_option_some_vec_one_el_standart_not_null_content_ts},
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => quote! {#impl_postgres_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_content_ts},
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_postgres_crud_default_option_some_vec_one_el_call_ts.clone(),
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts.clone(),
                                },
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let (
                                        ident_with_id_standart_not_null_select_content_ts,
                                        dimension1_pagination_content_ts
                                    ): (
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens
                                    ) = match &default_some_one_or_default_some_one_with_max_page_size {
                                        postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => (
                                            &postgres_crud_default_option_some_vec_one_el_call_ts,
                                            &postgres_crud_default_option_some_vec_one_el_call_ts
                                        ),
                                        postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => (
                                            &postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts,
                                            &postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts
                                        ),
                                    };
                                    quote! {
                                        Self {
                                            #ident_with_id_standart_not_null_select_sc: #ident_with_id_standart_not_null_select_content_ts,
                                            #dimension1_pagination_ts: #dimension1_pagination_content_ts,
                                        }
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_postgres_crud_default_option_some_vec_one_el_call_ts.clone(),
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts.clone(),
                                },
                            },
                        }
                    };
                    (
                        postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(
                            &ident_select_ucc,
                            &Ts2::new(),
                            &gen_default_some_one_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                        ),
                        postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_max_page_size_ts(
                            &ident_select_ucc,
                            &Ts2::new(),
                            &gen_default_some_one_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                        )
                    )
                };
                let gen_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_select_el_or_ident_with_id_select_el_ucc: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_standart_not_null_select_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_el_ucc,
                    };
                    let ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = {
                        let content_ts_bf3bd19e = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                            &{
                                let content_ts_ecc4a666 = get_vec_syn_field(is_standart_with_id).iter().map(|el_840c2253| {
                                    let field_ident = &el_840c2253.field_ident;
                                    let serialize_deserialize_field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                    let variant_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_json_type_select_ts = gen_type_as_postgres_json_type_select_ts(&el_840c2253.field_type);
                                    quote! {
                                        #[serde(rename(serialize = #serialize_deserialize_field_ident_double_quotes_ts, deserialize = #serialize_deserialize_field_ident_double_quotes_ts))]
                                        #variant_ident_ucc_ts(#field_type_as_json_type_select_ts)
                                    }
                                });
                                quote!{{#(#content_ts_ecc4a666),*}}
                            }
                        );
                        quote!{
                            #allow_clippy_arbitrary_source_item_ordering_ts
                            #content_ts_bf3bd19e
                        }
                    };
                    let impl_error_occurence_lib_to_std_string_string_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts = gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts(&ident_select_el_or_ident_with_id_select_el_ucc);
                    let (
                        impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts,
                        impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_with_max_page_size_ts
                    ) = {
                        let gen_default_some_one_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                            let vec_content_ts = {
                                let content_ts: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgres_crud_default_option_some_vec_one_el_call_ts,
                                    postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgres_crud_default_option_some_vec_one_el_max_page_size_call_ts,
                                };
                                let elements_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_20fc16d2| {
                                    let field_ident = &el_20fc16d2.field_ident;
                                    let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    quote! {#self_ucc::#field_ident_ucc_ts(#content_ts)}
                                });
                                quote! {#(#elements_ts),*}
                            };
                            quote! {vec![
                                #vec_content_ts
                            ]}
                        };
                        (
                            postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_ts(
                                &ident_select_el_or_ident_with_id_select_el_ucc,
                                &gen_default_some_one_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                            ),
                            postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_max_page_size_ts(
                                &ident_select_el_or_ident_with_id_select_el_ucc,
                                &gen_default_some_one_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                            )
                        )
                    };
                    quote! {
                        #ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_error_occurence_lib_to_std_string_string_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts
                        #impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_select_el_or_ident_with_id_standart_not_null_select_el_with_max_page_size_ts
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
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_select_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_with_id_standart_not_null_select_ucc);
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_select_ts = gen_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_ts(&ident_with_id_standart_not_null_select_ucc);
                    let impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_select_ts = postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(
                        &ident_with_id_standart_not_null_select_ucc,
                        &Ts2::new(),
                        &impl_postgres_crud_default_option_some_vec_one_el_standart_not_null_content_ts
                    );
                    let impl_postgres_crud_default_option_some_vec_one_el_max_page_size_for_ident_with_id_standart_not_null_select_ts = postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_max_page_size_ts(
                        &ident_with_id_standart_not_null_select_ucc,
                        &Ts2::new(),
                        &impl_postgres_crud_default_option_some_vec_one_el_max_page_size_standart_not_null_content_ts
                    );
                    let ident_with_id_select_el_ts = gen_ident_select_el_or_ident_with_id_standart_not_null_select_el_ts(&is_standart_with_id_true);
                    quote! {
                        #ident_with_id_standart_not_null_select_ts
                        #impl_ident_with_id_standart_not_null_select_ts
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_select_ts
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_select_ts
                        #impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_select_ts
                        #impl_postgres_crud_default_option_some_vec_one_el_max_page_size_for_ident_with_id_standart_not_null_select_ts
                        #ident_with_id_select_el_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_select_ts
                    #impl_ident_select_ts
                    #impl_sqlx_type_sqlx_postgres_for_ident_select_ts
                    #impl_sqlx_decode_sqlx_postgres_for_ident_select_ts
                    #impl_postgres_crud_default_option_some_vec_one_el_for_ident_select_ts
                    #impl_postgres_crud_default_option_some_vec_one_el_max_page_size_for_ident_select_ts
                    #maybe_ident_select_el_ts
                    #maybe_ident_with_id_standart_not_null_select_ts
                }
            };
            let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
            let ident_where_ts = match &not_null_or_nullable {
                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                    use postgres_crud_macros_common::NotNullOrNullable;
                    let gen_ident_where_field_variants_ts = |is_standart_with_id: &IsStandartWithId| {
                        let variants_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_622d1e96| {
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_622d1e96.field_ident.to_string());
                            let field_type_as_json_type_where_ts = gen_type_as_postgres_json_type_subtype_ts(
                                &el_622d1e96.field_type,
                                &postgres_json_type_subtype_where
                            );
                            quote! {
                                #field_ident_ucc_ts(#import_path::PostgresTypeWhere<
                                    #field_type_as_json_type_where_ts
                                >)
                            }
                        });
                        quote! {#(#variants_ts),*}
                    };
                    let gen_ident_where_ts = |
                        attributes_ts: &dyn quote::ToTokens,
                        current_ident_ts: &dyn quote::ToTokens,
                        content_ts_e1af2d89: &dyn quote::ToTokens
                    | {
                        let content_ts_60d5d187 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_enum(
                            &current_ident_ts,
                            &quote!{{#content_ts_e1af2d89}}
                        );
                        quote!{
                            #attributes_ts
                            #content_ts_60d5d187
                        }
                    };
                    let equal_variant_ident_ts = quote! {#equal_ucc(#import_path::PostgresJsonTypeWhereEqual<#ident_as_postgres_json_type_table_type_declaration_ts>)};
                    let equal_variant_query_part_ts = quote!{
                        #self_ucc::#equal_ucc(#value_sc) => #import_path::PostgresTypeWhereFilter::query_part(
                            #value_sc,
                            #increment_sc,
                            &#column_sc,
                            is_need_to_add_logical_operator
                        )
                    };
                    let equal_variant_query_bind_ts = quote!{
                        #self_ucc::#equal_ucc(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts
                    };
                    let maybe_ident_where_ts = {
                        let gen_ident_where_wrapper_ts = |content_ts: &dyn quote::ToTokens| gen_ident_where_ts(
                            &allow_clippy_arbitrary_source_item_ordering_ts,
                            &ident_where_ucc,
                            &content_ts
                        );
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => match &pattern {
                                Pattern::Standart => gen_ident_where_wrapper_ts(&{
                                    let ident_where_field_variants_ts = gen_ident_where_field_variants_ts(&is_standart_with_id_false);
                                    quote!{
                                        #ident_where_field_variants_ts,
                                        #equal_variant_ident_ts,
                                    }
                                }),
                                Pattern::Array => gen_ident_where_wrapper_ts(&{
                                    let dimension_one_equal_ts = quote! {
                                        DimensionOneEqual(#import_path::PostgresJsonTypeWhereDimensionOneEqual<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let length_equal_ts = quote! {
                                        LengthEqual(#import_path::PostgresJsonTypeWhereLengthEqual),
                                    };
                                    let length_greater_than_ts = quote! {
                                        LengthGreaterThan(#import_path::PostgresJsonTypeWhereLengthGreaterThan),
                                    };
                                    let in_ts = quote! {
                                        In(#import_path::PostgresJsonTypeWhereIn<#ident_as_postgres_json_type_table_type_declaration_ts>),
                                    };
                                    let dimension_one_in_ts = quote! {
                                        DimensionOneIn(#import_path::PostgresJsonTypeWhereDimensionOneIn<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let contains_all_elements_of_array_ts = quote! {
                                        ContainsAllElementsOfArray(#import_path::PostgresJsonTypeWhereContainsAllElementsOfArray<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let overlaps_with_array_ts = quote! {
                                        OverlapsWithArray(#import_path::PostgresJsonTypeWhereOverlapsWithArray<#ident_with_id_standart_not_null_table_type_declaration_ucc>),
                                    };
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_3e7f45d9| {
                                        let field_ident = &el_3e7f45d9.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let el_type_as_postgres_json_type_where_ts = gen_type_as_postgres_json_type_subtype_ts(
                                            &el_3e7f45d9.field_type,
                                            &PostgresJsonTypeSubtype::Where
                                        );
                                        quote! {
                                            #el_field_ident_ucc(#import_path::PostgresTypeWhere<
                                                #el_type_as_postgres_json_type_where_ts
                                            >)
                                        }
                                    });
                                    quote! {
                                        #equal_variant_ident_ts,
                                        #dimension_one_equal_ts
                                        #length_equal_ts
                                        #length_greater_than_ts
                                        #in_ts
                                        #dimension_one_in_ts
                                        #contains_all_elements_of_array_ts
                                        #overlaps_with_array_ts
                                        #(#el_filters_ts),*
                                    }
                                }),
                            },
                            NotNullOrNullable::Nullable => Ts2::new(),
                        }
                    };
                    let gen_where_filter_query_part_fields_content_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                        let query_part_variants_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_32d414b1| {
                            let field_ident_str = el_32d414b1.field_ident.to_string();
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&field_ident_str);
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{column}}->'{field_ident_str}'"));
                            quote! {
                                Self::#field_ident_ucc_ts(value) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                    value,
                                    increment,
                                    &format!(#format_handle_ts),
                                    is_need_to_add_logical_operator,
                                )
                            }
                        });
                        quote! {#(#query_part_variants_ts),*}
                    };
                    let gen_where_filter_query_bind_fields_content_standart_not_null_ts = |is_standart_with_id: &IsStandartWithId| {
                        let query_bind_variants_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_ee6a2665| {
                            let field_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_ee6a2665.field_ident.to_string());
                            quote! {
                                Self::#field_ident_ucc_ts(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts
                            }
                        });
                        quote! {#(#query_bind_variants_ts),*}
                    };
                    let gen_impl_postgres_type_where_filter_ts = |
                        current_ident_ts: &dyn quote::ToTokens,
                        query_part_content_ts: &dyn quote::ToTokens,
                        is_query_bind_mutable: postgres_crud_macros_common::IsQueryBindMutable,
                        query_bind_content_ts: &dyn quote::ToTokens
                    | {
                        postgres_crud_macros_common::impl_postgres_type_where_filter_for_ident_ts(
                            &quote! {<'lifetime>},
                            &current_ident_ts,
                            &Ts2::new(),
                            &postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            &postgres_crud_macros_common::ColumnParameterUnderscore::False,
                            &postgres_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                            &query_part_content_ts,
                            &is_query_bind_mutable,
                            &query_bind_content_ts,
                            &postgres_crud_macros_common::ImportPath::PostgresCrud,
                        )
                    };
                    let maybe_impl_postgres_crud_postgres_type_postgres_type_where_filter_for_ident_where_ts = {
                        let gen_impl_postgres_type_where_filter_for_ident_ts = |query_part_content_ts: &dyn quote::ToTokens, is_query_bind_mutable: postgres_crud_macros_common::IsQueryBindMutable, query_bind_content_ts: &dyn quote::ToTokens| gen_impl_postgres_type_where_filter_ts(&ident_where_ucc, &query_part_content_ts, is_query_bind_mutable, &query_bind_content_ts);
                        match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => gen_impl_postgres_type_where_filter_for_ident_ts(
                                    &{
                                        let fields_content_ts = gen_where_filter_query_part_fields_content_standart_not_null_ts(&is_standart_with_id_false);
                                        quote!{
                                            match &self {
                                                #fields_content_ts,
                                                #equal_variant_query_part_ts,
                                            }
                                        }
                                    },
                                    postgres_crud_macros_common::IsQueryBindMutable::False,
                                    &{
                                        let fields_content_ts = gen_where_filter_query_bind_fields_content_standart_not_null_ts(&is_standart_with_id_false);
                                        quote!{
                                            match self {
                                                #fields_content_ts,
                                                #equal_variant_query_bind_ts,
                                            }
                                        }
                                    }
                                ),
                                NotNullOrNullable::Nullable => Ts2::new(),
                            },
                            Pattern::Array => gen_impl_postgres_type_where_filter_for_ident_ts(
                                &{
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_7845d48a| {
                                        let field_ident = &el_7845d48a.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                        quote! {
                                            Self::#el_field_ident_ucc(#value_sc) => gen_el_query(
                                                #value_sc.get_logical_operator(),
                                                #value_sc,
                                                #field_ident_double_quotes_ts
                                            )
                                        }
                                    });
                                    quote! {
                                        let mut gen_el_query = |
                                            logical_operator: &#import_path::LogicalOperator,
                                            #value_sc: &dyn #import_path::PostgresTypeWhereFilter<'_>,
                                            field: &str
                                        | -> Result<#std_string_string_ts, #import_path_query_part_error_named_ts> {
                                            let logical_operator_query_part = logical_operator.to_query_part(is_need_to_add_logical_operator);
                                            let elem = "elem";
                                            let value_9696ee60 = match #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                &format!("{elem}->'{field}'"),
                                                false
                                            ) {
                                                Ok(value_c7ec4e53) => value_c7ec4e53,
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            };
                                            Ok(format!("{logical_operator_query_part}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {value_9696ee60}))"))
                                        };
                                        match &self {
                                            Self::Equal(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::DimensionOneEqual(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::LengthEqual(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::LengthGreaterThan(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::In(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::DimensionOneIn(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::ContainsAllElementsOfArray(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            Self::OverlapsWithArray(#value_sc) => #import_path::PostgresTypeWhereFilter::#query_part_sc(
                                                #value_sc,
                                                #increment_sc,
                                                #column_sc,
                                                #is_need_to_add_logical_operator_sc
                                            ),
                                            #(#el_filters_ts),*
                                        }
                                    }
                                },
                                postgres_crud_macros_common::IsQueryBindMutable::False,
                                &{
                                    let el_filters_ts = vec_syn_field_with_id.iter().map(|el_9956277c| {
                                        let field_ident = &el_9956277c.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        quote! {Self::#el_field_ident_ucc(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts}
                                    });
                                    quote! {
                                        match self {
                                            Self::Equal(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::DimensionOneEqual(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::LengthEqual(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::LengthGreaterThan(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::In(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::DimensionOneIn(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::ContainsAllElementsOfArray(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            Self::OverlapsWithArray(#value_sc) => #postgres_type_where_filter_query_bind_value_query_ts,
                                            #(#el_filters_ts),*
                                        }
                                    }
                                },
                            ),
                        }
                    };
                    let maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_ts = if matches!((&pattern, &not_null_or_nullable), (Pattern::Standart, NotNullOrNullable::Nullable)) {
                        Ts2::new()
                    } else {
                        gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts(&ident_where_ucc)
                    };
                    let gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_content_standart_not_null_where = |is_standart_with_id: &IsStandartWithId| {
                        let gen_self_variant_default_some_one_ts = |content_ts: &dyn quote::ToTokens|quote!{
                            Self::#content_ts(#postgres_crud_default_option_some_vec_one_el_call_ts)
                        };
                        let variants_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_7f3524bc| {
                            gen_self_variant_default_some_one_ts(&AsRefStrToUccTs::case_or_panic(&el_7f3524bc.field_ident.to_string()))
                        });
                        let self_equal_default_some_one_ts = gen_self_variant_default_some_one_ts(&equal_ucc);
                        quote! {vec![
                            #(#variants_ts),*,
                            #self_equal_default_some_one_ts
                        ]}
                    };
                    let maybe_impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_where_ts = match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_ts(&ident_where_ucc, &gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_content_standart_not_null_where(&is_standart_with_id_false)),
                            NotNullOrNullable::Nullable => Ts2::new(),
                        },
                        Pattern::Array => postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_ts(&ident_where_ucc, &{
                            let el_filters_ts = vec_syn_field_with_id.iter().map(|el_a3184731| {
                                let field_ident = &el_a3184731.field_ident;
                                let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                quote! {Self::#el_field_ident_ucc(#default_but_option_is_some_call_ts)}
                            });
                            quote! {
                                vec![
                                    Self::Equal(#default_but_option_is_some_call_ts),
                                    Self::DimensionOneEqual(#default_but_option_is_some_call_ts),
                                    Self::LengthEqual(#default_but_option_is_some_call_ts),
                                    Self::LengthGreaterThan(#default_but_option_is_some_call_ts),
                                    Self::In(#default_but_option_is_some_call_ts),
                                    Self::DimensionOneIn(#default_but_option_is_some_call_ts),
                                    Self::ContainsAllElementsOfArray(#default_but_option_is_some_call_ts),
                                    Self::OverlapsWithArray(#default_but_option_is_some_call_ts),
                                    #(#el_filters_ts),*
                                ]
                            }
                        }),
                    };
                    let maybe_ident_with_id_standart_not_null_where_ts = if is_standart_not_null {
                        let ident_with_id_standart_not_null_where_ts = gen_ident_where_ts(
                            &allow_clippy_arbitrary_source_item_ordering_ts,
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let ident_where_field_variants_ts = gen_ident_where_field_variants_ts(&is_standart_with_id_true);
                                quote!{
                                    #ident_where_field_variants_ts,
                                    #equal_ucc(#import_path::PostgresJsonTypeWhereEqual<#ident_with_id_standart_not_null_table_type_declaration_ucc>),//todo maybe reuse? variant generation
                                }
                            }
                        );
                        let impl_postgres_crud_postgres_type_postgres_type_where_filter_for_ident_with_id_standart_not_null_where_ts = gen_impl_postgres_type_where_filter_ts(
                            &ident_with_id_standart_not_null_where_ucc,
                            &{
                                let fields_content_ts = gen_where_filter_query_part_fields_content_standart_not_null_ts(&is_standart_with_id_true);
                                quote!{
                                    match &self {
                                        #fields_content_ts,
                                        Self::#equal_ucc(#value_sc) => postgres_crud::PostgresTypeWhereFilter::query_part(
                                            #value_sc,
                                            #increment_sc,
                                            &#column_sc,
                                            is_need_to_add_logical_operator
                                        ),//todo maybe reuse? variant generation
                                    }
                                }
                            },
                            postgres_crud_macros_common::IsQueryBindMutable::False,
                            &{
                                let fields_content_ts = gen_where_filter_query_bind_fields_content_standart_not_null_ts(&is_standart_with_id_true);
                                quote!{
                                    match self {
                                        #fields_content_ts,
                                        Self::#equal_ucc(#value_sc) => postgres_crud::PostgresTypeWhereFilter::query_bind(#value_sc, #query_sc),//todo maybe reuse? variant generation
                                    }
                                }
                            },
                        );
                        let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_ts = gen_gen_impl_error_occurence_lib_to_std_string_string_wrapper_ts(&ident_with_id_standart_not_null_where_ucc);
                        let impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_where_ts = postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_ts(
                            &ident_with_id_standart_not_null_where_ucc,
                            &gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_content_standart_not_null_where(&is_standart_with_id_true)
                        );
                        quote! {
                            #ident_with_id_standart_not_null_where_ts
                            #impl_postgres_crud_postgres_type_postgres_type_where_filter_for_ident_with_id_standart_not_null_where_ts
                            #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_ts
                            #impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_where_ts
                        }
                    } else {
                        Ts2::new()
                    };
                    quote! {
                        #maybe_ident_where_ts
                        #maybe_impl_postgres_crud_postgres_type_postgres_type_where_filter_for_ident_where_ts
                        #maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_ts
                        #maybe_impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_where_ts
                        #maybe_ident_with_id_standart_not_null_where_ts
                    }
                }
                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                    let ident_standart_or_ident_with_id_array_as_postgres_json_type_where_ts = gen_type_as_postgres_json_type_subtype_ts(
                        &match &pattern {
                            Pattern::Standart => &ident_standart_not_null_ucc,
                            Pattern::Array => &ident_with_id_array_not_null_ucc,
                        },
                        &postgres_json_type_subtype_where
                    );
                    quote! {
                        pub type #ident_where_ucc = #import_path::NullableJsonObjectPostgresTypeWhereFilter<
                            #ident_standart_or_ident_with_id_array_as_postgres_json_type_where_ts
                        >;
                    }
                }
            };
            let gen_field_ident_double_quotes_ts = |value: &macros_helpers::SynFieldWrapper| {
                gen_quotes::double_quotes_ts(&value.field_ident)
            };
            let gen_type_as_postgres_json_type_read_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_read);
            let gen_type_as_postgres_json_type_read_inner_ts = |type_ts: &dyn quote::ToTokens| gen_type_as_postgres_json_type_subtype_ts(&type_ts, &postgres_json_type_subtype_read_inner);
            let gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts = |
                is_standart_with_id: &IsStandartWithId,
                read_with_or_without_annotation_or_inner: &ReadWithOrWithoutAnnotationOrInner
            | {
                let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_274293a0| {
                    let maybe_serde_skip_serializing_if_option_is_none_ts = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation => quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::Inner => Ts2::new(),
                    };
                    let field_ident = &el_274293a0.field_ident;
                    let field_type_as_json_type_read_ts = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::Inner => gen_type_as_postgres_json_type_read_inner_ts(
                            &el_274293a0.field_type
                        ),
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation => gen_type_as_postgres_json_type_read_ts(
                            &el_274293a0.field_type
                        ),
                    };
                    let std_option_option_value_field_type_as_json_type_read_ts = postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                        &wrap_into_value_declaration_ts(&field_type_as_json_type_read_ts)
                    );
                    quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_ts
                        #field_ident: #std_option_option_value_field_type_as_json_type_read_ts
                    }
                });
                quote! {#(#content_ts),*}
            };
            let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_read_ts = {
                let ident_read_try_from_error_named_ucc = SelfReadTryFromErrorNamedUcc::from_tokens(&ident);
                let ident_with_id_standart_not_null_read_try_from_error_named_ucc = SelfReadTryFromErrorNamedUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
                let ident_standart_not_null_as_postgres_json_type_read_ts = gen_type_as_postgres_json_type_read_ts(&ident_standart_not_null_ucc);
                let ident_with_id_array_not_null_as_postgres_json_type_read_ts = gen_type_as_postgres_json_type_read_ts(&ident_with_id_array_not_null_ucc);
                let gen_ident_read_ts = |
                    current_ident_ts: &dyn quote::ToTokens,
                    content_ts_1c85ea2c: &dyn quote::ToTokens,
                    derive_serde_deserialize: macros_helpers::DeriveSerdeDeserialize
                | {
                    let content_ts_3a67b41f = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize_if(derive_serde_deserialize)
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &current_ident_ts,
                        &content_ts_1c85ea2c
                    );
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #content_ts_3a67b41f
                    }
                };
                let ident_read_ts = {
                    let (content_ts, derive_serde_deserialize) = match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => (
                                {
                                    let content_ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                        &is_standart_with_id_false,
                                        &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                                    );
                                    quote! {{#content_ts}}
                                },
                                macros_helpers::DeriveSerdeDeserialize::False,
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&ident_standart_not_null_as_postgres_json_type_read_ts)), macros_helpers::DeriveSerdeDeserialize::True),
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => (wrap_content_into_scopes_dot_comma_ts(&postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&ident_with_id_standart_not_null_read_ucc)), macros_helpers::DeriveSerdeDeserialize::True),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&ident_with_id_array_not_null_as_postgres_json_type_read_ts)), macros_helpers::DeriveSerdeDeserialize::True),
                        },
                    };
                    gen_ident_read_ts(&ident_read_ucc, &content_ts, derive_serde_deserialize)
                };
                let all_fields_are_none_ucc = AllFieldsAreNoneUcc;
                let gen_ident_read_try_from_error_named_ts = |current_ident_ts: &dyn quote::ToTokens|macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_error_occurence_lib_error_occurence()
                    .build_enum(
                        &current_ident_ts,
                        &quote!{{
                            #all_fields_are_none_ucc {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }}
                    );
                let maybe_ident_read_try_from_error_named_ts = match &pattern {
                    Pattern::Standart => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_read_try_from_error_named_ts(&ident_read_try_from_error_named_ucc),
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => Ts2::new(),
                    },
                    Pattern::Array => Ts2::new(),
                };
                let gen_ident_read_or_ident_with_id_standart_not_null_read_ucc = |is_standart_with_id: &IsStandartWithId| match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_ucc,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_ucc,
                };
                let gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                    let ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_ucc: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_try_from_error_named_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_read_try_from_error_named_ucc,
                    };
                    macros_helpers::gen_pub_try_new_ts(
                        &gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                            is_standart_with_id,
                            &ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation
                        ),
                        &ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_ucc,
                        &{
                            let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
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
                                    let fields_ts = current_vec_syn_field.iter().map(|el_a6b6e788| {
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
                                let current_vec_syn_field_len = current_vec_syn_field.len();
                                let maybe_wrap_into_braces_handle_ts = |content_ts: &dyn quote::ToTokens| postgres_crud_macros_common::maybe_wrap_into_braces_ts(
                                    content_ts,
                                    current_vec_syn_field_len > 1
                                );
                                let left_ts = maybe_wrap_into_braces_handle_ts(&fields_reference_ts);
                                let right_ts = maybe_wrap_into_braces_handle_ts(&{
                                    let nones_ts = repeat_with(||quote!{None}).take(current_vec_syn_field_len);
                                    quote! {#(#nones_ts),*}
                                });
                                let content_ts = if current_vec_syn_field_len == 1 {
                                    let content_ts = maybe_wrap_into_braces_handle_ts(&fields_ts);
                                    quote! {#content_ts.is_none()}
                                }
                                else {
                                    quote! {matches!(#left_ts, #right_ts)}
                                };
                                quote! {
                                    if #content_ts {
                                        return Err(#ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_ucc::#all_fields_are_none_ucc {
                                            code_occurence: error_occurence_lib::code_occurence!()
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
                        let std_vec_vec_ident_with_id_standart_not_null_read_ts = postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&ident_with_id_standart_not_null_read_ucc);
                        match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::gen_pub_const_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                            &ident_standart_not_null_as_postgres_json_type_read_ts
                                        )
                                    ),
                                    &self_value_ts
                                ),
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::gen_pub_const_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &std_vec_vec_ident_with_id_standart_not_null_read_ts
                                    ),
                                    &self_value_ts
                                ),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::gen_pub_new_ts(
                                    &must_use_ts,
                                    &gen_value_type_ts(
                                        &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                            &std_vec_vec_ident_with_id_standart_not_null_read_ts
                                        )
                                    ),
                                    &quote! {Self(#value_sc.map(#ident_with_id_array_not_null_as_postgres_json_type_read_ts::new))},
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
                    let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                    postgres_crud_macros_common::gen_impl_serde_deserialize_for_struct_ts(
                        &gen_ident_read_or_ident_with_id_standart_not_null_read_ucc(is_standart_with_id),
                        &current_vec_syn_field.iter().map(|el_00a75629|
                            (&el_00a75629.field_ident, &el_00a75629.field_type)
                        ).collect::<Vec<(&syn::Ident, &syn::Type)>>(),
                        current_vec_syn_field.len(),
                        &|_: &syn::Ident, syn_type: &syn::Type| {
                            let type_read_ts = gen_type_as_postgres_json_type_read_ts(&syn_type);
                            postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                &wrap_into_value_declaration_ts(&type_read_ts)
                            )
                        }
                    )
                };
                let maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts = match &pattern {
                    Pattern::Standart => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => Ts2::new(),
                    },
                    Pattern::Array => Ts2::new(),
                };
                let gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                    postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(&gen_ident_read_or_ident_with_id_standart_not_null_read_ucc(is_standart_with_id), &Ts2::new(), &{
                        let fields_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_6a2035df| {
                            let field_ident = &el_6a2035df.field_ident;
                            let value_content_ts = wrap_into_value_initialization_ts(
                                &postgres_crud_default_option_some_vec_one_el_call_ts
                            );
                            quote! {#field_ident: Some(#value_content_ts)}
                        });
                        quote! {Self{#(#fields_ts),*}}
                    })
                };
                let impl_postgres_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts = match &pattern {
                    Pattern::Standart => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_false),
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_postgres_crud_default_option_some_vec_one_el_call_ts),
                    },
                    Pattern::Array => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(
                            &ident_read_ucc,
                            &Ts2::new(),
                            &quote! {
                                Self(#vec_postgres_crud_default_option_some_vec_one_el_call_ts)
                            },
                        ),
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_postgres_crud_default_option_some_vec_one_el_call_ts),
                    },
                };
                let impl_sqlx_type_sqlx_postgres_for_ident_read_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_read_ucc);
                let impl_sqlx_encode_sqlx_postgres_for_ident_read_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(
                    &ident_read_ucc,
                    &quote!{sqlx::types::Json(#self_sc)}
                );
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_ts = gen_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_ts(&ident_read_ucc);
                let maybe_ident_with_id_read_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_ts = gen_ident_read_ts(
                        &ident_with_id_standart_not_null_read_ucc,
                        &{
                            let content_ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                &is_standart_with_id_true,
                                &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                            );
                            quote! {{#content_ts}}
                        },
                        macros_helpers::DeriveSerdeDeserialize::False,
                    );
                    let ident_with_id_standart_not_null_read_try_from_error_named_ts = gen_ident_read_try_from_error_named_ts(&ident_with_id_standart_not_null_read_try_from_error_named_ucc);
                    let impl_ident_with_id_standart_not_null_read_ts = {
                        let pub_try_new_ts = gen_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                        quote!{
                            impl #ident_with_id_standart_not_null_read_ucc {
                                #pub_try_new_ts
                            }
                        }
                    };
                    let impl_serde_deserialize_for_ident_with_id_standart_not_null_read_ts = gen_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                    let impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_read_ts = gen_impl_postgres_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&is_standart_with_id_true);
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_ts = gen_sqlx_types_json_type_declaration_wrapper_ts(&ident_with_id_standart_not_null_read_ucc);
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_ts = gen_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_ts(&ident_with_id_standart_not_null_read_ucc);
                    quote! {
                        #ident_with_id_standart_not_null_read_ts
                        #ident_with_id_standart_not_null_read_try_from_error_named_ts
                        #impl_ident_with_id_standart_not_null_read_ts
                        #impl_serde_deserialize_for_ident_with_id_standart_not_null_read_ts
                        #impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_id_standart_not_null_read_ts
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_ts
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_read_ts
                    #maybe_ident_read_try_from_error_named_ts
                    #impl_ident_read_ts
                    #maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_ts
                    #impl_postgres_crud_default_option_some_vec_one_el_for_ident_read_or_ident_with_id_standart_not_null_read_ts
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_ts
                    #impl_sqlx_encode_sqlx_postgres_for_ident_read_ts
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_ts
                    #maybe_ident_with_id_read_ts
                }
            };
            let ident_with_id_standart_not_null_read_only_ids_handle_ucc = SelfReadOnlyIdsHandleUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_standart_not_null_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_standart_not_null_ucc);
            let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
            let ident_read_only_ids_handle_ucc = SelfReadOnlyIdsHandleUcc::from_tokens(&ident);
            let gen_ident_read_only_ids_or_ident_with_id_read_only_ids_content_ts = |is_standart_with_id: &IsStandartWithId| {
                let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_5f9102af| {
                    let field_ident = &el_5f9102af.field_ident;
                    let field_type_as_postgres_json_type_read_only_ids_ts = gen_type_as_postgres_json_type_subtype_ts(
                        &el_5f9102af.field_type,
                        &PostgresJsonTypeSubtype::ReadOnlyIds
                    );
                    quote! {#field_ident: #field_type_as_postgres_json_type_read_only_ids_ts}
                });
                quote! {{#(#content_ts),*}}
            };
            let gen_impl_sqlx_decode_ts = |current_ident_ts: &dyn quote::ToTokens|{
                postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(
                    &current_ident_ts,
                    &quote!{sqlx::types::Json<Self>},
                    &quote!{Ok(value_147c3532.0)}
                )
            };
            let gen_impl_sqlx_type_ts = |current_ident_ts: &dyn quote::ToTokens|{
                postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(
                    &current_ident_ts,
                    &quote!{sqlx::types::Json<Self>}
                )
            };
            let gen_fields_read_only_ids_into_option_value_read_inner_ts = |is_standart_with_id: &IsStandartWithId, parameters_ts: &dyn quote::ToTokens|{
                let current_ident_ts: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_standart_not_null_read_inner_ucc,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                };
                let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_278a1e1d| {
                    let field_ident = &el_278a1e1d.field_ident;
                    let field_type = &el_278a1e1d.field_type;
                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&field_type);
                    let field_type_as_postgres_json_type_read_ts = gen_type_as_postgres_json_type_subtype_ts(&field_type, &PostgresJsonTypeSubtype::Read);
                    let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                    let value_content_ts = wrap_into_value_initialization_ts(&{
                        let default_but_option_is_some_call_ts_f378bbab = gen_ident_as_default_but_option_is_some_call_ts(
                            &field_type_as_postgres_json_type_read_ts
                        );
                        quote!{#field_type_as_postgres_json_type_ts::into_inner(#default_but_option_is_some_call_ts_f378bbab)}
                    });
                    quote! {
                        #field_ident: #field_type_as_postgres_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                            #parameters_ts.0.#value_sc.#field_ident
                        ).map_or_else(
                            || Some(#value_content_ts),
                            Some
                        )
                    }
                });
                let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                    #current_ident_ts {
                        #(#content_ts),*
                    }
                });
                quote!{Some(#value_content_ts)}
            };
            let ident_read_only_ids_ts = {
                let maybe_ident_read_only_ids_handle_ts = if is_standart_not_null {
                    let content_ts_1e087f4d = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_handle_ucc,
                        &gen_ident_read_only_ids_or_ident_with_id_read_only_ids_content_ts(&IsStandartWithId::False)
                    );
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #content_ts_1e087f4d
                    }
                }
                else {
                    Ts2::new()
                };
                let ident_read_only_ids_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_ucc,
                        &match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_ident_read_only_ids_handle_ucc_ts = wrap_into_value_declaration_ts(&ident_read_only_ids_handle_ucc);
                                    quote! {(#value_ident_read_only_ids_handle_ucc_ts);}
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_std_option_option_ident_read_only_ids_standart_not_null_ts = wrap_into_value_declaration_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                        &ident_standart_not_null_read_only_ids_ucc
                                    ));
                                    quote! {
                                        (#value_std_option_option_ident_read_only_ids_standart_not_null_ts);
                                    }
                                }
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_std_vec_vec_ident_with_id_standart_not_null_read_only_ids_ts = wrap_into_value_declaration_ts(&postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                                        &ident_with_id_standart_not_null_read_only_ids_ucc
                                    ));
                                    quote! {
                                        (#value_std_vec_vec_ident_with_id_standart_not_null_read_only_ids_ts);
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_std_option_option_ident_with_id_read_only_ids_array_not_null_ts = wrap_into_value_declaration_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                        &SelfReadOnlyIdsUcc::from_tokens(&gen_ident_ucc(&IdentPattern::ArrayNotNullWithId))
                                    ));
                                    quote! {(#value_std_option_option_ident_with_id_read_only_ids_array_not_null_ts);}
                                }
                            },
                        }
                    );
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_ts = gen_impl_sqlx_decode_ts(&ident_read_only_ids_ucc);
                let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_ts = gen_impl_sqlx_type_ts(&ident_read_only_ids_ucc);
                let maybe_ident_with_id_standart_not_null_read_only_ids_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_only_ids_ts = {
                        let ident_with_id_standart_not_null_read_only_ids_handle_ts = {
                            let content_ts_fe644945 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_handle_ucc,
                                &gen_ident_read_only_ids_or_ident_with_id_read_only_ids_content_ts(&IsStandartWithId::True)
                            );
                            quote!{
                                #allow_clippy_arbitrary_source_item_ordering_ts
                                #content_ts_fe644945
                            }
                        };
                        let ident_with_id_standart_not_null_read_only_ids_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_ucc,
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
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_ts = gen_impl_sqlx_decode_ts(&ident_with_id_standart_not_null_read_only_ids_ucc);
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_ts = gen_impl_sqlx_type_ts(&ident_with_id_standart_not_null_read_only_ids_ucc);
                    quote! {
                        #ident_with_id_standart_not_null_read_only_ids_ts
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_ts
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #maybe_ident_read_only_ids_handle_ts
                    #ident_read_only_ids_ts
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_ts
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_ts
                    #maybe_ident_with_id_standart_not_null_read_only_ids_ts
                }
            };
            let ident_read_inner_ts = {
                let gen_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_ts = |is_standart_with_id: &IsStandartWithId| {
                    let content_ts_3d7e760e = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .build_struct(
                        match &is_standart_with_id {
                            IsStandartWithId::False => &ident_read_inner_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                        },
                        &{
                            let content_ts = gen_ident_or_ident_with_id_read_or_read_inner_fields_declaration_ts(
                                is_standart_with_id,
                                &ReadWithOrWithoutAnnotationOrInner::Inner
                            );
                            quote!{{#content_ts}}
                        }
                    );
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #content_ts_3d7e760e
                    }
                };
                let ident_read_inner_ts = {
                    let gen_pub_type_ident_read_inner_alias_ts = |content_ts: &dyn quote::ToTokens| macros_helpers::gen_pub_type_alias_ts(&ident_read_inner_ucc, &content_ts);
                    match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_ts(&IsStandartWithId::False),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_pub_type_ident_read_inner_alias_ts(&postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&gen_type_as_postgres_json_type_read_inner_ts(&ident_standart_not_null_ucc))),
                        },
                        Pattern::Array => gen_pub_type_ident_read_inner_alias_ts(&match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                                &ident_with_id_standart_not_null_read_inner_ucc
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&gen_type_as_postgres_json_type_read_inner_ts(&ident_with_id_array_not_null_ucc)),
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
            let ident_standart_not_null_as_postgres_json_type_update_ts = gen_type_as_postgres_json_type_update_ts(&ident_standart_not_null_ucc);
            let ident_standart_not_null_as_postgres_json_type_update_for_query_ts = gen_type_as_postgres_json_type_update_for_query_ts(&ident_standart_not_null_ucc);
            let ident_with_id_array_not_null_as_postgres_json_type_update_ts = gen_type_as_postgres_json_type_update_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_array_not_null_as_postgres_json_type_update_for_query_ts = gen_type_as_postgres_json_type_update_for_query_ts(&ident_with_id_array_not_null_ucc);
            let ident_with_id_standart_not_null_update_el_ucc = &SelfUpdateElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let ident_with_id_standart_not_null_update_for_query_el_ucc = &SelfUpdateForQueryElementUcc::from_tokens(&ident_with_id_standart_not_null_ucc);
            let (gen_jsonb_set_target_sc, gen_jsonb_set_target_ts) = {
                let gen_jsonb_set_target_sc = GenJsonbSetTargetSc;
                let gen_jsonb_set_target_ts = {
                    let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{{jsonb_set_target_sc}}}->'{{value_12d082b5}}'"));
                    quote! {
                        let #gen_jsonb_set_target_sc = |value_12d082b5: &str|{
                            format!(#format_handle_ts)
                        };
                    }
                };
                (gen_jsonb_set_target_sc, gen_jsonb_set_target_ts)
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts = quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_el_ucc>
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_el_ts = quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_for_query_el_ucc>
            };
            let gen_create_update_delete_fields_ts_043c4057 = |
                should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation,
                create_ts: &dyn quote::ToTokens,
                update_ts: &dyn quote::ToTokens,
                delete_ts: &dyn quote::ToTokens
            | {
                let maybe_serde_skip_serializing_if_vec_is_empty_ts = match &should_add_serde_skip_serializing_if_vec_is_empty_annotation {
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False => Ts2::new(),
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True => quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
                };
                quote! {
                    #maybe_serde_skip_serializing_if_vec_is_empty_ts
                    #create_sc: #create_ts,
                    #update_sc: #update_ts,
                    #maybe_serde_skip_serializing_if_vec_is_empty_ts
                    #delete_sc: #delete_ts,
                }
            };
            let ident_update_ts = {
                let gen_ident_update_standart_not_null_content_ts = |is_standart_with_id: &IsStandartWithId| {
                    gen_unique_vec_wrapper_ts(match &is_standart_with_id {
                        IsStandartWithId::False => &ident_update_el_ucc,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_update_el_ucc,
                    })
                };
                let std_vec_vec_ident_with_id_standart_not_null_create_ts = postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                    &ident_with_id_standart_not_null_create_ucc
                );
                let std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_ts = postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                    &postgres_crud_path_postgres_json_type_uuid_uuid_update_ts
                );
                let gen_create_update_delete_fields_ts_ffcbdaf0 = |should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation| {
                    gen_create_update_delete_fields_ts_043c4057(
                        should_add_serde_skip_serializing_if_vec_is_empty_annotation,
                        &std_vec_vec_ident_with_id_standart_not_null_create_ts,
                        &import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts,
                        &std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_ts
                    )
                };
                let ident_update_ts = {
                    let gen_std_option_option_ident_type_ts = |current_ident_ts: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_ts(
                        &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&current_ident_ts)
                    );
                    let (
                        derive_serde_deserialize,
                        content_ts_975df5c5
                    ) = match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => (
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &wrap_content_into_scopes_dot_comma_ts(
                                    &gen_ident_update_standart_not_null_content_ts(&is_standart_with_id_false)
                                )
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => (
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &gen_std_option_option_ident_type_ts(&ident_standart_not_null_as_postgres_json_type_update_ts)
                            ),
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => (
                                macros_helpers::DeriveSerdeDeserialize::False,
                                &{
                                    let fields_ts = gen_create_update_delete_fields_ts_ffcbdaf0(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                    quote! {{#fields_ts}}
                                }
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => (
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &gen_std_option_option_ident_type_ts(&ident_with_id_array_not_null_as_postgres_json_type_update_ts)
                            ),
                        },
                    };
                    let content_ts_c9a843aa = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                        &content_ts_975df5c5
                    );
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #content_ts_c9a843aa
                    }
                };
                let not_unique_id_in_json_delete_array_ucc = NotUniqueIdInJsonDeleteArrayUcc;
                let not_unique_id_in_json_update_and_delete_arrays_ucc = NotUniqueIdInJsonUpdateAndDeleteArraysUcc;
                let create_update_delete_are_empty_ucc = CreateUpdateDeleteAreEmptyUcc;
                let ids_are_not_unique_uppper_camel_case = IdsAreNotUniqueUcc;
                let ident_update_try_new_error_named_ucc = &SelfUpdateTryNewErrorNamedUcc::from_tokens(&ident);
                let maybe_ident_update_try_new_error_named_ts = match &pattern {
                    Pattern::Standart => Ts2::new(),
                    Pattern::Array => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_error_occurence_lib_error_occurence()
                            .build_enum(
                                &ident_update_try_new_error_named_ucc,
                                &quote!{{
                                    #create_update_delete_are_empty_ucc {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #ids_are_not_unique_uppper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        duplicate: #std_string_string_ts,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #not_unique_id_in_json_delete_array_ucc {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        error: #std_string_string_ts,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #not_unique_id_in_json_update_and_delete_arrays_ucc {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        error: #std_string_string_ts,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                }}
                            ),
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => Ts2::new(),
                    },
                };
                let impl_ident_update_ts = {
                    let maybe_pub_new_or_try_new_for_ident_update_ts = match &pattern {
                        Pattern::Standart => macros_helpers::gen_pub_const_new_ts(
                            &must_use_ts,
                            &gen_value_type_ts(&match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_unique_vec_wrapper_ts(&ident_standart_not_null_update_el_ucc),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&ident_standart_not_null_as_postgres_json_type_update_ts)
                            }),
                            &self_value_ts
                        ),
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::gen_pub_try_new_ts(
                                &gen_create_update_delete_fields_ts_ffcbdaf0(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False),
                                &ident_update_try_new_error_named_ucc,
                                &{
                                    let custom_serde_error_deserializing_ident_update_str = format!("custom serde error deserializing {ident_update_ucc}");
                                    let check_if_all_empty_ts = {
                                        quote! {
                                            if create.is_empty() && update.is_empty() && delete.is_empty() {
                                                return Err(#ident_update_try_new_error_named_ucc::#create_update_delete_are_empty_ucc {
                                                    code_occurence: error_occurence_lib::code_occurence!()
                                                });
                                            }
                                        }
                                    };
                                    let check_if_ids_are_unique_ts = {
                                        let (
                                            uuid_as_postgres_json_type_update_to_std_string_string_el_id_ts,
                                            uuid_as_postgres_json_type_update_to_std_string_string_el_ts,
                                        ) = {
                                            #[allow(clippy::arbitrary_source_item_ordering)]
                                            enum UpdateOrDelete {
                                                Update,
                                                Delete
                                            }
                                            let gen_uuid_as_postgres_json_type_update_to_std_string_string_ts = |
                                                update_or_delete: &UpdateOrDelete,
                                                el_ts: &dyn quote::ToTokens,
                                            |{
                                                let content_ts: &dyn quote::ToTokens = match &update_or_delete {
                                                    UpdateOrDelete::Update => &quote!{&#el_ts.#id_sc},
                                                    UpdateOrDelete::Delete => &el_ts
                                                };
                                                quote!{
                                                    <#uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_update_ts as error_occurence_lib::ToStdStringString>::to_std_string_string(
                                                        #content_ts
                                                    )
                                                }
                                            };
                                            (
                                                gen_uuid_as_postgres_json_type_update_to_std_string_string_ts(
                                                    &UpdateOrDelete::Update,
                                                    &quote!{el_dff7634c}
                                                ),
                                                gen_uuid_as_postgres_json_type_update_to_std_string_string_ts(
                                                    &UpdateOrDelete::Delete,
                                                    &quote!{el_2b0181e6}
                                                )
                                            )
                                        };
                                        quote!{{
                                            let mut acc_2bf4e098 = Vec::new();
                                            for el_dff7634c in update.to_vec() {
                                                if acc_2bf4e098.contains(&&el_dff7634c.#id_sc) {
                                                    return Err(#ident_update_try_new_error_named_ucc::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgres_json_type_update_to_std_string_string_el_id_ts,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                acc_2bf4e098.push(&el_dff7634c.#id_sc);
                                            }
                                            for el_2b0181e6 in &delete {
                                                if acc_2bf4e098.contains(&el_2b0181e6) {
                                                    return Err(#ident_update_try_new_error_named_ucc::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgres_json_type_update_to_std_string_string_el_ts,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                acc_2bf4e098.push(el_2b0181e6);
                                            }
                                        }}
                                    };
                                    let check_not_unique_id_ts = {
                                        let check_not_unique_id_in_update_array_ts = quote! {
                                            let update_acc = #update_sc.to_vec().iter()
                                            .map(|el_b6af219f|&el_b6af219f.#id_sc)
                                            .collect::<Vec<&#uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_update_ts>>();
                                        };
                                        let check_not_unique_id_in_delete_aray_ts = {
                                            let not_unique_id_in_json_delete_array_double_quotes_ts = gen_quotes::double_quotes_ts(&format!("{custom_serde_error_deserializing_ident_update_str}: not unique {id_sc} in json delete array: {{}}"));
                                            quote! {
                                                let delete_acc = {
                                                    let mut delete_acc = Vec::new();
                                                    for el_2ecc509c in &delete {
                                                        if delete_acc.contains(&el_2ecc509c) {
                                                            return Err(#ident_update_try_new_error_named_ucc::#not_unique_id_in_json_delete_array_ucc {
                                                                #error_sc: format!(
                                                                    #not_unique_id_in_json_delete_array_double_quotes_ts,
                                                                    #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::get_inner(
                                                                        &el_2ecc509c.clone().into()
                                                                    )
                                                                ),
                                                                code_occurence: error_occurence_lib::code_occurence!()
                                                            });
                                                        }
                                                        delete_acc.push(el_2ecc509c);
                                                    }
                                                    delete_acc
                                                };
                                            }
                                        };
                                        let check_not_unique_id_in_update_and_delete_arrays_ts = {
                                            let not_unique_id_in_json_update_and_delete_arrays_double_quotes_ts = gen_quotes::double_quotes_ts(&format!("{custom_serde_error_deserializing_ident_update_str}: not unique {id_sc} in json update and delete arrays: {{}}"));
                                            quote! {
                                                for el_fefe9816 in update_acc {
                                                    if delete_acc.contains(&el_fefe9816) {
                                                        return Err(#ident_update_try_new_error_named_ucc::#not_unique_id_in_json_update_and_delete_arrays_ucc {
                                                            #error_sc: format!(
                                                                #not_unique_id_in_json_update_and_delete_arrays_double_quotes_ts,
                                                                #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::get_inner(
                                                                    &el_fefe9816.clone().into()
                                                                )
                                                            ),
                                                            code_occurence: error_occurence_lib::code_occurence!()
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
                                            #create_sc,
                                            #update_sc,
                                            #delete_sc
                                        })
                                    }
                                }
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_pub_const_new_value_type_content_self_value_ts(
                                &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(
                                    &ident_with_id_array_not_null_as_postgres_json_type_update_ts
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
                    Pattern::Array => match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                            //todo maybe reuse?
                            let tuple_struct_ident_update_double_quotes_ts = gen_quotes::double_quotes_ts(&format!("tuple struct {ident_update_ucc}"));
                            let ident_update_double_quotes_ts = gen_quotes::double_quotes_ts(&ident_update_ucc);
                            let match_try_new_in_deserialize_ts = postgres_crud_macros_common::gen_match_try_new_in_deserialize_ts(
                                &ident_update_ucc,
                                &quote! {__field0_value, __field1_value, __field2_value}
                            );
                            quote! {
                                #[allow(clippy::absolute_paths)]
                                #allow_clippy_arbitrary_source_item_ordering_ts
                                impl<'de> serde::Deserialize<'de> for #ident_update_ucc {
                                    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                    where
                                        __D: serde::Deserializer<'de>,
                                    {
                                        #[allow(non_camel_case_types)]
                                        #[doc(hidden)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        #[doc(hidden)]
                                        struct __FieldVisitor;
                                        impl serde::de::Visitor<'_> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                                serde::__private228::Formatter::write_str(__f, "field identifier")
                                            }
                                            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => Ok(__Field::__field0),
                                                    1u64 => Ok(__Field::__field1),
                                                    2u64 => Ok(__Field::__field2),
                                                    _ => Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    "create" => Ok(__Field::__field0),
                                                    "update" => Ok(__Field::__field1),
                                                    "delete" => Ok(__Field::__field2),
                                                    _ => Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    b"create" => Ok(__Field::__field0),
                                                    b"update" => Ok(__Field::__field1),
                                                    b"delete" => Ok(__Field::__field2),
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
                                                serde::__private228::Formatter::write_str(__f, #tuple_struct_ident_update_double_quotes_ts)
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let __field0_value = serde::de::SeqAccess::next_element::<#std_vec_vec_ident_with_id_standart_not_null_create_ts>(&mut __seq)?.unwrap_or_default();
                                                let __field1_value = serde::de::SeqAccess::next_element::<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts>(&mut __seq)?.unwrap_or_default();
                                                let __field2_value = serde::de::SeqAccess::next_element::<#std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_ts>(&mut __seq)?.unwrap_or_default();
                                                #match_try_new_in_deserialize_ts
                                            }
                                            #[inline]
                                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: Option<#std_vec_vec_ident_with_id_standart_not_null_create_ts> = None;
                                                let mut __field1: Option<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts> = None;
                                                let mut __field2: Option<#std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_ts> = None;
                                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if Option::is_some(&__field0) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                            }
                                                            __field0 = Some(serde::de::MapAccess::next_value::<#std_vec_vec_ident_with_id_standart_not_null_create_ts>(&mut __map)?);
                                                        }
                                                        __Field::__field1 => {
                                                            if Option::is_some(&__field1) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                            }
                                                            __field1 = Some(serde::de::MapAccess::next_value::<#import_path_unique_vec_ident_with_id_standart_not_null_update_el_ts>(&mut __map)?);
                                                        }
                                                        __Field::__field2 => {
                                                            if Option::is_some(&__field2) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                            }
                                                            __field2 = Some(serde::de::MapAccess::next_value::<#std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_ts>(&mut __map)?);
                                                        }
                                                        __Field::__ignore => {
                                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                let __field0_value = __field0.unwrap_or_default();
                                                let __field1_value = __field1.unwrap_or_default();
                                                let __field2_value = __field2.unwrap_or_default();
                                                #match_try_new_in_deserialize_ts
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &[&str] = &["create", "update", "delete"];
                                        serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_update_double_quotes_ts,
                                            FIELDS,
                                            __Visitor {
                                                marker: serde::__private228::PhantomData::<#self_ucc>,
                                                lifetime: serde::__private228::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        }
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => Ts2::new(),
                    },
                };
                let impl_postgres_crud_default_option_some_vec_one_el_for_ident_update_ts = postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(&ident_update_ucc, &Ts2::new(), &{
                    let value = match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {(#postgres_crud_default_option_some_vec_one_el_call_ts)},
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {(Some(#postgres_crud_default_option_some_vec_one_el_call_ts))},
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {{
                                #create_sc: #vec_postgres_crud_default_option_some_vec_one_el_call_ts,
                                #update_sc: #postgres_crud_default_option_some_vec_one_el_call_ts,
                                #delete_sc: #vec_postgres_crud_default_option_some_vec_one_el_call_ts,
                            }},
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                (Some(#postgres_crud_default_option_some_vec_one_el_call_ts))
                            },
                        },
                    };
                    quote! {Self #value}
                });
                let maybe_ident_update_el_ts = if is_standart_not_null {
                    let ident_update_el_ts = {
                        let content_ts_b258e2eb = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                            &{
                                let variants_ts = vec_syn_field.iter().map(|el_092057f6| {
                                    let field_ident = &el_092057f6.field_ident;
                                    let variant_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_ident_double_quotes_ts = gen_field_ident_double_quotes_ts(el_092057f6);
                                    let value_field_type_as_json_type_update_ts = wrap_into_value_declaration_ts(
                                        &gen_type_as_postgres_json_type_update_ts(&el_092057f6.field_type)
                                    );
                                    quote! {
                                        #[serde(rename(serialize = #field_ident_double_quotes_ts, deserialize = #field_ident_double_quotes_ts))]
                                        #variant_ident_ucc_ts(#value_field_type_as_json_type_update_ts)
                                    }
                                });
                                quote!{{#(#variants_ts),*}}
                            }
                        );
                        quote!{
                            #allow_clippy_arbitrary_source_item_ordering_ts
                            #content_ts_b258e2eb
                        }
                    };
                    let impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_update_el_ts = postgres_crud_macros_common::gen_impl_postgres_crud_all_variants_default_option_some_vec_one_el_ts(&ident_standart_not_null_update_el_ucc, &{
                        let elements_ts = vec_syn_field.iter().map(|el_2080bd7e| {
                            let field_ident = &el_2080bd7e.field_ident;
                            let variant_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                            let value_content_ts = wrap_into_value_initialization_ts(
                                &postgres_crud_default_option_some_vec_one_el_call_ts
                            );
                            quote! {#self_ucc::#variant_ident_ucc_ts(#value_content_ts)}
                        });
                        quote! {vec![#(#elements_ts),*]}
                    });
                    quote! {
                        #ident_update_el_ts
                        #impl_postgres_crud_all_variants_default_option_some_vec_one_el_for_ident_update_el_ts
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
                    // // error[E0369]: binary operation `==` cannot be applied to type `WrapperOfMyTrait<MyStruct>`
                    let ident_with_id_standart_not_null_update_el_fields_declaration_ts = quote! {
                        #id_sc: #postgres_crud_path_postgres_json_type_uuid_uuid_update_ts,
                        #fields_sc: #ident_standart_not_null_as_postgres_json_type_update_ts
                    };
                    let ident_with_id_standart_not_null_update_el_ts = {
                        let content_ts_d18600a2 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                            &quote!{{#ident_with_id_standart_not_null_update_el_fields_declaration_ts}}
                        );
                        quote!{
                            #allow_clippy_arbitrary_source_item_ordering_ts
                            #content_ts_d18600a2
                        }
                    };
                    let impl_pub_new_for_ident_with_id_standart_not_null_update_el_ts = macros_helpers::gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &must_use_ts,
                        &ident_with_id_standart_not_null_update_el_fields_declaration_ts,
                        &quote! {Self {
                            #id_sc,
                            #fields_sc
                        }},
                    );
                    let impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_standart_not_null_update_el_ts = postgres_crud_macros_common::gen_impl_postgres_crud_default_option_some_vec_one_el_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &Ts2::new(),
                        &quote! {Self {
                            #id_sc: #postgres_crud_default_option_some_vec_one_el_call_ts,
                            #fields_sc: #postgres_crud_default_option_some_vec_one_el_call_ts,
                        }},
                    );
                    quote! {
                        #ident_with_id_standart_not_null_update_el_ts
                        #impl_pub_new_for_ident_with_id_standart_not_null_update_el_ts
                        #impl_postgres_crud_default_option_some_vec_one_el_for_ident_with_standart_not_null_update_el_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #ident_update_ts
                    #maybe_ident_update_try_new_error_named_ts
                    #impl_ident_update_ts
                    #maybe_impl_serde_deserialize_for_ident_update_ts
                    #impl_postgres_crud_default_option_some_vec_one_el_for_ident_update_ts
                    #maybe_ident_update_el_ts
                    #maybe_ident_with_id_standart_not_null_update_el_ts
                }
            };
            let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
            let ident_update_for_query_ts = {
                let ident_update_for_query_ts = {
                    let gen_ident_update_for_query_ts = |content_ts: &dyn quote::ToTokens| {
                        gen_debug_clone_partialeq_serialize_pub_struct_ts(
                            &allow_clippy_arbitrary_source_item_ordering_ts,
                            &ident_update_for_query_ucc,
                            &content_ts
                        )
                    };
                    let gen_std_option_option_ident_type_ts = |current_ident_ts: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_ts(
                        &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&current_ident_ts)
                    );
                    let gen_ident_update_for_query_standart_not_null_content_ts = |is_standart_with_id: &IsStandartWithId| {
                        gen_unique_vec_wrapper_ts(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_update_for_query_el_ucc,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        })
                    };
                    let std_vec_vec_ident_with_id_standart_not_null_create_for_query_ts = postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                        &ident_with_id_standart_not_null_create_for_query_ucc
                    );
                    let std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_for_query_ts = postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(
                        &postgres_crud_path_postgres_json_type_uuid_uuid_update_for_query_ts
                    );
                    match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_update_for_query_ts(
                                &wrap_content_into_scopes_dot_comma_ts(
                                    &gen_ident_update_for_query_standart_not_null_content_ts(
                                        &is_standart_with_id_false
                                    )
                                )
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_ident_update_for_query_ts(
                                &gen_std_option_option_ident_type_ts(
                                    &ident_standart_not_null_as_postgres_json_type_update_for_query_ts
                                )
                            ),
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_ident_update_for_query_ts(
                                &{
                                    let fields_ts = gen_create_update_delete_fields_ts_043c4057(
                                        &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True,
                                        &std_vec_vec_ident_with_id_standart_not_null_create_for_query_ts,
                                        &import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_el_ts,
                                        &std_vec_vec_postgres_crud_path_postgres_json_type_uuid_uuid_update_for_query_ts,//todo maybe expand logic with where cases
                                    );
                                    quote! {{#fields_ts}}
                                }
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_ident_update_for_query_ts(
                                &gen_std_option_option_ident_type_ts(&ident_with_id_array_not_null_as_postgres_json_type_update_for_query_ts)
                            ),
                        },
                    }
                };
                let impl_ident_update_for_query_ts = {
                    let select_only_updated_ids_query_part_ts = {
                        let content_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let match_variants_ts = vec_syn_field.iter().map(|el_bca06812| {
                                        let field_ident = &el_bca06812.field_ident;
                                        let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                        let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_bca06812.field_type);
                                        let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_8e628eaf, "jsonb_build_object({value_c3ae3be4})||"},
                                            &return_err_query_part_error_named_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(#value_sc) => {
                                                match #field_type_as_postgres_json_type_ts::#select_only_updated_ids_query_part_sc(
                                                    &#value_sc.#value_sc,
                                                    #field_ident_double_quotes_ts,
                                                    column_name_and_maybe_field_getter,
                                                    #increment_sc
                                                ) {
                                                    Ok(mut value_c3ae3be4) => {
                                                        let _: Option<char> = value_c3ae3be4.pop();
                                                        #if_write_is_err_curly_braces_ts
                                                    },
                                                    Err(#error_sc) => {
                                                        return Err(#error_sc);
                                                    }
                                                }
                                            }
                                        }
                                    });
                                    quote!{
                                        let mut acc_8e628eaf = #std_string_string_ts::default();
                                        for el_0963b7df in self.0.to_vec() {
                                            match &el_0963b7df {
                                                #(#match_variants_ts),*
                                            }
                                        }
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        Ok(acc_8e628eaf)
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let match_content_ts = vec_syn_field.iter().map(|el_a8f45572| {
                                        let field_ident = &el_a8f45572.field_ident;
                                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                        let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_a8f45572.field_type);
                                        let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_f7537df2, "jsonb_build_object({value})||"},
                                            &return_err_query_part_error_named_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc_ts(
                                                value_92d002a5
                                            ) => match #field_type_as_postgres_json_type_ts::#select_only_updated_ids_query_part_sc(
                                                &value_92d002a5.#value_sc,
                                                #field_ident_double_quotes_ts,
                                                column_name_and_maybe_field_getter,
                                                #increment_sc
                                            ) {
                                                Ok(mut #value_sc) => {
                                                    let _: Option<char> = #value_sc.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                }
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    });
                                    quote!{
                                        Ok(match &self.0 {
                                            Some(value_9570957e) => {
                                                let mut acc_f7537df2 = #std_string_string_ts::default();
                                                for el_97687be3 in value_9570957e.0.to_vec() {
                                                    match &el_97687be3 {
                                                        #(#match_content_ts),*
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
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let match_variants_ts = vec_syn_field.iter().map(|el_74643094| {
                                        let field_ident = &el_74643094.field_ident;
                                        let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                        let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_74643094.field_type);
                                        let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_892857b1, "jsonb_build_object({value_33d3b52e})||"},
                                            &return_err_query_part_error_named_write_into_buffer_ts
                                        );
                                        quote! {
                                            #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(#value_sc) => match #field_type_as_postgres_json_type_ts::#select_only_updated_ids_query_part_sc(
                                                &#value_sc.#value_sc,
                                                #field_ident_double_quotes_ts,
                                                "elem",
                                                #increment_sc
                                            ) {
                                                Ok(mut value_33d3b52e) => {
                                                    let _: Option<char> = value_33d3b52e.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                }
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    });
                                    let select_only_created_ids_query_part_content_ts = vec_syn_field_with_id.iter().map(|el_e6d6df84| {
                                        let field_ident = &el_e6d6df84.field_ident;
                                        let field_ident_double_quotes_ts = gen_quotes::double_quotes_ts(&field_ident);
                                        let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_e6d6df84.field_type);
                                        let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                            &quote!{acc_57cd0744, "jsonb_build_object({value})||"},
                                            &return_err_query_part_error_named_write_into_buffer_ts
                                        );
                                        quote! {
                                            match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_part_sc(
                                                &el_b1359d90.#field_ident,
                                                #field_ident_double_quotes_ts,
                                                "elem",
                                                #increment_sc
                                            ) {
                                                Ok(mut #value_sc) => {
                                                    let _: Option<char> = #value_sc.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                },
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    });
                                    let if_write_is_err_curly_braces_0_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_892857b1, "jsonb_build_object({value})||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    let if_write_is_err_curly_braces_1_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_57cd0744, "{acc_892857b1}||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    let if_write_is_err_0_ts = macros_helpers::gen_if_write_is_err_ts(
                                        &quote!{acc_d497e8a5, "${value_c31cb081},"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    let if_write_is_err_1_ts = macros_helpers::gen_if_write_is_err_ts(
                                        &quote!{acc_d497e8a5, "${value_b52c3fe1},"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    quote!{
                                        Ok(format!(
                                            "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                                            {
                                                let mut acc_57cd0744 = #std_string_string_ts::new();
                                                for el_d7561f40 in self.#update_sc.to_vec() {
                                                    //todo maybe wrong for multiple updates by id?
                                                    let mut acc_892857b1 = #std_string_string_ts::new();
                                                    match #import_path_postgres_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_ts ::select_only_updated_ids_query_part(
                                                        &el_d7561f40.id,
                                                        "id",
                                                        "elem",
                                                        #increment_sc
                                                    ) {
                                                        Ok(mut #value_sc) => {
                                                            let _: Option<char> = #value_sc.pop();
                                                            #if_write_is_err_curly_braces_0_ts
                                                        }
                                                        Err(#error_sc) => {
                                                            return Err(#error_sc);
                                                        }
                                                    }
                                                    for el_738b2a83 in el_d7561f40.fields.0.to_vec() {
                                                        match &el_738b2a83 {
                                                            #(#match_variants_ts),*
                                                        }
                                                    }
                                                    let _: Option<char> = acc_892857b1.pop();
                                                    let _: Option<char> = acc_892857b1.pop();
                                                    #if_write_is_err_curly_braces_1_ts
                                                }
                                                for el_b1359d90 in &self.create {
                                                    #(#select_only_created_ids_query_part_content_ts)*
                                                }
                                                let _: Option<char> = acc_57cd0744.pop();
                                                let _: Option<char> = acc_57cd0744.pop();
                                                format!("jsonb_build_object('value',{acc_57cd0744})")
                                            },
                                            column_name_and_maybe_field_getter,
                                            {
                                                let mut acc_d497e8a5 = #std_string_string_ts::new();
                                                for _ in self.#update_sc.to_vec() {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                                        Ok(value_c31cb081) => {
                                                            #if_write_is_err_0_ts
                                                        },
                                                        Err(#error_sc) => {
                                                            return Err(#error_sc);
                                                        },
                                                    }
                                                }
                                                for _ in &self.#create_sc {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                                        Ok(value_b52c3fe1) => {
                                                            #if_write_is_err_1_ts
                                                        },
                                                        Err(#error_sc) => {
                                                            return Err(#error_sc);
                                                        },
                                                    }
                                                }
                                                let _: Option<char> = acc_d497e8a5.pop();
                                                acc_d497e8a5
                                            }
                                        ))
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => quote!{
                                    Ok(match &self.0 {
                                        Some(value_bc509c9a) => format!(
                                            "jsonb_build_object('value',{})",
                                            match #ident_array_not_null_update_for_query_ucc::#select_only_updated_ids_query_part_sc(
                                                value_bc509c9a,
                                                column_name_and_maybe_field_getter,
                                                #increment_sc
                                            ) {
                                                Ok(value_1e016751) => value_1e016751,
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
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
                            fn #select_only_updated_ids_query_part_sc(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                #increment_sc: &mut u64
                            ) -> Result<#std_string_string_ts, #import_path_query_part_error_named_ts> {
                                #content_ts
                            }
                        }
                    };
                    quote!{
                        impl #ident_update_for_query_ucc {
                            #select_only_updated_ids_query_part_ts
                        }
                    }
                };
                let impl_std_convert_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_ts = macros_helpers::gen_impl_std_convert_from_ts(
                    &quote!{#ident_as_import_path_postgres_json_type_ts::Update},
                    &quote!{#ident_as_import_path_postgres_json_type_ts::UpdateForQuery},
                    &match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                            Pattern::Standart => quote!{
                                Self(#import_path::NotEmptyUniqueVec::from_t1_impl_from_t2(#value_sc.0))
                            },
                            Pattern::Array => quote!{
                                Self {
                                    #create_sc: #value_sc.#create_sc.into_iter().map(#ident_with_id_standart_not_null_create_for_query_ucc::from).collect(),
                                    #update_sc: #import_path::UniqueVec::from_t1_impl_from_t2(#value_sc.#update_sc),
                                    #delete_sc: #value_sc.#delete_sc.into_iter().map(Into::into).collect(),
                                }
                            }
                        },
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                            let content_ts: &dyn quote::ToTokens = match &pattern {
                                Pattern::Standart => &ident_standart_not_null_as_import_path_postgres_json_type_ts,
                                Pattern::Array => &ident_array_not_null_as_import_path_postgres_json_type_ts
                            };
                            quote!{Self(#value_sc.0.map(#content_ts::UpdateForQuery::from))}
                        }
                    }
                );
                let maybe_ident_update_for_query_el_ts = if is_standart_not_null {
                    let ident_standart_not_null_update_for_query_el_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .build_enum(
                            &ident_standart_not_null_update_for_query_el_ucc,
                            &{
                                let variants_ts = vec_syn_field.iter().map(|el_9d8af887| {
                                    let field_ident = &el_9d8af887.field_ident;
                                    let variant_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_ident_double_quotes_ts = gen_field_ident_double_quotes_ts(el_9d8af887);
                                    let value_field_type_as_json_type_update_for_query_ts = wrap_into_value_declaration_ts(&gen_type_as_postgres_json_type_update_for_query_ts(&el_9d8af887.field_type));
                                    quote! {
                                        #[serde(rename(serialize = #field_ident_double_quotes_ts, deserialize = #field_ident_double_quotes_ts))]
                                        #variant_ident_ucc_ts(#value_field_type_as_json_type_update_for_query_ts)
                                    }
                                });
                                quote!{{#(#variants_ts),*}}
                            }
                        );
                    let impl_std_convert_from_ident_standart_not_null_update_el_for_ident_standart_not_null_update_for_query_el_ts = macros_helpers::gen_impl_std_convert_from_ts(
                        &ident_standart_not_null_update_el_ucc,
                        &ident_standart_not_null_update_for_query_el_ucc,
                        &{
                            let variants_ts = vec_syn_field.iter().map(|el_2a5d6ff3| {
                                let field_ident = &el_2a5d6ff3.field_ident;
                                let variant_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                let value_initialization_ts = gen_import_path_value_initialization_ts(&{
                                    let field_type_as_json_type_update_for_query_ts = gen_type_as_postgres_json_type_update_for_query_ts(
                                        &el_2a5d6ff3.field_type
                                    );
                                    quote!{
                                        #field_type_as_json_type_update_for_query_ts::from(value_121f1c54.#value_sc)
                                    }
                                });
                                quote! {
                                    #ident_standart_not_null_update_el_ucc::#variant_ident_ucc_ts(value_121f1c54) => #self_ucc::#variant_ident_ucc_ts(#value_initialization_ts)
                                }
                            });
                            quote!{
                                match #value_sc {
                                    #(#variants_ts),*
                                }
                            }
                        }
                    );
                    quote! {
                        #ident_standart_not_null_update_for_query_el_ts
                        #impl_std_convert_from_ident_standart_not_null_update_el_for_ident_standart_not_null_update_for_query_el_ts
                    }
                } else {
                    Ts2::new()
                };
                let maybe_ident_with_id_standart_not_null_update_for_query_el_ts = if is_standart_not_null {
                    let ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts = quote! {
                        #id_sc: #postgres_crud_path_postgres_json_type_uuid_uuid_update_for_query_ts,
                        #fields_sc: #ident_standart_not_null_as_postgres_json_type_update_for_query_ts
                    };
                    let ident_with_id_standart_not_null_update_for_query_el_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &allow_clippy_arbitrary_source_item_ordering_ts,
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &quote!{{#ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts}}
                    );
                    let impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_el_ts = macros_helpers::gen_impl_pub_const_new_for_ident_ts(
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &must_use_ts,
                        &ident_with_id_standart_not_null_update_for_query_el_fields_declaration_ts,
                        &quote! {Self {
                            #id_sc,
                            #fields_sc
                        }},
                    );
                    let impl_std_convert_from_ident_with_id_standart_not_null_update_el_for_ident_with_id_standart_not_null_update_for_query_el_ts = macros_helpers::gen_impl_std_convert_from_ts(
                        &ident_with_id_standart_not_null_update_el_ucc,
                        &ident_with_id_standart_not_null_update_for_query_el_ucc,
                        &quote! {Self {
                            #id_sc: #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgres_json_type_ts::UpdateForQuery::from(
                                #value_sc.#id_sc
                            ),
                            fields: #ident_standart_not_null_as_import_path_postgres_json_type_ts::UpdateForQuery::from(
                                #value_sc.fields
                            ),
                        }}
                    );
                    quote! {
                        #ident_with_id_standart_not_null_update_for_query_el_ts
                        #impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_el_ts
                        #impl_std_convert_from_ident_with_id_standart_not_null_update_el_for_ident_with_id_standart_not_null_update_for_query_el_ts
                    }
                } else {
                    Ts2::new()
                };
                quote!{
                    #ident_update_for_query_ts
                    #impl_ident_update_for_query_ts
                    #impl_std_convert_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_ts
                    #maybe_ident_update_for_query_el_ts
                    #maybe_ident_with_id_standart_not_null_update_for_query_el_ts
                }
            };
            let (impl_postgres_crud_postgres_json_type_for_ident_ts, maybe_impl_postgres_crud_postgres_types_postgres_type_postgres_type_ts) = {
                let postgres_type_or_postgres_json_type_postgres_type = postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType;
                let postgres_type_or_postgres_json_type_postgres_json_type = postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType;
                let gen_update_query_part_standart_nullable_ts = |postgres_type_or_postgres_json_type: &postgres_crud_macros_common::PostgresTypeOrPostgresJsonType|{
                    let format_handle_ts = gen_quotes::double_quotes_ts(&match &postgres_type_or_postgres_json_type {
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType => format!("jsonb_set({{{jsonb_set_accumulator_sc}}},'{{{{{{{jsonb_set_path_sc}}}}}}}',${{value_27b8537f}})"),
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType => "${value_27b8537f}".to_owned(),
                    });
                    quote! {
                        match &#value_sc.0 {
                            Some(value_92f34435) => #ident_standart_not_null_as_postgres_json_type_ts::#update_query_part_sc(
                                value_92f34435,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                increment,
                            ),
                            None => match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                Ok(value_27b8537f) => Ok(format!(#format_handle_ts)),
                                Err(#error_sc) => Err(#error_sc),
                            }
                        }
                    }
                };
                let gen_update_delete_create_array_ts = |format_handle_ts: &dyn quote::ToTokens|{
                    let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                        &quote!{acc_2e2ad041, "{value_8333f8f4}"},
                        &return_err_query_part_error_named_write_into_buffer_ts
                    );
                    let if_write_is_err_curly_braces_0_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                        &quote!{acc_5b4cd920, "{maybe_space_and_space}elem->>'id' <> ${increment_cb6ba4a7}"},
                        &return_err_query_part_error_named_write_into_buffer_ts
                    );
                    let if_write_is_err_curly_braces_1_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                        &quote!{acc_8554f572, "${increment_5bbc4961},"},
                        &return_err_query_part_error_named_write_into_buffer_ts
                    );
                    quote! {
                        let update_query_part_acc = {
                            if value_58d685d3.#update_sc.is_empty() {
                                #std_string_string_ts::from("elem")
                            } else {
                                let mut acc_2e2ad041 = #std_string_string_ts::default();
                                for el_a0a61823 in value_58d685d3.#update_sc.to_vec() {
                                    let ident_with_id_handle = {
                                        let id_increment = match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::increment_checked_add_one(#increment_sc) {
                                            Ok(value_15b44b54) => value_15b44b54,
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        };
                                        match #ident_standart_not_null_as_postgres_json_type_ts::#update_query_part_sc(
                                            &el_a0a61823.fields,
                                            "",
                                            "elem",
                                            "",
                                            #increment_sc
                                        ) {
                                            Ok(value_56c44461) => Ok(format!("when elem->>'id' = ${id_increment} then {value_56c44461} ")),
                                            Err(#error_sc) => Err(#error_sc)
                                        }
                                    };
                                    match ident_with_id_handle {
                                        Ok(value_8333f8f4) => {
                                            #if_write_is_err_ts
                                        }
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                }
                                let _: Option<char> = acc_2e2ad041.pop();
                                format!("case {acc_2e2ad041} else elem end")
                            }
                        };
                        let delete_query_part_acc = {
                            let mut acc_5b4cd920 = #std_string_string_ts::default();
                            for _ in &value_58d685d3.#delete_sc {
                                let increment_cb6ba4a7 = match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::increment_checked_add_one(#increment_sc) {
                                    Ok(value_110650cc) => value_110650cc,
                                    Err(#error_sc) => {
                                        return Err(#error_sc);
                                    }
                                };
                                let maybe_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                                #if_write_is_err_curly_braces_0_ts
                            }
                            acc_5b4cd920
                        };
                        let create_query_part_acc = {
                            let mut acc_8554f572 = #std_string_string_ts::default();
                            for _ in &value_58d685d3.#create_sc {
                                let increment_5bbc4961 = match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::increment_checked_add_one(#increment_sc) {
                                    Ok(value_27487842) => value_27487842,
                                    Err(#error_sc) => {
                                        return Err(#error_sc);
                                    }
                                };
                                #if_write_is_err_curly_braces_1_ts
                            }
                            let _: Option<char> = acc_8554f572.pop();
                            acc_8554f572
                        };
                        let maybe_where = if value_58d685d3.#delete_sc.is_empty() {
                            #std_string_string_ts::default()
                        } else {
                            format!(" where {delete_query_part_acc}")
                        };
                        let maybe_jsonb_build_array = if value_58d685d3.#create_sc.is_empty() {
                            #std_string_string_ts::default()
                        } else {
                            format!(" || jsonb_build_array({create_query_part_acc})")
                        };
                        Ok (format!(#format_handle_ts))
                    }
                };
                let gen_update_query_part_array_not_null_ts = |postgres_type_or_postgres_json_type: &postgres_crud_macros_common::PostgresTypeOrPostgresJsonType|{
                    let content_ts_c75c3cd1 = gen_update_delete_create_array_ts(&gen_quotes::double_quotes_ts(&match &postgres_type_or_postgres_json_type {
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType => "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})",
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType => "((select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) {maybe_jsonb_build_array})",
                    }));
                    quote!{
                        let value_58d685d3 = #value_sc;
                        #content_ts_c75c3cd1
                    }
                };
                let impl_postgres_crud_postgres_json_type_for_ident_ts = postgres_crud_macros_common::gen_impl_postgres_json_type_ts(
                    &postgres_crud_macros_common::ImportPath::PostgresCrud,
                    &ident,
                    &ident_table_type_declaration_ucc,
                    &ident_create_ucc,
                    &ident_create_for_query_ucc,
                    &ident_select_ucc,
                    &postgres_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                    &postgres_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::True,
                    &postgres_crud_macros_common::IsSelectQueryPartIsPostgresTypeUsed::True,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {
                                match #value_sc.#select_query_part_sc(
                                    &if is_postgres_type {
                                        column_name_and_maybe_field_getter.to_owned()
                                    } else {
                                        format!("{column_name_and_maybe_field_getter}->'{field_ident}'")
                                    },
                                    &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
                                ) {
                                    Ok(value_156121ad) => Ok(
                                        if is_postgres_type {
                                            value_156121ad
                                        } else {
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',{value_156121ad}))")
                                        }
                                    ),
                                    Err(#error_sc) => Err(#error_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_standart_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                    &ident_standart_not_null_as_postgres_json_type_select_ts
                                );
                                quote! {
                                    let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
                                    let value_46039f0e = value.0.as_ref().map_or_else(
                                        #ident_standart_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts,
                                        Clone::clone
                                    );
                                    match #ident_standart_not_null_as_postgres_json_type_ts::#select_query_part_sc(
                                        &value_46039f0e,
                                        field_ident,
                                        &column_name_and_maybe_field_getter_field_ident,
                                        column_name_and_maybe_field_getter_for_error_message,
                                        true
                                    ) {
                                        Ok(value_1f8de96a) => Ok(
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({value_1f8de96a}) end))")
                                        ),
                                        Err(#error_sc) => Err(#error_sc)
                                    }
                                }
                            },
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let acc_41dea548_ts = quote!{acc_41dea548};
                                let select_query_part_for_loop_ts = {
                                    let value_double_quotes_ts = gen_quotes::double_quotes_ts(&value_sc);
                                    gen_select_query_part_for_loop_ts(
                                        &acc_41dea548_ts,
                                        &is_standart_with_id_true,
                                        &quote!{#value_sc.#ident_with_id_standart_not_null_select_sc},
                                        &value_double_quotes_ts,
                                        &value_double_quotes_ts,
                                    )
                                };
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',case when (jsonb_array_length({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_sc}}})) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{dimension1_start}} and {{dimension1_end}}) end ))"
                                ));
                                quote! {
                                    let #ident_with_id_standart_not_null_select_sc = {
                                        let mut #acc_41dea548_ts = #std_string_string_ts::default();
                                        #select_query_part_for_loop_ts
                                        let _: Option<char> = #acc_41dea548_ts.pop();
                                        let _: Option<char> = #acc_41dea548_ts.pop();
                                        #acc_41dea548_ts
                                    };
                                    let dimension1_start = #value_sc.#dimension1_pagination_ts.start();
                                    let dimension1_end = #value_sc.#dimension1_pagination_ts.end();
                                    Ok(format!(#format_handle_ts))
                                }
                            }
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let format_handle_ts = gen_quotes::double_quotes_ts(
                                    &"case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({value_d7bbd03c}) end"
                                );
                                let ident_with_id_array_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts = gen_ident_as_default_but_option_is_some_ts(
                                    &ident_with_id_array_not_null_as_postgres_json_type_select_ts
                                );
                                quote! {
                                    let value_174d33cd = #value_sc.0.as_ref().map_or_else(
                                        #ident_with_id_array_not_null_as_postgres_json_type_select_as_default_but_option_is_some_ts,
                                        Clone::clone
                                    );
                                    match #ident_with_id_array_not_null_as_postgres_json_type_ts::#select_query_part_sc(
                                        &value_174d33cd,
                                        field_ident,
                                        column_name_and_maybe_field_getter,
                                        column_name_and_maybe_field_getter_for_error_message,
                                        true
                                    ) {
                                        Ok(value_d7bbd03c) => Ok(format!(#format_handle_ts)),
                                        Err(#error_sc) => Err(#error_sc)
                                    }
                                }
                            }
                        },
                    },
                    &ident_where_ucc,
                    &ident_read_ucc,
                    &ident_read_only_ids_ucc,
                    &match &not_null_or_nullable {
                        postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                            let content_ts = {
                                let content_ts = {
                                    let acc_push_ts = get_vec_syn_field(match &pattern {
                                        Pattern::Standart => &is_standart_with_id_false,
                                        Pattern::Array => &is_standart_with_id_true
                                    }).iter().map(|el_a6a15738| {
                                        let field_ident = &el_a6a15738.field_ident;
                                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!("jsonb_build_object('{field_ident}',{{}})||"));
                                        let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_a6a15738.field_type);
                                        let content_ts = match &pattern {
                                            Pattern::Standart => {
                                                let format_ts = gen_quotes::double_quotes_ts(&format!("{{column_name_and_maybe_field_getter}}->'{field_ident}'"));
                                                quote! {&format!(#format_ts)}
                                            },
                                            Pattern::Array => gen_quotes::double_quotes_ts(&format!("elem->'{field_ident}'"))
                                        };
                                        macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                            &quote!{
                                                acc_2912b128,
                                                #format_handle_ts,
                                                match #field_type_as_postgres_json_type_ts::#select_only_ids_query_part_sc(#content_ts) {
                                                    Ok(value_2317e0af) => value_2317e0af,
                                                    Err(#error_sc) => {
                                                        return Err(#error_sc);
                                                    }
                                                }
                                            },
                                            &return_err_query_part_error_named_write_into_buffer_ts
                                        )
                                    });
                                    quote! {{
                                        let mut acc_2912b128 = #std_string_string_ts::default();
                                        #(#acc_push_ts)*
                                        let _: Option<char> = acc_2912b128.pop();
                                        let _: Option<char> = acc_2912b128.pop();
                                        format!("jsonb_build_object('value',{acc_2912b128})")
                                    }}
                                };
                                match &pattern {
                                    Pattern::Standart => content_ts,
                                    Pattern::Array => {
                                        let format_handle_ts = gen_quotes::double_quotes_ts(
                                            &format!("jsonb_build_object('value',(select jsonb_agg({{}}) from jsonb_array_elements({{{column_name_and_maybe_field_getter_sc}}}) as elem))")
                                        );
                                        quote! {format!(#format_handle_ts, #content_ts)}
                                    },
                                }
                            };
                            quote! {Ok(#content_ts)}
                        },
                        postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                            let content_ts: &dyn quote::ToTokens = match &pattern {
                                Pattern::Standart => &ident_standart_not_null_as_postgres_json_type_ts,
                                Pattern::Array => &ident_with_id_array_not_null_as_postgres_json_type_ts,
                            };
                            let case_null_format_handle_ts = gen_quotes::double_quotes_ts(
                                &format!("jsonb_build_object('value',case when jsonb_typeof({{{column_name_and_maybe_field_getter_sc}}})='null' then 'null'::jsonb else {{value_21000130}} end)")
                            );
                            quote! {
                                match #content_ts::#select_only_ids_query_part_sc(#column_name_and_maybe_field_getter_sc) {
                                    Ok(value_21000130) => Ok(format!(#case_null_format_handle_ts)),
                                    Err(#error_sc) => Err(#error_sc)
                                }
                            }
                        }
                    },
                    &ident_read_inner_ucc,
                    &{
                        let gen_into_inner_ts = |current_ident_ts: &dyn quote::ToTokens, parameters_ts: &dyn quote::ToTokens|{
                            quote!{#current_ident_ts::into_inner(#parameters_ts)}
                        };
                        let gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts = |is_standart_with_id: &IsStandartWithId| {
                            let current_ident_ts: &dyn quote::ToTokens = match &is_standart_with_id {
                                IsStandartWithId::False => &ident_read_inner_ucc,
                                IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_ucc,
                            };
                            let content_ts = get_vec_syn_field(is_standart_with_id).iter().map(|el_d2c28655| {
                                let field_ident = &el_d2c28655.field_ident;
                                let content_ts = wrap_into_value_initialization_ts(&gen_into_inner_ts(
                                    &gen_type_as_postgres_json_type_ts(&el_d2c28655.field_type),
                                    &quote!{value_6e5af985.#value_sc},
                                ));
                                let parameter_ts: &dyn quote::ToTokens = match &is_standart_with_id {
                                    IsStandartWithId::False => &value_sc,
                                    IsStandartWithId::True => &quote!{el_34d57236},
                                };
                                quote! {#field_ident: #parameter_ts.#field_ident.map(|value_6e5af985| #content_ts)}
                            });
                            quote! {
                                #current_ident_ts {
                                    #(#content_ts),*
                                }
                            }
                        };
                        match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                Pattern::Standart => gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&IsStandartWithId::False),
                                Pattern::Array => {
                                    let content_ts = gen_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_ts(&IsStandartWithId::True);
                                    quote! {#value_sc.0.into_iter().map(|el_34d57236|#content_ts).collect()}
                                },
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let current_ident = gen_type_as_postgres_json_type_ts(&match &pattern {
                                    Pattern::Standart => ident_standart_not_null_ucc,
                                    Pattern::Array => ident_array_not_null_ucc,
                                });
                                quote! {#value_sc.0.map(#current_ident::into_inner)}
                            }
                        }
                    },
                    &ident_update_ucc,
                    &ident_update_for_query_ucc,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let object_acc_sc = StdOptionOptionObjectAccSc;
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!("jsonb_set({{{jsonb_set_accumulator_sc}}},'{{{{{{{jsonb_set_path_sc}}}}}}}',{{{object_acc_sc}}})"));
                                let query_part_variants_ts = vec_syn_field.iter().map(|el_ebd92dbf| {
                                    let variant_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_ebd92dbf.field_ident.to_string());
                                    let field_ident_double_quotes_ts = gen_field_ident_double_quotes_ts(el_ebd92dbf);
                                    let field_type_as_crud_postgres_json_type_from_field_ts = gen_field_type_as_crud_postgres_json_type_from_field_ts(el_ebd92dbf);
                                    quote! {
                                        #ident_update_for_query_el_ucc::#variant_ident_ucc_ts(value_3b3fae4c) => {
                                            match #field_type_as_crud_postgres_json_type_from_field_ts::#update_query_part_sc(
                                                &value_3b3fae4c.#value_sc,
                                                &#object_acc_sc,
                                                &#gen_jsonb_set_target_sc(#field_ident_double_quotes_ts),
                                                #field_ident_double_quotes_ts,
                                                #increment_sc,
                                            ) {
                                                Ok(value_5edc1648) => {
                                                    #object_acc_sc = value_5edc1648;
                                                }
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    }
                                });
                                let some_format_handle_ts = gen_quotes::double_quotes_ts(&format!("case when jsonb_typeof({{{jsonb_set_target_sc}}}) = 'object' then ({{{jsonb_set_target_sc}}})::jsonb else '{{{{}}}}'::jsonb end"));
                                quote! {
                                    let mut #object_acc_sc = format!(#some_format_handle_ts);
                                    #gen_jsonb_set_target_ts
                                    for el_157f4b80 in #value_sc.0.to_vec() {
                                        match el_157f4b80 {
                                            #(#query_part_variants_ts),*
                                        }
                                    }
                                    if #jsonb_set_path_sc.is_empty() {
                                        Ok(#object_acc_sc)
                                    }
                                    else {
                                        Ok(format!(#format_handle_ts))
                                    }
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_update_query_part_standart_nullable_ts(
                                &postgres_type_or_postgres_json_type_postgres_type
                            )
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_update_query_part_array_not_null_ts(
                                &postgres_type_or_postgres_json_type_postgres_type
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                match &#value_sc.0 {
                                    Some(value_3245b79f) => #ident_array_not_null_as_postgres_json_type_ts::#update_query_part_sc(
                                        value_3245b79f,
                                        jsonb_set_accumulator,
                                        jsonb_set_target,
                                        jsonb_set_path,
                                        #increment_sc,
                                    ),
                                    None => match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                        Ok(value_87e08bec) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${value_87e08bec})")),
                                        Err(#error_sc) => Err(#error_sc)
                                    }
                                }
                            },
                        },
                    },
                    &postgres_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::True,
                    &postgres_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::True,
                    &postgres_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let update_query_bind_variants_ts = vec_syn_field.iter().map(|el_9968a29b| {
                                    let variant_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el_9968a29b.field_ident.to_string());
                                    let field_type_as_crud_postgres_json_type_from_field_ts = gen_field_type_as_crud_postgres_json_type_from_field_ts(
                                        el_9968a29b
                                    );
                                    quote! {
                                        #ident_update_for_query_el_ucc::#variant_ident_ucc_ts(value_b27f5b09) => {
                                            match #field_type_as_crud_postgres_json_type_from_field_ts::#update_query_bind_sc(
                                                value_b27f5b09.#value_sc,
                                                #query_sc
                                            ) {
                                                Ok(value_a4870bad) => {
                                                    #query_sc = value_a4870bad;
                                                },
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote! {
                                    for el_f14dcf6b in #value_sc.0.into_vec() {
                                        match el_f14dcf6b {
                                            #(#update_query_bind_variants_ts),*
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                match #value_sc.0 {
                                    Some(value_269a0d34) => #ident_standart_not_null_as_postgres_json_type_ts::update_query_bind(
                                        value_269a0d34,
                                        #query_sc
                                    ),
                                    None => if let Err(#error_sc) = #query_sc.try_bind(sqlx::types::Json(#self_as_postgres_json_type_update_ts::new(None))) {
                                        Err(#error_sc.to_string())
                                    }
                                    else {
                                        Ok(#query_sc)
                                    },
                                }
                            }
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => quote! {
                                for el_30541f69 in #value_sc.#update_sc.into_vec() {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::query_bind_string_as_postgres_text_update_for_query(
                                        el_30541f69.#id_sc,
                                        #query_sc
                                    ) {
                                        Ok(value_7633dc9b) => {
                                            #query_sc = value_7633dc9b;
                                        },
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                    match #ident_standart_not_null_as_postgres_json_type_ts::update_query_bind(
                                        el_30541f69.#fields_sc,
                                        #query_sc
                                    ) {
                                        Ok(value_2073f07a) => {
                                            #query_sc = value_2073f07a;
                                        },
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                }
                                for el_4b6f8c01 in #value_sc.delete {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::query_bind_string_as_postgres_text_update_for_query(
                                        el_4b6f8c01,
                                        #query_sc
                                    ) {
                                        Ok(value_31262d92) => {
                                            #query_sc = value_31262d92;
                                        },
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                }
                                for el_a44eb132 in #value_sc.#create_sc {
                                    if let Err(#error_sc) = #query_sc.try_bind(sqlx::types::Json(el_a44eb132)) {
                                        return Err(#error_sc.to_string());
                                    }
                                }
                                Ok(#query_sc)
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                match #value_sc.0 {
                                    Some(value_a2156b3e) => #ident_array_not_null_as_postgres_json_type_ts::update_query_bind(
                                        value_a2156b3e,
                                        #query_sc
                                    ),
                                    None => if let Err(#error_sc) = #query_sc.try_bind(sqlx::types::Json(#self_as_postgres_json_type_update_ts::new(None))) {
                                        Err(#error_sc.to_string())
                                    }
                                    else {
                                        Ok(#query_sc)
                                    },
                                }
                            },
                        },
                    },
                    &quote!{
                        match #value_sc.#select_only_updated_ids_query_part_sc(
                            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                            #increment_sc
                        ) {
                            Ok(value_e137951b) => Ok(format!("'{field_ident}',jsonb_build_object('value',{value_e137951b}),")),
                            Err(#error_sc) => Err(#error_sc)
                        }
                    },
                    &postgres_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let match_content_ts = vec_syn_field.iter().map(|el_e3bd5731| {
                                    let field_ident = &el_e3bd5731.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_e3bd5731.field_type);
                                    let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                                    quote! {
                                        #ident_standart_not_null_update_for_query_el_ucc::#field_ident_ucc(value_b79c2851) => {
                                            match #field_type_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(
                                                &value_b79c2851.#value_sc,
                                                #query_sc
                                            ) {
                                                Ok(value_e8914f75) => {
                                                    #query_sc = value_e8914f75;
                                                },
                                                Err(#error_sc) => {
                                                    return Err(#error_sc);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    for el_31dd08ee in #value_sc.0.to_vec() {
                                        match el_31dd08ee {
                                            #(#match_content_ts),*
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote!{
                                if let Some(value_6334d77d) = &#value_sc.0 {
                                    match #ident_standart_not_null_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(value_6334d77d, #query_sc) {
                                        Ok(value_0bd3ba6f) => {
                                            #query_sc = value_0bd3ba6f;
                                        },
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                }
                                Ok(#query_sc)
                            },
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let select_only_created_ids_query_bind_content_ts = vec_syn_field_with_id.iter().map(|el_27e0de74| {
                                    let field_ident = &el_27e0de74.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_27e0de74.field_type);
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_bind_sc(
                                            &el_5fba4c1f.#field_ident,
                                            #query_sc
                                        ) {
                                            Ok(value_cb81ec2c) => {
                                                #query_sc = value_cb81ec2c;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    for el_e5af9b26 in #value_sc.#update_sc.to_vec() {
                                        match #import_path_postgres_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(
                                            &el_e5af9b26.#id_sc,
                                            #query_sc
                                        ) {
                                            Ok(value_0fd735de) => {
                                                #query_sc = value_0fd735de;
                                            },
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                        match #ident_standart_not_null_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(
                                            &el_e5af9b26.fields,
                                            #query_sc
                                        ) {
                                            Ok(value_4b52fa4f) => {
                                                #query_sc = value_4b52fa4f;
                                            },
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    for el_5fba4c1f in &#value_sc.create {
                                        #(#select_only_created_ids_query_bind_content_ts)*
                                    }
                                    for el_d9eff5ca in #value_sc.#update_sc.to_vec() {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::query_bind_string_as_postgres_text_update_for_query(
                                            el_d9eff5ca.#id_sc.clone(),
                                            #query_sc
                                        ) {
                                            Ok(value_b0da764b) => {
                                                #query_sc = value_b0da764b;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    for el_ae971756 in &#value_sc.#create_sc {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::query_bind_string_as_postgres_text_create_for_query(
                                            el_ae971756.#id_sc.clone(),
                                            #query_sc
                                        ) {
                                            Ok(value_dd8932e8) => {
                                                #query_sc = value_dd8932e8;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => quote!{
                                if let Some(value_107e6639) = &#value_sc.0 {
                                    match #ident_array_not_null_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(value_107e6639, #query_sc) {
                                        Ok(value_ecf1b8de) => {
                                            #query_sc = value_ecf1b8de;
                                        },
                                        Err(#error_sc) => {
                                            return Err(#error_sc);
                                        }
                                    }
                                }
                                Ok(#query_sc)
                            },
                        },
                    },
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts = vec_syn_field.iter().map(|el_6bcf3221| {
                                    let field_ident = &el_6bcf3221.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_6bcf3221.field_type);
                                    let field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(
                                        &format!("{{{column_name_and_maybe_field_getter_sc}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0fe559fa, "jsonb_build_object({value_cddc8a0a})||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_part_sc(
                                            &#value_sc.#field_ident,
                                            #field_ident_double_quotes_ts,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_ts),
                                            #increment_sc
                                        ) {
                                            Ok(mut value_cddc8a0a) => {
                                                let _: Option<char> = value_cddc8a0a.pop();
                                                #if_write_is_err_curly_braces_ts
                                            },
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        {
                                            let mut acc_0fe559fa = #std_string_string_ts::new();
                                            #(#content_ts)*
                                            let _: Option<char> = acc_0fe559fa.pop();
                                            let _: Option<char> = acc_0fe559fa.pop();
                                            acc_0fe559fa
                                        }
                                    ))
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_ts = vec_syn_field.iter().map(|el_88c65ca5| {
                                    let field_ident = &el_88c65ca5.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_88c65ca5.field_type);
                                    let field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(
                                        &format!("{{{column_name_and_maybe_field_getter_sc}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0e9170a3, "jsonb_build_object({value_93015133})||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_part_sc(
                                            &value_90219286.#field_ident,
                                            #field_ident_double_quotes_ts,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_ts),
                                            #increment_sc
                                        ) {
                                            Ok(mut value_93015133) => {
                                                let _: Option<char> = value_93015133.pop();
                                                #if_write_is_err_curly_braces_ts
                                            },
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#value_sc.0 {
                                            Some(value_90219286) => format!(
                                                "jsonb_build_object('value',{})",
                                                {
                                                    let mut acc_0e9170a3 = #std_string_string_ts::new();
                                                    #(#content_ts)*
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
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts = vec_syn_field_with_id.iter().map(|el_bfecacfd| {
                                    let field_ident = &el_bfecacfd.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_bfecacfd.field_type);
                                    let field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(&field_ident);
                                    let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_0f2b92d0, "jsonb_build_object({value_6d76c065})||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_part_sc(
                                            &el_3c1dab62.#field_ident,
                                            #field_ident_double_quotes_ts,
                                            "elem",
                                            #increment_sc
                                        ) {
                                            Ok(mut value_6d76c065) => {
                                                let _: Option<char> = value_6d76c065.pop();
                                                #if_write_is_err_curly_braces_ts
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                                    &quote!{acc_44b1f772, "${value_73b58d3a},"},
                                    &return_err_query_part_error_named_write_into_buffer_ts
                                );
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
                                        {
                                            let mut acc_0f2b92d0 = #std_string_string_ts::new();
                                            for el_3c1dab62 in &#value_sc.0 {
                                                #(#content_ts)*
                                            }
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            format!("jsonb_build_object('value',{acc_0f2b92d0})")
                                        },
                                        &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                        {
                                            let mut acc_44b1f772 = #std_string_string_ts::new();
                                            for _ in &#value_sc.0 {
                                                match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                                    Ok(value_73b58d3a) => {
                                                        #if_write_is_err_ts
                                                    },
                                                    Err(#error_sc) => {
                                                        return Err(#error_sc);
                                                    },
                                                }
                                            }
                                            let _: Option<char> = acc_44b1f772.pop();
                                            acc_44b1f772
                                        }
                                    ))
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_ts = vec_syn_field_with_id.iter().map(|el_76f33159| {
                                    let field_ident = &el_76f33159.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_76f33159.field_type);
                                    let field_ident_double_quotes_ts = &gen_quotes::double_quotes_ts(&field_ident);
                                    let if_write_is_err_curly_braces_ts = macros_helpers::gen_if_write_is_err_curly_braces_ts(
                                        &quote!{acc_1a91bdc7, "jsonb_build_object({value_d49fe9d8})||"},
                                        &return_err_query_part_error_named_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_part_sc(
                                            &el_9bdcd847.#field_ident,
                                            #field_ident_double_quotes_ts,
                                            "elem",
                                            #increment_sc
                                        ) {
                                            Ok(mut value_d49fe9d8) => {
                                                let _: Option<char> = value_d49fe9d8.pop();
                                                #if_write_is_err_curly_braces_ts
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                                    &quote!{acc_857ce631, "${value_7f11bec0},"},
                                    &return_err_query_part_error_named_write_into_buffer_ts
                                );
                                quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#value_sc.0 {
                                            Some(value_3c415c92) => format!(
                                                "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                                                {
                                                    let mut acc_1a91bdc7 = #std_string_string_ts::new();
                                                    for el_9bdcd847 in &value_3c415c92.0 {
                                                        #(#content_ts)*
                                                    }
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    format!("jsonb_build_object('value',{acc_1a91bdc7})")
                                                },
                                                &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                                {
                                                    let mut acc_857ce631 = #std_string_string_ts::new();
                                                    for _ in &value_3c415c92.0 {
                                                        match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                                            Ok(value_7f11bec0) => {
                                                                #if_write_is_err_ts
                                                            },
                                                            Err(#error_sc) => {
                                                                return Err(#error_sc);
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
                    &postgres_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts = vec_syn_field.iter().map(|el_9d72fe6e| {
                                    let field_ident = &el_9d72fe6e.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_9d72fe6e.field_type);
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_bind_sc(
                                            &#value_sc.#field_ident,
                                            #query_sc
                                        ) {
                                            Ok(value_231618d9) => {
                                                #query_sc = value_231618d9;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    #(#content_ts)*
                                    Ok(#query_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote!{
                                    if let Some(value_a1ccd526) = &#value_sc.0 {
                                        match #ident_standart_not_null_as_import_path_postgres_json_type_ts::#select_only_created_ids_query_bind_sc(
                                            value_a1ccd526,
                                            #query_sc
                                        ) {
                                            Ok(value_70ed6013) => {
                                                #query_sc = value_70ed6013;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts = vec_syn_field_with_id.iter().map(|el_43b720bb| {
                                    let field_ident = &el_43b720bb.field_ident;
                                    let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&el_43b720bb.field_type);
                                    quote! {
                                        match #field_type_as_postgres_json_type_ts::#select_only_created_ids_query_bind_sc(&el_9bdcd847.#field_ident, #query_sc) {
                                            Ok(value_ade27463) => {
                                                #query_sc = value_ade27463;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    for el_9bdcd847 in &#value_sc.0 {
                                        #(#content_ts)*
                                    }
                                    for el_b191a891 in &#value_sc.0 {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::query_bind_string_as_postgres_text_create_for_query(
                                            el_b191a891.#id_sc.clone(),
                                            #query_sc
                                        ) {
                                            Ok(value_a3749ea8) => {
                                                #query_sc = value_a3749ea8;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote!{
                                    if let Some(value_0b55a65a) = &#value_sc.0 {
                                        match #ident_array_not_null_as_import_path_postgres_json_type_ts::#select_only_created_ids_query_bind_sc(value_0b55a65a, #query_sc) {
                                            Ok(value_ad6a1ac5) => {
                                                #query_sc = value_ad6a1ac5;
                                            }
                                            Err(#error_sc) => {
                                                return Err(#error_sc);
                                            }
                                        }
                                    }
                                    Ok(#query_sc)
                                }
                            },
                        },
                    },
                );
                let impl_postgres_crud_postgres_types_postgres_type_postgres_type_ts = postgres_crud_macros_common::gen_impl_postgres_type_ts(
                    &postgres_crud_macros_common::ImportPath::PostgresCrud,
                    &ident,
                    &ident_table_type_declaration_ucc,
                    &postgres_crud_macros_common::IsPrimaryKeyUnderscore::True,
                    &{
                        let format_handle_ts = gen_quotes::double_quotes_ts(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_owned());
                        quote! {
                            format!(#format_handle_ts, serde_json::to_string(&schemars::schema_for!(#ident_table_type_declaration_ucc)).expect("59a1654b-cbde-40a6-a958-383d263ee19d"))
                        }
                    },
                    &ident_create_ucc,
                    &postgres_crud_macros_common::CreateQueryPartValueUnderscore::True,
                    &postgres_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                    &quote!{
                        match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
                            Err(#error_sc) => Err(#error_sc)
                        }
                    },
                    &postgres_crud_macros_common::CreateQueryBindValueUnderscore::False,
                    &postgres_crud_macros_common::IsCreateQueryBindMutable::True,
                    &quote!{
                        if let Err(#error_sc) = #query_sc.try_bind(
                            #self_as_postgres_json_type_create_for_query_ts::from(#value_sc)
                        ) {
                            return Err(#error_sc.to_string());
                        }
                        Ok(#query_sc)
                    },
                    &ident_select_ucc,
                    &postgres_crud_macros_common::SelectQueryPartValueUnderscore::False,
                    &quote! {
                        match #value_sc.#select_query_part_postgres_type_sc(#column_sc) {
                            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
                            Err(#error_sc) => Err(#error_sc)
                        }
                    },
                    &ident_where_ucc,
                    &ident_read_ucc,
                    &value_sc,
                    &ident_read_only_ids_ucc,
                    &quote! {
                        match #self_as_postgres_json_type_ts::#select_only_ids_query_part_sc(#column_sc) {
                            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
                            Err(#error_sc) => Err(#error_sc)
                        }
                    },
                    &ident_read_inner_ucc,
                    &quote!{#self_as_postgres_json_type_ts::into_inner(#value_sc)},
                    &ident_update_ucc,
                    &ident_update_for_query_ucc,
                    &postgres_crud_macros_common::UpdateQueryPartValueUnderscore::False,
                    &postgres_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::False,
                    &postgres_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::False,
                    &postgres_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::False,
                    &match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => quote!{#self_as_postgres_json_type_ts::#update_query_part_sc(
                                value,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                increment
                            )},
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_update_query_part_standart_nullable_ts(
                                &postgres_type_or_postgres_json_type_postgres_json_type
                            )
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_update_query_part_array_not_null_ts(
                                &postgres_type_or_postgres_json_type_postgres_json_type
                            ),
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_ts = gen_update_delete_create_array_ts(&quote!{
                                    "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
                                });
                                quote! {
                                    match &#value_sc.0 {
                                        Some(value_58d685d3) => {
                                            #content_ts
                                        },
                                        None => match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                                            Ok(value_d31ab6f0) => Ok(format!("${value_d31ab6f0}")),
                                            Err(#error_sc) => Err(#error_sc)
                                        }
                                    }
                                }
                            },
                        },
                    },
                    &postgres_crud_macros_common::IsUpdateQueryBindMutable::False,
                    &quote!{#self_as_postgres_json_type_ts::#update_query_bind_sc(
                        #value_sc,
                        #query_sc
                    )},
                    &quote!{
                        match #value_sc.#select_only_updated_ids_query_part_sc(
                            #column_sc,
                            #increment_sc
                        ) {
                            Ok(value_f0787243) => Ok(format!("jsonb_build_object('value',{value_f0787243}) as {column},")),
                            Err(#error_sc) => Err(#error_sc)
                        }
                    },
                    &postgres_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                    &quote!{#self_as_postgres_json_type_ts::#select_only_updated_ids_query_bind_sc(
                        #value_sc,
                        #query_sc
                    )},
                );
                match &trait_gen {
                    TraitGen::PostgresTypeAndPostgresJsonType => (impl_postgres_crud_postgres_json_type_for_ident_ts, impl_postgres_crud_postgres_types_postgres_type_postgres_type_ts),
                    TraitGen::PostgresJsonType => (impl_postgres_crud_postgres_json_type_for_ident_ts, Ts2::new()),
                }
            };
            let self_postgres_json_type_ts = quote!{#self_ucc::#postgres_json_type_ucc};
            let (impl_postgres_json_type_test_cases_for_ident_ts, impl_postgres_type_test_cases_for_ident_ts) = {
                let gen_dimension_equal_ts = |dimension: &postgres_crud_macros_common::Dimension|{
                    let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc = dimension.read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc();
                    let gen_nullable_ts = |content_ts: &dyn quote::ToTokens|quote! {
                        match #import_path::NotEmptyUniqueVec::try_new(
                            match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                (Some(read_only_ids_cdcb6239), Some(create_fdd53941)) => match <
                                    #content_ts
                                    as
                                    #import_path::PostgresJsonTypeTestCases
                                >::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc(
                                    read_only_ids_cdcb6239,
                                    create_fdd53941
                                ) {
                                    Some(value_d6124e21) => {
                                        let mut acc_bd78dc08 = Vec::new();
                                        for el_6739e82f in value_d6124e21.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![el_6739e82f]
                                            ) {
                                                Ok(value_7ed84f3b) => {
                                                    acc_bd78dc08.push(
                                                        #import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_7ed84f3b))
                                                    );
                                                },
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("23dca12f-65c0-4c0e-addd-cc392c663733")
                                                }
                                            }
                                        }
                                        let value_e48110ec = #import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_d6124e21));
                                        if !acc_bd78dc08.contains(&value_e48110ec) {
                                            acc_bd78dc08.push(value_e48110ec);
                                        }
                                        acc_bd78dc08
                                    },
                                    None => {
                                        return None;
                                    }
                                },
                                (Some(_), None) => panic!("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98"),
                                (None, Some(_)) => panic!("a2761cd2-27ff-4db0-ae81-948aa04573a6"),
                                (None, None) => vec![#import_path::NullableJsonObjectPostgresTypeWhereFilter(None)]
                            }
                        ) {
                            Ok(value_55f2dc3d) => Some(value_55f2dc3d),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("88912e24-3bee-4dc4-a373-6d96d260170f")
                            }
                        }
                    };
                    match &pattern {
                        Pattern::Standart => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts = vec_syn_field.iter().map(|el_3a1eac56| {
                                    let field_ident = &el_3a1eac56.field_ident;
                                    let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                    let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_3a1eac56.field_type);
                                    quote! {
                                        if let Some(value_2bbd2c96) = #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc(
                                            #read_only_ids_sc.0.#value_sc.#field_ident,
                                            #create_sc.#field_ident
                                        ) {
                                            for el_84537322 in value_2bbd2c96.clone().into_vec() {
                                                acc_2fe1cca8.push(
                                                    #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PostgresTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![el_84537322]
                                                        ).expect("9a25e058-6701-430f-a1d1-729aa5e8058a")
                                                    )
                                                );
                                            }
                                            let value_c45bab0d = #ident_where_ucc::#field_ident_ucc(
                                                #import_path::PostgresTypeWhere::new(
                                                    #import_path::LogicalOperator::And,
                                                    value_2bbd2c96
                                                )
                                            );
                                            if !acc_2fe1cca8.contains(&value_c45bab0d) {
                                                acc_2fe1cca8.push(value_c45bab0d);
                                            }
                                        }
                                    }
                                });
                                quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_2fe1cca8 = Vec::new();
                                        #(#content_ts)*
                                        acc_2fe1cca8
                                    }) {
                                        Ok(value_a5fa471d) => Some(value_a5fa471d),
                                        Err(error) => match error {
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("89e719cf-3a6d-4250-95fc-237aaf46659b")
                                        }
                                    }
                                }
                            }
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_standart_not_null_ucc)
                        },
                        Pattern::Array => match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_ts_f0710cd9 = {
                                    let content_ts_57d244f8 = vec_syn_field.iter().map(|el_18682ae5| {
                                        let field_ident = &el_18682ae5.field_ident;
                                        let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_18682ae5.field_type);
                                        quote! {
                                            if let Some(value_bf84026e) = #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc(
                                                read_only_ids_420d38ca.0.#value_sc.#field_ident.clone(),
                                                create_76f032c1.#field_ident.clone()
                                            ) {
                                                for el_f6be06df in value_bf84026e.clone().into_vec() {
                                                    let value_592e6b5f = #ident_where_ucc::#el_field_ident_ucc(
                                                        #import_path::PostgresTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![el_f6be06df]
                                                        ).expect("1f7ae335-461f-4215-8fb5-ee7cf2f32881")
                                                    );
                                                    if !acc_dd377eb1.contains(&value_592e6b5f) {
                                                        acc_dd377eb1.push(value_592e6b5f);
                                                    }
                                                }
                                                let value_03205172 = #ident_where_ucc::#el_field_ident_ucc(
                                                    #import_path::PostgresTypeWhere::new(
                                                        #import_path::LogicalOperator::And,
                                                        value_bf84026e
                                                    )
                                                );
                                                if !acc_dd377eb1.contains(&value_03205172) {
                                                    acc_dd377eb1.push(value_03205172);
                                                }
                                            }
                                        }
                                    });
                                    quote!{#(#content_ts_57d244f8)*}
                                };
                                let content_ts_2cc4a40e = match &dimension {
                                    postgres_crud_macros_common::Dimension::One => {
                                        let dimension_one_ts = {
                                            let content_ts_91a09fe2 = vec_syn_field.iter().map(|el_a83927c8| {
                                                let field_ident = &el_a83927c8.field_ident;
                                                let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_a83927c8.field_type);
                                                quote! {
                                                    #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                        read_only_ids_420d38ca.0.#value_sc.#field_ident,
                                                        create_76f032c1.#field_ident
                                                    )
                                                }
                                            });
                                            quote!{
                                                acc_dd377eb1.push(#ident_where_ucc::DimensionOneEqual(#import_path::PostgresJsonTypeWhereDimensionOneEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    dimensions: #import_path::BoundedStdVecVec::try_from(
                                                        vec![
                                                            #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                                i32::try_from(index_47620dcf).expect("5341936f-ce9e-4e14-ae30-765f04c12e14")
                                                            ).expect("76906f3c-4472-4ac0-a605-1b02f02fd680")
                                                        ]
                                                    ).expect("8a624c70-3701-4907-b361-5637c5361e1f"),
                                                    #value_sc: #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                        <#uuid_uuid_as_not_null_jsonb_string_ts as #import_path::PostgresJsonTypeTestCases>::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                            read_only_ids_420d38ca.0.#value_sc.#id_sc,
                                                            #postgres_crud_default_option_some_vec_one_el_call_ts
                                                        ),
                                                        #(#content_ts_91a09fe2),*
                                                    ),
                                                }));
                                            }
                                        };
                                        quote!{
                                            for (index_47620dcf, (read_only_ids_420d38ca, create_76f032c1)) in #read_only_ids_sc.0.#value_sc.into_iter()
                                                .zip(#create_sc.0.into_iter())
                                                .enumerate()
                                            {
                                                #content_ts_f0710cd9
                                                #dimension_one_ts
                                            }
                                        }
                                    },
                                    postgres_crud_macros_common::Dimension::Two |
                                    postgres_crud_macros_common::Dimension::Three |
                                    postgres_crud_macros_common::Dimension::Four => quote!{
                                        for (read_only_ids_420d38ca, create_76f032c1) in #read_only_ids_sc.0.#value_sc.into_iter()
                                            .zip(#create_sc.0.into_iter())
                                        {
                                            #content_ts_f0710cd9
                                        }
                                    },
                                };
                                quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_dd377eb1 = Vec::new();
                                        #content_ts_2cc4a40e
                                        acc_dd377eb1
                                    }) {
                                        Ok(value_dfac36e4) => Some(value_dfac36e4),
                                        Err(error) => match error {
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("93390f1a-8860-4bf5-b01d-41a6cea3c494")
                                        },
                                    }
                                }
                            }
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_array_not_null_ucc)
                        },
                    }
                };
                (
                    {
                        let option_vec_create_ts = {
                            let content_ts = match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                    Pattern::Standart => {
                                        let content_ts = vec_syn_field.iter().map(|el_4f2f70d2| {
                                            let field_ident = &el_4f2f70d2.field_ident;
                                            let field_type_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_4f2f70d2.field_type);
                                            let parameters_ts = vec_syn_field.iter().map(|el_value| {
                                                let current_field_ident = &el_value.field_ident;
                                                if field_ident == current_field_ident {
                                                    quote! {el_37154498}
                                                } else {
                                                    quote! {
                                                        #postgres_crud_default_option_some_vec_one_el_call_ts
                                                    }
                                                }
                                            });
                                            quote! {
                                                if let Some(value_0296b347) = #field_type_type_as_postgres_json_type_test_cases_ts::#option_vec_create_sc() {
                                                    for el_37154498 in value_0296b347 {
                                                        let #value_sc = #self_as_postgres_json_type_create_ts::new(
                                                            #(#parameters_ts),*
                                                        );
                                                        if !acc_ccd79a32.contains(&#value_sc) {
                                                            acc_ccd79a32.push(#value_sc);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {#(#content_ts)*}
                                    },
                                    Pattern::Array => {
                                        let content_ts = vec_syn_field.iter().map(|el_ddefdb90| {
                                            let field_ident = &el_ddefdb90.field_ident;
                                            let field_type_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_ddefdb90.field_type);
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
                                                    el_ts: &dyn quote::ToTokens,
                                                |{
                                                    vec_syn_field.iter().map(|el_value| {
                                                        let current_field_ident = &el_value.field_ident;
                                                        if field_ident == current_field_ident {
                                                            let maybe_dot_clone_ts = match should_add_dot_clone.clone() {
                                                                ShouldAddDotClone::False => Ts2::new(),
                                                                ShouldAddDotClone::True => quote! { .clone() },
                                                            };
                                                            quote! {#el_ts #maybe_dot_clone_ts}
                                                        } else {
                                                            quote! {#postgres_crud_default_option_some_vec_one_el_call_ts}
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
                                                if let Some(vec_create) = #field_type_type_as_postgres_json_type_test_cases_ts::#option_vec_create_sc() {
                                                    let mut acc_6a886d56 = Vec::new();
                                                    let option_additional = {
                                                        let mut option_additional = None;
                                                        for el_37154498 in &vec_create {
                                                            if option_additional.is_none() {
                                                                let #value_sc = #ident_with_id_standart_not_null_create_ucc::new(
                                                                    #(#option_additional_parameters_ts),*
                                                                );
                                                                option_additional = Some((
                                                                    #ident_create_ucc::new(vec![#value_sc.clone()]),
                                                                    #ident_create_ucc::new(vec![#value_sc.clone(), #value_sc])
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
                                                        let value_07c0c08c = #ident_create_ucc::new(acc_6a886d56);
                                                        if !acc_ccd79a32.contains(&value_07c0c08c) {
                                                            acc_ccd79a32.push(value_07c0c08c);
                                                        }
                                                    }
                                                    if let Some(value_f6686d5d) = option_additional {
                                                        if has_len_greater_than_one {
                                                            let value_60116463 = value_f6686d5d.0;
                                                            if !acc_ccd79a32.contains(&value_60116463) {
                                                                acc_ccd79a32.push(value_60116463);
                                                            }
                                                        }
                                                        else {
                                                            let value_7a843059 = value_f6686d5d.1;
                                                            if !acc_ccd79a32.contains(&value_7a843059) {
                                                                acc_ccd79a32.push(value_7a843059);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {#(#content_ts)*}
                                    },
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let (
                                        current_ident_not_null_as_postgres_json_type_test_cases_ts,
                                        content_ts
                                    ): (
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens
                                    ) = match &pattern {
                                        Pattern::Standart => (
                                            &ident_standart_not_null_as_postgres_json_type_test_cases_ts,
                                            &Ts2::new()
                                        ),
                                        Pattern::Array => (
                                            &ident_array_not_null_as_postgres_json_type_test_cases_ts,
                                            &quote!{.0}
                                        ),
                                    };
                                    quote! {
                                        if let Some(value_399e6a50) = #current_ident_not_null_as_postgres_json_type_test_cases_ts::#option_vec_create_sc() {
                                            for el_e2767811 in value_399e6a50 {
                                                let #value_sc = #self_as_postgres_json_type_ts::Create::new(Some(el_e2767811 #content_ts));
                                                if !acc_ccd79a32.contains(&#value_sc) {
                                                    acc_ccd79a32.push(#value_sc);
                                                }
                                            }
                                        }
                                        {
                                            let #value_sc = #self_as_postgres_json_type_ts::Create::new(None);
                                            if !acc_ccd79a32.contains(&#value_sc) {
                                                acc_ccd79a32.push(#value_sc);
                                            }
                                        }
                                    }
                                }
                            };
                            quote!{Some({
                                let mut acc_ccd79a32 = Vec::new();
                                #content_ts
                                acc_ccd79a32
                            })}
                        };
                        let read_only_ids_to_two_dimensional_vec_read_inner_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_last_initialization_ts = {
                                        if vec_syn_field.len() == 1 {
                                            Ts2::new()
                                        }
                                        else {
                                            let content_ts = vec_syn_field.iter().map(|el_43e09b9b| {
                                                let field_ident = &el_43e09b9b.field_ident;
                                                let field_ident_last_sc = SelfLastSc::from_display(&field_ident);
                                                let field_type_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_43e09b9b.field_type);
                                                quote! {
                                                    let mut #field_ident_last_sc = #field_type_type_as_postgres_json_type_test_cases_ts::#read_only_ids_into_option_value_read_inner_sc(
                                                        read_only_ids.0.value.#field_ident.clone()
                                                    );
                                                }
                                            });
                                            quote!{#(#content_ts)*}
                                        }
                                    };
                                    let content_ts = vec_syn_field.iter().map(|el_9b199f7f| {
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
                                            let current_field_ident = &el_value.field_ident;
                                            let current_field_ident_current_sc = SelfCurrentSc::from_display(&current_field_ident);
                                            let current_field_ident_last_sc = SelfLastSc::from_display(&current_field_ident);
                                            let content_ts: &dyn quote::ToTokens = if field_ident == current_field_ident {
                                                &current_field_ident_current_sc
                                            } else {
                                                &current_field_ident_last_sc
                                            };
                                            quote! {#current_field_ident: #content_ts.clone()}
                                        });
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_9b199f7f.field_type);
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{el_2720df8a});
                                        quote! {
                                            for el_7bf83754 in #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_to_two_dimensional_vec_read_inner_sc(&#read_only_ids_sc.0.value.#field_ident) {
                                                for el_2720df8a in el_7bf83754 {
                                                    let #field_ident_current_sc = Some(#value_content_ts);
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
                                        #fields_last_initialization_ts
                                        #(#content_ts)*
                                        acc_ef081dc3
                                    }
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote! {
                                        #read_only_ids_sc.0.#value_sc.as_ref().into_iter().flat_map(|value_5fa0668c| {
                                            #ident_standart_not_null_as_postgres_json_type_test_cases_ts::
                                                #read_only_ids_to_two_dimensional_vec_read_inner_sc(value_5fa0668c)
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
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_ts = vec_syn_field.iter().map(|el_bb247316| {
                                        let field_ident = &el_bb247316.field_ident;
                                        let fields_ts = vec_syn_field.iter().map(|el_value| {
                                            let current_field_ident = &el_value.field_ident;
                                            if field_ident == current_field_ident {
                                                let value_content_ts = wrap_into_value_initialization_ts(&quote!{el_18d1f553});
                                                quote! {
                                                    #current_field_ident: Some(#value_content_ts)
                                                }
                                            } else {
                                                let current_field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_value.field_type);
                                                quote! {
                                                    #current_field_ident: #current_field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_into_option_value_read_inner_sc(
                                                        el_49a5bb62.0.#value_sc.#current_field_ident.clone()
                                                    )
                                                }
                                            }
                                        });
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_bb247316.field_type);
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{el_49a5bb62.0.#value_sc.#id_sc.0.#value_sc});
                                        quote! {
                                            for el_4b4da5aa in #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_to_two_dimensional_vec_read_inner_sc(
                                                &el_49a5bb62.0.#value_sc.#field_ident.clone()
                                            ) {
                                                for el_18d1f553 in el_4b4da5aa {
                                                    acc_00b3df88.push(
                                                        vec![
                                                            #ident_with_id_standart_not_null_read_inner_ucc {
                                                                #id_sc: Some(#value_content_ts),
                                                                #(#fields_ts),*
                                                            }
                                                        ]
                                                    );
                                                }
                                            }
                                        }
                                    });
                                    quote! {
                                        #read_only_ids_sc.0.#value_sc.iter().map(|el_49a5bb62|{
                                            let mut acc_00b3df88 = Vec::new();
                                            #(#content_ts)*
                                            acc_00b3df88
                                        })
                                        .collect()
                                    }
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote! {
                                        let mut acc_fb5111f1 = Vec::new();
                                        if let Some(value_6ee5750e) = &#read_only_ids_sc.0.#value_sc {
                                            for el_4a5a4b09 in #ident_array_not_null_as_postgres_json_type_test_cases_ts::#read_only_ids_to_two_dimensional_vec_read_inner_sc(value_6ee5750e) {
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
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let self_el_as_postgres_type_read_ts = gen_type_as_postgres_type_subtype_ts(&self_postgres_json_type_ts, &PostgresTypeSubtype::Read);
                                    let parameters_ts = vec_syn_field.iter().map(|el_13640e7f| {
                                        let field_ident = &el_13640e7f.field_ident;
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_13640e7f.field_type);
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_inner_into_read_with_new_or_try_new_unwraped_sc(value_8ff65e09.#value_sc)
                                        });
                                        quote! {#value_sc.#field_ident.map(|value_8ff65e09|#value_content_ts)}
                                    });
                                    quote! {#self_el_as_postgres_type_read_ts::try_new(#(#parameters_ts),*).expect("3aeeabba-3ffc-4df2-a3bf-e389c40ec566")}
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let self_el_as_postgres_type_read_ts = gen_type_as_postgres_type_subtype_ts(&self_postgres_json_type_ts, &PostgresTypeSubtype::Read);
                                    quote! {
                                        #self_el_as_postgres_type_read_ts::new(
                                            #value_sc.map(#ident_standart_not_null_as_postgres_json_type_test_cases_ts::#read_inner_into_read_with_new_or_try_new_unwraped_sc)
                                        )
                                    }
                                }
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_ts = vec_syn_field_with_id.iter().map(|el_96f7b50a| {
                                        let field_ident = &el_96f7b50a.field_ident;
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_96f7b50a.field_type);
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_inner_into_read_with_new_or_try_new_unwraped_sc(value_3ac52220.#value_sc)
                                        });
                                        quote! {#field_ident: el_ffed1bfc.#field_ident.map(|value_3ac52220|#value_content_ts)}
                                    });
                                    quote!{
                                        #ident_read_ucc::new(
                                            #value_sc.into_iter().map(|el_ffed1bfc| #ident_with_id_standart_not_null_read_ucc {
                                                #(#content_ts),*
                                            }).collect()
                                        )
                                    }
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_ts = vec_syn_field_with_id.iter().map(|el_e47b9709| {
                                        let field_ident = &el_e47b9709.field_ident;
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_e47b9709.field_type);
                                        // let maybe_dot_clone_ts = if vec_syn_field.len() == 1 {
                                        //     Ts2::new()
                                        // }
                                        // else {
                                        //     quote!{.clone()}
                                        // };
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_inner_into_read_with_new_or_try_new_unwraped_sc(
                                                el_5c1f7f63.#value_sc
                                                // #maybe_dot_clone_ts
                                                .clone()
                                            )
                                        });
                                        quote! {
                                            #field_ident: el_ffed1bfc.#field_ident.as_ref().map(|el_5c1f7f63| #value_content_ts)
                                        }
                                    });
                                    let self_el_as_postgres_type_read_ts = gen_type_as_postgres_type_subtype_ts(&self_postgres_json_type_ts, &PostgresTypeSubtype::Read);
                                    quote! {
                                        #self_el_as_postgres_type_read_ts::new(
                                            #value_sc.map(|value_189e3c07|
                                                value_189e3c07
                                                .into_iter()
                                                .map(|el_ffed1bfc|#ident_with_id_standart_not_null_read_ucc {
                                                    #(#content_ts),*
                                                }).collect()
                                            )
                                        )
                                    }
                                }
                            },
                        };
                        let read_inner_into_update_with_new_or_try_new_unwraped_ts = match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                Pattern::Standart => {
                                    let self_el_as_postgres_type_update_ts = gen_type_as_postgres_type_subtype_ts(&self_postgres_json_type_ts, &PostgresTypeSubtype::Update);
                                    let parameters_ts = vec_syn_field.iter().map(|el_cefffeeb| {
                                        let field_ident = &el_cefffeeb.field_ident;
                                        let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_cefffeeb.field_type);
                                        let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_inner_into_update_with_new_or_try_new_unwraped_sc(el_23bdfe1e.#value_sc)
                                        });
                                        quote! {
                                            acc_ebea163e.extend(#value_sc.#field_ident.map(|el_23bdfe1e| {
                                                #ident_standart_not_null_update_el_ucc::#field_ident_ucc(#value_content_ts)
                                            }));
                                        }
                                    });
                                    quote! {
                                        #self_el_as_postgres_type_update_ts::new(
                                            #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_ebea163e = Vec::new();
                                                #(#parameters_ts)*
                                                acc_ebea163e
                                            }).expect("a06dbdc5-296c-48a8-ba00-913e1fe82ebf")
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
                                                #value_sc.into_iter().map(|el_ffed1bfc| #ident_with_id_standart_not_null_update_el_ucc {
                                                    #id_sc: #uuid_uuid_as_not_null_jsonb_string_update_ucc::new(el_ffed1bfc.#id_sc.clone().expect("f04a2c6d-bc0b-4693-b7e5-ede348cb229e").#value_sc),
                                                    fields: #ident_standart_not_null_as_postgres_json_type_test_cases_ts::#read_inner_into_update_with_new_or_try_new_unwraped_sc(
                                                        #ident_standart_not_null_read_inner_ucc {
                                                            #(#fields_ts),*
                                                        }
                                                    ),
                                                })
                                                .collect(),
                                            )
                                            .expect("ca51d559-d3d1-4855-8d9a-0f799cccd3e4"),
                                            Vec::new(),
                                        )
                                        .expect("0449fe82-4acc-4c83-9753-18f313b278d1")
                                    }
                                }
                            },
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_ts = gen_type_as_postgres_type_test_cases_ts(match &pattern {
                                    Pattern::Standart => &ident_standart_not_null_ucc,
                                    Pattern::Array => &ident_with_id_array_not_null_ucc,
                                });
                                let self_el_as_postgres_type_update_ts = gen_type_as_postgres_type_subtype_ts(&self_postgres_json_type_ts, &PostgresTypeSubtype::Update);
                                quote! {
                                    #self_el_as_postgres_type_update_ts::new(
                                        #value_sc.map(#content_ts::#read_inner_into_update_with_new_or_try_new_unwraped_sc)
                                    )
                                }
                            },
                        };
                        let read_only_ids_into_option_value_read_inner_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => gen_fields_read_only_ids_into_option_value_read_inner_ts(&is_standart_with_id_false, &value_sc),
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                        #value_sc.0.#value_sc.and_then(|value_5d7e3961| match #ident_standart_not_null_as_postgres_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                                            value_5d7e3961
                                        ) {
                                            Some(value_cfca0099) => Some(value_cfca0099.#value_sc),
                                            None => None,
                                        })
                                    });
                                    quote! {Some(#value_content_ts)}
                                }
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&{
                                        let content_ts = vec_syn_field_with_id.iter().map(|el_3ebdd830| {
                                            let field_ident = &el_3ebdd830.field_ident;
                                            let field_type = &el_3ebdd830.field_type;
                                            let field_type_as_postgres_json_type_ts = gen_type_as_postgres_json_type_ts(&field_type);
                                            let field_type_as_postgres_json_type_read_ts = gen_type_as_postgres_json_type_subtype_ts(&field_type, &PostgresJsonTypeSubtype::Read);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            let value_content_ts = wrap_into_value_initialization_ts(&{
                                                let default_but_option_is_some_call_ts_a3f714b3 = gen_ident_as_default_but_option_is_some_call_ts(
                                                    &field_type_as_postgres_json_type_read_ts
                                                );
                                                quote!{#field_type_as_postgres_json_type_ts::into_inner(#default_but_option_is_some_call_ts_a3f714b3)}
                                            });
                                            quote! {
                                                #field_ident: #field_type_as_postgres_json_type_test_cases_ts::read_only_ids_into_option_value_read_inner(
                                                    el_6603f209.0.#value_sc.#field_ident
                                                ).map_or_else(|| Some(#value_content_ts), Some)
                                            }
                                        });
                                        quote!{
                                            #value_sc.0.#value_sc.into_iter().fold(Vec::new(), |mut acc_cf4743b1, el_6603f209| {
                                                acc_cf4743b1.push(#ident_with_id_standart_not_null_read_inner_ucc {
                                                    #(#content_ts),*
                                                });
                                                acc_cf4743b1
                                            })
                                        }
                                    });
                                    quote! {Some(#value_content_ts)}
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                        #value_sc.0.#value_sc.and_then(|value_f816032d| match #ident_array_not_null_as_postgres_json_type_test_cases_ts::#read_only_ids_into_option_value_read_inner_sc(
                                            value_f816032d
                                        ) {
                                            Some(value_d0549423) => Some(value_d0549423.#value_sc),
                                            None => None,
                                        })
                                    });
                                    quote! {Some(#value_content_ts)}
                                }
                            },
                        };
                        let update_to_read_only_ids_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_initialization_content_ts = vec_syn_field.iter().map(|el_3f07f901| {
                                        let field_ident = &el_3f07f901.field_ident;
                                        quote! {let mut #field_ident = None;}
                                    });
                                    let match_content_ts = vec_syn_field.iter().map(|el_4fb1f3d0| {
                                        let field_ident = &el_4fb1f3d0.field_ident;
                                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_4fb1f3d0.field_type);
                                        quote! {
                                            #ident_update_el_ucc::#field_ident_ucc_ts(value_0f4d677e) => {
                                                #field_ident = Some(#field_type_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc(&value_0f4d677e.#value_sc));
                                            }
                                        }
                                    });
                                    let struct_fields_content_ts = vec_syn_field.iter().map(|el_af4d3d80| {
                                        let field_ident = &el_af4d3d80.field_ident;
                                        quote! {#field_ident: #field_ident.expect("106f16f2-ae03-48b3-9239-f4b1b60746d5")}
                                    });
                                    let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                        #ident_read_only_ids_handle_ucc{
                                            #(#struct_fields_content_ts),*
                                        }
                                    });
                                    quote! {
                                        #(#fields_initialization_content_ts)*
                                        for el_b3974846 in #value_sc.0.to_vec() {
                                            match el_b3974846 {
                                                #(#match_content_ts),*
                                            }
                                        }
                                        #ident_read_only_ids_ucc(#value_content_ts)
                                    }
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&{
                                        quote!{
                                            #value_sc.0.as_ref().map(#ident_standart_not_null_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc)
                                        }
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_content_ts)}
                                }
                            },
                            Pattern::Array => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&{
                                        let initialization_ts = vec_syn_field.iter().map(|el_09cee28c| {
                                            let field_ident = &el_09cee28c.field_ident;
                                            quote! {let mut #field_ident = None;}
                                        });
                                        let for_loop_ts = vec_syn_field.iter().map(|el_cf4923ce| {
                                            let field_ident = &el_cf4923ce.field_ident;
                                            let field_ident_ts = {
                                                let current_field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                                quote!{
                                                    #ident_standart_not_null_update_el_ucc::#current_field_ident_ucc_ts(value_d2a6daf8) => {
                                                        #field_ident = Some(value_d2a6daf8.#value_sc.clone());
                                                    },
                                                }
                                            };
                                            let fields_without_current_ident_ts = if vec_syn_field.is_empty() {
                                                Ts2::new()
                                            }
                                            else {
                                                let content_ts_e0bf4e67 = vec_syn_field
                                                .iter()
                                                .filter(|el_a1502880| el_a1502880.field_ident != *field_ident)
                                                .map(|el_2908bd7a| {
                                                    let current_field_ident = &el_2908bd7a.field_ident;
                                                    let current_field_ident_ucc_ts =
                                                        ToTokensToUccTs::case_or_panic(
                                                            &current_field_ident,
                                                        );
                                                    quote! {
                                                        #ident_standart_not_null_update_el_ucc::#current_field_ident_ucc_ts(_)
                                                    }
                                                })
                                                .fold(None, |acc_bbf653f7, el_2be3f4ee| Some(match acc_bbf653f7 {
                                                    None => el_2be3f4ee,
                                                    Some(value_5b375af0) => quote! { #value_5b375af0 | #el_2be3f4ee },
                                                }));
                                                content_ts_e0bf4e67.map_or_else(
                                                    Ts2::new,
                                                    |value_5c826b8c| quote!{#value_5c826b8c => (),}
                                                )
                                            };
                                            quote! {
                                                for el_da177c5a in el_4634bb8a.fields.0.to_vec() {
                                                    match &el_da177c5a {
                                                        #field_ident_ts
                                                        #fields_without_current_ident_ts
                                                    }
                                                }
                                            }
                                        });
                                        let value_content_ts = wrap_into_value_initialization_ts(&{
                                            let uuid_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
                                            let fields_ts = vec_syn_field.iter().map(|el_134779da| {
                                                let field_ident = &el_134779da.field_ident;
                                                let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_134779da.field_type);
                                                quote! {
                                                    #field_ident: #field_type_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc(&#field_ident.expect("a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d"))
                                                }
                                            });
                                            quote!{
                                                #ident_with_id_standart_not_null_read_only_ids_handle_ucc {
                                                    #id_sc: #uuid_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc(&el_4634bb8a.#id_sc),
                                                    #(#fields_ts),*
                                                }
                                            }
                                        });
                                        quote!{
                                            #value_sc.#update_sc.to_vec().iter().map(|el_4634bb8a|{
                                                #(#initialization_ts)*
                                                #(#for_loop_ts)*
                                                #ident_with_id_standart_not_null_read_only_ids_ucc(#value_content_ts)
                                            }).collect()
                                        }
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_content_ts)}
                                }
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_ts = wrap_into_value_initialization_ts(&quote!{
                                        #value_sc.0.as_ref().map(#ident_array_not_null_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc)
                                    });
                                    quote! {#ident_read_only_ids_ucc(#value_content_ts)}
                                }
                            },
                        };
                        let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = {
                            let value_initialization_ts = gen_import_path_value_initialization_ts(&match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_2b018c89| {
                                            let field_ident = &el_2b018c89.field_ident;
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_2b018c89.field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                    &#value_sc.0.#value_sc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::try_new(
                                                #(#parameters_ts),*
                                            ).expect("57820868-38ac-4f77-aa0f-dc734399d8b2")
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                        #ident_read_ucc::new(
                                            #value_sc.0.#value_sc.as_ref().and_then(|value_dfa7815e| match #ident_standart_not_null_as_postgres_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                value_dfa7815e
                                            ) {
                                                Some(value_02cef266) => Some(value_02cef266.#value_sc),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_ts = vec_syn_field_with_id.iter().map(|el_f4599b96| {
                                            let field_ident = &el_f4599b96.field_ident;
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_f4599b96.field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                    &el_629b1544.0.#value_sc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::new({
                                                let mut acc_5f587d35 = #value_sc.0.#value_sc.clone().into_iter().map(|el_629b1544|{
                                                    #ident_with_id_standart_not_null_read_ucc::try_new(
                                                        #(#parameters_ts),*
                                                    ).expect("8f6fb6b6-6e84-4819-b9c6-526d39e1ac88")
                                                }).collect::<Vec<#ident_with_id_standart_not_null_read_ucc>>();
                                                acc_5f587d35.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.id, &second.id) {
                                                        //maybe remove .clone - add .get by ref method
                                                        #import_path_postgres_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_ts::into_inner(
                                                            value_first.#value_sc.clone()
                                                        )
                                                        .cmp(&#import_path_postgres_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_ts::into_inner(
                                                            value_second.#value_sc.clone()
                                                        ))
                                                    }
                                                    else {
                                                        panic!("0bdf0f44-0012-4cda-9a27-3a89804d871b");
                                                    }
                                                });
                                                acc_5f587d35
                                            })
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => quote! {
                                        #ident_read_ucc::new(
                                            #value_sc.0.#value_sc.as_ref().and_then(|value_16ab4136| match #ident_array_not_null_as_postgres_json_type_test_cases_ts::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                value_16ab4136
                                            ) {
                                                Some(value_71a66429) => Some(value_71a66429.#value_sc.0),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                            });
                            quote!{Some(#value_initialization_ts)}
                        };
                        let previous_read_merged_with_option_update_into_read_ts = {
                            let fields_initialization_content_ts = vec_syn_field.iter().map(|el_8caae90c| {
                                let field_ident = &el_8caae90c.field_ident;
                                quote! {let mut #field_ident = None;}
                            });
                            let match_content_ts = vec_syn_field.iter().map(|el_b625a4b0| {
                                let field_ident = &el_b625a4b0.field_ident;
                                let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                quote! {
                                    #ident_standart_not_null_update_el_ucc::#field_ident_ucc_ts(#value_sc) => {
                                        #field_ident = Some(#value_sc.#value_sc);
                                    }
                                }
                            });
                            let gen_struct_initialization_ts = |function: &dyn Fn(&dyn quote::ToTokens) -> Ts2|{//content_ts: &dyn quote::ToTokens
                                let ts = vec_syn_field.iter().map(|el_96e0a086| {
                                    let field_ident = &el_96e0a086.field_ident;
                                    let value_initialization_ts = gen_import_path_value_initialization_ts(&{
                                        let content_ts = function(&field_ident);
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_96e0a086.field_type);
                                        quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::previous_read_merged_with_option_update_into_read(
                                                #content_ts,
                                                #field_ident
                                            )
                                        }
                                    });
                                    quote! {#field_ident: Some(#value_initialization_ts)}
                                });
                                quote!{#(#ts),*}
                            };
                            let gen_option_ts = |current_pattern: &Pattern|{
                                let current_ident_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(match &current_pattern {
                                    Pattern::Standart => &ident_standart_not_null_ucc,
                                    Pattern::Array => &ident_array_not_null_ucc
                                });
                                quote! {
                                    match #option_update_sc {
                                        Some(value_fca601b5) => #ident_read_ucc(
                                            match value_fca601b5.0 {
                                                Some(value_8d7747f1) => Some(
                                                    #current_ident_as_postgres_json_type_test_cases_ts::previous_read_merged_with_option_update_into_read(
                                                        #read_sc.0.unwrap_or_else(#default_but_option_is_some_ts),
                                                        Some(value_8d7747f1),
                                                    )
                                                ),
                                                None => None,
                                            }
                                        ),
                                        None => #read_sc,
                                    }
                                }
                            };
                            match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                    Pattern::Standart => {
                                        let struct_initializattion_ts = gen_struct_initialization_ts(&|content_ts: &dyn quote::ToTokens|{
                                            quote!{
                                                #read_sc.#content_ts.expect("a2d26e36-48f9-4739-aabe-adc0f0593570").#value_sc
                                            }
                                        });
                                        quote!{
                                            match #option_update_sc {
                                                Some(value_e5377436) => {
                                                    #(#fields_initialization_content_ts)*
                                                    for el_629b1544 in value_e5377436.0.into_vec() {
                                                        match el_629b1544 {
                                                            #(#match_content_ts),*
                                                        }
                                                    }
                                                    #ident_read_ucc {
                                                        #struct_initializattion_ts
                                                    }
                                                },
                                                None => #read_sc
                                            }
                                        }
                                    },
                                    Pattern::Array => {
                                        let struct_initializattion_ts = gen_struct_initialization_ts(&|content_ts: &dyn quote::ToTokens|{
                                            quote!{
                                                found_read_element.#content_ts.expect("2e8229f7-38d6-48c1-93c9-e77631a3e155").#value_sc
                                            }
                                        });
                                        quote! {
                                            match #option_update_sc {
                                                Some(value_47f5a20b) => #ident_read_ucc({
                                                    let mut acc_04a67ef2 = Vec::new();
                                                    for el_377d1bb4 in value_47f5a20b.#update_sc.into_vec() {
                                                        let mut option_read_element = None;
                                                        for el_d2daee30 in &#read_sc.0 {
                                                            if *#uuid_uuid_as_not_null_jsonb_string_as_postgres_json_type_object_vec_el_id_ts::get_inner(&el_377d1bb4.#id_sc.clone().into())
                                                            ==
                                                            #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgres_json_type_ts::into_inner(
                                                                el_d2daee30.#id_sc.clone().expect("df2413fe-e703-451b-ab75-add67da716f7").#value_sc
                                                            )
                                                            {
                                                                option_read_element = Some(el_d2daee30.clone());
                                                                break;
                                                            }
                                                        }
                                                        let found_read_element = option_read_element.expect("139882b9-d38f-4cb5-98ea-af0ab23ec474");
                                                        #(#fields_initialization_content_ts)*
                                                        for el_629b1544 in el_377d1bb4.fields.0.into_vec() {
                                                            match el_629b1544 {
                                                                #(#match_content_ts),*
                                                            }
                                                        }
                                                        acc_04a67ef2.push(#ident_with_id_standart_not_null_read_ucc {
                                                            #id_sc: found_read_element.#id_sc,
                                                            #struct_initializattion_ts
                                                        });
                                                    }
                                                    acc_04a67ef2
                                                }),
                                                None => #read_sc
                                            }
                                        }
                                    },
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_option_ts(pattern)
                            }
                        };
                        let read_only_ids_merged_with_create_into_read_ts = {
                            let gen_nullable_ts = |current_ident_ts: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens|{
                                let current_ident_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&current_ident_ts);
                                quote! {
                                    #ident_read_ucc::new(
                                        match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                            (Some(read_only_ids_2b2ab8a1), Some(create_4a1adaa3)) => {
                                                Some(
                                                    #current_ident_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_option_value_read_sc(
                                                        read_only_ids_2b2ab8a1,
                                                        create_4a1adaa3
                                                    ).expect("56ac4450-0feb-4ea7-aca7-6f51c8f4893c").#value_sc #content_ts
                                                )
                                            },
                                            (Some(_), None) => panic!("75be9ae0-ca9f-4251-bfff-2156a90b10c6"),
                                            (None, Some(_)) => panic!("6a95d7ae-54f5-4e04-9217-223ba156b799"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_9bdce5ca| {
                                            let field_ident = &el_9bdce5ca.field_ident;
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_9bdce5ca.field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_option_value_read_sc(
                                                    #read_only_ids_sc.0.#value_sc.#field_ident,
                                                    #create_sc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_read_ucc::try_new(
                                                #(#parameters_ts),*
                                            ).expect("52ad3994-8392-4fc5-8427-4ca42d87d726")
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(
                                        &ident_standart_not_null_ucc,
                                        &Ts2::new()
                                    )
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let gen_parameter_ts = |field_type: &dyn quote::ToTokens, field_ident: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens|{
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_option_value_read_sc(
                                                    read_only_ids_225e2b76.0.#value_sc.#field_ident,
                                                    #content_ts,
                                                )
                                            }
                                        };
                                        let id_parameter_ts = gen_parameter_ts(
                                            &uuid_uuid_as_not_null_jsonb_string_ts,
                                            &id_sc,
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
                                                assert_eq!(#read_only_ids_sc.0.#value_sc.len(), #create_sc.0.len(), "90d33ddd-e08d-488c-8577-c75febe97301");
                                                let mut acc_37909420 = Vec::new();
                                                for (read_only_ids_225e2b76, create_3c660445) in #read_only_ids_sc.0.#value_sc.into_iter().zip(#create_sc.0.into_iter()) {
                                                    acc_37909420.push(#ident_with_id_standart_not_null_read_ucc::try_new(
                                                        #id_parameter_ts,
                                                        #(#parameters_ts),*
                                                    ).expect("1330ac8d-ebf2-4c79-b25e-6394d58de927"));
                                                }
                                                acc_37909420
                                            })
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(
                                        &ident_array_not_null_ucc,
                                        &quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_option_value_read_ts = {
                            let value_initialization_ts = gen_import_path_value_initialization_ts(&quote!{
                                <#self_ucc as #import_path::PostgresJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_sc(
                                    #read_only_ids_sc,
                                    #create_sc
                                )
                            });
                            quote!{Some(#value_initialization_ts)}
                        };
                        let read_only_ids_merged_with_create_into_table_type_declaration_ts = {
                            let gen_nullable_ts = |current_ident_ts: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens|{
                                let current_ident_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&current_ident_ts);
                                quote! {
                                    #ident_table_type_declaration_ucc::new(
                                        match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                            (Some(read_only_ids_fb2ec2e4), Some(create_2f615d4f)) => {
                                                Some(
                                                    #current_ident_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                        read_only_ids_fb2ec2e4,
                                                        create_2f615d4f
                                                    ) #content_ts
                                                )
                                            },
                                            (Some(_), None) => panic!("9349dcd5-3ed3-4157-b1ef-14c51d55262f"),
                                            (None, Some(_)) => panic!("45f8e70a-ffca-41b6-ac70-96f101ac3c80"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_ts = vec_syn_field.iter().map(|el_ca3a96db| {
                                            let field_ident = &el_ca3a96db.field_ident;
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_ca3a96db.field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                    #read_only_ids_sc.0.#value_sc.#field_ident,
                                                    #create_sc.#field_ident
                                                )
                                            }
                                        });
                                        quote! {
                                            #ident_table_type_declaration_ucc::new(
                                                #(#parameters_ts),*
                                            )
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(
                                        &ident_standart_not_null_ucc,
                                        &Ts2::new()
                                    )
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let gen_parameter_ts = |field_type: &dyn quote::ToTokens, field_ident: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens|{
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                    read_only_ids_94b49496.0.#value_sc.#field_ident,
                                                    #content_ts,
                                                )
                                            }
                                        };
                                        let id_parameter_ts = gen_parameter_ts(
                                            &uuid_uuid_as_not_null_jsonb_string_ts,
                                            &id_sc,
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
                                                assert_eq!(#read_only_ids_sc.0.#value_sc.len(), #create_sc.0.len(), "7776a146-06a8-4972-8e16-371d41ee164c");
                                                let mut acc_319e1fb1 = Vec::new();
                                                for (read_only_ids_94b49496, create_24629087) in #read_only_ids_sc.0.#value_sc.into_iter().zip(#create_sc.0.into_iter()) {
                                                    acc_319e1fb1.push(#ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                        #id_parameter_ts,
                                                        #(#parameters_ts),*
                                                    ));
                                                }
                                                acc_319e1fb1
                                            })
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(
                                        &ident_array_not_null_ucc,
                                        &quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_where_equal_ts = match &not_null_or_nullable {
                            postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                Pattern::Standart => {
                                    let parameters_ts = vec_syn_field.iter().map(|el_9990b32d| {
                                        let field_ident = &el_9990b32d.field_ident;
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_9990b32d.field_type);
                                        quote! {
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                #read_only_ids_sc.0.#value_sc.#field_ident,
                                                #create_sc.#field_ident
                                            )
                                        }
                                    });
                                    quote! {
                                        #ident_where_ucc::#equal_ucc(
                                            #import_path::PostgresJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                #value_sc: #ident_table_type_declaration_ucc::new(
                                                    #(#parameters_ts),*
                                                )
                                            }
                                        )
                                    }
                                },
                                Pattern::Array => {
                                    let gen_read_only_ids_merged_with_create_into_table_type_declaration_ts = |
                                        field_ident: &dyn quote::ToTokens,
                                        field_type: &dyn quote::ToTokens,
                                        content_ts: &dyn quote::ToTokens
                                    |{
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                        quote!{
                                            #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                read_only_ids_ea32954c.0.#value_sc.#field_ident,
                                                #content_ts
                                            )
                                        }
                                    };
                                    let current_ident_ts = gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                        &id_sc,
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
                                        #ident_where_ucc::#equal_ucc(
                                            #import_path::PostgresJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::And,
                                                #value_sc: #ident_table_type_declaration_ucc::new({
                                                    let mut acc_321b3fcd = Vec::new();
                                                    for (read_only_ids_ea32954c, create_3cbe8967) in #read_only_ids_sc.0.#value_sc.into_iter().zip(#create_sc.0.into_iter()) {
                                                        acc_321b3fcd.push(
                                                            #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                                #current_ident_ts,
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
                            postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_ts = {
                                    let current_ident_ts = gen_type_as_postgres_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                        Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote!{
                                        vec![
                                            #current_ident_ts::#read_only_ids_merged_with_create_into_where_equal_sc(
                                                read_only_ids_ce30c0fe,
                                                create_8fd81ed8
                                            )
                                        ]
                                    }
                                };
                                quote! {
                                    #import_path::NullableJsonObjectPostgresTypeWhereFilter(
                                        match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                            (Some(read_only_ids_ce30c0fe), Some(create_8fd81ed8)) => match #import_path::NotEmptyUniqueVec::try_new(#content_ts) {
                                                Ok(value_7a9cd49b) => Some(value_7a9cd49b),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("463769fc-19da-49dc-9b79-8f6ed360fd2b")
                                                }
                                            },
                                            (Some(_), None) => panic!("1a2b314c-289e-4dc7-bec8-654c60966abf"),
                                            (None, Some(_)) => panic!("9faea0f9-78ef-4241-98fc-2acde83d07ce"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            },
                        };
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = {
                            let content_ts = match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                    Pattern::Standart => {
                                        let elements_ts = vec_syn_field.iter().map(|el_23a78055| {
                                            let field_ident = &el_23a78055.field_ident;
                                            let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_23a78055.field_type);
                                            quote! {
                                                #ident_where_ucc::#field_ident_ucc_ts(
                                                    #import_path::PostgresTypeWhere::new(
                                                        #import_path::LogicalOperator::And,
                                                        #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                                                            #read_only_ids_sc.0.#value_sc.#field_ident,
                                                            #create_sc.#field_ident
                                                        )
                                                    )
                                                )
                                            }
                                        });
                                        quote! {#(#elements_ts),*}
                                    },
                                    Pattern::Array => {
                                        let gen_read_only_ids_merged_with_create_into_table_type_declaration_ts = |
                                            field_ident: &dyn quote::ToTokens,
                                            field_type: &dyn quote::ToTokens,
                                            content_ts: &dyn quote::ToTokens
                                        |{
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            quote!{
                                                #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                                                    read_only_ids_319c9e78.0.#value_sc.#field_ident,
                                                    #content_ts
                                                )
                                            }
                                        };
                                        let current_ident_ts = gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
                                            &id_sc,
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
                                            #ident_where_ucc::#equal_ucc(
                                                #import_path::PostgresJsonTypeWhereEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    #value_sc: #ident_table_type_declaration_ucc::new({
                                                        let mut acc_97ebf7d6 = Vec::new();
                                                        for (read_only_ids_319c9e78, create_00ae06d2) in #read_only_ids_sc.0.#value_sc.into_iter().zip(#create_sc.0.into_iter()) {
                                                            acc_97ebf7d6.push(
                                                                #ident_with_id_standart_not_null_table_type_declaration_ucc::new(
                                                                    #current_ident_ts,
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
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_ts = {
                                        let current_ident_ts = gen_type_as_postgres_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                            Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                            Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                        }));
                                        quote! {
                                            #current_ident_ts::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                                                read_only_ids_2898c440,
                                                create_f1c4667c
                                            )
                                        }
                                    };
                                    quote! {
                                        #import_path::NullableJsonObjectPostgresTypeWhereFilter(
                                            match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                                (Some(read_only_ids_2898c440), Some(create_f1c4667c)) => Some(#content_ts),
                                                (Some(_), None) => panic!("49e4c289-b37d-4365-96e3-5d896d6860f7"),
                                                (None, Some(_)) => panic!("ad71caa2-2503-4f9a-952c-e796abf5bbbe"),
                                                (None, None) => None,
                                            }
                                        )
                                    }
                                },
                            };
                            quote!{
                                #import_path::NotEmptyUniqueVec::try_new(vec![
                                    #content_ts
                                ]).expect("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
                            }
                        };
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = match &pattern {
                            Pattern::Standart => match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_ts = vec_syn_field.iter().map(|el_459c3da8| {
                                        let field_ident = &el_459c3da8.field_ident;
                                        let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                        let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_459c3da8.field_type);
                                        quote! {
                                            for el_d830c061 in #field_type_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_sc(
                                                #read_only_ids_sc.0.#value_sc.#field_ident,
                                                #create_sc.#field_ident
                                            ).into_vec() {
                                                acc_89ec072c.push(
                                                    #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PostgresTypeWhere::try_new(
                                                            #import_path::LogicalOperator::Or,
                                                            vec![el_d830c061],
                                                        )
                                                        .expect("0c6ccad1-6ffc-451f-9b16-0731010fee9f"),
                                                    )
                                                );
                                            }
                                        }
                                    });
                                    quote!{
                                        #import_path::NotEmptyUniqueVec::try_new({
                                            let mut acc_89ec072c = Vec::new();
                                            #(#content_ts)*
                                            acc_89ec072c
                                        }).expect("9c50391c-001e-4a4f-aac0-14bb614de456")
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => quote!{
                                    #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_12b6f16d = Vec::new();
                                        match (#read_only_ids_sc.0.#value_sc, #create_sc.0) {
                                            (Some(read_only_ids_2f024927), Some(create_120c1dad)) => {
                                                for el_a8b181a0 in #ident_standart_not_null_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_sc(
                                                    read_only_ids_2f024927,
                                                    create_120c1dad
                                                ).into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                                        Ok(value_8e72cfd7) => {
                                                            acc_12b6f16d.push(#import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_8e72cfd7)));
                                                        },
                                                        Err(error) => match error {
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("2a88b17f-cf3d-4793-a221-d6fc5922b218")
                                                        }
                                                    }
                                                }
                                            },
                                            (Some(_), None) => panic!("b4507b4c-5282-4d91-9a50-190b2d789849"),
                                            (None, Some(_)) => panic!("8f458c1d-a286-404f-b3b7-cd8f7b4c8bed"),
                                            (None, None) => {
                                                acc_12b6f16d.push(#import_path::NullableJsonObjectPostgresTypeWhereFilter(None));
                                            },
                                        }
                                        acc_12b6f16d
                                    }).expect("7efc9aae-4b7c-4821-b916-72f5eb2fbd6b")
                                }
                            },
                            Pattern::Array => quote!{
                                #self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                                    #read_only_ids_sc,
                                    #create_sc
                                )
                            }
                        };
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts = gen_dimension_equal_ts(&postgres_crud_macros_common::Dimension::One);
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts = gen_dimension_equal_ts(&postgres_crud_macros_common::Dimension::Two);
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts = gen_dimension_equal_ts(&postgres_crud_macros_common::Dimension::Three);
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts = gen_dimension_equal_ts(&postgres_crud_macros_common::Dimension::Four);
                        let create_into_postgres_json_type_option_vec_where_length_equal_ts = {
                            let gen_nullable_ts = |content_ts: &dyn quote::ToTokens|quote! {
                                match #import_path::NotEmptyUniqueVec::try_new(
                                    match #create_sc.0 {
                                        Some(create_09a81dae) => match <
                                            #content_ts
                                            as
                                            #import_path::PostgresJsonTypeTestCases
                                        >::#create_into_postgres_json_type_option_vec_where_length_equal_sc(create_09a81dae) {
                                            Some(value_3680a4c9) => {
                                                let mut acc_5c441d3a = Vec::new();
                                                for el_a8b181a0 in value_3680a4c9.clone().into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                                        Ok(value_15097b27) => {
                                                            acc_5c441d3a.push(
                                                                #import_path::NullableJsonObjectPostgresTypeWhereFilter(
                                                                    Some(value_15097b27)
                                                                )
                                                            );
                                                        },
                                                        Err(error) => match error {
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("6c4da72e-e16c-4c17-a939-9a52195e89a9")
                                                        }
                                                    }
                                                }
                                                let value_84ea8e4c = #import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_3680a4c9));
                                                if !acc_5c441d3a.contains(&value_84ea8e4c) {
                                                    acc_5c441d3a.push(value_84ea8e4c);
                                                }
                                                acc_5c441d3a
                                            },
                                            None => {
                                                return None;
                                            }
                                        },
                                        None => vec![#import_path::NullableJsonObjectPostgresTypeWhereFilter(None)],
                                    }
                                ) {
                                    Ok(value_72dbefbc) => Some(value_72dbefbc),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("d41bcbca-5d4c-436c-a465-4920c9da6a43")
                                    }
                                }
                            };
                            match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_ts = vec_syn_field.iter().map(|el_d41dce84| {
                                            let field_ident = &el_d41dce84.field_ident;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_d41dce84.field_type);
                                            quote! {
                                                if let Some(value_927601a4) = #field_type_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_equal_sc(
                                                    #create_sc.#field_ident
                                                ) {
                                                    for el_194a660a in value_927601a4.clone().into_vec() {
                                                        acc_587bf907.push(
                                                            #ident_where_ucc::#field_ident_ucc(
                                                                #import_path::PostgresTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_194a660a]
                                                                ).expect("2f437949-0c13-4b15-83dd-8ef0399b7d61")
                                                            )
                                                        );
                                                    }
                                                    let value_84ea8e4c = #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PostgresTypeWhere::new(
                                                            #import_path::LogicalOperator::And,
                                                            value_927601a4
                                                        )
                                                    );
                                                    if !acc_587bf907.contains(&value_84ea8e4c) {
                                                        acc_587bf907.push(value_84ea8e4c);
                                                    }
                                                }
                                            }
                                        });
                                        quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_587bf907 = Vec::new();
                                                #(#content_ts)*
                                                acc_587bf907
                                            }) {
                                                Ok(value_ea661a62) => Some(value_ea661a62),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("7786dfd4-66c1-4d63-acce-794ef80d8bb6")
                                                },
                                            }
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_standart_not_null_ucc)
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_ts = vec_syn_field.iter().map(|el_c776b608| {
                                            let field_ident = &el_c776b608.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_c776b608.field_type);
                                            quote! {
                                                for create_e06a9fe2 in #create_sc.0.clone() {
                                                    if let Some(value_ee015fcc) = #field_type_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_equal_sc(
                                                        create_e06a9fe2.#field_ident
                                                    ) {
                                                        for el_63008daa in value_ee015fcc.clone().into_vec() {
                                                            let value_0ae29f5f = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PostgresTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_63008daa]
                                                                )
                                                                .expect("38ca88dc-ab40-4a76-8bcd-223df66a1f81"),
                                                            );
                                                            if !acc_480d72e5.contains(&value_0ae29f5f) {
                                                                acc_480d72e5.push(value_0ae29f5f);
                                                            }
                                                        }
                                                        let value_4e4cfda3 = #ident_where_ucc::#el_field_ident_ucc(
                                                            #import_path::PostgresTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
                                                                value_ee015fcc
                                                            )
                                                        );
                                                        if !acc_480d72e5.contains(&value_4e4cfda3) {
                                                            acc_480d72e5.push(value_4e4cfda3);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_480d72e5 = Vec::new();
                                                #(#content_ts)*
                                                acc_480d72e5.push(#ident_where_ucc::LengthEqual(
                                                    #import_path::PostgresJsonTypeWhereLengthEqual {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #value_sc: #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(#create_sc.0.len()).expect("1811faf7-c0a5-4e05-b866-546affd441df")
                                                        ).expect("a590f39b-ad2c-4002-afac-f7c18156362e"),
                                                    }
                                                ));
                                                acc_480d72e5
                                            }) {
                                                Ok(value_cc01db9a) => Some(value_cc01db9a),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("bad01dd0-163c-4ea5-99c0-a8594a4066e1")
                                                },
                                            }
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_array_not_null_ucc)
                                },
                            }
                        };
                        let create_into_postgres_json_type_option_vec_where_length_greater_than_ts = {
                            let gen_nullable_ts = |content_ts: &dyn quote::ToTokens|quote! {
                                #create_sc.0.map_or_else(|| None, |create_612f2a61| <
                                    #content_ts
                                    as
                                    #import_path::PostgresJsonTypeTestCases
                                >::create_into_postgres_json_type_option_vec_where_length_greater_than(create_612f2a61).map_or_else(
                                    || None,
                                    |value_1ea95b5d| match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_87f84b5c = Vec::new();
                                        for el_9bbf8527 in value_1ea95b5d.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(vec![el_9bbf8527]) {
                                                Ok(value_1d0202fc) => {
                                                    acc_87f84b5c.push(#import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_1d0202fc)));
                                                }
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => (),
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("bdb0a112-6f75-481c-ad28-f540252d8525"),
                                                },
                                            }
                                        }
                                        let value_4e4cfda3 = #import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_1ea95b5d));
                                        if !acc_87f84b5c.contains(&value_4e4cfda3) {
                                            acc_87f84b5c.push(value_4e4cfda3);
                                        }
                                        acc_87f84b5c
                                    }) {
                                        Ok(value_ea4ca151) => Some(value_ea4ca151),
                                        Err(error) => match error {
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("c7ecc36f-d510-40ff-a740-e796e112eee5"),
                                        },
                                    },
                                ))
                            };
                            match &pattern {
                                Pattern::Standart => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_ts = vec_syn_field.iter().map(|el_9d0245f1| {
                                            let field_ident = &el_9d0245f1.field_ident;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_9d0245f1.field_type);
                                            quote! {
                                                if let Some(value_3432b965) = #field_type_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_greater_than_sc(
                                                    #create_sc.#field_ident
                                                ) {
                                                    for el_9bbf8527 in value_3432b965.clone().into_vec() {
                                                        acc_f5866fb6.push(
                                                            #ident_where_ucc::#field_ident_ucc(
                                                                #import_path::PostgresTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_9bbf8527]
                                                                ).expect("479db858-6f36-48ba-9ab0-741b7df7956c")
                                                            )
                                                        );
                                                    }
                                                    let el_4a00ab02 = #ident_where_ucc::#field_ident_ucc(
                                                        #import_path::PostgresTypeWhere::new(
                                                            #import_path::LogicalOperator::And,
                                                            value_3432b965
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
                                                #(#content_ts)*
                                                acc_f5866fb6
                                            }) {
                                                Ok(value_c4c01cd9) => Some(value_c4c01cd9),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("91d713b5-fcb1-4876-ab05-70a52a91bce8")
                                                },
                                            }
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_standart_not_null_ucc)
                                },
                                Pattern::Array => match &not_null_or_nullable {
                                    postgres_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_ts = vec_syn_field.iter().map(|el_47c8f26c| {
                                            let field_ident = &el_47c8f26c.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&el_47c8f26c.field_type);
                                            quote! {
                                                for create_34a1e540 in #create_sc.0.clone() {
                                                    if let Some(value_51fe384b) = #field_type_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_greater_than_sc(
                                                        create_34a1e540.#field_ident
                                                    ) {
                                                        for el_4a00ab02 in value_51fe384b.clone().into_vec() {
                                                            let el_938f8b34 = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PostgresTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![el_4a00ab02]
                                                                )
                                                                .expect("955c6c27-863d-4b9b-9d88-e71f11161b3e"),
                                                            );
                                                            if !acc_acceb7eb.contains(&el_938f8b34) {
                                                                acc_acceb7eb.push(el_938f8b34);
                                                            }
                                                        }
                                                        let el_e17d9fba = #ident_where_ucc::#el_field_ident_ucc(
                                                            #import_path::PostgresTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
                                                                value_51fe384b
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
                                                #(#content_ts)*
                                                acc_acceb7eb.push(#ident_where_ucc::LengthGreaterThan(
                                                    #import_path::PostgresJsonTypeWhereLengthGreaterThan {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #value_sc: #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(
                                                                #create_sc.0.len().checked_sub(1).unwrap_or_else(|| {
                                                                    panic!("e411b8ca-9419-4c9f-b555-2b6a661fed62");
                                                                })
                                                            ).expect("1fbbd897-2fae-4271-9fac-4b4007aecf32")
                                                        ).expect("0eb5d915-bbbe-44c0-9096-d3d858d3a937"),
                                                    }
                                                ));
                                                acc_acceb7eb
                                            }) {
                                                Ok(value_a889de37) => Some(value_a889de37),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("a9e99f81-aa41-4535-ac15-56f1beb0eb49")
                                                },
                                            }
                                        }
                                    }
                                    postgres_crud_macros_common::NotNullOrNullable::Nullable => gen_nullable_ts(&ident_array_not_null_ucc)
                                },
                            }
                        };
                        let (
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts
                        ) = {
                            let gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts = |method_name_ts: &dyn quote::ToTokens|match &not_null_or_nullable {
                                postgres_crud_macros_common::NotNullOrNullable::NotNull => match &pattern {
                                    Pattern::Standart => {
                                        let content_ts = vec_syn_field.iter().map(|el_59346ba9| {
                                            let field_ident = &el_59346ba9.field_ident;
                                            let field_type = &el_59346ba9.field_type;
                                            let field_ident_ucc = &ToTokensToUccTs::case_or_panic(&field_ident);
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                if let Some(value_a2900ac9) = #field_type_as_postgres_json_type_test_cases_ts::#method_name_ts(
                                                    #read_only_ids_sc.0.#value_sc.#field_ident,
                                                    #create_sc.#field_ident
                                                ) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    for el_3e86d33d in value_a2900ac9.clone().into_vec() {
                                                        match el_3e86d33d {
                                                            #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PostgresTypeWhere::new(
                                                                            and,
                                                                            multiple
                                                                        ))
                                                                    )
                                                                );
                                                            },
                                                            #import_path::SingleOrMultiple::Single(single) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PostgresTypeWhere::try_new(
                                                                            and,
                                                                            vec![single]
                                                                        ).expect("2635ede5-e733-4793-a2b5-110dda258c90"))
                                                                    )
                                                                );
                                                            },
                                                        }
                                                    }
                                                    let value_3e75a2f2 = #import_path::SingleOrMultiple::Single(
                                                        #ident_where_ucc::#field_ident_ucc(#import_path::PostgresTypeWhere::try_new(
                                                            and,
                                                            value_a2900ac9.into_vec().into_iter().flat_map(|el_9efefcdc| match el_9efefcdc {
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
                                                        ).expect("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))
                                                    );
                                                    if !acc_a94bd7fb.contains(&value_3e75a2f2) {
                                                        acc_a94bd7fb.push(value_3e75a2f2);
                                                    }
                                                }
                                            }
                                        });
                                        quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_a94bd7fb = Vec::new();
                                                #(#content_ts)*
                                                acc_a94bd7fb
                                            }) {
                                                Ok(value_ebe930f0) => Some(value_ebe930f0),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                                                }
                                            }
                                        }
                                    },
                                    Pattern::Array => {
                                        let initialization_ts = vec_syn_field.iter().map(|el_3fde3bb4| {
                                            let field_ident = &el_3fde3bb4.field_ident;
                                            let field_type = &el_3fde3bb4.field_type;
                                            let field_type_as_postgres_json_type_test_cases_ts = gen_type_as_postgres_json_type_test_cases_ts(&field_type);
                                            quote! {
                                                let #field_ident = #field_type_as_postgres_json_type_test_cases_ts::#method_name_ts(
                                                    read_only_ids_629675e2.0.#value_sc.#field_ident,
                                                    create_82796400.#field_ident
                                                );
                                            }
                                        });
                                        let if_some_content_ts = {
                                            let (last, rest) = vec_syn_field.split_last().expect("a8e7b6d6-d46c-4d15-880d-c5c14723966c");
                                            let gen_field_ident_is_some_ts = |field_ident: &syn::Ident|quote!{#field_ident.is_some()};
                                            let rest_ts = rest.iter().map(|el_cd54f3c6| {
                                                let field_ident_is_some_ts = gen_field_ident_is_some_ts(&el_cd54f3c6.field_ident);
                                                quote!{#field_ident_is_some_ts || }
                                            });
                                            let last_ts = gen_field_ident_is_some_ts(&last.field_ident);
                                            quote! {#(#rest_ts)* #last_ts}
                                        };
                                        let content_ts = vec_syn_field.iter().map(|el_dbdd7930| {
                                            let field_ident = &el_dbdd7930.field_ident;
                                            let el_field_ident_ucc = ElementSelfUcc::from_tokens(&field_ident);
                                            quote! {
                                                if let Some(value_f190793e) = #field_ident {
                                                    for el_22ac4087 in value_f190793e.clone().into_vec() {
                                                        let current_where = #ident_where_ucc::#el_field_ident_ucc(
                                                            match el_22ac4087 {
                                                                #import_path::SingleOrMultiple::Multiple(multiple) => #import_path::PostgresTypeWhere::new(
                                                                    and,
                                                                    multiple.clone()
                                                                ),
                                                                #import_path::SingleOrMultiple::Single(single) => #import_path::PostgresTypeWhere::try_new(
                                                                    and,
                                                                    vec![single]
                                                                ).expect("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd"),
                                                            }
                                                        );
                                                        all_fields_acc.push(current_where.clone());
                                                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                                                            #id_sc.clone(),
                                                            current_where
                                                        ]) {
                                                            Ok(value_fdd1b3eb) => {
                                                                let multiple_current_where_with_id = #import_path::SingleOrMultiple::Multiple(value_fdd1b3eb);
                                                                if !acc_359c0b3f.contains(&multiple_current_where_with_id) {
                                                                    acc_359c0b3f.push(multiple_current_where_with_id);
                                                                }
                                                            },
                                                            Err(error) => match error {
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("f0e3d01b-ac0c-43d4-b31b-45f02e274696")
                                                            }
                                                        }
                                                    }
                                                    match #import_path::NotEmptyUniqueVec::try_new(
                                                        value_f190793e.into_vec().into_iter().flat_map(|el_6df4f0be| match el_6df4f0be {
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
                                                        Ok(value_a4000d70) => {
                                                            let value_d6218307 = #ident_where_ucc::#el_field_ident_ucc(
                                                                #import_path::PostgresTypeWhere::new(
                                                                    and,
                                                                    value_a4000d70
                                                                )
                                                            );
                                                            if !all_fields_acc.contains(&value_d6218307) {
                                                                all_fields_acc.push(value_d6218307);
                                                            }
                                                        },
                                                        Err(error) => match error {
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7")
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_359c0b3f = Vec::new();
                                                for (read_only_ids_629675e2, create_82796400) in #read_only_ids_sc.0.#value_sc.into_iter().zip(#create_sc.0.into_iter()) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    let #id_sc = #ident_where_ucc::ElementId(
                                                        #import_path::PostgresTypeWhere::try_new(
                                                            and,
                                                            vec![
                                                                #uuid_uuid_as_not_null_jsonb_string_where_ucc::Equal(#import_path::PostgresJsonTypeWhereEqual {
                                                                    logical_operator: #import_path::LogicalOperator::Or,
                                                                    #value_sc: #uuid_uuid_as_not_null_jsonb_string_table_type_declaration_ucc::new(
                                                                        read_only_ids_629675e2.0.#value_sc.#id_sc.0.#value_sc
                                                                    ),
                                                                })
                                                            ],
                                                        )
                                                        .expect("31db8e1e-28cd-44f7-9f32-a41cc6675660"), 
                                                    );
                                                    #(#initialization_ts)*
                                                    if #if_some_content_ts {
                                                        let mut all_fields_acc = vec![];
                                                        #(#content_ts)*
                                                        match #import_path::NotEmptyUniqueVec::try_new({
                                                            all_fields_acc.push(#id_sc);
                                                            all_fields_acc
                                                        }) {
                                                            Ok(value_80199720) => {
                                                                acc_359c0b3f.push(#import_path::SingleOrMultiple::Multiple(value_80199720));
                                                            },
                                                            Err(error) => match error {
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("32a3da97-c772-44d7-91f9-2916759034e0")
                                                            }
                                                        }
                                                    }
                                                }
                                                acc_359c0b3f
                                            }) {
                                                Ok(value_752f0e8d) => Some(value_752f0e8d),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                                                }
                                            }
                                        }
                                    }
                                },
                                postgres_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let current_ident_ts = gen_type_as_postgres_json_type_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                        Pattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        Pattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote! {
                                        match (#read_only_ids_sc.0.value, #create_sc.0) {
                                            (Some(read_only_ids_3e2e30c8), Some(create_79039a2f)) => #current_ident_ts::#method_name_ts(
                                                read_only_ids_3e2e30c8,
                                                create_79039a2f
                                            ).map_or_else(|| None, |value_35662b3a| match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_e0d72451 = vec![];
                                                for el_4632f100 in value_35662b3a.into_vec() {
                                                    match el_4632f100 {
                                                        #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                            acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(multiple))));
                                                        },
                                                        #import_path::SingleOrMultiple::Single(single) => match #import_path::NotEmptyUniqueVec::try_new(vec![single]) {
                                                            Ok(value_4ce6ecd3) => {
                                                                acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPostgresTypeWhereFilter(Some(value_4ce6ecd3))));
                                                            }
                                                            Err(error) => match error {
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => (),
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("626ffa77-f81a-46ce-b5a0-44663fe1f182"),
                                                            },
                                                        },
                                                    }
                                                }
                                                acc_e0d72451
                                            }) {
                                                Ok(value_5d381053) => Some(value_5d381053),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("23a17416-0bac-4a1b-90df-cfd9d61ae86c"),
                                                },
                                            }),
                                            (Some(_), None) => panic!("994082bf-aa95-45ea-9f80-ce91ae8661fc"),
                                            (None, Some(_)) => panic!("04f4d016-619e-4326-a260-cdec59c23907"),
                                            (None, None) => None,
                                        }
                                    }
                                }
                            };
                            (
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_sc
                                ),
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_sc
                                ),
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_sc
                                ),
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_sc
                                ),
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_sc
                                ),
                                gen_read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_filter_ts(
                                    &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_sc
                                )
                            )
                        };
                        postgres_crud_macros_common::gen_impl_postgres_json_type_test_cases_for_ident_ts(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_ucc,
                            &ident,
                            &option_vec_create_ts,
                            &read_only_ids_to_two_dimensional_vec_read_inner_ts,
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
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts,
                            &create_into_postgres_json_type_option_vec_where_length_equal_ts,
                            &create_into_postgres_json_type_option_vec_where_length_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts,
                        )
                    },
                    {
                        let option_vec_create_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#option_vec_create_sc()};
                        let read_only_ids_to_two_dimensional_vec_read_inner_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_to_two_dimensional_vec_read_inner_sc(#read_only_ids_sc)};
                        let read_inner_into_read_with_new_or_try_new_unwraped_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_inner_into_read_with_new_or_try_new_unwraped_sc(#value_sc)};
                        let read_inner_into_update_with_new_or_try_new_unwraped_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_inner_into_update_with_new_or_try_new_unwraped_sc(#value_sc)};
                        let update_to_read_only_ids_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#update_to_read_only_ids_sc(#value_sc)};
                        let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_to_option_value_read_default_option_some_vec_one_el_sc(#value_sc)};
                        let previous_read_merged_with_option_update_into_read_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#previous_read_merged_with_option_update_into_read_sc(#read_sc, #option_update_sc)};
                        let read_only_ids_merged_with_create_into_read_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_read_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_option_value_read_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_option_value_read_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_table_type_declaration_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_table_type_declaration_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_where_equal_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_where_equal_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts = quote!{Some(#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_sc(
                            #read_only_ids_sc,
                            #create_sc
                        ))};
                        let create_into_postgres_type_option_vec_where_dimension_one_equal_ts = &none_ts;
                        let postgres_type_option_vec_where_greater_than_test_ts = &none_ts;
                        let read_only_ids_merged_with_table_type_declaration_into_postgres_type_option_where_greater_than_ts = &none_ts;

                        let (
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts,
                            read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts
                        ) = {
                            let gen_dimension_equal_handle_ts = |dimension: &postgres_crud_macros_common::Dimension|{
                                let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc = dimension.read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc();
                                quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_number_equal_sc(
                                    #read_only_ids_sc,
                                    #create_sc
                                )}
                            };
                            (
                                gen_dimension_equal_handle_ts(&postgres_crud_macros_common::Dimension::One),
                                gen_dimension_equal_handle_ts(&postgres_crud_macros_common::Dimension::Two),
                                gen_dimension_equal_handle_ts(&postgres_crud_macros_common::Dimension::Three),
                                gen_dimension_equal_handle_ts(&postgres_crud_macros_common::Dimension::Four)
                            )
                        };
                        let create_into_postgres_json_type_option_vec_where_length_equal_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_equal_sc(
                            #create_sc
                        )};
                        let create_into_postgres_json_type_option_vec_where_length_greater_than_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#create_into_postgres_json_type_option_vec_where_length_greater_than_sc(
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts = quote!{#self_as_postgres_json_type_test_cases_ts::#read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_sc(
                            #read_only_ids_sc,
                            #create_sc
                        )};
                        postgres_crud_macros_common::gen_impl_postgres_type_test_cases_for_ident_ts(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_ucc,
                            &ident,
                            &option_vec_create_ts,
                            &read_only_ids_to_two_dimensional_vec_read_inner_ts,
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
                            &create_into_postgres_type_option_vec_where_dimension_one_equal_ts,
                            &postgres_type_option_vec_where_greater_than_test_ts,
                            &read_only_ids_merged_with_table_type_declaration_into_postgres_type_option_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts,
                            &create_into_postgres_json_type_option_vec_where_length_equal_ts,
                            &create_into_postgres_json_type_option_vec_where_length_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts,
                            &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts,
                        )
                    },
                )
            };
            let impl_postgres_type_not_primary_key_for_ident_ts = postgres_crud_macros_common::gen_impl_postgres_type_not_primary_key_for_ident_ts(&import_path, &ident);
            let gend = quote! {
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
                #impl_postgres_crud_postgres_json_type_for_ident_ts
                #maybe_impl_postgres_crud_postgres_types_postgres_type_postgres_type_ts
                #impl_postgres_json_type_test_cases_for_ident_ts
                #impl_postgres_type_test_cases_for_ident_ts
                #impl_postgres_type_not_primary_key_for_ident_ts
            };
            (
                {
                    let field_ident = format!("field_{index}").parse::<Ts2>().expect("7f9a06a5-db0f-420d-ae83-581ccc02c99f");
                    quote! {
                        pub #field_ident: #ident,
                    }
                },
                gend,
            )
        })
        .collect::<(Vec<Ts2>, Vec<Ts2>)>();
    macros_helpers::maybe_write_ts_into_file(
        gen_postgres_json_object_type_config
            .postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_json_object_types,
        "postgres_table_columns_using_postgres_json_object_types",
        &quote! {
            pub struct PostgresTableColumnsContentWriteIntoPostgresTableColumnsUsingPostgresJsonObjectTypes {
                #(#fields_ts)*
            }
        },
        &macros_helpers::FormatWithCargofmt::True,
    );
    let gend: Ts2 = {
        let ident_gen_postgres_json_object_type_mod =
            SelfGenPostgresJsonObjectTypeModSc::from_tokens(&syn_derive_input.ident);
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #ident_gen_postgres_json_object_type_mod {
                #(#postgres_json_object_type_array)*
            }
            pub use #ident_gen_postgres_json_object_type_mod::*;
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        gen_postgres_json_object_type_config.whole_content_write_into_gen_postgres_json_object_type,
        "gen_postgres_json_object_type",
        &gend,
        &macros_helpers::FormatWithCargofmt::True,
    );
    gend
}
