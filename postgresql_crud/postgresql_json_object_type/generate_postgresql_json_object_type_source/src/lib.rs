//todo generate authorization rights enum for json fields
//todo bug in update if updating array and creating element in jsonb array without anything - read_only_ids generation logic of vec returns wrong query part
#[must_use]
pub fn generate_postgresql_json_object_type(
    input_token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    enum TraitGen {
        PostgresqlJsonType,
        PostgresqlTypeAndPostgresqlJsonType,
    }
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
    enum PostgresqlJsonObjectTypePattern {
        Standart,
        Array,
    }
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct PostgresqlJsonObjectTypeRecord {
        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern,
        trait_gen: TraitGen,
    }
    #[derive(Debug, serde::Deserialize)]
    struct GeneratePostgresqlJsonTypesConfig {
        postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_object_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_generate_postgresql_json_object_type:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        variant: PostgresqlJsonObjectTypeRecord,
    }
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse2(input_token_stream).expect("e5f0e27b-e1fc-4d4d-b1f6-dbd56501ad66");
    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrud;
    let generate_postgresql_json_object_type_config =
        serde_json::from_str::<GeneratePostgresqlJsonTypesConfig>(
            &macros_helpers::get_macro_attribute_meta_list_token_stream(
                &syn_derive_input.attrs,
                &format!(
                    "{}::postgresql_json_object_type_config",
                    import_path.snake_case_std_primitive_str()
                ),
            )
            .to_string(),
        )
        .expect("246de453-00ee-420a-a502-c3db0c1b984d");
    let postgresql_json_object_type_record_vec = {
        let postgresql_json_object_type_record = generate_postgresql_json_object_type_config.variant;
        match (&postgresql_json_object_type_record.not_null_or_nullable, &postgresql_json_object_type_record.postgresql_json_object_type_pattern) {
            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart) => vec![postgresql_json_object_type_record],
            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Standart) |
            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Array) => vec![
                PostgresqlJsonObjectTypeRecord {
                    not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern::Standart,
                    trait_gen: postgresql_json_object_type_record.trait_gen.clone(),
                },
                postgresql_json_object_type_record
            ],
            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Array) => vec![
                PostgresqlJsonObjectTypeRecord {
                    not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern::Standart,
                    trait_gen: postgresql_json_object_type_record.trait_gen.clone(),
                },
                PostgresqlJsonObjectTypeRecord {
                    not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::Nullable,
                    postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern::Standart,
                    trait_gen: postgresql_json_object_type_record.trait_gen.clone(),
                },
                PostgresqlJsonObjectTypeRecord {
                    not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern::Array,
                    trait_gen: postgresql_json_object_type_record.trait_gen.clone(),
                },
                postgresql_json_object_type_record
            ]
        }
    }
    // .into_iter()
    // .filter(|element_2f2d1e6c| {
    //     use postgresql_crud_macros_common::NotNullOrNullable;
    //     let not_null_or_nullable_filter = match &element_2f2d1e6c.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => true,
    //         NotNullOrNullable::Nullable => true,
    //     };
    //     let postgresql_json_object_type_pattern_filter = match &element_2f2d1e6c.postgresql_json_object_type_pattern {
    //         PostgresqlJsonObjectTypePattern::Standart => match &element_2f2d1e6c.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //         PostgresqlJsonObjectTypePattern::Array => match &element_2f2d1e6c.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //     };
    //     let trait_gen_filter = match &element_2f2d1e6c.trait_gen {
    //         TraitGen::PostgresqlJsonType => true,
    //         TraitGen::PostgresqlTypeAndPostgresqlJsonType => true,
    //     };
    //     not_null_or_nullable_filter && postgresql_json_object_type_pattern_filter && trait_gen_filter
    // })
    // .collect::<Vec<PostgresqlJsonObjectTypeRecord>>()
    ;
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonObjectTypeJsonVariants",
    //     &serde_json::to_string(&postgresql_json_object_type_record_vec).expect("efc7a263-f6cd-44ca-aacf-470a37971f7f"),
    // );

    // element.iter().enumerate().fold(String::new(), |mut acc_1e1c6a6e, (index, element)| {
    //     let element_snake_case_stringified = naming_common::AsRefStrToSnakeCaseStringified::case(element);
    //     if index == 0 {
    //         acc_1e1c6a6e.push_str(&element_snake_case_stringified);
    //     } else {
    //         acc_1e1c6a6e.push_str(&format!("_{element_snake_case_stringified}"));
    //     }
    //     acc_1e1c6a6e
    // });
    // let postgresql_json_object_type_array
    let (fields_token_stream, postgresql_json_object_type_array) = postgresql_json_object_type_record_vec
        .into_iter()
        .enumerate()
        .map(|(index, element)| {
            #[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
            enum IsStandartWithId {
                False,
                True,
            }
            enum IdentPattern {
                StandartNotNullWithoutId,
                StandartNotNullWithId,
                StandartNullableWithoutId,
                ArrayNotNullWithId,
                ArrayNullableWithIdentifier,//Identifier instead of Id - just to fix clippy lint
            }
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresqlJsonTypeSubtype {
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
            impl quote::ToTokens for PostgresqlJsonTypeSubtype {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    self.to_string().parse::<proc_macro2::TokenStream>().expect("43ac0b62-551a-421e-aee0-9bf3dfffa3cc").to_tokens(tokens);
                }
            }
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresqlTypeSubtype {
                // TableTypeDeclaration,
                // Create,
                // Select,
                // Where,
                Read,
                // ReadOnlyIds,
                // ReadInner,
                Update,
            }
            impl quote::ToTokens for PostgresqlTypeSubtype {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    self.to_string().parse::<proc_macro2::TokenStream>().expect("5825d4b7-dd55-41e4-b54e-7b31557181b6").to_tokens(tokens);
                }
            }
            enum PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate {
                TableTypeDeclaration,
                Create,
            }
            impl From<&PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate> for PostgresqlJsonTypeSubtype {
                fn from(value: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate) -> Self {
                    match &value {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => Self::TableTypeDeclaration,
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Self::Create,
                    }
                }
            }
            enum ReadWithOrWithoutAnnotationOrInner {
                WithSerdeOptionIsNoneAnnotation,
                WithoutSerdeOptionIsNoneAnnotation,
                Inner,
            }
            enum ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation {
                True,
                False,
            }
            enum StructDeclarationOrNewType {
                StructDeclaration,
                NewType,
            }
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_json_object_type_pattern = &element.postgresql_json_object_type_pattern;
            let trait_gen = &element.trait_gen;

            let create_snake_case = naming::CreateSnakeCase;
            let update_snake_case = naming::UpdateSnakeCase;
            let delete_snake_case = naming::DeleteSnakeCase;
            let value_upper_camel_case = naming::ValueUpperCamelCase;
            let value_snake_case = naming::ValueSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let select_query_part_postgresql_type_snake_case = naming::SelectQueryPartPostgresqlTypeSnakeCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let id_snake_case = naming::IdSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            let fields_snake_case = naming::FieldsSnakeCase;
            let self_upper_camel_case = naming::SelfUpperCamelCase;
            let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
            let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
            let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
            let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
            let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
            let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
            let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
            let column_snake_case = naming::ColumnSnakeCase;
            let self_snake_case = naming::SelfSnakeCase;
            let read_snake_case = naming::ReadSnakeCase;
            let equal_upper_camel_case = naming::EqualUpperCamelCase;
            let option_update_snake_case = naming::OptionUpdateSnakeCase;
            let query_part_snake_case = naming::QueryPartSnakeCase;
            let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
            let read_only_ids_to_two_dimensional_vec_read_inner_snake_case = naming::ReadOnlyIdsToTwoDimensionalVecReadInnerSnakeCase;
            let select_only_ids_query_part_snake_case = naming::SelectOnlyIdsQueryPartSnakeCase;
            let read_inner_into_read_with_new_or_try_new_unwraped_snake_case = naming::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase;
            let read_only_ids_into_option_value_read_inner_snake_case = naming::ReadOnlyIdsIntoOptionValueReadInnerSnakeCase;
            let update_to_read_only_ids_snake_case = naming::UpdateToReadOnlyIdsSnakeCase;
            let select_only_updated_ids_query_part_snake_case = naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
            let select_only_created_ids_query_part_snake_case = naming::SelectOnlyCreatedIdsQueryPartSnakeCase;
            let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
            let select_only_updated_ids_query_bind_snake_case = naming::SelectOnlyUpdatedIdsQueryBindSnakeCase;
            let select_only_created_ids_query_bind_snake_case = naming::SelectOnlyCreatedIdsQueryBindSnakeCase;
            let read_inner_into_update_with_new_or_try_new_unwraped_snake_case = naming::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase;
            let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
            let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
            let read_only_ids_merged_with_create_into_read_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoReadSnakeCase;
            let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::ReadOnlyIdsToOptionValueReadDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
            let previous_read_merged_with_option_update_into_read_snake_case = naming::PreviousReadMergedWithOptionUpdateIntoReadSnakeCase;
            let read_only_ids_merged_with_create_into_option_value_read_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoOptionValueReadSnakeCase;
            let read_only_ids_merged_with_create_into_where_equal_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;
            let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSnakeCase;
            let read_only_ids_merged_with_create_into_table_type_declaration_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSnakeCase;
            let create_into_postgresql_json_type_option_vec_where_length_equal_snake_case = naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSnakeCase;
            let create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case = naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereGreaterThanSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereBetweenSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereInSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereRegularExpressionSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElementGreaterThanSnakeCase;
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElementRegularExpressionSnakeCase;
            let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementUpperCamelCase;
            let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;

            let std_string_string_token_stream = token_patterns::StdStringString;
            let self_field_vec_token_stream = quote::quote! {.0.to_vec()};
            let cfg_feature_test_utils = quote::quote! {#[cfg(feature = "test-utils")]};
            let return_err_query_part_error_named_write_into_buffer_token_stream = postgresql_crud_macros_common::generate_return_err_query_part_error_named_write_into_buffer_token_stream(import_path);
            let none_token_stream = quote::quote!{None};
            let must_use_token_stream = token_patterns::MustUse;

            let generate_import_path_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                postgresql_crud_macros_common::generate_value_initialization_token_stream(
                    &import_path,
                    &content_token_stream
                )
            };

            let import_path_query_part_error_named_token_stream = {
                let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
                quote::quote! {#import_path::#query_part_error_named_upper_camel_case}
            };
            let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
            let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeCall;
            let vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote!{vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]};
            let import_path_default_but_option_is_always_some_token_stream = quote::quote!{
                #import_path::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case
            };
            let import_path_default_but_option_is_always_some_call_token_stream = quote::quote!{
                #import_path_default_but_option_is_always_some_token_stream()
            };
            let generate_ident_as_default_but_option_is_always_some_call_token_stream = |ident_token_stream: &dyn quote::ToTokens, |{
                quote::quote!{
                    <#ident_token_stream as #import_path::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case>::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                }
            };
            let generate_ident_as_default_but_option_is_always_some_token_stream = |ident_token_stream_2e6aba01: &dyn quote::ToTokens|quote::quote!{
                <
                    #ident_token_stream_2e6aba01
                    as
                    #import_path::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
                >::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case
            };
            let import_path_value_token_stream = quote::quote!{#import_path::#value_upper_camel_case};
            let wrap_into_value_declaration_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                quote::quote!{#import_path_value_token_stream<#ident_token_stream>}
            };
            let wrap_into_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                quote::quote!{#import_path_value_token_stream { #value_snake_case: #content_token_stream }}
            };

            let syn_derive_input_ident = &syn_derive_input.ident;
            let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
                if let syn::Fields::Named(fields_named) = &data_struct.fields {
                    fields_named.named.iter()
                    .collect::<Vec<&syn::Field>>()
                    .iter()
                    .map(|element_f01f3f33|macros_helpers::SynFieldWrapper {
                        field_visibility: element_f01f3f33.vis.clone(),
                        field_ident: element_f01f3f33.ident.clone().expect("3ac7f263-e0bf-4c5b-9d44-57edf50f79b5"),
                        field_type: element_f01f3f33.ty.clone(),
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
            let generate_ident_upper_camel_case = |ident_pattern: &IdentPattern| {
                let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
                let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
                let jsonb_object_upper_camel_case = naming::JsonbObjectUpperCamelCase;
                let with_id_upper_camel_case = naming::WithIdUpperCamelCase;
                let syn_derive_input_ident_stringified = syn_derive_input_ident.to_string();
                let jsonb_object_upper_camel_case_stringified = jsonb_object_upper_camel_case.to_string();
                let vec_of_syn_derive_input_ident_with_id = format!("{vec_of_upper_camel_case}{syn_derive_input_ident}{with_id_upper_camel_case}");
                let array_of_not_null_jsonb_object_with_id = format!("{array_of_upper_camel_case}{}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}", postgresql_crud_macros_common::NotNullOrNullable::NotNull);
                let (rust_part, postgresql_part, current_not_null_or_nullable) = match &ident_pattern {
                    IdentPattern::StandartNotNullWithoutId => (syn_derive_input_ident_stringified, jsonb_object_upper_camel_case_stringified, postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::StandartNotNullWithId => (format!("{syn_derive_input_ident}{with_id_upper_camel_case}"), format!("{jsonb_object_upper_camel_case}{with_id_upper_camel_case}"), postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::StandartNullableWithoutId => (syn_derive_input_ident_stringified, jsonb_object_upper_camel_case_stringified, postgresql_crud_macros_common::NotNullOrNullable::Nullable),
                    IdentPattern::ArrayNotNullWithId => (vec_of_syn_derive_input_ident_with_id, array_of_not_null_jsonb_object_with_id, postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                    IdentPattern::ArrayNullableWithIdentifier => (vec_of_syn_derive_input_ident_with_id, array_of_not_null_jsonb_object_with_id, postgresql_crud_macros_common::NotNullOrNullable::Nullable),
                };
                let current_not_null_or_nullable_rust = current_not_null_or_nullable.rust();
                format!("{current_not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().expect("43784dd3-f37a-438d-8bc8-d17f63feac66")
            };

            let ident = &generate_ident_upper_camel_case(&match (&not_null_or_nullable, &postgresql_json_object_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart) => IdentPattern::StandartNotNullWithoutId,
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Array) => IdentPattern::ArrayNotNullWithId,
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Standart) => IdentPattern::StandartNullableWithoutId,
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Array) => IdentPattern::ArrayNullableWithIdentifier,
            });
            let ident_standart_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::StandartNotNullWithoutId);
            let ident_array_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId);
            let ident_with_id_standart_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::StandartNotNullWithId);
            let ident_with_id_array_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId);
            let is_standart_not_null = matches!((&not_null_or_nullable, postgresql_json_object_type_pattern), (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart));
            let generate_type_as_import_path_token_stream = |first_type_token_stream: &dyn quote::ToTokens, second_type_token_stream: &dyn quote::ToTokens|{
                quote::quote! {<#first_type_token_stream as #import_path::#second_type_token_stream>}
            };
            let generate_type_as_postgresql_json_type_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                generate_type_as_import_path_token_stream(&type_token_stream, &naming::PostgresqlJsonTypeUpperCamelCase)
            };
            let ident_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident);
            let ident_standart_not_null_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_array_not_null_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_array_not_null_upper_camel_case);

            let uuid_uuid_as_not_null_jsonb_string_upper_camel_case = naming::UuidUuidAsNotNullJsonbStringUpperCamelCase;
            let uuid_uuid_as_not_null_jsonb_string_token_stream = quote::quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_upper_camel_case};
            let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_upper_camel_case = {
                let uuid_uuid_as_not_null_jsonb_string_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_display(&uuid_uuid_as_not_null_jsonb_string_upper_camel_case);
                quote::quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_table_type_declaration_upper_camel_case}
            };
            let uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case = {
                let uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_display(&uuid_uuid_as_not_null_jsonb_string_upper_camel_case);
                quote::quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case}
            };
            let uuid_uuid_as_not_null_jsonb_string_where_upper_camel_case = {
                let uuid_uuid_as_not_null_jsonb_string_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_display(&uuid_uuid_as_not_null_jsonb_string_upper_camel_case);
                quote::quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_where_upper_camel_case}
            };
            let uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream = quote::quote!{
                #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream::Update
            };
            let uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream = quote::quote!{
                <#uuid_uuid_as_not_null_jsonb_string_token_stream as #import_path::PostgresqlJsonTypeObjectVecElementId>
            };
            let id_syn_field = {
                let value = syn::Field {
                    attrs: Vec::new(),
                    vis: syn::Visibility::Public(syn::token::Pub { span: proc_macro2::Span::call_site() }),
                    mutability: syn::FieldMutability::None,
                    ident: Some(syn::Ident::new(&id_snake_case.to_string(), proc_macro2::Span::call_site())),
                    colon_token: Some(syn::token::Colon { spans: [proc_macro2::Span::call_site()] }),
                    ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: macros_helpers::generate_simple_syn_punctuated_punctuated(
                                &[import_path.to_path(), &uuid_uuid_as_not_null_jsonb_string_upper_camel_case.to_string()]
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
            let vec_syn_field_with_id: Vec<macros_helpers::SynFieldWrapper> = vec_syn_field.clone().into_iter().fold(vec![id_syn_field], |mut acc_9db5e042, element_f01f3f33| {
                acc_9db5e042.push(element_f01f3f33);
                acc_9db5e042
            });
            let get_vec_syn_field = |is_standart_with_id: &IsStandartWithId| -> &Vec<macros_helpers::SynFieldWrapper> {
                match &is_standart_with_id {
                    IsStandartWithId::False => &vec_syn_field,
                    IsStandartWithId::True => &vec_syn_field_with_id,
                }
            };
            let generate_type_as_postgresql_type_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                generate_type_as_import_path_token_stream(&type_token_stream, &naming::PostgresqlTypeUpperCamelCase)
            };
            let generate_type_as_postgresql_json_type_test_cases_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                generate_type_as_import_path_token_stream(&type_token_stream, &naming::PostgresqlJsonTypeTestCasesUpperCamelCase)
            };
            let generate_type_as_postgresql_type_test_cases_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                generate_type_as_import_path_token_stream(&type_token_stream, &naming::PostgresqlTypeTestCasesUpperCamelCase)
            };
            let self_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&self_upper_camel_case);
            let ident_standart_not_null_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(
                &ident_standart_not_null_upper_camel_case
            );
            let ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_standart_not_null_upper_camel_case);
            let import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(
                &uuid_uuid_as_not_null_jsonb_string_token_stream
            );
            let ident_with_id_standart_not_null_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_token_stream = {
                let generate_struct_ident_token_stream = |current_ident: &dyn quote::ToTokens| macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy()
                    .build_struct(
                        &current_ident,
                        &quote::quote!{;}
                    );
                let ident_token_stream = generate_struct_ident_token_stream(&ident);
                let maybe_ident_with_id_standart_not_null_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_token_stream = generate_struct_ident_token_stream(&ident_with_id_standart_not_null_upper_camel_case);
                    let cfg_feature_test_utils_impl_ident_with_id_standart_not_null_token_stream = {
                        let read_only_ids_merged_with_create_into_where_equal_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_where_equal_token_stream(
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &{
                                let generate_token_stream = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_token_stream: &dyn quote::ToTokens,
                                |{
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                    quote::quote!{
                                        #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                            #second_argument_token_stream
                                        )
                                    }
                                };
                                let current_ident_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = vec_syn_field.iter().map(|element_e970b03b| {
                                    let field_ident = &element_e970b03b.field_ident;
                                    generate_token_stream(
                                        &field_ident,
                                        &element_e970b03b.field_type,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    #ident_with_id_standart_not_null_where_upper_camel_case::#equal_upper_camel_case(postgresql_crud::PostgresqlJsonTypeWhereEqual {
                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                        #value_snake_case: #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                            #current_ident_token_stream,
                                            #(#content_token_stream),*
                                        ),
                                    })
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream(
                            &import_path,
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &{
                                let generate_token_stream = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_token_stream: &dyn quote::ToTokens,
                                |{
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                    quote::quote!{
                                        #ident_with_id_standart_not_null_where_upper_camel_case::#field_ident_upper_camel_case(
                                            postgresql_crud::PostgresqlTypeWhere::new(
                                                postgresql_crud::LogicalOperator::And,
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #second_argument_token_stream
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = vec_syn_field.iter().map(|element_4fafbc5e| {
                                    let field_ident = &element_4fafbc5e.field_ident;
                                    generate_token_stream(
                                        &field_ident,
                                        &element_4fafbc5e.field_type,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_token_stream,
                                        #(#content_token_stream),*
                                    ]).expect("5473d8c4-2b10-4ba8-8a4a-704fde84f6ff")
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream(
                            import_path,
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &{
                                let generate_token_stream = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_token_stream: &dyn quote::ToTokens,
                                |{
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                    quote::quote!{
                                        #ident_with_id_standart_not_null_where_upper_camel_case::#field_ident_upper_camel_case(
                                            #import_path::PostgresqlTypeWhere::new(
                                                #import_path::LogicalOperator::Or,
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #second_argument_token_stream
                                                ),
                                            ),
                                        )
                                    }
                                };
                                let id_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = vec_syn_field.iter().map(|element_649e1691| {
                                    let field_ident = &element_649e1691.field_ident;
                                    generate_token_stream(
                                        &field_ident,
                                        &element_649e1691.field_type,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    #import_path::NotEmptyUniqueVec::try_new(vec![
                                        #id_token_stream,
                                        #(#content_token_stream),*
                                    ]).expect("221a4c55-5467-44f1-94bb-b748a92cfada")
                                }
                            },
                        );
                        quote::quote! {
                            #[cfg(feature = "test-utils")]
                            impl #ident_with_id_standart_not_null_upper_camel_case {
                                #read_only_ids_merged_with_create_into_where_equal_token_stream
                                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream
                                #read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream
                            }
                        }
                    };
                    quote::quote! {
                        #ident_with_id_standart_not_null_token_stream
                        #cfg_feature_test_utils_impl_ident_with_id_standart_not_null_token_stream
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_token_stream
                    #maybe_ident_with_id_standart_not_null_token_stream
                }
            };
            let ident_array_not_null_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_array_not_null_upper_camel_case);
            let ident_with_id_array_not_null_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_with_id_array_not_null_upper_camel_case);
            let postgresql_json_type_subtype_table_type_declaration = PostgresqlJsonTypeSubtype::TableTypeDeclaration;
            let postgresql_json_type_subtype_create = PostgresqlJsonTypeSubtype::Create;
            let postgresql_json_type_subtype_create_for_query = PostgresqlJsonTypeSubtype::CreateForQuery;
            let postgresql_json_type_subtype_select = PostgresqlJsonTypeSubtype::Select;
            let postgresql_json_type_subtype_where = PostgresqlJsonTypeSubtype::Where;
            let postgresql_json_type_subtype_read = PostgresqlJsonTypeSubtype::Read;
            let postgresql_json_type_subtype_read_inner = PostgresqlJsonTypeSubtype::ReadInner;
            let postgresql_json_type_subtype_update = PostgresqlJsonTypeSubtype::Update;
            let postgresql_json_type_subtype_update_for_query = PostgresqlJsonTypeSubtype::UpdateForQuery;
            let generate_type_as_postgresql_json_type_subtype_token_stream = |type_token_stream: &dyn quote::ToTokens, postgresql_json_type_subtype: &PostgresqlJsonTypeSubtype| {
                let type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&type_token_stream);
                quote::quote! {#type_as_postgresql_json_type_token_stream::#postgresql_json_type_subtype}
            };
            let generate_type_as_postgresql_type_subtype_token_stream = |type_token_stream: &dyn quote::ToTokens, postgresql_type_subtype: &PostgresqlTypeSubtype| {
                let type_as_postgresql_type_token_stream = generate_type_as_postgresql_type_token_stream(&type_token_stream);
                quote::quote! {#type_as_postgresql_type_token_stream::#postgresql_type_subtype}
            };
            let generate_field_type_as_crud_postgresql_json_type_from_field_token_stream = |
                syn_field_wrapper: &macros_helpers::SynFieldWrapper
            | generate_type_as_postgresql_json_type_token_stream(
                &syn_field_wrapper.field_type
            );
            let generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                &proc_macro2::TokenStream::new(),
                &current_ident_token_stream,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self:?}")}
            );
            let ident_as_postgresql_json_type_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&ident, &postgresql_json_type_subtype_table_type_declaration);
            let self_value_token_stream = quote::quote! {Self(#value_snake_case)};
            let postgresql_type_where_filter_query_bind_value_query_token_stream = quote::quote!{#import_path::PostgresqlTypeWhereFilter::query_bind(#value_snake_case, #query_snake_case)};

            let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
            let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
            let ident_array_not_null_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident_array_not_null_upper_camel_case);
            let ident_standart_not_null_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_create_for_query_upper_camel_case = naming::parameter::SelfCreateForQueryUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let wrap_into_scopes_token_stream = |content: &dyn quote::ToTokens| {
                quote::quote! {(#content);}
            };
            let generate_ident_table_type_declaration_or_ident_create_common_token_stream = |postgresql_json_type_subtype_table_type_declaration_or_create: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate| {
                let ident_table_type_declaration_or_ident_create_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_table_type_declaration_upper_camel_case,
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_create_upper_camel_case,
                };
                let generate_ident_table_type_declaration_or_create_token_stream = |
                    current_ident_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens
                | macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &current_ident_token_stream,
                        &content_token_stream
                    );
                let struct_declaration_or_new_type_struct_declaration = StructDeclarationOrNewType::StructDeclaration;
                let struct_declaration_or_new_type_new_type = StructDeclarationOrNewType::NewType;
                let generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream = |
                    is_standart_with_id: &IsStandartWithId,
                    current_postgresql_json_type_subtype_table_type_declaration_or_create: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate,
                    struct_declaration_or_new_type: &StructDeclarationOrNewType
                | {
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_42f25108| {
                        let field_ident = &element_42f25108.field_ident;
                        let type_as_postgresql_json_type_subtype_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element_42f25108.field_type,
                            &PostgresqlJsonTypeSubtype::from(current_postgresql_json_type_subtype_table_type_declaration_or_create)
                        );
                        quote::quote! {#field_ident: #type_as_postgresql_json_type_subtype_table_type_declaration_token_stream}
                    });
                    let fields_content_token_stream = quote::quote! {#(#content_token_stream),*};
                    match &struct_declaration_or_new_type {
                        StructDeclarationOrNewType::StructDeclaration => quote::quote! {{#fields_content_token_stream}},
                        StructDeclarationOrNewType::NewType => fields_content_token_stream,
                    }
                };
                let generate_tokens_table_type_declaration_or_create_token_stream = |tokens: &dyn quote::ToTokens| {
                    let value: &dyn quote::ToTokens = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&tokens),
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &naming::parameter::SelfCreateUpperCamelCase::from_tokens(&tokens),
                    };
                    quote::quote!{#value}
                };
                let ident_table_type_declaration_or_ident_create_token_stream = generate_ident_table_type_declaration_or_create_token_stream(&ident_table_type_declaration_or_ident_create_upper_camel_case, &{
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(&is_standart_with_id_false, postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_struct_declaration),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_tokens_table_type_declaration_or_create_token_stream(ident_standart_not_null_upper_camel_case))),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => wrap_into_scopes_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                                &generate_tokens_table_type_declaration_or_create_token_stream(&ident_with_id_standart_not_null_upper_camel_case)
                            )),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_tokens_table_type_declaration_or_create_token_stream(&ident_with_id_array_not_null_upper_camel_case))),
                        },
                    }
                });
                let generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_42f25108|&element_42f25108.field_ident);
                    quote::quote! {Self {#(#content_token_stream),*}}
                };
                let impl_pub_new_for_ident_table_type_declaration_or_ident_create_token_stream = {
                    let parameters_token_stream = {
                        let generate_wrap_into_value_parameter_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {value: #type_token_stream}
                        };
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(&is_standart_with_id_false, postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_new_type),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_tokens_table_type_declaration_or_create_token_stream(ident_standart_not_null_upper_camel_case))),
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&generate_tokens_table_type_declaration_or_create_token_stream(&ident_with_id_standart_not_null_upper_camel_case))),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&generate_tokens_table_type_declaration_or_create_token_stream(
                                    &ident_with_id_standart_not_null_upper_camel_case,
                                )))),
                            },
                        }
                    };
                    let content_token_stream = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(&is_standart_with_id_false),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_value_token_stream.clone(),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_array_not_null_with_id_postfix_upper_camel_case = generate_tokens_table_type_declaration_or_create_token_stream(&generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId));
                                quote::quote! {Self(#value_snake_case.map(#ident_array_not_null_with_id_postfix_upper_camel_case::new))}
                            }
                        },
                    };
                    if matches!(&postgresql_json_object_type_pattern, PostgresqlJsonObjectTypePattern::Array) && matches!(&not_null_or_nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) {
                        macros_helpers::generate_impl_pub_new_for_ident_token_stream(
                            &ident_table_type_declaration_or_ident_create_upper_camel_case,
                            &must_use_token_stream,
                            &parameters_token_stream,
                            &content_token_stream,
                        )
                    }
                    else {
                        macros_helpers::generate_impl_pub_const_new_for_ident_token_stream(
                            &ident_table_type_declaration_or_ident_create_upper_camel_case,
                            &must_use_token_stream,
                            &parameters_token_stream,
                            &content_token_stream,
                        )
                    }
                };
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream = |
                    current_ident_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens
                | postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &current_ident_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {Self #content_token_stream}
                );
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_6c071492| {
                        let field_ident = &element_6c071492.field_ident;
                        quote::quote! {#field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                    });
                    quote::quote! {{
                        #(#content_token_stream),*
                    }}
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream(&is_standart_with_id_false);
                let scopes_vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote! {(#vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)};
                let scopes_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote! {
                    (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_ident_create_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => &impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream,
                            PostgresqlJsonObjectTypePattern::Array => &scopes_vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &scopes_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    },
                );
                let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &quote::quote!{sqlx::types::Json(#self_snake_case)}
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &quote::quote!{sqlx::types::Json<#self_upper_camel_case>}
                );
                let maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = if is_standart_not_null {
                    let ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_with_id_standart_not_null_table_type_declaration_upper_camel_case,
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_with_id_standart_not_null_create_upper_camel_case,
                    };
                    let current_is_standart_with_id = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &is_standart_with_id_true,
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &is_standart_with_id_false,
                    };
                    let ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = generate_ident_table_type_declaration_or_create_token_stream(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_upper_camel_case,
                        &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(current_is_standart_with_id, postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_struct_declaration),
                    );
                    let impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = macros_helpers::generate_impl_pub_const_new_for_ident_token_stream(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_upper_camel_case,
                        &must_use_token_stream,
                        &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(current_is_standart_with_id, postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_new_type),
                        &generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(current_is_standart_with_id),
                    );
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_upper_camel_case,
                        &match &postgresql_json_type_subtype_table_type_declaration_or_create {
                            PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream(&is_standart_with_id_true),
                            PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream,
                        },
                    );
                    quote::quote! {
                        #ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                        #impl_pub_const_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_table_type_declaration_or_ident_create_token_stream
                    #impl_pub_new_for_ident_table_type_declaration_or_ident_create_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_ident_create_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream
                    #maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                }
            };

            let ident_table_type_declaration_token_stream = {
                let ident_table_type_declaration_common_token_stream = generate_ident_table_type_declaration_or_ident_create_common_token_stream(&PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration);
                quote::quote! {
                    #ident_table_type_declaration_common_token_stream
                }
            };
            let generate_type_as_postgresql_json_type_create_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_create);
            let generate_type_as_postgresql_json_type_create_for_query_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_create_for_query);
            let ident_create_token_stream = {
                let ident_create_common_token_stream = generate_ident_table_type_declaration_or_ident_create_common_token_stream(&PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create);
                let generate_impl_std_fmt_display_for_ident_create_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| macros_helpers::generate_impl_std_fmt_display_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &current_ident_token_stream, &proc_macro2::TokenStream::new(),
                    &quote::quote! {write!(f, "{self:?}")}
                );
                let impl_std_fmt_display_for_ident_create_token_stream = generate_impl_std_fmt_display_for_ident_create_token_stream(&ident_create_upper_camel_case);
                let impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_create_upper_camel_case);
                let maybe_ident_with_id_create_standart_not_null_token_stream = if is_standart_not_null {
                    let impl_std_fmt_display_for_ident_with_id_create_standart_not_null_token_stream = generate_impl_std_fmt_display_for_ident_create_token_stream(&ident_with_id_standart_not_null_create_upper_camel_case);
                    let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_with_id_standart_not_null_create_upper_camel_case);
                    quote::quote! {
                        #impl_std_fmt_display_for_ident_with_id_create_standart_not_null_token_stream
                        #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_create_common_token_stream
                    #impl_std_fmt_display_for_ident_create_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream
                    #maybe_ident_with_id_create_standart_not_null_token_stream
                }
            };
            let ident_create_for_query_upper_camel_case = naming::parameter::SelfCreateForQueryUpperCamelCase::from_tokens(&ident);
            let self_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_create_token_stream(&self_upper_camel_case);
            let ident_standart_not_null_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_array_not_null_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&ident_array_not_null_upper_camel_case);
            let ident_array_not_null_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_array_not_null_upper_camel_case);
            let postgresql_crud_path_postgresql_json_type_uuid_uuid_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let generate_debug_clone_partialeq_serialize_pub_struct_token_stream = |
                current_ident_token_stream: &dyn quote::ToTokens,
                content_token_stream: &dyn quote::ToTokens
            | macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &current_ident_token_stream,
                    &content_token_stream
                );
            let ident_create_for_query_token_stream = {
                let generate_struct_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_53c802d8| {
                        let field_ident = &element_53c802d8.field_ident;
                        let type_as_postgresql_json_type_subtype_crate_for_query_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element_53c802d8.field_type,
                            &PostgresqlJsonTypeSubtype::CreateForQuery
                        );
                        quote::quote! {#field_ident: #type_as_postgresql_json_type_subtype_crate_for_query_token_stream}
                    });
                    quote::quote! {{#(#content_token_stream),*}}
                };
                let impl_std_convert_from_standart_not_null_without_id_content_token_stream = {
                    let content_token_stream = vec_syn_field.iter().map(|element_0fc1e145| {
                        let field_ident = &element_0fc1e145.field_ident;
                        let type_as_postgresql_json_type_subtype_crate_for_query_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element_0fc1e145.field_type,
                            &PostgresqlJsonTypeSubtype::CreateForQuery
                        );
                        quote::quote! {#field_ident: #type_as_postgresql_json_type_subtype_crate_for_query_token_stream::from(#value_snake_case.#field_ident)}
                    });
                    quote::quote! {#(#content_token_stream),*}
                };
                let ident_create_for_query_token_stream = {
                    let ident_create_for_query_token_stream = generate_debug_clone_partialeq_serialize_pub_struct_token_stream(
                        &ident_create_for_query_upper_camel_case,
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_struct_standart_not_null_content_token_stream(&is_standart_with_id_false),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    wrap_into_scopes_token_stream(
                                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                                &ident_standart_not_null_upper_camel_case,
                                                &postgresql_json_type_subtype_create_for_query,
                                            )
                                        )
                                    )
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => wrap_into_scopes_token_stream(
                                    &postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                                        &ident_with_id_standart_not_null_create_for_query_upper_camel_case
                                    )
                                ),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_token_stream(
                                    &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                        &generate_type_as_postgresql_json_type_subtype_token_stream(
                                            &ident_array_not_null_upper_camel_case,
                                            &postgresql_json_type_subtype_create_for_query,
                                        )
                                    )
                                ),
                            },
                        }
                    );
                    let impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(
                        &ident_create_upper_camel_case,
                        &ident_create_for_query_upper_camel_case,
                        &{
                            let content_token_stream = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => quote::quote! {{#impl_std_convert_from_standart_not_null_without_id_content_token_stream}},
                                    PostgresqlJsonObjectTypePattern::Array => quote::quote!{(
                                        #value_snake_case.0.into_iter().map(#ident_with_id_standart_not_null_create_for_query_upper_camel_case::from).collect()
                                    )},
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream: &dyn quote::ToTokens = match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_postgresql_json_type_create_for_query_token_stream,
                                        PostgresqlJsonObjectTypePattern::Array => &ident_array_not_null_as_postgresql_json_type_create_for_query_token_stream,
                                    };
                                    quote::quote!{(#value_snake_case.0.map(#content_token_stream::from))}
                                },
                            };
                            quote::quote! {Self #content_token_stream}
                        }
                    );
                    let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
                        &ident_create_for_query_upper_camel_case,
                        &quote::quote!{sqlx::types::Json(#self_snake_case)}
                    );
                    let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                        &ident_create_for_query_upper_camel_case,
                        &quote::quote!{sqlx::types::Json<#self_upper_camel_case>}
                    );
                    quote::quote! {
                        #ident_create_for_query_token_stream
                        #impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream
                        #impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream
                        #impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream
                    }
                };
                let maybe_ident_with_id_standart_not_null_create_for_query_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_create_for_query_token_stream = generate_debug_clone_partialeq_serialize_pub_struct_token_stream(
                        &ident_with_id_standart_not_null_create_for_query_upper_camel_case,
                        &generate_struct_standart_not_null_content_token_stream(&is_standart_with_id_true)
                    );
                    let impl_std_convert_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(
                        &ident_with_id_standart_not_null_create_upper_camel_case,
                        &ident_with_id_standart_not_null_create_for_query_upper_camel_case,
                        &quote::quote! {Self {
                            #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_create_for_query_token_stream::new(
                                uuid::Uuid::new_v4()
                            ),
                            #impl_std_convert_from_standart_not_null_without_id_content_token_stream
                        }}
                    );
                    quote::quote! {
                        #ident_with_id_standart_not_null_create_for_query_token_stream
                        #impl_std_convert_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_create_for_query_token_stream
                    #maybe_ident_with_id_standart_not_null_create_for_query_token_stream
                }
            };
            let generate_sqlx_types_json_type_declaration_wrapper_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                &current_ident_token_stream,
                &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&self_upper_camel_case)
            );
            let generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                &current_ident_token_stream,
                &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&self_upper_camel_case),
                &quote::quote! {Ok(value_147c3532.0)}
            );
            let generate_value_type_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#value_snake_case: #type_token_stream}
            };
            let generate_pub_const_new_value_type_content_self_value_token_stream = |content_token_stream: &dyn quote::ToTokens|macros_helpers::generate_pub_const_new_token_stream(
                &must_use_token_stream,
                &generate_value_type_token_stream(&content_token_stream),
                &self_value_token_stream
            );
            let generate_unique_vec_wrapper_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#import_path::NotEmptyUniqueVec<#type_token_stream>}
            };
            let self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote! {
                Self(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
            };
            let self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream = quote::quote! {
                Self(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream))
            };
            let wrap_content_into_scopes_dot_comma_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                let scopes_content_token_stream = postgresql_crud_macros_common::wrap_content_into_scopes_token_stream(&content_token_stream);
                quote::quote! {#scopes_content_token_stream;}
            };
            let generate_type_as_postgresql_json_type_update_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_update);
            let generate_type_as_postgresql_json_type_update_for_query_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_update_for_query);
            let self_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&self_upper_camel_case);
            let self_as_postgresql_json_type_update_token_stream = generate_type_as_postgresql_json_type_update_token_stream(&self_upper_camel_case);
            let self_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&self_upper_camel_case);
            let postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream = generate_type_as_postgresql_json_type_update_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let postgresql_crud_path_postgresql_json_type_uuid_uuid_update_for_query_token_stream = generate_type_as_postgresql_json_type_update_for_query_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
            let ident_with_id_standart_not_null_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let generate_type_as_postgresql_json_type_select_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_select);
            let ident_standart_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_select_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_select_token_stream(&ident_with_id_array_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_select_snake_case = naming::parameter::SelfSelectSnakeCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let dimension1_pagination_token_stream = quote::quote! {dimension1_pagination};
            let ident_standart_not_null_select_element_upper_camel_case = naming::parameter::SelfSelectElementUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_select_element_upper_camel_case = naming::parameter::SelfSelectElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let generate_select_query_part_for_loop_token_stream = |
                acc_token_stream: &dyn quote::ToTokens,
                is_standart_with_id: &IsStandartWithId,
                in_token_stream: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_field_ident_token_stream: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_for_error_message_field_ident_token_stream: &dyn quote::ToTokens,
            |{
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_f3a1af0f| {
                    let field_ident_stringified = element_f3a1af0f.field_ident.to_string();
                    let variant_name_token_stream: &dyn quote::ToTokens = &naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                    let field_ident_double_quotes_token_stream: &dyn quote::ToTokens = &generate_quotes::double_quotes_token_stream(&field_ident_stringified);
                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_f3a1af0f.field_type);
                    let ident_or_ident_with_id_standart_not_null_select_element_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_element_upper_camel_case,
                        IsStandartWithId::False => &ident_standart_not_null_select_element_upper_camel_case
                    };
                    quote::quote! {
                        #ident_or_ident_with_id_standart_not_null_select_element_upper_camel_case::#variant_name_token_stream(value_3c8acf6a) => match #field_type_as_crud_postgresql_json_type_from_field_token_stream::#select_query_part_snake_case(
                            value_3c8acf6a,
                            #field_ident_double_quotes_token_stream,
                            #column_name_and_maybe_field_getter_field_ident_token_stream,
                            #column_name_and_maybe_field_getter_for_error_message_field_ident_token_stream,
                            false,
                        ) {
                            Ok(value_d54cf786) => value_d54cf786,
                            Err(#error_snake_case) => {
                                return Err(#error_snake_case);
                            }
                        }
                    }
                });
                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                    &quote::quote!{
                        #acc_token_stream,
                        "{}||",
                        match element_0127bf54 {
                            #(#content_token_stream),*
                        }
                    },
                    &return_err_query_part_error_named_write_into_buffer_token_stream
                );
                quote::quote!{
                    for element_0127bf54 in #in_token_stream #self_field_vec_token_stream {
                        #if_write_is_err_token_stream
                    }
                }
            };
            let ident_select_token_stream = {
                let generate_pub_struct_ident_select_token_stream = |
                    current_ident_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens
                | macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &current_ident_token_stream,
                    &content_token_stream
                );
                let generate_ident_select_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let ident_standart_not_null_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
                    generate_pub_struct_ident_select_token_stream(
                        match &is_standart_with_id {
                            IsStandartWithId::False => &ident_standart_not_null_select_upper_camel_case,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_select_upper_camel_case,
                        },
                        &wrap_content_into_scopes_dot_comma_token_stream(&generate_unique_vec_wrapper_token_stream(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_standart_not_null_select_element_upper_camel_case,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_select_element_upper_camel_case,
                        })),
                    )
                };
                let import_path_pagination_token_stream = quote::quote! {#import_path::PaginationStartsWithZero};
                let ident_select_token_stream = match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => generate_ident_select_standart_not_null_token_stream(&is_standart_with_id_false),
                        PostgresqlJsonObjectTypePattern::Array => generate_pub_struct_ident_select_token_stream(
                            &ident_select_upper_camel_case,
                            &quote::quote! {{
                                #ident_with_id_standart_not_null_select_snake_case: #ident_with_id_standart_not_null_select_upper_camel_case,
                                #dimension1_pagination_token_stream: #import_path_pagination_token_stream
                            }},
                        ),
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_struct_ident_select_token_stream(
                        &ident_select_upper_camel_case,
                        &wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_postgresql_json_type_select_token_stream,
                            PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream,
                        })),
                    ),
                };
                let impl_ident_select_token_stream = {
                    let pub_new_token_stream = {
                        let parameters_token_stream = {
                            let unique_vec_ident_select_element_standart_not_null_token_stream = generate_unique_vec_wrapper_token_stream(&ident_standart_not_null_select_element_upper_camel_case);
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_value_type_token_stream(&unique_vec_ident_select_element_standart_not_null_token_stream),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_type_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&unique_vec_ident_select_element_standart_not_null_token_stream)),
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                        #ident_with_id_standart_not_null_select_snake_case: #ident_with_id_standart_not_null_select_upper_camel_case,
                                        #dimension1_pagination_token_stream: #import_path_pagination_token_stream
                                    },
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_type_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream)),
                                },
                            }
                        };
                        let content_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                    Self(#value_snake_case.map(#ident_standart_not_null_as_postgresql_json_type_select_token_stream::new))
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    quote::quote! {Self {
                                        #ident_with_id_standart_not_null_select_snake_case,
                                        #dimension1_pagination_token_stream,
                                    }}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_value_token_stream.clone(),
                            },
                        };
                        if matches!(&postgresql_json_object_type_pattern, PostgresqlJsonObjectTypePattern::Standart) && matches!(&not_null_or_nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) {
                            macros_helpers::generate_pub_new_token_stream(
                                &must_use_token_stream,
                                &parameters_token_stream,
                                &content_token_stream
                            )
                        }
                        else {
                             macros_helpers::generate_pub_const_new_token_stream(
                                &must_use_token_stream,
                                &parameters_token_stream,
                                &content_token_stream
                            )
                        }
                    };
                    let maybe_select_query_part_token_stream = if matches!(&postgresql_json_object_type_pattern, PostgresqlJsonObjectTypePattern::Standart) &&
                    matches!(&not_null_or_nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) {
                        let acc_ac57d097_token_stream = quote::quote!{acc_ac57d097};
                        let select_query_part_for_loop_token_stream = generate_select_query_part_for_loop_token_stream(
                            &acc_ac57d097_token_stream,
                            &is_standart_with_id_false,
                            &self_snake_case,
                            &quote::quote!{column_name_and_maybe_field_getter},
                            &quote::quote!{column_name_and_maybe_field_getter_for_error_message},
                        );
                        quote::quote! {
                            fn #select_query_part_snake_case(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                column_name_and_maybe_field_getter_for_error_message: &str,
                            ) -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                                let mut #acc_ac57d097_token_stream = #std_string_string_token_stream::default();
                                #select_query_part_for_loop_token_stream
                                let _: Option<char> = #acc_ac57d097_token_stream.pop();
                                let _: Option<char> = #acc_ac57d097_token_stream.pop();
                                Ok(#acc_ac57d097_token_stream)
                            }
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    };
                    let select_query_part_postgresql_type_token_stream = {
                        let content_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                    #self_snake_case.#select_query_part_snake_case(
                                        #column_snake_case,
                                        #column_snake_case,
                                    )
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let ident_740c9034 = match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_postgresql_json_type_select_token_stream,
                                        PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream,
                                    };
                                    quote::quote! {
                                        let #value_snake_case = self.0.as_ref().map_or_else(
                                            <#ident_740c9034 as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element,
                                            Clone::clone
                                        );
                                        match #value_snake_case.#select_query_part_postgresql_type_snake_case(#column_snake_case) {
                                            Ok(value_c69f1ffe) => Ok(format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c69f1ffe}) end")),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    }
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let acc_399d9786_token_stream = quote::quote!{acc_399d9786};
                                    let select_query_part_for_loop_token_stream = generate_select_query_part_for_loop_token_stream(
                                        &acc_399d9786_token_stream,
                                        &is_standart_with_id_true,
                                        &quote::quote!{#self_snake_case.#ident_with_id_standart_not_null_select_snake_case},
                                        &generate_quotes::double_quotes_token_stream(&value_snake_case),
                                        &column_snake_case
                                    );
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                                        "(case when (jsonb_array_length({{column}}) = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_snake_case}}})) from jsonb_array_elements((select {{column}})) with ordinality where ordinality between {{dimension1_start}} and {{dimension1_end}}) end)"
                                    ));
                                    quote::quote! {
                                        let #ident_with_id_standart_not_null_select_snake_case = {
                                            let mut #acc_399d9786_token_stream = #std_string_string_token_stream::default();
                                            #select_query_part_for_loop_token_stream
                                            let _: Option<char> = #acc_399d9786_token_stream.pop();
                                            let _: Option<char> = #acc_399d9786_token_stream.pop();
                                            #acc_399d9786_token_stream
                                        };
                                        let dimension1_start = self.#dimension1_pagination_token_stream.start();
                                        let dimension1_end = self.#dimension1_pagination_token_stream.end();
                                        Ok(format!(#format_handle_token_stream))
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c2ca032e}) end");
                                    let ident_with_id_array_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream = generate_ident_as_default_but_option_is_always_some_token_stream(
                                        &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream
                                    );
                                    quote::quote! {
                                        let #value_snake_case = self.0.as_ref().map_or_else(
                                            #ident_with_id_array_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream,
                                            Clone::clone
                                        );
                                        match #value_snake_case.#select_query_part_postgresql_type_snake_case(column) {
                                            Ok(value_c2ca032e) => Ok(format!(#format_handle_token_stream)),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    }
                                },
                            },
                        };
                        quote::quote! {
                            fn #select_query_part_postgresql_type_snake_case(
                                &self,
                                #column_snake_case: &str,
                            ) -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                                #content_token_stream
                            }
                        }
                    };
                    quote::quote! {
                        impl #ident_select_upper_camel_case {
                            #pub_new_token_stream
                            #maybe_select_query_part_token_stream
                            #select_query_part_postgresql_type_token_stream
                        }
                    }
                };
                let impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_select_upper_camel_case);
                let impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_select_upper_camel_case);
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream = quote::quote! {
                    Self(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_standart_not_null_content_token_stream = quote::quote! {
                    Self(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream)
                };
                let (
                    impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream,
                    impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_select_token_stream
                ) = {
                    let generate_default_some_one_content_token_stream = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => quote::quote! {#impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream},
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => quote::quote! {#impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_standart_not_null_content_token_stream},
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.clone(),
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream.clone(),
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let (
                                        ident_with_id_standart_not_null_select_content_token_stream,
                                        dimension1_pagination_content_token_stream
                                    ): (
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens
                                    ) = match &default_some_one_or_default_some_one_with_max_page_size {
                                        postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => (
                                            &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                            &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                        ),
                                        postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => (
                                            &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream,
                                            &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream
                                        ),
                                    };
                                    quote::quote! {
                                        Self {
                                            #ident_with_id_standart_not_null_select_snake_case: #ident_with_id_standart_not_null_select_content_token_stream,
                                            #dimension1_pagination_token_stream: #dimension1_pagination_content_token_stream,
                                        }
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.clone(),
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream.clone(),
                                },
                            },
                        }
                    };
                    (
                        postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_select_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                        ),
                        postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
                            &ident_select_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                        )
                    )
                };
                let generate_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let ident_select_element_or_ident_with_id_select_element_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_standart_not_null_select_element_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_element_upper_camel_case,
                    };
                    let ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream = {
                        let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_840c2253| {
                            let field_ident = &element_840c2253.field_ident;
                            let serialize_deserialize_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let field_type_as_json_type_select_token_stream = generate_type_as_postgresql_json_type_select_token_stream(&element_840c2253.field_type);
                            quote::quote! {
                                #[serde(rename(serialize = #serialize_deserialize_field_ident_double_quotes_token_stream, deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream))]
                                #variant_ident_upper_camel_case_token_stream(#field_type_as_json_type_select_token_stream)
                            }
                        });
                        macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_utoipa_to_schema()
                            .derive_schemars_json_schema()
                            .build_enum(
                                &ident_select_element_or_ident_with_id_select_element_upper_camel_case,
                                &quote::quote!{{#(#content_token_stream),*}}
                            )
                    };
                    let impl_error_occurence_lib_to_std_string_string_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_select_element_or_ident_with_id_select_element_upper_camel_case);
                    let (
                        impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream,
                        impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_with_max_page_size_token_stream
                    ) = {
                        let generate_default_some_one_content_token_stream = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                            let vec_content_token_stream = {
                                let content_token_stream: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                    postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream,
                                };
                                let elements_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_20fc16d2| {
                                    let field_ident = &element_20fc16d2.field_ident;
                                    let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    quote::quote! {#self_upper_camel_case::#field_ident_upper_camel_case_token_stream(#content_token_stream)}
                                });
                                quote::quote! {#(#elements_token_stream),*}
                            };
                            quote::quote! {vec![
                                #vec_content_token_stream
                            ]}
                        };
                        (
                            postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                                &ident_select_element_or_ident_with_id_select_element_upper_camel_case,
                                &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                            ),
                            postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
                                &ident_select_element_or_ident_with_id_select_element_upper_camel_case,
                                &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                            )
                        )
                    };
                    quote::quote! {
                        #ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream
                        #impl_error_occurence_lib_to_std_string_string_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream
                        #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream
                        #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_standart_not_null_select_element_with_max_page_size_token_stream
                    }
                };
                let maybe_ident_select_element_token_stream = if is_standart_not_null { generate_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream(&is_standart_with_id_false) } else { proc_macro2::TokenStream::new() };
                let maybe_ident_with_id_standart_not_null_select_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_select_token_stream = generate_ident_select_standart_not_null_token_stream(&is_standart_with_id_true);
                    let impl_ident_with_id_standart_not_null_select_token_stream = {
                        let impl_new_for_ident_with_id_standart_not_null_select_token_stream = generate_pub_const_new_value_type_content_self_value_token_stream(
                            &generate_unique_vec_wrapper_token_stream(
                                &ident_with_id_standart_not_null_select_element_upper_camel_case
                            )
                        );
                        quote::quote!{
                            impl #ident_with_id_standart_not_null_select_upper_camel_case {
                                #impl_new_for_ident_with_id_standart_not_null_select_token_stream
                            }
                        }
                    };
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_select_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_with_id_standart_not_null_select_upper_camel_case);
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_select_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_with_id_standart_not_null_select_upper_camel_case);
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_with_id_standart_not_null_select_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream
                    );
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_with_id_standart_not_null_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
                        &ident_with_id_standart_not_null_select_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_standart_not_null_content_token_stream
                    );
                    let ident_with_id_select_element_token_stream = generate_ident_select_element_or_ident_with_id_standart_not_null_select_element_token_stream(&is_standart_with_id_true);
                    quote::quote! {
                        #ident_with_id_standart_not_null_select_token_stream
                        #impl_ident_with_id_standart_not_null_select_token_stream
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_select_token_stream
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_select_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_select_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_with_id_standart_not_null_select_token_stream
                        #ident_with_id_select_element_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_select_token_stream
                    #impl_ident_select_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_select_token_stream
                    #maybe_ident_select_element_token_stream
                    #maybe_ident_with_id_standart_not_null_select_token_stream
                }
            };
            let ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident);
            let ident_where_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let generate_ident_where_field_variants_token_stream = |is_standart_with_id: &IsStandartWithId| {
                        let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_622d1e96| {
                            let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&element_622d1e96.field_ident.to_string());
                            let field_type_as_json_type_where_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element_622d1e96.field_type,
                                &postgresql_json_type_subtype_where
                            );
                            quote::quote! {
                                #field_ident_upper_camel_case_token_stream(#import_path::PostgresqlTypeWhere<
                                    #field_type_as_json_type_where_token_stream
                                >)
                            }
                        });
                        quote::quote! {#(#variants_token_stream),*}
                    };
                    let generate_ident_where_token_stream = |
                        current_ident_token_stream: &dyn quote::ToTokens,
                        content_token_stream: &dyn quote::ToTokens
                    | macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_enum(
                            &current_ident_token_stream,
                            &quote::quote!{{#content_token_stream}}
                        );
                    let equal_variant_ident_token_stream = quote::quote! {#equal_upper_camel_case(#import_path::PostgresqlJsonTypeWhereEqual<#ident_as_postgresql_json_type_table_type_declaration_token_stream>)};
                    let equal_variant_query_part_token_stream = quote::quote!{
                        #self_upper_camel_case::#equal_upper_camel_case(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::query_part(
                            #value_snake_case,
                            #increment_snake_case,
                            &#column_snake_case,
                            is_need_to_add_logical_operator
                        )
                    };
                    let equal_variant_query_bind_token_stream = quote::quote!{
                        #self_upper_camel_case::#equal_upper_camel_case(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream
                    };
                    let maybe_ident_where_token_stream = {
                        let generate_ident_where_wrapper_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_ident_where_token_stream(
                            &ident_where_upper_camel_case,
                            &content_token_stream
                        );
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => generate_ident_where_wrapper_token_stream(&{
                                    let ident_where_field_variants_token_stream = generate_ident_where_field_variants_token_stream(&is_standart_with_id_false);
                                    quote::quote!{
                                        #ident_where_field_variants_token_stream,
                                        #equal_variant_ident_token_stream,
                                    }
                                }),
                                PostgresqlJsonObjectTypePattern::Array => generate_ident_where_wrapper_token_stream(&{
                                    let dimension_one_equal_token_stream = quote::quote! {
                                        DimensionOneEqual(#import_path::PostgresqlJsonTypeWhereDimensionOneEqual<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let length_equal_token_stream = quote::quote! {
                                        LengthEqual(#import_path::PostgresqlJsonTypeWhereLengthEqual),
                                    };
                                    let length_greater_than_token_stream = quote::quote! {
                                        LengthGreaterThan(#import_path::PostgresqlJsonTypeWhereLengthGreaterThan),
                                    };
                                    let in_token_stream = quote::quote! {
                                        In(#import_path::PostgresqlJsonTypeWhereIn<#ident_as_postgresql_json_type_table_type_declaration_token_stream>),
                                    };
                                    let dimension_one_in_token_stream = quote::quote! {
                                        DimensionOneIn(#import_path::PostgresqlJsonTypeWhereDimensionOneIn<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let contains_all_elements_of_array_token_stream = quote::quote! {
                                        ContainsAllElementsOfArray(#import_path::PostgresqlJsonTypeWhereContainsAllElementsOfArray<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let overlaps_with_array_token_stream = quote::quote! {
                                        OverlapsWithArray(#import_path::PostgresqlJsonTypeWhereOverlapsWithArray<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element_3e7f45d9| {
                                        let field_ident = &element_3e7f45d9.field_ident;
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        let element_type_as_postgresql_json_type_where_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                            &element_3e7f45d9.field_type,
                                            &PostgresqlJsonTypeSubtype::Where
                                        );
                                        quote::quote! {
                                            #element_field_ident_upper_camel_case(#import_path::PostgresqlTypeWhere<
                                                #element_type_as_postgresql_json_type_where_token_stream
                                            >)
                                        }
                                    });
                                    quote::quote! {
                                        #equal_variant_ident_token_stream,
                                        #dimension_one_equal_token_stream
                                        #length_equal_token_stream
                                        #length_greater_than_token_stream
                                        #in_token_stream
                                        #dimension_one_in_token_stream
                                        #contains_all_elements_of_array_token_stream
                                        #overlaps_with_array_token_stream
                                        #(#element_filters_token_stream),*
                                    }
                                }),
                            },
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        }
                    };
                    let generate_where_filter_query_part_fields_content_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId| {
                        let query_part_variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_32d414b1| {
                            let field_ident_stringified = element_32d414b1.field_ident.to_string();
                            let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{column}}->'{field_ident_stringified}'"));
                            quote::quote! {
                                Self::#field_ident_upper_camel_case_token_stream(value) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                    value,
                                    increment,
                                    &format!(#format_handle_token_stream),
                                    is_need_to_add_logical_operator,
                                )
                            }
                        });
                        quote::quote! {#(#query_part_variants_token_stream),*}
                    };
                    let generate_where_filter_query_bind_fields_content_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId| {
                        let query_bind_variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_ee6a2665| {
                            let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&element_ee6a2665.field_ident.to_string());
                            quote::quote! {
                                Self::#field_ident_upper_camel_case_token_stream(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream
                            }
                        });
                        quote::quote! {#(#query_bind_variants_token_stream),*}
                    };
                    let generate_impl_postgresql_type_where_filter_token_stream = |
                        current_ident_token_stream: &dyn quote::ToTokens,
                        query_part_content_token_stream: &dyn quote::ToTokens,
                        is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable,
                        query_bind_content_token_stream: &dyn quote::ToTokens
                    | {
                        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                            &quote::quote! {<'lifetime>},
                            &current_ident_token_stream,
                            &proc_macro2::TokenStream::new(),
                            &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
                            &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                            &query_part_content_token_stream,
                            &is_query_bind_mutable,
                            &query_bind_content_token_stream,
                            &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                        )
                    };
                    let maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_token_stream = {
                        let generate_impl_postgresql_type_where_filter_for_ident_token_stream = |query_part_content_token_stream: &dyn quote::ToTokens, is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable, query_bind_content_token_stream: &dyn quote::ToTokens| generate_impl_postgresql_type_where_filter_token_stream(&ident_where_upper_camel_case, &query_part_content_token_stream, is_query_bind_mutable, &query_bind_content_token_stream);
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => generate_impl_postgresql_type_where_filter_for_ident_token_stream(
                                    &{
                                        let fields_content_token_stream = generate_where_filter_query_part_fields_content_standart_not_null_token_stream(&is_standart_with_id_false);
                                        quote::quote!{
                                            match &self {
                                                #fields_content_token_stream,
                                                #equal_variant_query_part_token_stream,
                                            }
                                        }
                                    },
                                    postgresql_crud_macros_common::IsQueryBindMutable::False,
                                    &{
                                        let fields_content_token_stream = generate_where_filter_query_bind_fields_content_standart_not_null_token_stream(&is_standart_with_id_false);
                                        quote::quote!{
                                            match self {
                                                #fields_content_token_stream,
                                                #equal_variant_query_bind_token_stream,
                                            }
                                        }
                                    }
                                ),
                                NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                            },
                            PostgresqlJsonObjectTypePattern::Array => generate_impl_postgresql_type_where_filter_for_ident_token_stream(
                                &{
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element_7845d48a| {
                                        let field_ident = &element_7845d48a.field_ident;
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        quote::quote! {
                                            Self::#element_field_ident_upper_camel_case(#value_snake_case) => generate_element_query(
                                                #value_snake_case.get_logical_operator(),
                                                #value_snake_case,
                                                #field_ident_double_quotes_token_stream
                                            )
                                        }
                                    });
                                    quote::quote! {
                                        let mut generate_element_query = |
                                            logical_operator: &#import_path::LogicalOperator,
                                            #value_snake_case: &dyn #import_path::PostgresqlTypeWhereFilter<'_>,
                                            field: &str
                                        | -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                                            let logical_operator_query_part = logical_operator.to_query_part(is_need_to_add_logical_operator);
                                            let elem = "elem";
                                            let value_9696ee60 = match #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                &format!("{elem}->'{field}'"),
                                                false
                                            ) {
                                                Ok(value_c7ec4e53) => value_c7ec4e53,
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            };
                                            Ok(format!("{logical_operator_query_part}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {value_9696ee60}))"))
                                        };
                                        match &self {
                                            Self::Equal(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::DimensionOneEqual(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::LengthEqual(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::LengthGreaterThan(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::In(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::DimensionOneIn(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::ContainsAllElementsOfArray(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            Self::OverlapsWithArray(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                #column_snake_case,
                                                #is_need_to_add_logical_operator_snake_case
                                            ),
                                            #(#element_filters_token_stream),*
                                        }
                                    }
                                },
                                postgresql_crud_macros_common::IsQueryBindMutable::False,
                                &{
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element_9956277c| {
                                        let field_ident = &element_9956277c.field_ident;
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        quote::quote! {Self::#element_field_ident_upper_camel_case(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream}
                                    });
                                    quote::quote! {
                                        match self {
                                            Self::Equal(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::DimensionOneEqual(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::LengthEqual(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::LengthGreaterThan(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::In(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::DimensionOneIn(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::ContainsAllElementsOfArray(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::OverlapsWithArray(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            #(#element_filters_token_stream),*
                                        }
                                    }
                                },
                            ),
                        }
                    };
                    let maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_token_stream = if matches!((&postgresql_json_object_type_pattern, &not_null_or_nullable), (PostgresqlJsonObjectTypePattern::Standart, NotNullOrNullable::Nullable)) {
                        proc_macro2::TokenStream::new()
                    } else {
                        generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_where_upper_camel_case)
                    };
                    let generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where = |is_standart_with_id: &IsStandartWithId| {
                        let generate_self_variant_default_some_one_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{
                            Self::#content_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                        };
                        let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_7f3524bc| {
                            generate_self_variant_default_some_one_token_stream(&naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&element_7f3524bc.field_ident.to_string()))
                        });
                        let self_equal_default_some_one_token_stream = generate_self_variant_default_some_one_token_stream(&equal_upper_camel_case);
                        quote::quote! {vec![
                            #(#variants_token_stream),*,
                            #self_equal_default_some_one_token_stream
                        ]}
                    };
                    let maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_token_stream = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_where_upper_camel_case, &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where(&is_standart_with_id_false)),
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                        PostgresqlJsonObjectTypePattern::Array => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_where_upper_camel_case, &{
                            let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element_a3184731| {
                                let field_ident = &element_a3184731.field_ident;
                                let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                quote::quote! {Self::#element_field_ident_upper_camel_case(#import_path_default_but_option_is_always_some_call_token_stream)}
                            });
                            quote::quote! {
                                vec![
                                    Self::Equal(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::DimensionOneEqual(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::LengthEqual(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::LengthGreaterThan(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::In(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::DimensionOneIn(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::ContainsAllElementsOfArray(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::OverlapsWithArray(#import_path_default_but_option_is_always_some_call_token_stream),
                                    #(#element_filters_token_stream),*
                                ]
                            }
                        }),
                    };
                    let maybe_ident_with_id_standart_not_null_where_token_stream = if is_standart_not_null {
                        let ident_with_id_standart_not_null_where_token_stream = generate_ident_where_token_stream(
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &{
                                let ident_where_field_variants_token_stream = generate_ident_where_field_variants_token_stream(&is_standart_with_id_true);
                                quote::quote!{
                                    #ident_where_field_variants_token_stream,
                                    #equal_upper_camel_case(#import_path::PostgresqlJsonTypeWhereEqual<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),//todo maybe reuse? variant generation
                                }
                            }
                        );
                        let impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_standart_not_null_where_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &{
                                let fields_content_token_stream = generate_where_filter_query_part_fields_content_standart_not_null_token_stream(&is_standart_with_id_true);
                                quote::quote!{
                                    match &self {
                                        #fields_content_token_stream,
                                        Self::#equal_upper_camel_case(#value_snake_case) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                                            #value_snake_case,
                                            #increment_snake_case,
                                            &#column_snake_case,
                                            is_need_to_add_logical_operator
                                        ),//todo maybe reuse? variant generation
                                    }
                                }
                            },
                            postgresql_crud_macros_common::IsQueryBindMutable::False,
                            &{
                                let fields_content_token_stream = generate_where_filter_query_bind_fields_content_standart_not_null_token_stream(&is_standart_with_id_true);
                                quote::quote!{
                                    match self {
                                        #fields_content_token_stream,
                                        Self::#equal_upper_camel_case(#value_snake_case) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#value_snake_case, #query_snake_case),//todo maybe reuse? variant generation
                                    }
                                }
                            },
                        );
                        let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_with_id_standart_not_null_where_upper_camel_case);
                        let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_where_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_with_id_standart_not_null_where_upper_camel_case,
                            &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where(&is_standart_with_id_true)
                        );
                        quote::quote! {
                            #ident_with_id_standart_not_null_where_token_stream
                            #impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_standart_not_null_where_token_stream
                            #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_token_stream
                            #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_where_token_stream
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    quote::quote! {
                        #maybe_ident_where_token_stream
                        #maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_token_stream
                        #maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_token_stream
                        #maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_token_stream
                        #maybe_ident_with_id_standart_not_null_where_token_stream
                    }
                }
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let ident_standart_or_ident_with_id_array_as_postgresql_json_type_where_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                            PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_upper_camel_case,
                        },
                        &postgresql_json_type_subtype_where
                    );
                    quote::quote! {
                        pub type #ident_where_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                            #ident_standart_or_ident_with_id_array_as_postgresql_json_type_where_token_stream
                        >;
                    }
                }
            };
            let generate_field_ident_double_quotes_token_stream = |value: &macros_helpers::SynFieldWrapper| {
                generate_quotes::double_quotes_token_stream(&value.field_ident)
            };
            let generate_type_as_postgresql_json_type_read_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_read);
            let generate_type_as_postgresql_json_type_read_inner_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_read_inner);
            let generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream = |
                is_standart_with_id: &IsStandartWithId,
                read_with_or_without_annotation_or_inner: &ReadWithOrWithoutAnnotationOrInner
            | {
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_274293a0| {
                    let maybe_serde_skip_serializing_if_option_is_none_token_stream = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation => quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::Inner => proc_macro2::TokenStream::new(),
                    };
                    let field_ident = &element_274293a0.field_ident;
                    let field_type_as_json_type_read_token_stream = match &read_with_or_without_annotation_or_inner {
                        ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation |
                        ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation => generate_type_as_postgresql_json_type_read_token_stream(
                            &element_274293a0.field_type
                        ),
                        ReadWithOrWithoutAnnotationOrInner::Inner => generate_type_as_postgresql_json_type_read_inner_token_stream(
                            &element_274293a0.field_type
                        ),
                    };
                    let std_option_option_value_field_type_as_json_type_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                        &wrap_into_value_declaration_token_stream(&field_type_as_json_type_read_token_stream)
                    );
                    quote::quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_token_stream
                        #field_ident: #std_option_option_value_field_type_as_json_type_read_token_stream
                    }
                });
                quote::quote! {#(#content_token_stream),*}
            };
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_read_token_stream = {
                let ident_read_try_from_error_named_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident);
                let ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
                let ident_standart_not_null_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&ident_standart_not_null_upper_camel_case);
                let ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&ident_with_id_array_not_null_upper_camel_case);
                let generate_ident_read_token_stream = |
                    current_ident_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    derive_serde_deserialize: macros_helpers::DeriveSerdeDeserialize
                | macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize_if(derive_serde_deserialize)
                    .derive_utoipa_to_schema()
                    .derive_schemars_json_schema()
                    .build_struct(
                        &current_ident_token_stream,
                        &content_token_stream
                    );
                let ident_read_token_stream = {
                    let (content_token_stream, derive_serde_deserialize) = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                {
                                    let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(
                                        &is_standart_with_id_false,
                                        &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                                    );
                                    quote::quote! {{#content_token_stream}}
                                },
                                macros_helpers::DeriveSerdeDeserialize::False,
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_as_postgresql_json_type_read_token_stream)), macros_helpers::DeriveSerdeDeserialize::True),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case)), macros_helpers::DeriveSerdeDeserialize::True),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream)), macros_helpers::DeriveSerdeDeserialize::True),
                        },
                    };
                    generate_ident_read_token_stream(&ident_read_upper_camel_case, &content_token_stream, derive_serde_deserialize)
                };
                let all_fields_are_none_upper_camel_case = naming::AllFieldsAreNoneUpperCamelCase;
                let generate_ident_read_try_from_error_named_token_stream = |current_ident_token_stream: &dyn quote::ToTokens|macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_error_occurence_lib_error_occurence()
                    .build_enum(
                        &current_ident_token_stream,
                        &quote::quote!{{
                            #all_fields_are_none_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }}
                    );
                let maybe_ident_read_try_from_error_named_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_read_try_from_error_named_token_stream(&ident_read_try_from_error_named_upper_camel_case),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => proc_macro2::TokenStream::new(),
                };
                let generate_ident_read_or_ident_with_id_standart_not_null_read_upper_camel_case = |is_standart_with_id: &IsStandartWithId| match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_upper_camel_case,
                };
                let generate_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_try_from_error_named_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case,
                    };
                    macros_helpers::generate_pub_try_new_token_stream(
                        &generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(
                            is_standart_with_id,
                            &ReadWithOrWithoutAnnotationOrInner::WithoutSerdeOptionIsNoneAnnotation
                        ),
                        &ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case,
                        &{
                            let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                            let (fields_reference_token_stream, fields_token_stream) = {
                                enum WithReference {
                                    True,
                                    False,
                                }
                                let generate_fields_token_stream = |with_reference: &WithReference| {
                                    let maybe_reference_symbol_token_stream = match &with_reference {
                                        WithReference::True => quote::quote! {&},
                                        WithReference::False => proc_macro2::TokenStream::new(),
                                    };
                                    let fields_token_stream = current_vec_syn_field.iter().map(|element_a6b6e788| {
                                        let field_ident = &element_a6b6e788.field_ident;
                                        quote::quote! {#maybe_reference_symbol_token_stream #field_ident}
                                    });
                                    quote::quote! {
                                        #(#fields_token_stream),*
                                    }
                                };
                                (generate_fields_token_stream(&WithReference::True), generate_fields_token_stream(&WithReference::False))
                            };
                            let check_if_all_fields_are_none_token_stream = {
                                let current_vec_syn_field_len = current_vec_syn_field.len();
                                let maybe_wrap_into_braces_handle_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::maybe_wrap_into_braces_token_stream(
                                    content_token_stream,
                                    current_vec_syn_field_len > 1
                                );
                                let left_token_stream = maybe_wrap_into_braces_handle_token_stream(&fields_reference_token_stream);
                                let right_token_stream = maybe_wrap_into_braces_handle_token_stream(&{
                                    let nones_token_stream = std::iter::repeat_with(||quote::quote!{None}).take(current_vec_syn_field_len);
                                    quote::quote! {#(#nones_token_stream),*}
                                });
                                let content_token_stream = if current_vec_syn_field_len == 1 {
                                    let content_token_stream = maybe_wrap_into_braces_handle_token_stream(&fields_token_stream);
                                    quote::quote! {#content_token_stream.is_none()}
                                }
                                else {
                                    quote::quote! {matches!(#left_token_stream, #right_token_stream)}
                                };
                                quote::quote! {
                                    if #content_token_stream {
                                        return Err(#ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case::#all_fields_are_none_upper_camel_case {
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            };
                            quote::quote!{
                                #check_if_all_fields_are_none_token_stream
                                Ok(Self{#fields_token_stream})
                            }
                        }
                    )
                };
                let impl_ident_read_token_stream = {
                    let pub_new_or_try_new_token_stream = {
                        let std_vec_vec_ident_with_id_standart_not_null_read_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_false),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_pub_const_new_token_stream(
                                    &must_use_token_stream,
                                    &generate_value_type_token_stream(
                                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                            &ident_standart_not_null_as_postgresql_json_type_read_token_stream
                                        )
                                    ),
                                    &self_value_token_stream
                                ),
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_pub_const_new_token_stream(
                                    &must_use_token_stream,
                                    &generate_value_type_token_stream(
                                        &std_vec_vec_ident_with_id_standart_not_null_read_token_stream
                                    ),
                                    &self_value_token_stream
                                ),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_pub_new_token_stream(
                                    &must_use_token_stream,
                                    &generate_value_type_token_stream(
                                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                            &std_vec_vec_ident_with_id_standart_not_null_read_token_stream
                                        )
                                    ),
                                    &quote::quote! {Self(#value_snake_case.map(#ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream::new))},
                                ),
                            },
                        }
                    };
                    quote::quote!{
                        impl #ident_read_upper_camel_case {
                            #pub_new_or_try_new_token_stream
                        }
                    }
                };
                let generate_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                    postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(
                        &generate_ident_read_or_ident_with_id_standart_not_null_read_upper_camel_case(is_standart_with_id),
                        &current_vec_syn_field.iter().map(|element_00a75629|
                            (&element_00a75629.field_ident, &element_00a75629.field_type)
                        ).collect::<Vec<(&syn::Ident, &syn::Type)>>(),
                        current_vec_syn_field.len(),
                        &|_: &syn::Ident, syn_type: &syn::Type| {
                            let type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&syn_type);
                            postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                &wrap_into_value_declaration_token_stream(&type_read_token_stream)
                            )
                        }
                    )
                };
                let maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_false),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => proc_macro2::TokenStream::new(),
                };
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&generate_ident_read_or_ident_with_id_standart_not_null_read_upper_camel_case(is_standart_with_id), &proc_macro2::TokenStream::new(), &{
                        let fields_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_6a2035df| {
                            let field_ident = &element_6a2035df.field_ident;
                            let value_content_token_stream = wrap_into_value_initialization_token_stream(
                                &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            );
                            quote::quote! {#field_ident: Some(#value_content_token_stream)}
                        });
                        quote::quote! {Self{#(#fields_token_stream),*}}
                    })
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_false),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &proc_macro2::TokenStream::new(), &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_read_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &quote::quote! {
                                Self(#vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                            },
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &proc_macro2::TokenStream::new(), &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                    },
                };
                let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_read_upper_camel_case);
                let impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
                    &ident_read_upper_camel_case,
                    &quote::quote!{sqlx::types::Json(#self_snake_case)}
                );
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_read_upper_camel_case);
                let maybe_ident_with_id_read_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_token_stream = generate_ident_read_token_stream(
                        &ident_with_id_standart_not_null_read_upper_camel_case,
                        &{
                            let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(
                                &is_standart_with_id_true,
                                &ReadWithOrWithoutAnnotationOrInner::WithSerdeOptionIsNoneAnnotation
                            );
                            quote::quote! {{#content_token_stream}}
                        },
                        macros_helpers::DeriveSerdeDeserialize::False,
                    );
                    let ident_with_id_standart_not_null_read_try_from_error_named_token_stream = generate_ident_read_try_from_error_named_token_stream(&ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case);
                    let impl_ident_with_id_standart_not_null_read_token_stream = {
                        let pub_try_new_token_stream = generate_pub_try_new_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_true);
                        quote::quote!{
                            impl #ident_with_id_standart_not_null_read_upper_camel_case {
                                #pub_try_new_token_stream
                            }
                        }
                    };
                    let impl_serde_deserialize_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_true);
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&is_standart_with_id_true);
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                    quote::quote! {
                        #ident_with_id_standart_not_null_read_token_stream
                        #ident_with_id_standart_not_null_read_try_from_error_named_token_stream
                        #impl_ident_with_id_standart_not_null_read_token_stream
                        #impl_serde_deserialize_for_ident_with_id_standart_not_null_read_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_read_token_stream
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_read_token_stream
                    #maybe_ident_read_try_from_error_named_token_stream
                    #impl_ident_read_token_stream
                    #maybe_impl_serde_deserialize_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream
                    #maybe_ident_with_id_read_token_stream
                }
            };
            let ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case = naming::parameter::SelfReadOnlyIdsHandleUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_standart_not_null_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
            let ident_read_only_ids_handle_upper_camel_case = naming::parameter::SelfReadOnlyIdsHandleUpperCamelCase::from_tokens(&ident);
            let generate_ident_read_only_ids_or_ident_with_id_read_only_ids_content_token_stream = |is_standart_with_id: &IsStandartWithId| {
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_5f9102af| {
                    let field_ident = &element_5f9102af.field_ident;
                    let field_type_as_postgresql_json_type_read_only_ids_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &element_5f9102af.field_type,
                        &PostgresqlJsonTypeSubtype::ReadOnlyIds
                    );
                    quote::quote! {#field_ident: #field_type_as_postgresql_json_type_read_only_ids_token_stream}
                });
                quote::quote! {{#(#content_token_stream),*}}
            };
            let generate_impl_sqlx_decode_token_stream = |current_ident_token_stream: &dyn quote::ToTokens|{
                postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                    &current_ident_token_stream,
                    &quote::quote!{sqlx::types::Json<Self>},
                    &quote::quote!{Ok(value_147c3532.0)}
                )
            };
            let generate_impl_sqlx_type_token_stream = |current_ident_token_stream: &dyn quote::ToTokens|{
                postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &current_ident_token_stream,
                    &quote::quote!{sqlx::types::Json<Self>}
                )
            };
            let generate_fields_read_only_ids_into_option_value_read_inner_token_stream = |is_standart_with_id: &IsStandartWithId, parameters_token_stream: &dyn quote::ToTokens|{
                let current_ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                    IsStandartWithId::False => &ident_standart_not_null_read_inner_upper_camel_case
                };
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_278a1e1d| {
                    let field_ident = &element_278a1e1d.field_ident;
                    let field_type = &element_278a1e1d.field_type;
                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&field_type);
                    let field_type_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&field_type, &PostgresqlJsonTypeSubtype::Read);
                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                        let default_but_option_is_always_some_call_token_stream = generate_ident_as_default_but_option_is_always_some_call_token_stream(
                            &field_type_as_postgresql_json_type_read_token_stream
                        );
                        quote::quote!{#field_type_as_postgresql_json_type_token_stream::into_inner(#default_but_option_is_always_some_call_token_stream)}
                    });
                    quote::quote! {
                        #field_ident: #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_into_option_value_read_inner(
                            #parameters_token_stream.0.#value_snake_case.#field_ident
                        ).map_or_else(
                            || Some(#value_content_token_stream),
                            Some
                        )
                    }
                });
                let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                    #current_ident_token_stream {
                        #(#content_token_stream),*
                    }
                });
                quote::quote!{Some(#value_content_token_stream)}
            };
            let ident_read_only_ids_token_stream = {
                let maybe_ident_read_only_ids_handle_token_stream = if is_standart_not_null {
                    macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_handle_upper_camel_case,
                        &generate_ident_read_only_ids_or_ident_with_id_read_only_ids_content_token_stream(&IsStandartWithId::False)
                    )
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                let ident_read_only_ids_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_only_ids_upper_camel_case,
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_ident_read_only_ids_handle_upper_camel_case_token_stream = wrap_into_value_declaration_token_stream(&ident_read_only_ids_handle_upper_camel_case);
                                    quote::quote! {(#value_ident_read_only_ids_handle_upper_camel_case_token_stream);}
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_std_option_option_ident_read_only_ids_standart_not_null_token_stream = wrap_into_value_declaration_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                        &ident_standart_not_null_read_only_ids_upper_camel_case
                                    ));
                                    quote::quote! {
                                        (#value_std_option_option_ident_read_only_ids_standart_not_null_token_stream);
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_std_vec_vec_ident_with_id_standart_not_null_read_only_ids_token_stream = wrap_into_value_declaration_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                                        &ident_with_id_standart_not_null_read_only_ids_upper_camel_case
                                    ));
                                    quote::quote! {
                                        (#value_std_vec_vec_ident_with_id_standart_not_null_read_only_ids_token_stream);
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_std_option_option_ident_with_id_read_only_ids_array_not_null_token_stream = wrap_into_value_declaration_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                        &naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId))
                                    ));
                                    quote::quote! {(#value_std_option_option_ident_with_id_read_only_ids_array_not_null_token_stream);}
                                }
                            },
                        }
                    );
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream = generate_impl_sqlx_decode_token_stream(&ident_read_only_ids_upper_camel_case);
                let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream = generate_impl_sqlx_type_token_stream(&ident_read_only_ids_upper_camel_case);
                let maybe_ident_with_id_standart_not_null_read_only_ids_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_only_ids_token_stream = {
                        let ident_with_id_standart_not_null_read_only_ids_handle_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case,
                                &generate_ident_read_only_ids_or_ident_with_id_read_only_ids_content_token_stream(&IsStandartWithId::True)
                            );
                        let ident_with_id_standart_not_null_read_only_ids_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_clone()
                            .derive_partial_eq()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .build_struct(
                                &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                                &{
                                    let value_ident_with_id_standart_not_null_read_only_ids_handle_token_stream = wrap_into_value_declaration_token_stream(
                                        &ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case
                                    );
                                    quote::quote!{(pub #value_ident_with_id_standart_not_null_read_only_ids_handle_token_stream);}
                                }
                            );
                        quote::quote! {
                            #ident_with_id_standart_not_null_read_only_ids_handle_token_stream
                            #ident_with_id_standart_not_null_read_only_ids_token_stream
                        }
                    };
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_token_stream = generate_impl_sqlx_decode_token_stream(&ident_with_id_standart_not_null_read_only_ids_upper_camel_case);
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_token_stream = generate_impl_sqlx_type_token_stream(&ident_with_id_standart_not_null_read_only_ids_upper_camel_case);
                    quote::quote! {
                        #ident_with_id_standart_not_null_read_only_ids_token_stream
                        #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_token_stream
                        #impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_only_ids_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #maybe_ident_read_only_ids_handle_token_stream
                    #ident_read_only_ids_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream
                    #maybe_ident_with_id_standart_not_null_read_only_ids_token_stream
                }
            };
            let ident_read_inner_token_stream = {
                let generate_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_token_stream = |is_standart_with_id: &IsStandartWithId| macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_partial_eq()
                    .build_struct(
                        match &is_standart_with_id {
                            IsStandartWithId::False => &ident_read_inner_upper_camel_case,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                        },
                        &{
                            let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(
                                is_standart_with_id,
                                &ReadWithOrWithoutAnnotationOrInner::Inner
                            );
                            quote::quote!{{#content_token_stream}}
                        }
                    );
                let ident_read_inner_token_stream = {
                    let generate_pub_type_ident_read_inner_alias_token_stream = |content_token_stream: &dyn quote::ToTokens| macros_helpers::generate_pub_type_alias_token_stream(&ident_read_inner_upper_camel_case, &content_token_stream);
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_token_stream(&IsStandartWithId::False),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_type_ident_read_inner_alias_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_type_as_postgresql_json_type_read_inner_token_stream(&ident_standart_not_null_upper_camel_case))),
                        },
                        PostgresqlJsonObjectTypePattern::Array => generate_pub_type_ident_read_inner_alias_token_stream(&match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                                &ident_with_id_standart_not_null_read_inner_upper_camel_case
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_type_as_postgresql_json_type_read_inner_token_stream(&ident_with_id_array_not_null_upper_camel_case)),
                        }),
                    }
                };
                let maybe_ident_with_id_read_inner_token_stream = if is_standart_not_null {
                    let ident_with_id_read_inner_token_stream = generate_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_token_stream(&IsStandartWithId::True);
                    quote::quote! {
                        #ident_with_id_read_inner_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_read_inner_token_stream
                    #maybe_ident_with_id_read_inner_token_stream
                }
            };
            let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
            let ident_standart_not_null_update_element_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_update_for_query_element_upper_camel_case = &naming::parameter::SelfUpdateForQueryElementUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_update_element_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident);
            let ident_update_for_query_element_upper_camel_case = &naming::parameter::SelfUpdateForQueryElementUpperCamelCase::from_tokens(&ident);
            let ident_standart_not_null_as_postgresql_json_type_update_token_stream = generate_type_as_postgresql_json_type_update_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_as_postgresql_json_type_update_for_query_token_stream = generate_type_as_postgresql_json_type_update_for_query_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_array_not_null_as_postgresql_json_type_update_token_stream = generate_type_as_postgresql_json_type_update_token_stream(&ident_with_id_array_not_null_upper_camel_case);
            let ident_with_id_array_not_null_as_postgresql_json_type_update_for_query_token_stream = generate_type_as_postgresql_json_type_update_for_query_token_stream(&ident_with_id_array_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_update_element_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_update_for_query_element_upper_camel_case = &naming::parameter::SelfUpdateForQueryElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let (generate_jsonb_set_target_snake_case, generate_jsonb_set_target_token_stream) = {
                let generate_jsonb_set_target_snake_case = naming::GenerateJsonbSetTargetSnakeCase;
                let generate_jsonb_set_target_token_stream = {
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{jsonb_set_target_snake_case}}}->'{{value_12d082b5}}'"));
                    quote::quote! {
                        let #generate_jsonb_set_target_snake_case = |value_12d082b5: &str|{
                            format!(#format_handle_token_stream)
                        };
                    }
                };
                (generate_jsonb_set_target_snake_case, generate_jsonb_set_target_token_stream)
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream = quote::quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_element_upper_camel_case>
            };
            let import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_element_token_stream = quote::quote!{
                #import_path::UniqueVec::<#ident_with_id_standart_not_null_update_for_query_element_upper_camel_case>
            };
            let ident_update_token_stream = {
                let generate_ident_update_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    generate_unique_vec_wrapper_token_stream(match &is_standart_with_id {
                        IsStandartWithId::False => &ident_update_element_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_update_element_upper_camel_case,
                    })
                };
                let std_vec_vec_ident_with_id_standart_not_null_create_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                    &ident_with_id_standart_not_null_create_upper_camel_case
                );
                let std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                    &postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream
                );
                let generate_create_update_delete_fields_token_stream = |should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation| {
                    let maybe_serde_skip_serializing_if_vec_is_empty_token_stream = match &should_add_serde_skip_serializing_if_vec_is_empty_annotation {
                        ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True => quote::quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
                        ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {
                        #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                        #create_snake_case: #std_vec_vec_ident_with_id_standart_not_null_create_token_stream,
                        #update_snake_case: #import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream,
                        #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                        #delete_snake_case: #std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream,
                    }
                };

                let ident_update_token_stream = {
                    let generate_ident_update_token_stream = |
                        derive_serde_deserialize: macros_helpers::DeriveSerdeDeserialize,
                        content_token_stream: &dyn quote::ToTokens
                    |macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize_if(derive_serde_deserialize)
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_struct(
                            &ident_update_upper_camel_case,
                            &content_token_stream
                        );
                    let generate_std_option_option_ident_type_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_token_stream(
                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&current_ident_token_stream)
                    );
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &wrap_content_into_scopes_dot_comma_token_stream(
                                    &generate_ident_update_standart_not_null_content_token_stream(&is_standart_with_id_false)
                                )
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &generate_std_option_option_ident_type_token_stream(&ident_standart_not_null_as_postgresql_json_type_update_token_stream)
                            ),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(
                                macros_helpers::DeriveSerdeDeserialize::False,
                                &{
                                    let fields_token_stream = generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                    quote::quote! {{#fields_token_stream}}
                                }
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(
                                macros_helpers::DeriveSerdeDeserialize::True,
                                &generate_std_option_option_ident_type_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_update_token_stream)
                            ),
                        },
                    }
                };
                let not_unique_id_in_json_delete_array_upper_camel_case = naming::NotUniqueIdInJsonDeleteArrayUpperCamelCase;
                let not_unique_id_in_json_update_and_delete_arrays_upper_camel_case = naming::NotUniqueIdInJsonUpdateAndDeleteArraysUpperCamelCase;
                let create_update_delete_are_empty_upper_camel_case = naming::CreateUpdateDeleteAreEmptyUpperCamelCase;
                let ids_are_not_unique_uppper_camel_case = naming::IdsAreNotUniqueUpperCamelCase;
                let ident_update_try_new_error_named_upper_camel_case = &naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                let maybe_ident_update_try_new_error_named_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => proc_macro2::TokenStream::new(),
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_error_occurence_lib_error_occurence()
                            .build_enum(
                                &ident_update_try_new_error_named_upper_camel_case,
                                &quote::quote!{{
                                    #create_update_delete_are_empty_upper_camel_case {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #ids_are_not_unique_uppper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        duplicate: #std_string_string_token_stream,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #not_unique_id_in_json_delete_array_upper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        error: #std_string_string_token_stream,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        error: #std_string_string_token_stream,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                }}
                            ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                };
                let impl_ident_update_token_stream = {
                    let maybe_pub_new_or_try_new_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => macros_helpers::generate_pub_const_new_token_stream(
                            &must_use_token_stream,
                            &generate_value_type_token_stream(&match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_unique_vec_wrapper_token_stream(&ident_standart_not_null_update_element_upper_camel_case),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_as_postgresql_json_type_update_token_stream)
                            }),
                            &self_value_token_stream
                        ),
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_pub_try_new_token_stream(
                                &generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False),
                                &ident_update_try_new_error_named_upper_camel_case,
                                &{
                                    let custom_serde_error_deserializing_ident_update_stringified = format!("custom serde error deserializing {ident_update_upper_camel_case}");
                                    let check_if_all_empty_token_stream = {
                                        quote::quote! {
                                            if create.is_empty() && update.is_empty() && delete.is_empty() {
                                                return Err(#ident_update_try_new_error_named_upper_camel_case::#create_update_delete_are_empty_upper_camel_case {
                                                    code_occurence: error_occurence_lib::code_occurence!()
                                                });
                                            }
                                        }
                                    };
                                    let check_if_ids_are_unique_token_stream = {
                                        let (
                                            uuid_as_postgresql_json_type_update_to_std_string_string_element_id_token_stream,
                                            uuid_as_postgresql_json_type_update_to_std_string_string_element_token_stream,
                                        ) = {
                                            enum UpdateOrDelete {
                                                Update,
                                                Delete
                                            }
                                            let generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream = |
                                                update_or_delete: &UpdateOrDelete,
                                                element_token_stream: &dyn quote::ToTokens,
                                            |{
                                                let content_token_stream: &dyn quote::ToTokens = match &update_or_delete {
                                                    UpdateOrDelete::Update => &quote::quote!{&#element_token_stream.#id_snake_case},
                                                    UpdateOrDelete::Delete => &element_token_stream
                                                };
                                                quote::quote!{
                                                    <#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream as error_occurence_lib::ToStdStringString>::to_std_string_string(
                                                        #content_token_stream
                                                    )
                                                }
                                            };
                                            (
                                                generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream(
                                                    &UpdateOrDelete::Update,
                                                    &quote::quote!{element_dff7634c}
                                                ),
                                                generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream(
                                                    &UpdateOrDelete::Delete,
                                                    &quote::quote!{element_2b0181e6}
                                                )
                                            )
                                        };
                                        quote::quote!{{
                                            let mut acc_2bf4e098 = Vec::new();
                                            for element_dff7634c in update.to_vec() {
                                                if acc_2bf4e098.contains(&&element_dff7634c.#id_snake_case) {
                                                    return Err(#ident_update_try_new_error_named_upper_camel_case::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgresql_json_type_update_to_std_string_string_element_id_token_stream,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                acc_2bf4e098.push(&element_dff7634c.#id_snake_case);
                                            }
                                            for element_2b0181e6 in &delete {
                                                if acc_2bf4e098.contains(&element_2b0181e6) {
                                                    return Err(#ident_update_try_new_error_named_upper_camel_case::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgresql_json_type_update_to_std_string_string_element_token_stream,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                acc_2bf4e098.push(element_2b0181e6);
                                            }
                                        }}
                                    };
                                    let check_not_unique_id_token_stream = {
                                        let check_not_unique_id_in_update_array_token_stream = quote::quote! {
                                            let update_acc = #update_snake_case.to_vec().iter()
                                            .map(|element_b6af219f|&element_b6af219f.#id_snake_case)
                                            .collect::<Vec<&#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream>>();
                                        };
                                        let check_not_unique_id_in_delete_aray_token_stream = {
                                            let not_unique_id_in_json_delete_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json delete array: {{}}"));
                                            quote::quote! {
                                                let delete_acc = {
                                                    let mut delete_acc = Vec::new();
                                                    for element_2ecc509c in &delete {
                                                        if delete_acc.contains(&element_2ecc509c) {
                                                            return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_delete_array_upper_camel_case {
                                                                #error_snake_case: format!(
                                                                    #not_unique_id_in_json_delete_array_double_quotes_token_stream,
                                                                    #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(
                                                                        &element_2ecc509c.clone().into()
                                                                    )
                                                                ),
                                                                code_occurence: error_occurence_lib::code_occurence!()
                                                            });
                                                        }
                                                        delete_acc.push(element_2ecc509c);
                                                    }
                                                    delete_acc
                                                };
                                            }
                                        };
                                        let check_not_unique_id_in_update_and_delete_arrays_token_stream = {
                                            let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json update and delete arrays: {{}}"));
                                            quote::quote! {
                                                for element_fefe9816 in update_acc {
                                                    if delete_acc.contains(&element_fefe9816) {
                                                        return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                                            #error_snake_case: format!(
                                                                #not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream,
                                                                #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(
                                                                    &element_fefe9816.clone().into()
                                                                )
                                                            ),
                                                            code_occurence: error_occurence_lib::code_occurence!()
                                                        });
                                                    }
                                                }
                                            }
                                        };
                                        quote::quote! {
                                            {
                                                #check_not_unique_id_in_update_array_token_stream
                                                #check_not_unique_id_in_delete_aray_token_stream
                                                #check_not_unique_id_in_update_and_delete_arrays_token_stream
                                            }
                                        }
                                    };
                                    quote::quote!{
                                        #check_if_all_empty_token_stream
                                        #check_if_ids_are_unique_token_stream
                                        #check_not_unique_id_token_stream
                                        Ok(Self {
                                            #create_snake_case,
                                            #update_snake_case,
                                            #delete_snake_case
                                        })
                                    }
                                }
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_const_new_value_type_content_self_value_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                    &ident_with_id_array_not_null_as_postgresql_json_type_update_token_stream
                                )
                            )
                        },
                    };
                    quote::quote!{
                        impl #ident_update_upper_camel_case {
                            #maybe_pub_new_or_try_new_for_ident_update_token_stream
                        }
                    }
                };
                let maybe_impl_serde_deserialize_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => proc_macro2::TokenStream::new(),
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            //todo maybe reuse?
                            let tuple_struct_ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("tuple struct {ident_update_upper_camel_case}"));
                            let ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_update_upper_camel_case);
                            let match_try_new_in_deserialize_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(
                                &ident_update_upper_camel_case,
                                &quote::quote! {__field0_value, __field1_value, __field2_value}
                            );
                            quote::quote! {
                                impl<'de> serde::Deserialize<'de> for #ident_update_upper_camel_case {
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
                                            marker: serde::__private228::PhantomData<#ident_update_upper_camel_case>,
                                            lifetime: serde::__private228::PhantomData<&'de ()>,
                                        }
                                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #ident_update_upper_camel_case;
                                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                                serde::__private228::Formatter::write_str(__f, #tuple_struct_ident_update_double_quotes_token_stream)
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let __field0_value = serde::de::SeqAccess::next_element::<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream>(&mut __seq)?.unwrap_or_default();
                                                let __field1_value = serde::de::SeqAccess::next_element::<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream>(&mut __seq)?.unwrap_or_default();
                                                let __field2_value = serde::de::SeqAccess::next_element::<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>(&mut __seq)?.unwrap_or_default();
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                            #[inline]
                                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: Option<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream> = None;
                                                let mut __field1: Option<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream> = None;
                                                let mut __field2: Option<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream> = None;
                                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if Option::is_some(&__field0) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                            }
                                                            __field0 = Some(serde::de::MapAccess::next_value::<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream>(&mut __map)?);
                                                        }
                                                        __Field::__field1 => {
                                                            if Option::is_some(&__field1) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                            }
                                                            __field1 = Some(serde::de::MapAccess::next_value::<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream>(&mut __map)?);
                                                        }
                                                        __Field::__field2 => {
                                                            if Option::is_some(&__field2) {
                                                                return Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                            }
                                                            __field2 = Some(serde::de::MapAccess::next_value::<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>(&mut __map)?);
                                                        }
                                                        __Field::__ignore => {
                                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                let __field0_value = __field0.unwrap_or_default();
                                                let __field1_value = __field1.unwrap_or_default();
                                                let __field2_value = __field2.unwrap_or_default();
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &[&str] = &["create", "update", "delete"];
                                        serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_update_double_quotes_token_stream,
                                            FIELDS,
                                            __Visitor {
                                                marker: serde::__private228::PhantomData::<#self_upper_camel_case>,
                                                lifetime: serde::__private228::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        }
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_upper_camel_case, &proc_macro2::TokenStream::new(), &{
                    let value = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {{
                                #create_snake_case: #vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                #update_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                #delete_snake_case: #vec_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            }},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                            },
                        },
                    };
                    quote::quote! {Self #value}
                });
                let maybe_ident_update_element_token_stream = if is_standart_not_null {
                    let ident_update_element_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_enum(
                            &ident_standart_not_null_update_element_upper_camel_case,
                            &{
                                let variants_token_stream = vec_syn_field.iter().map(|element_092057f6| {
                                    let field_ident = &element_092057f6.field_ident;
                                    let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element_092057f6);
                                    let value_field_type_as_json_type_update_token_stream = wrap_into_value_declaration_token_stream(
                                        &generate_type_as_postgresql_json_type_update_token_stream(&element_092057f6.field_type)
                                    );
                                    quote::quote! {
                                        #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                                        #variant_ident_upper_camel_case_token_stream(#value_field_type_as_json_type_update_token_stream)
                                    }
                                });
                                quote::quote!{{#(#variants_token_stream),*}}
                            }
                        );
                    let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_standart_not_null_update_element_upper_camel_case, &{
                        let elements_token_stream = vec_syn_field.iter().map(|element_2080bd7e| {
                            let field_ident = &element_2080bd7e.field_ident;
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let value_content_token_stream = wrap_into_value_initialization_token_stream(
                                &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            );
                            quote::quote! {#self_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_content_token_stream)}
                        });
                        quote::quote! {vec![#(#elements_token_stream),*]}
                    });
                    quote::quote! {
                        #ident_update_element_token_stream
                        #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_ident_with_id_standart_not_null_update_element_token_stream = if is_standart_not_null {
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
                    let ident_with_id_standart_not_null_update_element_fields_declaration_token_stream = quote::quote! {
                        #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream,
                        #fields_snake_case: #ident_standart_not_null_as_postgresql_json_type_update_token_stream
                    };
                    let ident_with_id_standart_not_null_update_element_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .derive_schemars_json_schema()
                        .build_struct(
                            &ident_with_id_standart_not_null_update_element_upper_camel_case,
                            &quote::quote!{{#ident_with_id_standart_not_null_update_element_fields_declaration_token_stream}}
                        );
                    let impl_pub_new_for_ident_with_id_standart_not_null_update_element_token_stream = macros_helpers::generate_impl_pub_const_new_for_ident_token_stream(
                        &ident_with_id_standart_not_null_update_element_upper_camel_case,
                        &must_use_token_stream,
                        &ident_with_id_standart_not_null_update_element_fields_declaration_token_stream,
                        &quote::quote! {Self {
                            #id_snake_case,
                            #fields_snake_case
                        }},
                    );
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_standart_not_null_update_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_with_id_standart_not_null_update_element_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &quote::quote! {Self {
                            #id_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            #fields_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                        }},
                    );
                    quote::quote! {
                        #ident_with_id_standart_not_null_update_element_token_stream
                        #impl_pub_new_for_ident_with_id_standart_not_null_update_element_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_standart_not_null_update_element_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_update_token_stream
                    #maybe_ident_update_try_new_error_named_token_stream
                    #impl_ident_update_token_stream
                    #maybe_impl_serde_deserialize_for_ident_update_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
                    #maybe_ident_update_element_token_stream
                    #maybe_ident_with_id_standart_not_null_update_element_token_stream
                }
            };
            let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
            let ident_update_for_query_token_stream = {
                let ident_update_for_query_token_stream = {
                    let generate_ident_update_for_query_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        generate_debug_clone_partialeq_serialize_pub_struct_token_stream(
                            &ident_update_for_query_upper_camel_case,
                            &content_token_stream
                        )
                    };
                    let generate_std_option_option_ident_type_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_token_stream(
                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&current_ident_token_stream)
                    );
                    let generate_ident_update_for_query_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId| {
                        generate_unique_vec_wrapper_token_stream(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_update_for_query_element_upper_camel_case,
                            IsStandartWithId::True => &ident_with_id_standart_not_null_update_for_query_element_upper_camel_case,
                        })
                    };
                    let std_vec_vec_ident_with_id_standart_not_null_create_for_query_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                        &ident_with_id_standart_not_null_create_for_query_upper_camel_case
                    );
                    let std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_for_query_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                        &postgresql_crud_path_postgresql_json_type_uuid_uuid_update_for_query_token_stream
                    );
                    let generate_create_update_delete_fields_token_stream = |should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation| {
                        let maybe_serde_skip_serializing_if_vec_is_empty_token_stream = match &should_add_serde_skip_serializing_if_vec_is_empty_annotation {
                            ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True => quote::quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
                            ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False => proc_macro2::TokenStream::new(),
                        };
                        quote::quote! {
                            #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                            #create_snake_case: #std_vec_vec_ident_with_id_standart_not_null_create_for_query_token_stream,
                            #update_snake_case: #import_path_unique_vec_ident_with_id_standart_not_null_update_for_query_element_token_stream,
                            #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                            #delete_snake_case: #std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_for_query_token_stream,//todo maybe expand logic with where cases
                        }
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_for_query_token_stream(
                                &wrap_content_into_scopes_dot_comma_token_stream(
                                    &generate_ident_update_for_query_standart_not_null_content_token_stream(
                                        &is_standart_with_id_false
                                    )
                                )
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_for_query_token_stream(
                                &generate_std_option_option_ident_type_token_stream(
                                    &ident_standart_not_null_as_postgresql_json_type_update_for_query_token_stream
                                )
                            ),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_for_query_token_stream(
                                &{
                                    let fields_token_stream = generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                    quote::quote! {{#fields_token_stream}}
                                }
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_for_query_token_stream(
                                &generate_std_option_option_ident_type_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_update_for_query_token_stream)
                            ),
                        },
                    }
                };
                let impl_ident_update_for_query_token_stream = {
                    let select_only_updated_ids_query_part_token_stream = {
                        let content_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let match_variants_token_stream = vec_syn_field.iter().map(|element_bca06812| {
                                        let field_ident = &element_bca06812.field_ident;
                                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_bca06812.field_type);
                                        let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                            &quote::quote!{acc_8e628eaf, "jsonb_build_object({value_c3ae3be4})||"},
                                            &return_err_query_part_error_named_write_into_buffer_token_stream
                                        );
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(#value_snake_case) => {
                                                match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                    &#value_snake_case.#value_snake_case,
                                                    #field_ident_double_quotes_token_stream,
                                                    column_name_and_maybe_field_getter,
                                                    #increment_snake_case
                                                ) {
                                                    Ok(mut value_c3ae3be4) => {
                                                        let _: Option<char> = value_c3ae3be4.pop();
                                                        #if_write_is_err_curly_braces_token_stream
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    }
                                                }
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        let mut acc_8e628eaf = #std_string_string_token_stream::default();
                                        for element_0963b7df in self.0.to_vec() {
                                            match &element_0963b7df {
                                                #(#match_variants_token_stream),*
                                            }
                                        }
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        let _: Option<char> = acc_8e628eaf.pop();
                                        Ok(acc_8e628eaf)
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let match_content_token_stream = vec_syn_field.iter().map(|element_a8f45572| {
                                        let field_ident = &element_a8f45572.field_ident;
                                        let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_a8f45572.field_type);
                                        let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                            &quote::quote!{acc_f7537df2, "jsonb_build_object({value})||"},
                                            &return_err_query_part_error_named_write_into_buffer_token_stream
                                        );
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(
                                                value_92d002a5
                                            ) => match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                &value_92d002a5.#value_snake_case,
                                                #field_ident_double_quotes_token_stream,
                                                column_name_and_maybe_field_getter,
                                                #increment_snake_case
                                            ) {
                                                Ok(mut #value_snake_case) => {
                                                    let _: Option<char> = #value_snake_case.pop();
                                                    #if_write_is_err_curly_braces_token_stream
                                                }
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        Ok(match &self.0 {
                                            Some(value_9570957e) => {
                                                let mut acc_f7537df2 = #std_string_string_token_stream::default();
                                                for element_97687be3 in value_9570957e.0.to_vec() {
                                                    match &element_97687be3 {
                                                        #(#match_content_token_stream),*
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
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let match_variants_token_stream = vec_syn_field.iter().map(|element_74643094| {
                                        let field_ident = &element_74643094.field_ident;
                                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_74643094.field_type);
                                        let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                            &quote::quote!{acc_892857b1, "jsonb_build_object({value_33d3b52e})||"},
                                            &return_err_query_part_error_named_write_into_buffer_token_stream
                                        );
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(#value_snake_case) => match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                &#value_snake_case.#value_snake_case,
                                                #field_ident_double_quotes_token_stream,
                                                "elem",
                                                #increment_snake_case
                                            ) {
                                                Ok(mut value_33d3b52e) => {
                                                    let _: Option<char> = value_33d3b52e.pop();
                                                    #if_write_is_err_curly_braces_token_stream
                                                }
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    let select_only_created_ids_query_part_content_token_stream = vec_syn_field_with_id.iter().map(|element_e6d6df84| {
                                        let field_ident = &element_e6d6df84.field_ident;
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_e6d6df84.field_type);
                                        let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                            &quote::quote!{acc_57cd0744, "jsonb_build_object({value})||"},
                                            &return_err_query_part_error_named_write_into_buffer_token_stream
                                        );
                                        quote::quote! {
                                            match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                                &element_b1359d90.#field_ident,
                                                #field_ident_double_quotes_token_stream,
                                                "elem",
                                                #increment_snake_case
                                            ) {
                                                Ok(mut #value_snake_case) => {
                                                    let _: Option<char> = #value_snake_case.pop();
                                                    #if_write_is_err_curly_braces_token_stream
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    let if_write_is_err_curly_braces_0_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_892857b1, "jsonb_build_object({value})||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    let if_write_is_err_curly_braces_1_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_57cd0744, "{acc_892857b1}||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    let if_write_is_err_0_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                                        &quote::quote!{acc_d497e8a5, "${value_c31cb081},"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    let if_write_is_err_1_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                                        &quote::quote!{acc_d497e8a5, "${value_b52c3fe1},"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    quote::quote!{
                                        Ok(format!(
                                            "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                                            {
                                                let mut acc_57cd0744 = #std_string_string_token_stream::new();
                                                for element_d7561f40 in self.#update_snake_case.to_vec() {
                                                    //todo maybe wrong for multiple updates by id?
                                                    let mut acc_892857b1 = #std_string_string_token_stream::new();
                                                    match #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream ::select_only_updated_ids_query_part(
                                                        &element_d7561f40.id,
                                                        "id",
                                                        "elem",
                                                        #increment_snake_case
                                                    ) {
                                                        Ok(mut #value_snake_case) => {
                                                            let _: Option<char> = #value_snake_case.pop();
                                                            #if_write_is_err_curly_braces_0_token_stream
                                                        }
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        }
                                                    }
                                                    for element_738b2a83 in element_d7561f40.fields.0.to_vec() {
                                                        match &element_738b2a83 {
                                                            #(#match_variants_token_stream),*
                                                        }
                                                    }
                                                    let _: Option<char> = acc_892857b1.pop();
                                                    let _: Option<char> = acc_892857b1.pop();
                                                    #if_write_is_err_curly_braces_1_token_stream
                                                }
                                                for element_b1359d90 in &self.create {
                                                    #(#select_only_created_ids_query_part_content_token_stream)*
                                                }
                                                let _: Option<char> = acc_57cd0744.pop();
                                                let _: Option<char> = acc_57cd0744.pop();
                                                format!("jsonb_build_object('value',{acc_57cd0744})")
                                            },
                                            column_name_and_maybe_field_getter,
                                            {
                                                let mut acc_d497e8a5 = #std_string_string_token_stream::new();
                                                for _ in self.#update_snake_case.to_vec() {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                        Ok(value_c31cb081) => {
                                                            #if_write_is_err_0_token_stream
                                                        },
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        },
                                                    }
                                                }
                                                for _ in &self.#create_snake_case {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                        Ok(value_b52c3fe1) => {
                                                            #if_write_is_err_1_token_stream
                                                        },
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        },
                                                    }
                                                }
                                                let _: Option<char> = acc_d497e8a5.pop();
                                                acc_d497e8a5
                                            }
                                        ))
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                    Ok(match &self.0 {
                                        Some(value_bc509c9a) => format!(
                                            "jsonb_build_object('value',{})",
                                            match #ident_array_not_null_update_for_query_upper_camel_case::#select_only_updated_ids_query_part_snake_case(
                                                value_bc509c9a,
                                                column_name_and_maybe_field_getter,
                                                #increment_snake_case
                                            ) {
                                                Ok(value_1e016751) => value_1e016751,
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        ),
                                        None => "'null'::jsonb".to_owned(),
                                    })
                                },
                            },
                        };
                        quote::quote!{
                            #[allow(clippy::single_call_fn)]//for some reason lint ignoring this function call in other struct trait methonds(array not null)
                            fn #select_only_updated_ids_query_part_snake_case(
                                &self,
                                column_name_and_maybe_field_getter: &str,
                                #increment_snake_case: &mut u64
                            ) -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                                #content_token_stream
                            }
                        }
                    };
                    quote::quote!{
                        impl #ident_update_for_query_upper_camel_case {
                            #select_only_updated_ids_query_part_token_stream
                        }
                    }
                };
                let impl_std_convert_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(
                    &quote::quote!{#ident_as_import_path_postgresql_json_type_token_stream::Update},
                    &quote::quote!{#ident_as_import_path_postgresql_json_type_token_stream::UpdateForQuery},
                    &match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => quote::quote!{
                                Self(#import_path::NotEmptyUniqueVec::from_t1_impl_from_t2(#value_snake_case.0))
                            },
                            PostgresqlJsonObjectTypePattern::Array => quote::quote!{
                                Self {
                                    #create_snake_case: #value_snake_case.#create_snake_case.into_iter().map(#ident_with_id_standart_not_null_create_for_query_upper_camel_case::from).collect(),
                                    #update_snake_case: #import_path::UniqueVec::from_t1_impl_from_t2(#value_snake_case.#update_snake_case),
                                    #delete_snake_case: #value_snake_case.#delete_snake_case.into_iter().map(Into::into).collect(),
                                }
                            }
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let content_token_stream: &dyn quote::ToTokens = match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_import_path_postgresql_json_type_token_stream,
                                PostgresqlJsonObjectTypePattern::Array => &ident_array_not_null_as_import_path_postgresql_json_type_token_stream
                            };
                            quote::quote!{Self(#value_snake_case.0.map(#content_token_stream::UpdateForQuery::from))}
                        }
                    }
                );
                let maybe_ident_update_for_query_element_token_stream = if is_standart_not_null {
                    let ident_standart_not_null_update_for_query_element_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone()
                        .derive_partial_eq()
                        .derive_serde_serialize()
                        .build_enum(
                            &ident_standart_not_null_update_for_query_element_upper_camel_case,
                            &{
                                let variants_token_stream = vec_syn_field.iter().map(|element_9d8af887| {
                                    let field_ident = &element_9d8af887.field_ident;
                                    let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element_9d8af887);
                                    let value_field_type_as_json_type_update_for_query_token_stream = wrap_into_value_declaration_token_stream(&generate_type_as_postgresql_json_type_update_for_query_token_stream(&element_9d8af887.field_type));
                                    quote::quote! {
                                        #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                                        #variant_ident_upper_camel_case_token_stream(#value_field_type_as_json_type_update_for_query_token_stream)
                                    }
                                });
                                quote::quote!{{#(#variants_token_stream),*}}
                            }
                        );
                    let impl_std_convert_from_ident_standart_not_null_update_element_for_ident_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(
                        &ident_standart_not_null_update_element_upper_camel_case,
                        &ident_standart_not_null_update_for_query_element_upper_camel_case,
                        &{
                            let variants_token_stream = vec_syn_field.iter().map(|element_2a5d6ff3| {
                                let field_ident = &element_2a5d6ff3.field_ident;
                                let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                                    let field_type_as_json_type_update_for_query_token_stream = generate_type_as_postgresql_json_type_update_for_query_token_stream(
                                        &element_2a5d6ff3.field_type
                                    );
                                    quote::quote!{
                                        #field_type_as_json_type_update_for_query_token_stream::from(value_121f1c54.#value_snake_case)
                                    }
                                });
                                quote::quote! {
                                    #ident_standart_not_null_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value_121f1c54) => #self_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_initialization_token_stream)
                                }
                            });
                            quote::quote!{
                                match #value_snake_case {
                                    #(#variants_token_stream),*
                                }
                            }
                        }
                    );
                    quote::quote! {
                        #ident_standart_not_null_update_for_query_element_token_stream
                        #impl_std_convert_from_ident_standart_not_null_update_element_for_ident_standart_not_null_update_for_query_element_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_ident_with_id_standart_not_null_update_for_query_element_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_update_for_query_element_fields_declaration_token_stream = quote::quote! {
                        #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_update_for_query_token_stream,
                        #fields_snake_case: #ident_standart_not_null_as_postgresql_json_type_update_for_query_token_stream
                    };
                    let ident_with_id_standart_not_null_update_for_query_element_token_stream = generate_debug_clone_partialeq_serialize_pub_struct_token_stream(
                        &ident_with_id_standart_not_null_update_for_query_element_upper_camel_case,
                        &quote::quote!{{#ident_with_id_standart_not_null_update_for_query_element_fields_declaration_token_stream}}
                    );
                    let impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_pub_const_new_for_ident_token_stream(
                        &ident_with_id_standart_not_null_update_for_query_element_upper_camel_case,
                        &must_use_token_stream,
                        &ident_with_id_standart_not_null_update_for_query_element_fields_declaration_token_stream,
                        &quote::quote! {Self {
                            #id_snake_case,
                            #fields_snake_case
                        }},
                    );
                    let impl_std_convert_from_ident_with_id_standart_not_null_update_element_for_ident_with_id_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(
                        &ident_with_id_standart_not_null_update_element_upper_camel_case,
                        &ident_with_id_standart_not_null_update_for_query_element_upper_camel_case,
                        &quote::quote! {Self {
                            #id_snake_case: #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream::UpdateForQuery::from(
                                #value_snake_case.#id_snake_case
                            ),
                            fields: #ident_standart_not_null_as_import_path_postgresql_json_type_token_stream::UpdateForQuery::from(
                                #value_snake_case.fields
                            ),
                        }}
                    );
                    quote::quote! {
                        #ident_with_id_standart_not_null_update_for_query_element_token_stream
                        #impl_pub_const_new_for_ident_with_id_standart_not_null_update_for_query_element_token_stream
                        #impl_std_convert_from_ident_with_id_standart_not_null_update_element_for_ident_with_id_standart_not_null_update_for_query_element_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote!{
                    #ident_update_for_query_token_stream
                    #impl_ident_update_for_query_token_stream
                    #impl_std_convert_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_token_stream
                    #maybe_ident_update_for_query_element_token_stream
                    #maybe_ident_with_id_standart_not_null_update_for_query_element_token_stream
                }
            };
            let (impl_postgresql_crud_postgresql_json_type_for_ident_token_stream, maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream) = {
                let postgresql_type_or_postgresql_json_type_postgresql_type = postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType;
                let postgresql_type_or_postgresql_json_type_postgresql_json_type = postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType;
                let generate_update_query_part_standart_nullable_token_stream = |postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType|{
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{value_27b8537f}})"),
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => "${value_27b8537f}".to_owned(),
                    });
                    quote::quote! {
                        match &#value_snake_case.0 {
                            Some(value_92f34435) => #ident_standart_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                value_92f34435,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                increment,
                            ),
                            None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                Ok(value_27b8537f) => Ok(format!(#format_handle_token_stream)),
                                Err(#error_snake_case) => Err(#error_snake_case),
                            }
                        }
                    }
                };
                let generate_update_delete_create_array_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
                    let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                        &quote::quote!{acc_2e2ad041, "{value_8333f8f4}"},
                        &return_err_query_part_error_named_write_into_buffer_token_stream
                    );
                    let if_write_is_err_curly_braces_0_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                        &quote::quote!{acc_5b4cd920, "{maybe_space_and_space}elem->>'id' <> ${increment_cb6ba4a7}"},
                        &return_err_query_part_error_named_write_into_buffer_token_stream
                    );
                    let if_write_is_err_curly_braces_1_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                        &quote::quote!{acc_8554f572, "${increment_5bbc4961},"},
                        &return_err_query_part_error_named_write_into_buffer_token_stream
                    );
                    quote::quote! {
                        let update_query_part_acc = {
                            if value_58d685d3.#update_snake_case.is_empty() {
                                #std_string_string_token_stream::from("elem")
                            } else {
                                let mut acc_2e2ad041 = #std_string_string_token_stream::default();
                                for element_a0a61823 in value_58d685d3.#update_snake_case.to_vec() {
                                    let ident_with_id_handle = {
                                        let id_increment = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                            Ok(value_15b44b54) => value_15b44b54,
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        };
                                        match #ident_standart_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                            &element_a0a61823.fields,
                                            "",
                                            "elem",
                                            "",
                                            #increment_snake_case
                                        ) {
                                            Ok(value_56c44461) => Ok(format!("when elem->>'id' = ${id_increment} then {value_56c44461} ")),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    };
                                    match ident_with_id_handle {
                                        Ok(value_8333f8f4) => {
                                            #if_write_is_err_token_stream
                                        }
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                let _: Option<char> = acc_2e2ad041.pop();
                                format!("case {acc_2e2ad041} else elem end")
                            }
                        };
                        let delete_query_part_acc = {
                            let mut acc_5b4cd920 = #std_string_string_token_stream::default();
                            for _ in &value_58d685d3.#delete_snake_case {
                                let increment_cb6ba4a7 = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                    Ok(value_110650cc) => value_110650cc,
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                };
                                let maybe_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                                #if_write_is_err_curly_braces_0_token_stream
                            }
                            acc_5b4cd920
                        };
                        let create_query_part_acc = {
                            let mut acc_8554f572 = #std_string_string_token_stream::default();
                            for _ in &value_58d685d3.#create_snake_case {
                                let increment_5bbc4961 = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                    Ok(value_27487842) => value_27487842,
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                };
                                #if_write_is_err_curly_braces_1_token_stream
                            }
                            let _: Option<char> = acc_8554f572.pop();
                            acc_8554f572
                        };
                        let maybe_where = if value_58d685d3.#delete_snake_case.is_empty() {
                            #std_string_string_token_stream::default()
                        } else {
                            format!(" where {delete_query_part_acc}")
                        };
                        let maybe_jsonb_build_array = if value_58d685d3.#create_snake_case.is_empty() {
                            #std_string_string_token_stream::default()
                        } else {
                            format!(" || jsonb_build_array({create_query_part_acc})")
                        };
                        Ok (format!(#format_handle_token_stream))
                    }
                };
                let generate_update_query_part_array_not_null_token_stream = |postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType|{
                    let content_token_stream_c75c3cd1 = generate_update_delete_create_array_token_stream(&generate_quotes::double_quotes_token_stream(&match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})",
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => "((select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) {maybe_jsonb_build_array})",
                    }));
                    quote::quote!{
                        let value_58d685d3 = #value_snake_case;
                        #content_token_stream_c75c3cd1
                    }
                };
                let impl_postgresql_crud_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_json_type_token_stream(
                    &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                    &ident,
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_create_upper_camel_case,
                    &ident_create_for_query_upper_camel_case,
                    &ident_select_upper_camel_case,
                    &postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                    &postgresql_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::True,
                    &postgresql_crud_macros_common::IsSelectQueryPartIsPostgresqlTypeUsed::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                match #value_snake_case.#select_query_part_snake_case(
                                    &if is_postgresql_type {
                                        column_name_and_maybe_field_getter.to_owned()
                                    } else {
                                        format!("{column_name_and_maybe_field_getter}->'{field_ident}'")
                                    },
                                    &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
                                ) {
                                    Ok(value_156121ad) => Ok(
                                        if is_postgresql_type {
                                            value_156121ad
                                        } else {
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',{value_156121ad}))")
                                        }
                                    ),
                                    Err(#error_snake_case) => Err(#error_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_standart_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream = generate_ident_as_default_but_option_is_always_some_token_stream(
                                    &ident_standart_not_null_as_postgresql_json_type_select_token_stream
                                );
                                quote::quote! {
                                    let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
                                    let value_46039f0e = value.0.as_ref().map_or_else(
                                        #ident_standart_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream,
                                        Clone::clone
                                    );
                                    match #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_query_part_snake_case(
                                        &value_46039f0e,
                                        field_ident,
                                        &column_name_and_maybe_field_getter_field_ident,
                                        column_name_and_maybe_field_getter_for_error_message,
                                        true
                                    ) {
                                        Ok(value_1f8de96a) => Ok(
                                            format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({value_1f8de96a}) end))")
                                        ),
                                        Err(#error_snake_case) => Err(#error_snake_case)
                                    }
                                }
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let acc_41dea548_token_stream = quote::quote!{acc_41dea548};
                                let select_query_part_for_loop_token_stream = {
                                    let value_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&value_snake_case);
                                    generate_select_query_part_for_loop_token_stream(
                                        &acc_41dea548_token_stream,
                                        &is_standart_with_id_true,
                                        &quote::quote!{#value_snake_case.#ident_with_id_standart_not_null_select_snake_case},
                                        &value_double_quotes_token_stream,
                                        &value_double_quotes_token_stream,
                                    )
                                };
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                                    "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',case when (jsonb_array_length({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_standart_not_null_select_snake_case}}})) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{dimension1_start}} and {{dimension1_end}}) end ))"
                                ));
                                quote::quote! {
                                    let #ident_with_id_standart_not_null_select_snake_case = {
                                        let mut #acc_41dea548_token_stream = #std_string_string_token_stream::default();
                                        #select_query_part_for_loop_token_stream
                                        let _: Option<char> = #acc_41dea548_token_stream.pop();
                                        let _: Option<char> = #acc_41dea548_token_stream.pop();
                                        #acc_41dea548_token_stream
                                    };
                                    let dimension1_start = #value_snake_case.#dimension1_pagination_token_stream.start();
                                    let dimension1_end = #value_snake_case.#dimension1_pagination_token_stream.end();
                                    Ok(format!(#format_handle_token_stream))
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                    &"case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({value_d7bbd03c}) end"
                                );
                                let ident_with_id_array_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream = generate_ident_as_default_but_option_is_always_some_token_stream(
                                    &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream
                                );
                                quote::quote! {
                                    let value_174d33cd = #value_snake_case.0.as_ref().map_or_else(
                                        #ident_with_id_array_not_null_as_postgresql_json_type_select_as_default_but_option_is_always_some_token_stream,
                                        Clone::clone
                                    );
                                    match #ident_with_id_array_not_null_as_postgresql_json_type_token_stream::#select_query_part_snake_case(
                                        &value_174d33cd,
                                        field_ident,
                                        column_name_and_maybe_field_getter,
                                        column_name_and_maybe_field_getter_for_error_message,
                                        true
                                    ) {
                                        Ok(value_d7bbd03c) => Ok(format!(#format_handle_token_stream)),
                                        Err(#error_snake_case) => Err(#error_snake_case)
                                    }
                                }
                            }
                        },
                    },
                    &ident_where_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &ident_read_only_ids_upper_camel_case,
                    &match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let content_token_stream = {
                                let content_token_stream = {
                                    let acc_push_token_stream = get_vec_syn_field(match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => &is_standart_with_id_false,
                                        PostgresqlJsonObjectTypePattern::Array => &is_standart_with_id_true
                                    }).iter().map(|element_a6a15738| {
                                        let field_ident = &element_a6a15738.field_ident;
                                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{field_ident}',{{}})||"));
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_a6a15738.field_type);
                                        let content_token_stream = match &postgresql_json_object_type_pattern {
                                            PostgresqlJsonObjectTypePattern::Standart => {
                                                let format_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{column_name_and_maybe_field_getter}}->'{field_ident}'"));
                                                quote::quote! {&format!(#format_token_stream)}
                                            },
                                            PostgresqlJsonObjectTypePattern::Array => generate_quotes::double_quotes_token_stream(&format!("elem->'{field_ident}'"))
                                        };
                                        macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                            &quote::quote!{
                                                acc_2912b128,
                                                #format_handle_token_stream,
                                                match #field_type_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(#content_token_stream) {
                                                    Ok(value_2317e0af) => value_2317e0af,
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    }
                                                }
                                            },
                                            &return_err_query_part_error_named_write_into_buffer_token_stream
                                        )
                                    });
                                    quote::quote! {{
                                        let mut acc_2912b128 = #std_string_string_token_stream::default();
                                        #(#acc_push_token_stream)*
                                        let _: Option<char> = acc_2912b128.pop();
                                        let _: Option<char> = acc_2912b128.pop();
                                        format!("jsonb_build_object('value',{acc_2912b128})")
                                    }}
                                };
                                match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => content_token_stream,
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                            &format!("jsonb_build_object('value',(select jsonb_agg({{}}) from jsonb_array_elements({{{column_name_and_maybe_field_getter_snake_case}}}) as elem))")
                                        );
                                        quote::quote! {format!(#format_handle_token_stream, #content_token_stream)}
                                    },
                                }
                            };
                            quote::quote! {Ok(#content_token_stream)}
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let content_token_stream: &dyn quote::ToTokens = match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_postgresql_json_type_token_stream,
                                PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_as_postgresql_json_type_token_stream,
                            };
                            let case_null_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("jsonb_build_object('value',case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}})='null' then 'null'::jsonb else {{value_21000130}} end)")
                            );
                            quote::quote! {
                                match #content_token_stream::#select_only_ids_query_part_snake_case(#column_name_and_maybe_field_getter_snake_case) {
                                    Ok(value_21000130) => Ok(format!(#case_null_format_handle_token_stream)),
                                    Err(#error_snake_case) => Err(#error_snake_case)
                                }
                            }
                        }
                    },
                    &ident_read_inner_upper_camel_case,
                    &{
                        let generate_into_inner_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens|{
                            quote::quote!{#current_ident_token_stream::into_inner(#parameters_token_stream)}
                        };
                        let generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                            let current_ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                                IsStandartWithId::False => &ident_read_inner_upper_camel_case,
                                IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                            };
                            let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element_d2c28655| {
                                let field_ident = &element_d2c28655.field_ident;
                                let content_token_stream = wrap_into_value_initialization_token_stream(&generate_into_inner_token_stream(
                                    &generate_type_as_postgresql_json_type_token_stream(&element_d2c28655.field_type),
                                    &quote::quote!{value_6e5af985.#value_snake_case},
                                ));
                                let parameter_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                                    IsStandartWithId::False => &value_snake_case,
                                    IsStandartWithId::True => &quote::quote!{element_34d57236},
                                };
                                quote::quote! {#field_ident: #parameter_token_stream.#field_ident.map(|value_6e5af985| #content_token_stream)}
                            });
                            quote::quote! {
                                #current_ident_token_stream {
                                    #(#content_token_stream),*
                                }
                            }
                        };
                        match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&IsStandartWithId::False),
                                PostgresqlJsonObjectTypePattern::Array => {
                                    let content_token_stream = generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&IsStandartWithId::True);
                                    quote::quote! {#value_snake_case.0.into_iter().map(|element_34d57236|#content_token_stream).collect()}
                                },
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let current_ident = generate_type_as_postgresql_json_type_token_stream(&match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => ident_standart_not_null_upper_camel_case,
                                    PostgresqlJsonObjectTypePattern::Array => ident_array_not_null_upper_camel_case,
                                });
                                quote::quote! {#value_snake_case.0.map(#current_ident::into_inner)}
                            }
                        }
                    },
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let object_acc_snake_case = naming::StdOptionOptionObjectAccSnakeCase;
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',{{{object_acc_snake_case}}})"));
                                let query_part_variants_token_stream = vec_syn_field.iter().map(|element_ebd92dbf| {
                                    let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&element_ebd92dbf.field_ident.to_string());
                                    let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element_ebd92dbf);
                                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element_ebd92dbf);
                                    quote::quote! {
                                        #ident_update_for_query_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value_3b3fae4c) => {
                                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream::#update_query_part_snake_case(
                                                &value_3b3fae4c.#value_snake_case,
                                                &#object_acc_snake_case,
                                                &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                                #field_ident_double_quotes_token_stream,
                                                #increment_snake_case,
                                            ) {
                                                Ok(value_5edc1648) => {
                                                    #object_acc_snake_case = value_5edc1648;
                                                }
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    }
                                });
                                let some_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end"));
                                quote::quote! {
                                    let mut #object_acc_snake_case = format!(#some_format_handle_token_stream);
                                    #generate_jsonb_set_target_token_stream
                                    for element_157f4b80 in #value_snake_case.0.to_vec() {
                                        match element_157f4b80 {
                                            #(#query_part_variants_token_stream),*
                                        }
                                    }
                                    if #jsonb_set_path_snake_case.is_empty() {
                                        Ok(#object_acc_snake_case)
                                    }
                                    else {
                                        Ok(format!(#format_handle_token_stream))
                                    }
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_update_query_part_standart_nullable_token_stream(
                                &postgresql_type_or_postgresql_json_type_postgresql_type
                            )
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_update_query_part_array_not_null_token_stream(
                                &postgresql_type_or_postgresql_json_type_postgresql_type
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                match &#value_snake_case.0 {
                                    Some(value_3245b79f) => #ident_array_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                        value_3245b79f,
                                        jsonb_set_accumulator,
                                        jsonb_set_target,
                                        jsonb_set_path,
                                        #increment_snake_case,
                                    ),
                                    None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                        Ok(value_87e08bec) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${value_87e08bec})")),
                                        Err(#error_snake_case) => Err(#error_snake_case)
                                    }
                                }
                            },
                        },
                    },
                    &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::True,
                    &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::True,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let update_query_bind_variants_token_stream = vec_syn_field.iter().map(|element_9968a29b| {
                                    let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&element_9968a29b.field_ident.to_string());
                                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(
                                        element_9968a29b
                                    );
                                    quote::quote! {
                                        #ident_update_for_query_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value_b27f5b09) => {
                                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream::#update_query_bind_snake_case(
                                                value_b27f5b09.#value_snake_case,
                                                #query_snake_case
                                            ) {
                                                Ok(value_a4870bad) => {
                                                    #query_snake_case = value_a4870bad;
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote::quote! {
                                    for element_f14dcf6b in #value_snake_case.0.into_vec() {
                                        match element_f14dcf6b {
                                            #(#update_query_bind_variants_token_stream),*
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                match #value_snake_case.0 {
                                    Some(value_269a0d34) => #ident_standart_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        value_269a0d34,
                                        #query_snake_case
                                    ),
                                    None => if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(#self_as_postgresql_json_type_update_token_stream::new(None))) {
                                        Err(#error_snake_case.to_string())
                                    }
                                    else {
                                        Ok(#query_snake_case)
                                    },
                                }
                            }
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                for element_30541f69 in #value_snake_case.#update_snake_case.into_vec() {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                        element_30541f69.#id_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(value_7633dc9b) => {
                                            #query_snake_case = value_7633dc9b;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                    match #ident_standart_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        element_30541f69.#fields_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(value_2073f07a) => {
                                            #query_snake_case = value_2073f07a;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                for element_4b6f8c01 in #value_snake_case.delete {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                        element_4b6f8c01,
                                        #query_snake_case
                                    ) {
                                        Ok(value_31262d92) => {
                                            #query_snake_case = value_31262d92;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                for element_a44eb132 in #value_snake_case.#create_snake_case {
                                    if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(element_a44eb132)) {
                                        return Err(#error_snake_case.to_string());
                                    }
                                }
                                Ok(#query_snake_case)
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                match #value_snake_case.0 {
                                    Some(value_a2156b3e) => #ident_array_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        value_a2156b3e,
                                        #query_snake_case
                                    ),
                                    None => if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(#self_as_postgresql_json_type_update_token_stream::new(None))) {
                                        Err(#error_snake_case.to_string())
                                    }
                                    else {
                                        Ok(#query_snake_case)
                                    },
                                }
                            },
                        },
                    },
                    &quote::quote!{
                        match #value_snake_case.#select_only_updated_ids_query_part_snake_case(
                            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                            #increment_snake_case
                        ) {
                            Ok(value_e137951b) => Ok(format!("'{field_ident}',jsonb_build_object('value',{value_e137951b}),")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let match_content_token_stream = vec_syn_field.iter().map(|element_e3bd5731| {
                                    let field_ident = &element_e3bd5731.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_e3bd5731.field_type);
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    quote::quote! {
                                        #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(value_b79c2851) => {
                                            match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                                &value_b79c2851.#value_snake_case,
                                                #query_snake_case
                                            ) {
                                                Ok(value_e8914f75) => {
                                                    #query_snake_case = value_e8914f75;
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for element_31dd08ee in #value_snake_case.0.to_vec() {
                                        match element_31dd08ee {
                                            #(#match_content_token_stream),*
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                if let Some(value_6334d77d) = &#value_snake_case.0 {
                                    match #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(value_6334d77d, #query_snake_case) {
                                        Ok(value_0bd3ba6f) => {
                                            #query_snake_case = value_0bd3ba6f;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                Ok(#query_snake_case)
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let select_only_created_ids_query_bind_content_token_stream = vec_syn_field_with_id.iter().map(|element_27e0de74| {
                                    let field_ident = &element_27e0de74.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_27e0de74.field_type);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            &element_5fba4c1f.#field_ident,
                                            #query_snake_case
                                        ) {
                                            Ok(value_cb81ec2c) => {
                                                #query_snake_case = value_cb81ec2c;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for element_e5af9b26 in #value_snake_case.#update_snake_case.to_vec() {
                                        match #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                            &element_e5af9b26.#id_snake_case,
                                            #query_snake_case
                                        ) {
                                            Ok(value_0fd735de) => {
                                                #query_snake_case = value_0fd735de;
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                        match #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                            &element_e5af9b26.fields,
                                            #query_snake_case
                                        ) {
                                            Ok(value_4b52fa4f) => {
                                                #query_snake_case = value_4b52fa4f;
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    for element_5fba4c1f in &#value_snake_case.create {
                                        #(#select_only_created_ids_query_bind_content_token_stream)*
                                    }
                                    for element_d9eff5ca in #value_snake_case.#update_snake_case.to_vec() {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                            element_d9eff5ca.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(value_b0da764b) => {
                                                #query_snake_case = value_b0da764b;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    for element_ae971756 in &#value_snake_case.#create_snake_case {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_create_for_query(
                                            element_ae971756.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(value_dd8932e8) => {
                                                #query_snake_case = value_dd8932e8;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                if let Some(value_107e6639) = &#value_snake_case.0 {
                                    match #ident_array_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(value_107e6639, #query_snake_case) {
                                        Ok(value_ecf1b8de) => {
                                            #query_snake_case = value_ecf1b8de;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                Ok(#query_snake_case)
                            },
                        },
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = vec_syn_field.iter().map(|element_6bcf3221| {
                                    let field_ident = &element_6bcf3221.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_6bcf3221.field_type);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(
                                        &format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_0fe559fa, "jsonb_build_object({value_cddc8a0a})||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &#value_snake_case.#field_ident,
                                            #field_ident_double_quotes_token_stream,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream),
                                            #increment_snake_case
                                        ) {
                                            Ok(mut value_cddc8a0a) => {
                                                let _: Option<char> = value_cddc8a0a.pop();
                                                #if_write_is_err_curly_braces_token_stream
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        {
                                            let mut acc_0fe559fa = #std_string_string_token_stream::new();
                                            #(#content_token_stream)*
                                            let _: Option<char> = acc_0fe559fa.pop();
                                            let _: Option<char> = acc_0fe559fa.pop();
                                            acc_0fe559fa
                                        }
                                    ))
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = vec_syn_field.iter().map(|element_88c65ca5| {
                                    let field_ident = &element_88c65ca5.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_88c65ca5.field_type);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(
                                        &format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{field_ident}'")
                                    );
                                    let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_0e9170a3, "jsonb_build_object({value_93015133})||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &value_90219286.#field_ident,
                                            #field_ident_double_quotes_token_stream,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream),
                                            #increment_snake_case
                                        ) {
                                            Ok(mut value_93015133) => {
                                                let _: Option<char> = value_93015133.pop();
                                                #if_write_is_err_curly_braces_token_stream
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#value_snake_case.0 {
                                            Some(value_90219286) => format!(
                                                "jsonb_build_object('value',{})",
                                                {
                                                    let mut acc_0e9170a3 = #std_string_string_token_stream::new();
                                                    #(#content_token_stream)*
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
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = vec_syn_field_with_id.iter().map(|element_bfecacfd| {
                                    let field_ident = &element_bfecacfd.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_bfecacfd.field_type);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_0f2b92d0, "jsonb_build_object({value_6d76c065})||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &element_3c1dab62.#field_ident,
                                            #field_ident_double_quotes_token_stream,
                                            "elem",
                                            #increment_snake_case
                                        ) {
                                            Ok(mut value_6d76c065) => {
                                                let _: Option<char> = value_6d76c065.pop();
                                                #if_write_is_err_curly_braces_token_stream
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                                    &quote::quote!{acc_44b1f772, "${value_73b58d3a},"},
                                    &return_err_query_part_error_named_write_into_buffer_token_stream
                                );
                                quote::quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
                                        {
                                            let mut acc_0f2b92d0 = #std_string_string_token_stream::new();
                                            for element_3c1dab62 in &#value_snake_case.0 {
                                                #(#content_token_stream)*
                                            }
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            let _: Option<char> = acc_0f2b92d0.pop();
                                            format!("jsonb_build_object('value',{acc_0f2b92d0})")
                                        },
                                        &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                        {
                                            let mut acc_44b1f772 = #std_string_string_token_stream::new();
                                            for _ in &#value_snake_case.0 {
                                                match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                    Ok(value_73b58d3a) => {
                                                        #if_write_is_err_token_stream
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    },
                                                }
                                            }
                                            let _: Option<char> = acc_44b1f772.pop();
                                            acc_44b1f772
                                        }
                                    ))
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = vec_syn_field_with_id.iter().map(|element_76f33159| {
                                    let field_ident = &element_76f33159.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_76f33159.field_type);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let if_write_is_err_curly_braces_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                                        &quote::quote!{acc_1a91bdc7, "jsonb_build_object({value_d49fe9d8})||"},
                                        &return_err_query_part_error_named_write_into_buffer_token_stream
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &element_9bdcd847.#field_ident,
                                            #field_ident_double_quotes_token_stream,
                                            "elem",
                                            #increment_snake_case
                                        ) {
                                            Ok(mut value_d49fe9d8) => {
                                                let _: Option<char> = value_d49fe9d8.pop();
                                                #if_write_is_err_curly_braces_token_stream
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                                    &quote::quote!{acc_857ce631, "${value_7f11bec0},"},
                                    &return_err_query_part_error_named_write_into_buffer_token_stream
                                );
                                quote::quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',{}),",
                                        match &#value_snake_case.0 {
                                            Some(value_3c415c92) => format!(
                                                "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                                                {
                                                    let mut acc_1a91bdc7 = #std_string_string_token_stream::new();
                                                    for element_9bdcd847 in &value_3c415c92.0 {
                                                        #(#content_token_stream)*
                                                    }
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    let _: Option<char> = acc_1a91bdc7.pop();
                                                    format!("jsonb_build_object('value',{acc_1a91bdc7})")
                                                },
                                                &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                                {
                                                    let mut acc_857ce631 = #std_string_string_token_stream::new();
                                                    for _ in &value_3c415c92.0 {
                                                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                            Ok(value_7f11bec0) => {
                                                                #if_write_is_err_token_stream
                                                            },
                                                            Err(#error_snake_case) => {
                                                                return Err(#error_snake_case);
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
                    &postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = vec_syn_field.iter().map(|element_9d72fe6e| {
                                    let field_ident = &element_9d72fe6e.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_9d72fe6e.field_type);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            &#value_snake_case.#field_ident,
                                            #query_snake_case
                                        ) {
                                            Ok(value_231618d9) => {
                                                #query_snake_case = value_231618d9;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    #(#content_token_stream)*
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote::quote!{
                                    if let Some(value_a1ccd526) = &#value_snake_case.0 {
                                        match #ident_standart_not_null_as_import_path_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            value_a1ccd526,
                                            #query_snake_case
                                        ) {
                                            Ok(value_70ed6013) => {
                                                #query_snake_case = value_70ed6013;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = vec_syn_field_with_id.iter().map(|element_43b720bb| {
                                    let field_ident = &element_43b720bb.field_ident;
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element_43b720bb.field_type);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(&element_9bdcd847.#field_ident, #query_snake_case) {
                                            Ok(value_ade27463) => {
                                                #query_snake_case = value_ade27463;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for element_9bdcd847 in &#value_snake_case.0 {
                                        #(#content_token_stream)*
                                    }
                                    for element_b191a891 in &#value_snake_case.0 {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_create_for_query(
                                            element_b191a891.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(value_a3749ea8) => {
                                                #query_snake_case = value_a3749ea8;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote::quote!{
                                    if let Some(value_0b55a65a) = &#value_snake_case.0 {
                                        match #ident_array_not_null_as_import_path_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(value_0b55a65a, #query_snake_case) {
                                            Ok(value_ad6a1ac5) => {
                                                #query_snake_case = value_ad6a1ac5;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                        },
                    },
                );
                let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_type_token_stream(
                    &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                    &ident,
                    &ident_table_type_declaration_upper_camel_case,
                    &postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                    &{
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_owned());
                        quote::quote! {
                            format!(#format_handle_token_stream, serde_json::to_string(&schemars::schema_for!(#ident_table_type_declaration_upper_camel_case)).expect("59a1654b-cbde-40a6-a958-383d263ee19d"))
                        }
                    },
                    &ident_create_upper_camel_case,
                    &postgresql_crud_macros_common::CreateQueryPartValueUnderscore::True,
                    &postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                    &quote::quote!{
                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                    &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                    &quote::quote!{
                        if let Err(#error_snake_case) = #query_snake_case.try_bind(
                            #self_as_postgresql_json_type_create_for_query_token_stream::from(#value_snake_case)
                        ) {
                            return Err(#error_snake_case.to_string());
                        }
                        Ok(#query_snake_case)
                    },
                    &ident_select_upper_camel_case,
                    &postgresql_crud_macros_common::SelectQueryPartValueUnderscore::False,
                    &quote::quote! {
                        match #value_snake_case.#select_query_part_postgresql_type_snake_case(#column_snake_case) {
                            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &ident_where_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &value_snake_case,
                    &ident_read_only_ids_upper_camel_case,
                    &quote::quote! {
                        match #self_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(#column_snake_case) {
                            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &ident_read_inner_upper_camel_case,
                    &quote::quote!{#self_as_postgresql_json_type_token_stream::into_inner(#value_snake_case)},
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &postgresql_crud_macros_common::UpdateQueryPartValueUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::False,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#self_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                value,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                increment
                            )},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_update_query_part_standart_nullable_token_stream(
                                &postgresql_type_or_postgresql_json_type_postgresql_json_type
                            )
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_update_query_part_array_not_null_token_stream(
                                &postgresql_type_or_postgresql_json_type_postgresql_json_type
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = generate_update_delete_create_array_token_stream(&quote::quote!{
                                    "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
                                });
                                quote::quote! {
                                    match &#value_snake_case.0 {
                                        Some(value_58d685d3) => {
                                            #content_token_stream
                                        },
                                        None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                            Ok(value_d31ab6f0) => Ok(format!("${value_d31ab6f0}")),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    }
                                }
                            },
                        },
                    },
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::False,
                    &quote::quote!{#self_as_postgresql_json_type_token_stream::#update_query_bind_snake_case(
                        #value_snake_case,
                        #query_snake_case
                    )},
                    &quote::quote!{
                        match #value_snake_case.#select_only_updated_ids_query_part_snake_case(
                            #column_snake_case,
                            #increment_snake_case
                        ) {
                            Ok(value_f0787243) => Ok(format!("jsonb_build_object('value',{value_f0787243}) as {column},")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                    &quote::quote!{#self_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                        #value_snake_case,
                        #query_snake_case
                    )},
                );
                match &trait_gen {
                    TraitGen::PostgresqlTypeAndPostgresqlJsonType => (impl_postgresql_crud_postgresql_json_type_for_ident_token_stream, impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream),
                    TraitGen::PostgresqlJsonType => (impl_postgresql_crud_postgresql_json_type_for_ident_token_stream, proc_macro2::TokenStream::new()),
                }
            };
            let self_postgresql_json_type_token_stream = quote::quote!{#self_upper_camel_case::#postgresql_json_type_upper_camel_case};
            let (impl_postgresql_json_type_test_cases_for_ident_token_stream, impl_postgresql_type_test_cases_for_ident_token_stream) = {
                let generate_dimension_equal_token_stream = |dimension: &postgresql_crud_macros_common::Dimension|{
                    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case();
                    let generate_nullable_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote! {
                        match #import_path::NotEmptyUniqueVec::try_new(
                            match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                (Some(read_only_ids_cdcb6239), Some(create_fdd53941)) => match <
                                    #content_token_stream
                                    as
                                    #import_path::PostgresqlJsonTypeTestCases
                                >::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                    read_only_ids_cdcb6239,
                                    create_fdd53941
                                ) {
                                    Some(value_d6124e21) => {
                                        let mut acc_bd78dc08 = Vec::new();
                                        for element_6739e82f in value_d6124e21.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![element_6739e82f]
                                            ) {
                                                Ok(value_7ed84f3b) => {
                                                    acc_bd78dc08.push(
                                                        #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_7ed84f3b))
                                                    );
                                                },
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("23dca12f-65c0-4c0e-addd-cc392c663733")
                                                }
                                            }
                                        }
                                        let value_e48110ec = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_d6124e21));
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
                                (None, None) => vec![#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(None)]
                            }
                        ) {
                            Ok(value_55f2dc3d) => Some(value_55f2dc3d),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("88912e24-3bee-4dc4-a373-6d96d260170f")
                            }
                        }
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = vec_syn_field.iter().map(|element_3a1eac56| {
                                    let field_ident = &element_3a1eac56.field_ident;
                                    let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_3a1eac56.field_type);
                                    quote::quote! {
                                        if let Some(value_2bbd2c96) = #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                            #create_snake_case.#field_ident
                                        ) {
                                            for element_84537322 in value_2bbd2c96.clone().into_vec() {
                                                acc_2fe1cca8.push(
                                                    #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![element_84537322]
                                                        ).expect("9a25e058-6701-430f-a1d1-729aa5e8058a")
                                                    )
                                                );
                                            }
                                            let value_c45bab0d = #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                #import_path::PostgresqlTypeWhere::new(
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
                                quote::quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_2fe1cca8 = Vec::new();
                                        #(#content_token_stream)*
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
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_standart_not_null_upper_camel_case)
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream_f0710cd9 = {
                                    let content_token_stream_57d244f8 = vec_syn_field.iter().map(|element_18682ae5| {
                                        let field_ident = &element_18682ae5.field_ident;
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_18682ae5.field_type);
                                        quote::quote! {
                                            if let Some(value_bf84026e) = #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                                read_only_ids_420d38ca.0.#value_snake_case.#field_ident.clone(),
                                                create_76f032c1.#field_ident.clone()
                                            ) {
                                                for element_f6be06df in value_bf84026e.clone().into_vec() {
                                                    let value_592e6b5f = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![element_f6be06df]
                                                        ).expect("1f7ae335-461f-4215-8fb5-ee7cf2f32881")
                                                    );
                                                    if !acc_dd377eb1.contains(&value_592e6b5f) {
                                                        acc_dd377eb1.push(value_592e6b5f);
                                                    }
                                                }
                                                let value_03205172 = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                    #import_path::PostgresqlTypeWhere::new(
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
                                    quote::quote!{#(#content_token_stream_57d244f8)*}
                                };
                                let content_token_stream_2cc4a40e = match &dimension {
                                    postgresql_crud_macros_common::Dimension::One => {
                                        let dimension_one_token_stream = {
                                            let content_token_stream_91a09fe2 = vec_syn_field.iter().map(|element_a83927c8| {
                                                let field_ident = &element_a83927c8.field_ident;
                                                let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_a83927c8.field_type);
                                                quote::quote! {
                                                    #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                        read_only_ids_420d38ca.0.#value_snake_case.#field_ident,
                                                        create_76f032c1.#field_ident
                                                    )
                                                }
                                            });
                                            quote::quote!{
                                                acc_dd377eb1.push(#ident_where_upper_camel_case::DimensionOneEqual(#import_path::PostgresqlJsonTypeWhereDimensionOneEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    dimensions: #import_path::BoundedStdVecVec::try_from(
                                                        vec![
                                                            #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                                i32::try_from(index_47620dcf).expect("5341936f-ce9e-4e14-ae30-765f04c12e14")
                                                            ).expect("76906f3c-4472-4ac0-a605-1b02f02fd680")
                                                        ]
                                                    ).expect("8a624c70-3701-4907-b361-5637c5361e1f"),
                                                    #value_snake_case: #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                        <#uuid_uuid_as_not_null_jsonb_string_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                            read_only_ids_420d38ca.0.#value_snake_case.#id_snake_case,
                                                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                        ),
                                                        #(#content_token_stream_91a09fe2),*
                                                    ),
                                                }));
                                            }
                                        };
                                        quote::quote!{
                                            for (index_47620dcf, (read_only_ids_420d38ca, create_76f032c1)) in #read_only_ids_snake_case.0.#value_snake_case.into_iter()
                                                .zip(#create_snake_case.0.into_iter())
                                                .enumerate()
                                            {
                                                #content_token_stream_f0710cd9
                                                #dimension_one_token_stream
                                            }
                                        }
                                    },
                                    postgresql_crud_macros_common::Dimension::Two |
                                    postgresql_crud_macros_common::Dimension::Three |
                                    postgresql_crud_macros_common::Dimension::Four => quote::quote!{
                                        for (read_only_ids_420d38ca, create_76f032c1) in #read_only_ids_snake_case.0.#value_snake_case.into_iter()
                                            .zip(#create_snake_case.0.into_iter())
                                        {
                                            #content_token_stream_f0710cd9
                                        }
                                    },
                                };
                                quote::quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_dd377eb1 = Vec::new();
                                        #content_token_stream_2cc4a40e
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
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_array_not_null_upper_camel_case)
                        },
                    }
                };
                (
                    {
                        let option_vec_create_token_stream = {
                            let content_token_stream = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_4f2f70d2| {
                                            let field_ident = &element_4f2f70d2.field_ident;
                                            let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_4f2f70d2.field_type);
                                            let parameters_token_stream = vec_syn_field.iter().map(|element_value| {
                                                let current_field_ident = &element_value.field_ident;
                                                if field_ident == current_field_ident {
                                                    quote::quote! {element_37154498}
                                                } else {
                                                    quote::quote! {
                                                        #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                    }
                                                }
                                            });
                                            quote::quote! {
                                                if let Some(value_0296b347) = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                                    for element_37154498 in value_0296b347 {
                                                        let #value_snake_case = #self_as_postgresql_json_type_create_token_stream::new(
                                                            #(#parameters_token_stream),*
                                                        );
                                                        if !acc_ccd79a32.contains(&#value_snake_case) {
                                                            acc_ccd79a32.push(#value_snake_case);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote::quote! {#(#content_token_stream)*}
                                    },
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_ddefdb90| {
                                            let field_ident = &element_ddefdb90.field_ident;
                                            let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_ddefdb90.field_type);
                                            let (
                                                option_additional_parameters_token_stream,
                                                parameters_token_stream
                                            ) = {
                                                #[derive(Clone)]
                                                enum ShouldAddDotClone {
                                                    True,
                                                    False
                                                }
                                                let generate_parameters_token_stream = |
                                                    should_add_dot_clone: ShouldAddDotClone,
                                                    element_token_stream: &dyn quote::ToTokens,
                                                |{
                                                    vec_syn_field.iter().map(|element_value| {
                                                        let current_field_ident = &element_value.field_ident;
                                                        if field_ident == current_field_ident {
                                                            let maybe_dot_clone_token_stream = match should_add_dot_clone.clone() {
                                                                ShouldAddDotClone::True => quote::quote! { .clone() },
                                                                ShouldAddDotClone::False => proc_macro2::TokenStream::new(),
                                                            };
                                                            quote::quote! {#element_token_stream #maybe_dot_clone_token_stream}
                                                        } else {
                                                            quote::quote! {#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                                                        }
                                                    }).collect::<Vec<proc_macro2::TokenStream>>()
                                                };
                                                (
                                                    generate_parameters_token_stream(
                                                        ShouldAddDotClone::True,
                                                        &quote::quote!{element_37154498}
                                                    ),
                                                    generate_parameters_token_stream(
                                                        ShouldAddDotClone::False,
                                                        &quote::quote!{element_37154498}
                                                    )
                                                )
                                            };
                                            quote::quote! {
                                                if let Some(vec_create) = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                                    let mut acc_6a886d56 = Vec::new();
                                                    let option_additional = {
                                                        let mut option_additional = None;
                                                        for element_37154498 in &vec_create {
                                                            if option_additional.is_none() {
                                                                let #value_snake_case = #ident_with_id_standart_not_null_create_upper_camel_case::new(
                                                                    #(#option_additional_parameters_token_stream),*
                                                                );
                                                                option_additional = Some((
                                                                    #ident_create_upper_camel_case::new(vec![#value_snake_case.clone()]),
                                                                    #ident_create_upper_camel_case::new(vec![#value_snake_case.clone(), #value_snake_case])
                                                                ));
                                                            }
                                                            else {
                                                                break;
                                                            }
                                                        }
                                                        option_additional
                                                    };
                                                    let has_len_greater_than_one = vec_create.len() > 1;
                                                    for element_37154498 in vec_create {
                                                        acc_6a886d56.push(#ident_with_id_standart_not_null_create_upper_camel_case::new(
                                                            #(#parameters_token_stream),*
                                                        ));
                                                    }
                                                    {
                                                        let value_07c0c08c = #ident_create_upper_camel_case::new(acc_6a886d56);
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
                                        quote::quote! {#(#content_token_stream)*}
                                    },
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let (
                                        current_ident_not_null_as_postgresql_json_type_test_cases_token_stream,
                                        content_token_stream
                                    ): (
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens
                                    ) = match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => (
                                            &ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream,
                                            &proc_macro2::TokenStream::new()
                                        ),
                                        PostgresqlJsonObjectTypePattern::Array => (
                                            &ident_array_not_null_as_postgresql_json_type_test_cases_token_stream,
                                            &quote::quote!{.0}
                                        ),
                                    };
                                    quote::quote! {
                                        if let Some(value_399e6a50) = #current_ident_not_null_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                            for element_e2767811 in value_399e6a50 {
                                                let #value_snake_case = #self_as_postgresql_json_type_token_stream::Create::new(Some(element_e2767811 #content_token_stream));
                                                if !acc_ccd79a32.contains(&#value_snake_case) {
                                                    acc_ccd79a32.push(#value_snake_case);
                                                }
                                            }
                                        }
                                        {
                                            let #value_snake_case = #self_as_postgresql_json_type_token_stream::Create::new(None);
                                            if !acc_ccd79a32.contains(&#value_snake_case) {
                                                acc_ccd79a32.push(#value_snake_case);
                                            }
                                        }
                                    }
                                }
                            };
                            quote::quote!{Some({
                                let mut acc_ccd79a32 = Vec::new();
                                #content_token_stream
                                acc_ccd79a32
                            })}
                        };
                        let read_only_ids_to_two_dimensional_vec_read_inner_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_last_initialization_token_stream = {
                                        if vec_syn_field.len() == 1 {
                                            proc_macro2::TokenStream::new()
                                        }
                                        else {
                                            let content_token_stream = vec_syn_field.iter().map(|element_43e09b9b| {
                                                let field_ident = &element_43e09b9b.field_ident;
                                                let field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&field_ident);
                                                let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_43e09b9b.field_type);
                                                quote::quote! {
                                                    let mut #field_ident_last_snake_case = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                        read_only_ids.0.value.#field_ident.clone()
                                                    );
                                                }
                                            });
                                            quote::quote!{#(#content_token_stream)*}
                                        }
                                    };
                                    let content_token_stream = vec_syn_field.iter().map(|element_9b199f7f| {
                                        let field_ident = &element_9b199f7f.field_ident;
                                        let field_ident_current_snake_case = naming::parameter::SelfCurrentSnakeCase::from_display(&field_ident);
                                        let field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&field_ident);
                                        let maybe_field_ident_last_clone_from_field_ident_current = if vec_syn_field.len() == 1 {
                                            proc_macro2::TokenStream::new()
                                        }
                                        else {
                                            quote::quote!{#field_ident_last_snake_case.clone_from(&#field_ident_current_snake_case);}
                                        };
                                        let fields_token_stream = vec_syn_field.iter().map(|element_value| {
                                            let current_field_ident = &element_value.field_ident;
                                            let current_field_ident_current_snake_case = naming::parameter::SelfCurrentSnakeCase::from_display(&current_field_ident);
                                            let current_field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&current_field_ident);
                                            let content_token_stream: &dyn quote::ToTokens = if field_ident == current_field_ident {
                                                &current_field_ident_current_snake_case
                                            } else {
                                                &current_field_ident_last_snake_case
                                            };
                                            quote::quote! {#current_field_ident: #content_token_stream.clone()}
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_9b199f7f.field_type);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element_2720df8a});
                                        quote::quote! {
                                            for element_7bf83754 in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#read_only_ids_snake_case.0.value.#field_ident) {
                                                for element_2720df8a in element_7bf83754 {
                                                    let #field_ident_current_snake_case = Some(#value_content_token_stream);
                                                    #maybe_field_ident_last_clone_from_field_ident_current
                                                    acc_ef081dc3.push(
                                                        vec![
                                                            #ident_standart_not_null_read_inner_upper_camel_case {
                                                                #(#fields_token_stream),*
                                                            }
                                                        ]
                                                    );
                                                }
                                            }
                                        }
                                    });
                                    quote::quote! {
                                        let mut acc_ef081dc3 = Vec::new();
                                        #fields_last_initialization_token_stream
                                        #(#content_token_stream)*
                                        acc_ef081dc3
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        #read_only_ids_snake_case.0.#value_snake_case.as_ref().into_iter().flat_map(|value_5fa0668c| {
                                            #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::
                                                #read_only_ids_to_two_dimensional_vec_read_inner_snake_case(value_5fa0668c)
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
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = vec_syn_field.iter().map(|element_bb247316| {
                                        let field_ident = &element_bb247316.field_ident;
                                        let fields_token_stream = vec_syn_field.iter().map(|element_value| {
                                            let current_field_ident = &element_value.field_ident;
                                            if field_ident == current_field_ident {
                                                let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element_18d1f553});
                                                quote::quote! {
                                                    #current_field_ident: Some(#value_content_token_stream)
                                                }
                                            } else {
                                                let current_field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_value.field_type);
                                                quote::quote! {
                                                    #current_field_ident: #current_field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                        element_49a5bb62.0.#value_snake_case.#current_field_ident.clone()
                                                    )
                                                }
                                            }
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_bb247316.field_type);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element_49a5bb62.0.#value_snake_case.#id_snake_case.0.#value_snake_case});
                                        quote::quote! {
                                            for element_4b4da5aa in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(
                                                &element_49a5bb62.0.#value_snake_case.#field_ident.clone()
                                            ) {
                                                for element_18d1f553 in element_4b4da5aa {
                                                    acc_00b3df88.push(
                                                        vec![
                                                            #ident_with_id_standart_not_null_read_inner_upper_camel_case {
                                                                #id_snake_case: Some(#value_content_token_stream),
                                                                #(#fields_token_stream),*
                                                            }
                                                        ]
                                                    );
                                                }
                                            }
                                        }
                                    });
                                    quote::quote! {
                                        #read_only_ids_snake_case.0.#value_snake_case.iter().map(|element_49a5bb62|{
                                            let mut acc_00b3df88 = Vec::new();
                                            #(#content_token_stream)*
                                            acc_00b3df88
                                        })
                                        .collect()
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        let mut acc_fb5111f1 = Vec::new();
                                        if let Some(value_6ee5750e) = &#read_only_ids_snake_case.0.#value_snake_case {
                                            for element_4a5a4b09 in #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(value_6ee5750e) {
                                                for element_264980ec in element_4a5a4b09 {
                                                    acc_fb5111f1.push(vec![Some(element_264980ec)]);
                                                }
                                            }
                                        }
                                        acc_fb5111f1.push(vec![None]);
                                        acc_fb5111f1
                                    }
                                }
                            },
                        };
                        let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    let parameters_token_stream = vec_syn_field.iter().map(|element_13640e7f| {
                                        let field_ident = &element_13640e7f.field_ident;
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_13640e7f.field_type);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(value_8ff65e09.#value_snake_case)
                                        });
                                        quote::quote! {#value_snake_case.#field_ident.map(|value_8ff65e09|#value_content_token_stream)}
                                    });
                                    quote::quote! {#self_element_as_postgresql_type_read_token_stream::try_new(#(#parameters_token_stream),*).expect("3aeeabba-3ffc-4df2-a3bf-e389c40ec566")}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_read_token_stream::new(
                                            #value_snake_case.map(#ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case)
                                        )
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = vec_syn_field_with_id.iter().map(|element_96f7b50a| {
                                        let field_ident = &element_96f7b50a.field_ident;
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_96f7b50a.field_type);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(value_3ac52220.#value_snake_case)
                                        });
                                        quote::quote! {#field_ident: element_ffed1bfc.#field_ident.map(|value_3ac52220|#value_content_token_stream)}
                                    });
                                    quote::quote!{
                                        #ident_read_upper_camel_case::new(
                                            #value_snake_case.into_iter().map(|element_ffed1bfc| #ident_with_id_standart_not_null_read_upper_camel_case {
                                                #(#content_token_stream),*
                                            }).collect()
                                        )
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = vec_syn_field_with_id.iter().map(|element_e47b9709| {
                                        let field_ident = &element_e47b9709.field_ident;
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_e47b9709.field_type);
                                        // let maybe_dot_clone_token_stream = if vec_syn_field.len() == 1 {
                                        //     proc_macro2::TokenStream::new()
                                        // }
                                        // else {
                                        //     quote::quote!{.clone()}
                                        // };
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(
                                                element_5c1f7f63.#value_snake_case
                                                // #maybe_dot_clone_token_stream
                                                .clone()
                                            )
                                        });
                                        quote::quote! {
                                            #field_ident: element_ffed1bfc.#field_ident.as_ref().map(|element_5c1f7f63| #value_content_token_stream)
                                        }
                                    });
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_read_token_stream::new(
                                            #value_snake_case.map(|value_189e3c07|
                                                value_189e3c07
                                                .into_iter()
                                                .map(|element_ffed1bfc|#ident_with_id_standart_not_null_read_upper_camel_case {
                                                    #(#content_token_stream),*
                                                }).collect()
                                            )
                                        )
                                    }
                                }
                            },
                        };
                        let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => {
                                    let self_element_as_postgresql_type_update_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Update);
                                    let parameters_token_stream = vec_syn_field.iter().map(|element_cefffeeb| {
                                        let field_ident = &element_cefffeeb.field_ident;
                                        let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_cefffeeb.field_type);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(element_23bdfe1e.#value_snake_case)
                                        });
                                        quote::quote! {
                                            acc_ebea163e.extend(#value_snake_case.#field_ident.map(|element_23bdfe1e| {
                                                #ident_standart_not_null_update_element_upper_camel_case::#field_ident_upper_camel_case(#value_content_token_stream)
                                            }));
                                        }
                                    });
                                    quote::quote! {
                                        #self_element_as_postgresql_type_update_token_stream::new(
                                            #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_ebea163e = Vec::new();
                                                #(#parameters_token_stream)*
                                                acc_ebea163e
                                            }).expect("a06dbdc5-296c-48a8-ba00-913e1fe82ebf")
                                        )
                                    }
                                },
                                PostgresqlJsonObjectTypePattern::Array => {
                                    let fields_token_stream = vec_syn_field.iter().map(|element_d13faa4c| {
                                        let field_ident = &element_d13faa4c.field_ident;
                                        quote::quote! {#field_ident: element_ffed1bfc.#field_ident}
                                    });
                                    quote::quote! {
                                        #ident_update_upper_camel_case::try_new(
                                            Vec::new(),
                                            #import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream::try_new(
                                                #value_snake_case.into_iter().map(|element_ffed1bfc| #ident_with_id_standart_not_null_update_element_upper_camel_case {
                                                    #id_snake_case: #uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case::new(element_ffed1bfc.#id_snake_case.clone().expect("f04a2c6d-bc0b-4693-b7e5-ede348cb229e").#value_snake_case),
                                                    fields: #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(
                                                        #ident_standart_not_null_read_inner_upper_camel_case {
                                                            #(#fields_token_stream),*
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
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = generate_type_as_postgresql_type_test_cases_token_stream(match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                                    PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_upper_camel_case,
                                });
                                let self_element_as_postgresql_type_update_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Update);
                                quote::quote! {
                                    #self_element_as_postgresql_type_update_token_stream::new(
                                        #value_snake_case.map(#content_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case)
                                    )
                                }
                            },
                        };
                        let read_only_ids_into_option_value_read_inner_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_fields_read_only_ids_into_option_value_read_inner_token_stream(&is_standart_with_id_false, &value_snake_case),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        #value_snake_case.0.#value_snake_case.and_then(|value_5d7e3961| match #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::read_only_ids_into_option_value_read_inner(
                                            value_5d7e3961
                                        ) {
                                            Some(value_cfca0099) => Some(value_cfca0099.#value_snake_case),
                                            None => None,
                                        })
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        let content_token_stream = vec_syn_field_with_id.iter().map(|element_3ebdd830| {
                                            let field_ident = &element_3ebdd830.field_ident;
                                            let field_type = &element_3ebdd830.field_type;
                                            let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&field_type);
                                            let field_type_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&field_type, &PostgresqlJsonTypeSubtype::Read);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                                let default_but_option_is_always_some_call_token_stream = generate_ident_as_default_but_option_is_always_some_call_token_stream(
                                                    &field_type_as_postgresql_json_type_read_token_stream
                                                );
                                                quote::quote!{#field_type_as_postgresql_json_type_token_stream::into_inner(#default_but_option_is_always_some_call_token_stream)}
                                            });
                                            quote::quote! {
                                                #field_ident: #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_into_option_value_read_inner(
                                                    element_6603f209.0.#value_snake_case.#field_ident
                                                ).map_or_else(|| Some(#value_content_token_stream), Some)
                                            }
                                        });
                                        quote::quote!{
                                            #value_snake_case.0.#value_snake_case.into_iter().fold(Vec::new(), |mut acc_cf4743b1, element_6603f209| {
                                                acc_cf4743b1.push(#ident_with_id_standart_not_null_read_inner_upper_camel_case {
                                                    #(#content_token_stream),*
                                                });
                                                acc_cf4743b1
                                            })
                                        }
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        #value_snake_case.0.#value_snake_case.and_then(|value_f816032d| match #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                            value_f816032d
                                        ) {
                                            Some(value_d0549423) => Some(value_d0549423.#value_snake_case),
                                            None => None,
                                        })
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                            },
                        };
                        let update_to_read_only_ids_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_initialization_content_token_stream = vec_syn_field.iter().map(|element_3f07f901| {
                                        let field_ident = &element_3f07f901.field_ident;
                                        quote::quote! {let mut #field_ident = None;}
                                    });
                                    let match_content_token_stream = vec_syn_field.iter().map(|element_4fb1f3d0| {
                                        let field_ident = &element_4fb1f3d0.field_ident;
                                        let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_4fb1f3d0.field_type);
                                        quote::quote! {
                                            #ident_update_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(value_0f4d677e) => {
                                                #field_ident = Some(#field_type_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&value_0f4d677e.#value_snake_case));
                                            }
                                        }
                                    });
                                    let struct_fields_content_token_stream = vec_syn_field.iter().map(|element_af4d3d80| {
                                        let field_ident = &element_af4d3d80.field_ident;
                                        quote::quote! {#field_ident: #field_ident.expect("106f16f2-ae03-48b3-9239-f4b1b60746d5")}
                                    });
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        #ident_read_only_ids_handle_upper_camel_case{
                                            #(#struct_fields_content_token_stream),*
                                        }
                                    });
                                    quote::quote! {
                                        #(#fields_initialization_content_token_stream)*
                                        for element_b3974846 in #value_snake_case.0.to_vec() {
                                            match element_b3974846 {
                                                #(#match_content_token_stream),*
                                            }
                                        }
                                        #ident_read_only_ids_upper_camel_case(#value_content_token_stream)
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        quote::quote!{
                                            #value_snake_case.0.as_ref().map(#ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case)
                                        }
                                    });
                                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_content_token_stream)}
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        let initialization_token_stream = vec_syn_field.iter().map(|element_09cee28c| {
                                            let field_ident = &element_09cee28c.field_ident;
                                            quote::quote! {let mut #field_ident = None;}
                                        });
                                        let for_loop_token_stream = vec_syn_field.iter().map(|element_cf4923ce| {
                                            let field_ident = &element_cf4923ce.field_ident;
                                            let field_ident_token_stream = {
                                                let current_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                                quote::quote!{
                                                    #ident_standart_not_null_update_element_upper_camel_case::#current_field_ident_upper_camel_case_token_stream(value_d2a6daf8) => {
                                                        #field_ident = Some(value_d2a6daf8.#value_snake_case.clone());
                                                    },
                                                }
                                            };
                                            let fields_without_current_ident_token_stream = if vec_syn_field.is_empty() {
                                                proc_macro2::TokenStream::new()
                                            }
                                            else {
                                                let content_token_stream_e0bf4e67 = vec_syn_field
                                                .iter()
                                                .filter(|element_a1502880| element_a1502880.field_ident != *field_ident)
                                                .map(|element_2908bd7a| {
                                                    let current_field_ident = &element_2908bd7a.field_ident;
                                                    let current_field_ident_upper_camel_case_token_stream =
                                                        naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
                                                            &current_field_ident,
                                                        );
                                                    quote::quote! {
                                                        #ident_standart_not_null_update_element_upper_camel_case::#current_field_ident_upper_camel_case_token_stream(_)
                                                    }
                                                })
                                                .fold(None, |acc_bbf653f7, element_2be3f4ee| Some(match acc_bbf653f7 {
                                                    None => element_2be3f4ee,
                                                    Some(value_5b375af0) => quote::quote! { #value_5b375af0 | #element_2be3f4ee },
                                                }));
                                                content_token_stream_e0bf4e67.map_or_else(
                                                    proc_macro2::TokenStream::new,
                                                    |value_5c826b8c| quote::quote!{#value_5c826b8c => (),}
                                                )
                                            };
                                            quote::quote! {
                                                for element_da177c5a in element_4634bb8a.fields.0.to_vec() {
                                                    match &element_da177c5a {
                                                        #field_ident_token_stream
                                                        #fields_without_current_ident_token_stream
                                                    }
                                                }
                                            }
                                        });
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                            let uuid_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
                                            let fields_token_stream = vec_syn_field.iter().map(|element_134779da| {
                                                let field_ident = &element_134779da.field_ident;
                                                let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_134779da.field_type);
                                                quote::quote! {
                                                    #field_ident: #field_type_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#field_ident.expect("a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d"))
                                                }
                                            });
                                            quote::quote!{
                                                #ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case {
                                                    #id_snake_case: #uuid_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&element_4634bb8a.#id_snake_case),
                                                    #(#fields_token_stream),*
                                                }
                                            }
                                        });
                                        quote::quote!{
                                            #value_snake_case.#update_snake_case.to_vec().iter().map(|element_4634bb8a|{
                                                #(#initialization_token_stream)*
                                                #(#for_loop_token_stream)*
                                                #ident_with_id_standart_not_null_read_only_ids_upper_camel_case(#value_content_token_stream)
                                            }).collect()
                                        }
                                    });
                                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_content_token_stream)}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        #value_snake_case.0.as_ref().map(#ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case)
                                    });
                                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_content_token_stream)}
                                }
                            },
                        };
                        let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = {
                            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_2b018c89| {
                                            let field_ident = &element_2b018c89.field_ident;
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_2b018c89.field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                    &#value_snake_case.0.#value_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::try_new(
                                                #(#parameters_token_stream),*
                                            ).expect("57820868-38ac-4f77-aa0f-dc734399d8b2")
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                        #ident_read_upper_camel_case::new(
                                            #value_snake_case.0.#value_snake_case.as_ref().and_then(|value_dfa7815e| match #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                value_dfa7815e
                                            ) {
                                                Some(value_02cef266) => Some(value_02cef266.#value_snake_case),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = vec_syn_field_with_id.iter().map(|element_f4599b96| {
                                            let field_ident = &element_f4599b96.field_ident;
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_f4599b96.field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                    &element_629b1544.0.#value_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::new({
                                                let mut acc_5f587d35 = #value_snake_case.0.#value_snake_case.clone().into_iter().map(|element_629b1544|{
                                                    #ident_with_id_standart_not_null_read_upper_camel_case::try_new(
                                                        #(#parameters_token_stream),*
                                                    ).expect("8f6fb6b6-6e84-4819-b9c6-526d39e1ac88")
                                                }).collect::<Vec<#ident_with_id_standart_not_null_read_upper_camel_case>>();
                                                acc_5f587d35.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.id, &second.id) {
                                                        //maybe remove .clone - add .get by ref method
                                                        #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::into_inner(
                                                            value_first.#value_snake_case.clone()
                                                        )
                                                        .cmp(&#import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::into_inner(
                                                            value_second.#value_snake_case.clone()
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
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                        #ident_read_upper_camel_case::new(
                                            #value_snake_case.0.#value_snake_case.as_ref().and_then(|value_16ab4136| match #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                value_16ab4136
                                            ) {
                                                Some(value_71a66429) => Some(value_71a66429.#value_snake_case.0),
                                                None => None,
                                            })
                                        )
                                    }
                                },
                            });
                            quote::quote!{Some(#value_initialization_token_stream)}
                        };
                        let previous_read_merged_with_option_update_into_read_token_stream = {
                            let fields_initialization_content_token_stream = vec_syn_field.iter().map(|element_8caae90c| {
                                let field_ident = &element_8caae90c.field_ident;
                                quote::quote! {let mut #field_ident = None;}
                            });
                            let match_content_token_stream = vec_syn_field.iter().map(|element_b625a4b0| {
                                let field_ident = &element_b625a4b0.field_ident;
                                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                quote::quote! {
                                    #ident_standart_not_null_update_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                        #field_ident = Some(#value_snake_case.#value_snake_case);
                                    }
                                }
                            });
                            let generate_struct_initialization_token_stream = |function: &dyn Fn(&dyn quote::ToTokens) -> proc_macro2::TokenStream|{//content_token_stream: &dyn quote::ToTokens
                                let token_stream = vec_syn_field.iter().map(|element_96e0a086| {
                                    let field_ident = &element_96e0a086.field_ident;
                                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                                        let content_token_stream = function(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_96e0a086.field_type);
                                        quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::previous_read_merged_with_option_update_into_read(
                                                #content_token_stream,
                                                #field_ident
                                            )
                                        }
                                    });
                                    quote::quote! {#field_ident: Some(#value_initialization_token_stream)}
                                });
                                quote::quote!{#(#token_stream),*}
                            };
                            let generate_option_token_stream = |current_postgresql_json_object_type_pattern: &PostgresqlJsonObjectTypePattern|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(match &current_postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                                    PostgresqlJsonObjectTypePattern::Array => &ident_array_not_null_upper_camel_case
                                });
                                quote::quote! {
                                    match #option_update_snake_case {
                                        Some(value_fca601b5) => #ident_read_upper_camel_case(
                                            match value_fca601b5.0 {
                                                Some(value_8d7747f1) => Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::previous_read_merged_with_option_update_into_read(
                                                        #read_snake_case.0.unwrap_or_else(#import_path_default_but_option_is_always_some_token_stream),
                                                        Some(value_8d7747f1),
                                                    )
                                                ),
                                                None => None,
                                            }
                                        ),
                                        None => #read_snake_case,
                                    }
                                }
                            };
                            match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let struct_initializattion_token_stream = generate_struct_initialization_token_stream(&|content_token_stream: &dyn quote::ToTokens|{
                                            quote::quote!{
                                                #read_snake_case.#content_token_stream.expect("a2d26e36-48f9-4739-aabe-adc0f0593570").#value_snake_case
                                            }
                                        });
                                        quote::quote!{
                                            match #option_update_snake_case {
                                                Some(value_e5377436) => {
                                                    #(#fields_initialization_content_token_stream)*
                                                    for element_629b1544 in value_e5377436.0.into_vec() {
                                                        match element_629b1544 {
                                                            #(#match_content_token_stream),*
                                                        }
                                                    }
                                                    #ident_read_upper_camel_case {
                                                        #struct_initializattion_token_stream
                                                    }
                                                },
                                                None => #read_snake_case
                                            }
                                        }
                                    },
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let struct_initializattion_token_stream = generate_struct_initialization_token_stream(&|content_token_stream: &dyn quote::ToTokens|{
                                            quote::quote!{
                                                found_read_element.#content_token_stream.expect("2e8229f7-38d6-48c1-93c9-e77631a3e155").#value_snake_case
                                            }
                                        });
                                        quote::quote! {
                                            match #option_update_snake_case {
                                                Some(value_47f5a20b) => #ident_read_upper_camel_case({
                                                    let mut acc_04a67ef2 = Vec::new();
                                                    for element_377d1bb4 in value_47f5a20b.#update_snake_case.into_vec() {
                                                        let mut option_read_element = None;
                                                        for element_d2daee30 in &#read_snake_case.0 {
                                                            if *#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(&element_377d1bb4.#id_snake_case.clone().into())
                                                            ==
                                                            #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream::into_inner(
                                                                element_d2daee30.#id_snake_case.clone().expect("df2413fe-e703-451b-ab75-add67da716f7").#value_snake_case
                                                            )
                                                            {
                                                                option_read_element = Some(element_d2daee30.clone());
                                                                break;
                                                            }
                                                        }
                                                        let found_read_element = option_read_element.expect("139882b9-d38f-4cb5-98ea-af0ab23ec474");
                                                        #(#fields_initialization_content_token_stream)*
                                                        for element_629b1544 in element_377d1bb4.fields.0.into_vec() {
                                                            match element_629b1544 {
                                                                #(#match_content_token_stream),*
                                                            }
                                                        }
                                                        acc_04a67ef2.push(#ident_with_id_standart_not_null_read_upper_camel_case {
                                                            #id_snake_case: found_read_element.#id_snake_case,
                                                            #struct_initializattion_token_stream
                                                        });
                                                    }
                                                    acc_04a67ef2
                                                }),
                                                None => #read_snake_case
                                            }
                                        }
                                    },
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_option_token_stream(postgresql_json_object_type_pattern)
                            }
                        };
                        let read_only_ids_merged_with_create_into_read_token_stream = {
                            let generate_nullable_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&current_ident_token_stream);
                                quote::quote! {
                                    #ident_read_upper_camel_case::new(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(read_only_ids_2b2ab8a1), Some(create_4a1adaa3)) => {
                                                Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                                                        read_only_ids_2b2ab8a1,
                                                        create_4a1adaa3
                                                    ).expect("56ac4450-0feb-4ea7-aca7-6f51c8f4893c").#value_snake_case #content_token_stream
                                                )
                                            },
                                            (Some(_), None) => panic!("75be9ae0-ca9f-4251-bfff-2156a90b10c6"),
                                            (None, Some(_)) => panic!("6a95d7ae-54f5-4e04-9217-223ba156b799"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_9bdce5ca| {
                                            let field_ident = &element_9bdce5ca.field_ident;
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_9bdce5ca.field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #create_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::try_new(
                                                #(#parameters_token_stream),*
                                            ).expect("52ad3994-8392-4fc5-8427-4ca42d87d726")
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_standart_not_null_upper_camel_case,
                                        &proc_macro2::TokenStream::new()
                                    )
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let generate_parameter_token_stream = |field_type: &dyn quote::ToTokens, field_ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                                                    read_only_ids_225e2b76.0.#value_snake_case.#field_ident,
                                                    #content_token_stream,
                                                )
                                            }
                                        };
                                        let id_parameter_token_stream = generate_parameter_token_stream(
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &id_snake_case,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_2a1148f0|{
                                            let field_ident = &element_2a1148f0.field_ident;
                                            generate_parameter_token_stream(
                                                &element_2a1148f0.field_type,
                                                &field_ident,
                                                &quote::quote! {create_3c660445.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::new({
                                                assert_eq!(#read_only_ids_snake_case.0.#value_snake_case.len(), #create_snake_case.0.len(), "90d33ddd-e08d-488c-8577-c75febe97301");
                                                let mut acc_37909420 = Vec::new();
                                                for (read_only_ids_225e2b76, create_3c660445) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()) {
                                                    acc_37909420.push(#ident_with_id_standart_not_null_read_upper_camel_case::try_new(
                                                        #id_parameter_token_stream,
                                                        #(#parameters_token_stream),*
                                                    ).expect("1330ac8d-ebf2-4c79-b25e-6394d58de927"));
                                                }
                                                acc_37909420
                                            })
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_array_not_null_upper_camel_case,
                                        &quote::quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_option_value_read_token_stream = {
                            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote!{
                                <#self_upper_camel_case as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_snake_case(
                                    #read_only_ids_snake_case,
                                    #create_snake_case
                                )
                            });
                            quote::quote!{Some(#value_initialization_token_stream)}
                        };
                        let read_only_ids_merged_with_create_into_table_type_declaration_token_stream = {
                            let generate_nullable_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&current_ident_token_stream);
                                quote::quote! {
                                    #ident_table_type_declaration_upper_camel_case::new(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(read_only_ids_fb2ec2e4), Some(create_2f615d4f)) => {
                                                Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                        read_only_ids_fb2ec2e4,
                                                        create_2f615d4f
                                                    ) #content_token_stream
                                                )
                                            },
                                            (Some(_), None) => panic!("9349dcd5-3ed3-4157-b1ef-14c51d55262f"),
                                            (None, Some(_)) => panic!("45f8e70a-ffca-41b6-ac70-96f101ac3c80"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_ca3a96db| {
                                            let field_ident = &element_ca3a96db.field_ident;
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_ca3a96db.field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #create_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote! {
                                            #ident_table_type_declaration_upper_camel_case::new(
                                                #(#parameters_token_stream),*
                                            )
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_standart_not_null_upper_camel_case,
                                        &proc_macro2::TokenStream::new()
                                    )
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let generate_parameter_token_stream = |field_type: &dyn quote::ToTokens, field_ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                    read_only_ids_94b49496.0.#value_snake_case.#field_ident,
                                                    #content_token_stream,
                                                )
                                            }
                                        };
                                        let id_parameter_token_stream = generate_parameter_token_stream(
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &id_snake_case,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_d5acef17|{
                                            let field_ident = &element_d5acef17.field_ident;
                                            generate_parameter_token_stream(
                                                &element_d5acef17.field_type,
                                                &field_ident,
                                                &quote::quote! {create_24629087.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_table_type_declaration_upper_camel_case::new({
                                                assert_eq!(#read_only_ids_snake_case.0.#value_snake_case.len(), #create_snake_case.0.len(), "7776a146-06a8-4972-8e16-371d41ee164c");
                                                let mut acc_319e1fb1 = Vec::new();
                                                for (read_only_ids_94b49496, create_24629087) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()) {
                                                    acc_319e1fb1.push(#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                        #id_parameter_token_stream,
                                                        #(#parameters_token_stream),*
                                                    ));
                                                }
                                                acc_319e1fb1
                                            })
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_array_not_null_upper_camel_case,
                                        &quote::quote!{.0}
                                    )
                                },
                            }
                        };
                        let read_only_ids_merged_with_create_into_where_equal_token_stream = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => {
                                    let parameters_token_stream = vec_syn_field.iter().map(|element_9990b32d| {
                                        let field_ident = &element_9990b32d.field_ident;
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_9990b32d.field_type);
                                        quote::quote! {
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                #create_snake_case.#field_ident
                                            )
                                        }
                                    });
                                    quote::quote! {
                                        #ident_where_upper_camel_case::#equal_upper_camel_case(
                                            #import_path::PostgresqlJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                #value_snake_case: #ident_table_type_declaration_upper_camel_case::new(
                                                    #(#parameters_token_stream),*
                                                )
                                            }
                                        )
                                    }
                                },
                                PostgresqlJsonObjectTypePattern::Array => {
                                    let generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream = |
                                        field_ident: &dyn quote::ToTokens,
                                        field_type: &dyn quote::ToTokens,
                                        content_token_stream: &dyn quote::ToTokens
                                    |{
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                        quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                read_only_ids_ea32954c.0.#value_snake_case.#field_ident,
                                                #content_token_stream
                                            )
                                        }
                                    };
                                    let current_ident_token_stream = generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                        &id_snake_case,
                                        &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                        &import_path_default_but_option_is_always_some_call_token_stream
                                    );
                                    let parameters_token_stream = vec_syn_field.iter().map(|element_fc74a022| {
                                        let field_ident = &element_fc74a022.field_ident;
                                        generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                            &field_ident,
                                            &element_fc74a022.field_type,
                                            &quote::quote!{create_3cbe8967.#field_ident}
                                        )
                                    });
                                    quote::quote! {
                                        #ident_where_upper_camel_case::#equal_upper_camel_case(
                                            #import_path::PostgresqlJsonTypeWhereEqual {
                                                logical_operator: #import_path::LogicalOperator::And,
                                                #value_snake_case: #ident_table_type_declaration_upper_camel_case::new({
                                                    let mut acc_321b3fcd = Vec::new();
                                                    for (read_only_ids_ea32954c, create_3cbe8967) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()) {
                                                        acc_321b3fcd.push(
                                                            #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                                #current_ident_token_stream,
                                                                #(#parameters_token_stream),*
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
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = {
                                    let current_ident_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&generate_ident_upper_camel_case(&match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        PostgresqlJsonObjectTypePattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote::quote!{
                                        vec![
                                            #current_ident_token_stream::#read_only_ids_merged_with_create_into_where_equal_snake_case(
                                                read_only_ids_ce30c0fe,
                                                create_8fd81ed8
                                            )
                                        ]
                                    }
                                };
                                quote::quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(read_only_ids_ce30c0fe), Some(create_8fd81ed8)) => match #import_path::NotEmptyUniqueVec::try_new(#content_token_stream) {
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
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = {
                            let content_token_stream = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let elements_token_stream = vec_syn_field.iter().map(|element_23a78055| {
                                            let field_ident = &element_23a78055.field_ident;
                                            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_23a78055.field_type);
                                            quote::quote! {
                                                #ident_where_upper_camel_case::#field_ident_upper_camel_case_token_stream(
                                                    #import_path::PostgresqlTypeWhere::new(
                                                        #import_path::LogicalOperator::And,
                                                        #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                            #create_snake_case.#field_ident
                                                        )
                                                    )
                                                )
                                            }
                                        });
                                        quote::quote! {#(#elements_token_stream),*}
                                    },
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream = |
                                            field_ident: &dyn quote::ToTokens,
                                            field_type: &dyn quote::ToTokens,
                                            content_token_stream: &dyn quote::ToTokens
                                        |{
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            quote::quote!{
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                    read_only_ids_319c9e78.0.#value_snake_case.#field_ident,
                                                    #content_token_stream
                                                )
                                            }
                                        };
                                        let current_ident_token_stream = generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                            &id_snake_case,
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = vec_syn_field.iter().map(|element_a8f4df4f| {
                                            let field_ident = &element_a8f4df4f.field_ident;
                                            generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                                &field_ident,
                                                &element_a8f4df4f.field_type,
                                                &quote::quote!{create_00ae06d2.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_where_upper_camel_case::#equal_upper_camel_case(
                                                #import_path::PostgresqlJsonTypeWhereEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    #value_snake_case: #ident_table_type_declaration_upper_camel_case::new({
                                                        let mut acc_97ebf7d6 = Vec::new();
                                                        for (read_only_ids_319c9e78, create_00ae06d2) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()) {
                                                            acc_97ebf7d6.push(
                                                                #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                                    #current_ident_token_stream,
                                                                    #(#parameters_token_stream),*
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
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = {
                                        let current_ident_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&generate_ident_upper_camel_case(&match &postgresql_json_object_type_pattern {
                                            PostgresqlJsonObjectTypePattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                            PostgresqlJsonObjectTypePattern::Array => IdentPattern::ArrayNotNullWithId,
                                        }));
                                        quote::quote! {
                                            #current_ident_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                                                read_only_ids_2898c440,
                                                create_f1c4667c
                                            )
                                        }
                                    };
                                    quote::quote! {
                                        #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                            match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                                (Some(read_only_ids_2898c440), Some(create_f1c4667c)) => Some(#content_token_stream),
                                                (Some(_), None) => panic!("49e4c289-b37d-4365-96e3-5d896d6860f7"),
                                                (None, Some(_)) => panic!("ad71caa2-2503-4f9a-952c-e796abf5bbbe"),
                                                (None, None) => None,
                                            }
                                        )
                                    }
                                },
                            };
                            quote::quote!{
                                #import_path::NotEmptyUniqueVec::try_new(vec![
                                    #content_token_stream
                                ]).expect("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
                            }
                        };
                        let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = vec_syn_field.iter().map(|element_459c3da8| {
                                        let field_ident = &element_459c3da8.field_ident;
                                        let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_459c3da8.field_type);
                                        quote::quote! {
                                            for element_d830c061 in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_snake_case(
                                                #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                #create_snake_case.#field_ident
                                            ).into_vec() {
                                                acc_89ec072c.push(
                                                    #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::Or,
                                                            vec![element_d830c061],
                                                        )
                                                        .expect("0c6ccad1-6ffc-451f-9b16-0731010fee9f"),
                                                    )
                                                );
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        #import_path::NotEmptyUniqueVec::try_new({
                                            let mut acc_89ec072c = Vec::new();
                                            #(#content_token_stream)*
                                            acc_89ec072c
                                        }).expect("9c50391c-001e-4a4f-aac0-14bb614de456")
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                    #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_12b6f16d = Vec::new();
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(read_only_ids_2f024927), Some(create_120c1dad)) => {
                                                for element_a8b181a0 in #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_snake_case(
                                                    read_only_ids_2f024927,
                                                    create_120c1dad
                                                ).into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![element_a8b181a0]) {
                                                        Ok(value_8e72cfd7) => {
                                                            acc_12b6f16d.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_8e72cfd7)));
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
                                                acc_12b6f16d.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(None));
                                            },
                                        }
                                        acc_12b6f16d
                                    }).expect("7efc9aae-4b7c-4821-b916-72f5eb2fbd6b")
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => quote::quote!{
                                #self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                                    #read_only_ids_snake_case,
                                    #create_snake_case
                                )
                            }
                        };
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream = generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::One);
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream = generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Two);
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream = generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Three);
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream = generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Four);
                        let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = {
                            let generate_nullable_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote! {
                                match #import_path::NotEmptyUniqueVec::try_new(
                                    match #create_snake_case.0 {
                                        Some(create_09a81dae) => match <
                                            #content_token_stream
                                            as
                                            #import_path::PostgresqlJsonTypeTestCases
                                        >::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(create_09a81dae) {
                                            Some(value_3680a4c9) => {
                                                let mut acc_5c441d3a = Vec::new();
                                                for element_a8b181a0 in value_3680a4c9.clone().into_vec() {
                                                    match #import_path::NotEmptyUniqueVec::try_new(vec![element_a8b181a0]) {
                                                        Ok(value_15097b27) => {
                                                            acc_5c441d3a.push(
                                                                #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
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
                                                let value_84ea8e4c = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_3680a4c9));
                                                if !acc_5c441d3a.contains(&value_84ea8e4c) {
                                                    acc_5c441d3a.push(value_84ea8e4c);
                                                }
                                                acc_5c441d3a
                                            },
                                            None => {
                                                return None;
                                            }
                                        },
                                        None => vec![#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(None)],
                                    }
                                ) {
                                    Ok(value_72dbefbc) => Some(value_72dbefbc),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("d41bcbca-5d4c-436c-a465-4920c9da6a43")
                                    }
                                }
                            };
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_d41dce84| {
                                            let field_ident = &element_d41dce84.field_ident;
                                            let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_d41dce84.field_type);
                                            quote::quote! {
                                                if let Some(value_927601a4) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
                                                    #create_snake_case.#field_ident
                                                ) {
                                                    for element_194a660a in value_927601a4.clone().into_vec() {
                                                        acc_587bf907.push(
                                                            #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                                #import_path::PostgresqlTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![element_194a660a]
                                                                ).expect("2f437949-0c13-4b15-83dd-8ef0399b7d61")
                                                            )
                                                        );
                                                    }
                                                    let value_84ea8e4c = #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::new(
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
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_587bf907 = Vec::new();
                                                #(#content_token_stream)*
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
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_standart_not_null_upper_camel_case)
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_c776b608| {
                                            let field_ident = &element_c776b608.field_ident;
                                            let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_c776b608.field_type);
                                            quote::quote! {
                                                for create_e06a9fe2 in #create_snake_case.0.clone() {
                                                    if let Some(value_ee015fcc) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
                                                        create_e06a9fe2.#field_ident
                                                    ) {
                                                        for element_63008daa in value_ee015fcc.clone().into_vec() {
                                                            let value_0ae29f5f = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                                #import_path::PostgresqlTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![element_63008daa]
                                                                )
                                                                .expect("38ca88dc-ab40-4a76-8bcd-223df66a1f81"),
                                                            );
                                                            if !acc_480d72e5.contains(&value_0ae29f5f) {
                                                                acc_480d72e5.push(value_0ae29f5f);
                                                            }
                                                        }
                                                        let value_4e4cfda3 = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                            #import_path::PostgresqlTypeWhere::new(
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
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_480d72e5 = Vec::new();
                                                #(#content_token_stream)*
                                                acc_480d72e5.push(#ident_where_upper_camel_case::LengthEqual(
                                                    #import_path::PostgresqlJsonTypeWhereLengthEqual {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #value_snake_case: #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(#create_snake_case.0.len()).expect("1811faf7-c0a5-4e05-b866-546affd441df")
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
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_array_not_null_upper_camel_case)
                                },
                            }
                        };
                        let create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = {
                            let generate_nullable_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote! {
                                #create_snake_case.0.map_or_else(|| None, |create_612f2a61| <
                                    #content_token_stream
                                    as
                                    #import_path::PostgresqlJsonTypeTestCases
                                >::create_into_postgresql_json_type_option_vec_where_length_greater_than(create_612f2a61).map_or_else(
                                    || None,
                                    |value_1ea95b5d| match #import_path::NotEmptyUniqueVec::try_new({
                                        let mut acc_87f84b5c = Vec::new();
                                        for element_9bbf8527 in value_1ea95b5d.clone().into_vec() {
                                            match #import_path::NotEmptyUniqueVec::try_new(vec![element_9bbf8527]) {
                                                Ok(value_1d0202fc) => {
                                                    acc_87f84b5c.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_1d0202fc)));
                                                }
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => (),
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("bdb0a112-6f75-481c-ad28-f540252d8525"),
                                                },
                                            }
                                        }
                                        let value_4e4cfda3 = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_1ea95b5d));
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
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_9d0245f1| {
                                            let field_ident = &element_9d0245f1.field_ident;
                                            let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_9d0245f1.field_type);
                                            quote::quote! {
                                                if let Some(value_3432b965) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
                                                    #create_snake_case.#field_ident
                                                ) {
                                                    for element_9bbf8527 in value_3432b965.clone().into_vec() {
                                                        acc_f5866fb6.push(
                                                            #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                                #import_path::PostgresqlTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![element_9bbf8527]
                                                                ).expect("479db858-6f36-48ba-9ab0-741b7df7956c")
                                                            )
                                                        );
                                                    }
                                                    let element_4a00ab02 = #ident_where_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::new(
                                                            #import_path::LogicalOperator::And,
                                                            value_3432b965
                                                        )
                                                    );
                                                    if !acc_f5866fb6.contains(&element_4a00ab02) {
                                                        acc_f5866fb6.push(element_4a00ab02);
                                                    }
                                                }
                                            }
                                        });
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_f5866fb6 = Vec::new();
                                                #(#content_token_stream)*
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
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_standart_not_null_upper_camel_case)
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_47c8f26c| {
                                            let field_ident = &element_47c8f26c.field_ident;
                                            let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element_47c8f26c.field_type);
                                            quote::quote! {
                                                for create_34a1e540 in #create_snake_case.0.clone() {
                                                    if let Some(value_51fe384b) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
                                                        create_34a1e540.#field_ident
                                                    ) {
                                                        for element_4a00ab02 in value_51fe384b.clone().into_vec() {
                                                            let element_938f8b34 = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                                #import_path::PostgresqlTypeWhere::try_new(
                                                                    #import_path::LogicalOperator::And,
                                                                    vec![element_4a00ab02]
                                                                )
                                                                .expect("955c6c27-863d-4b9b-9d88-e71f11161b3e"),
                                                            );
                                                            if !acc_acceb7eb.contains(&element_938f8b34) {
                                                                acc_acceb7eb.push(element_938f8b34);
                                                            }
                                                        }
                                                        let element_e17d9fba = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                            #import_path::PostgresqlTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
                                                                value_51fe384b
                                                            )
                                                        );
                                                        if !acc_acceb7eb.contains(&element_e17d9fba) {
                                                            acc_acceb7eb.push(element_e17d9fba);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_acceb7eb = Vec::new();
                                                #(#content_token_stream)*
                                                acc_acceb7eb.push(#ident_where_upper_camel_case::LengthGreaterThan(
                                                    #import_path::PostgresqlJsonTypeWhereLengthGreaterThan {
                                                        logical_operator: #import_path::LogicalOperator::And,
                                                        #value_snake_case: #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(
                                                                #create_snake_case.0.len().checked_sub(1).unwrap_or_else(|| {
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
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_array_not_null_upper_camel_case)
                                },
                            }
                        };
                        let (
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream
                        ) = {
                            let generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream = |method_name_token_stream: &dyn quote::ToTokens|match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let content_token_stream = vec_syn_field.iter().map(|element_59346ba9| {
                                            let field_ident = &element_59346ba9.field_ident;
                                            let field_type = &element_59346ba9.field_type;
                                            let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            quote::quote! {
                                                if let Some(value_a2900ac9) = #field_type_as_postgresql_json_type_test_cases_token_stream::#method_name_token_stream(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #create_snake_case.#field_ident
                                                ) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    for element_3e86d33d in value_a2900ac9.clone().into_vec() {
                                                        match element_3e86d33d {
                                                            #import_path::SingleOrMultiple::Single(single) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_upper_camel_case::#field_ident_upper_camel_case(#import_path::PostgresqlTypeWhere::try_new(
                                                                            and,
                                                                            vec![single]
                                                                        ).expect("2635ede5-e733-4793-a2b5-110dda258c90"))
                                                                    )
                                                                );
                                                            },
                                                            #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                                acc_a94bd7fb.push(
                                                                    #import_path::SingleOrMultiple::Single(
                                                                        #ident_where_upper_camel_case::#field_ident_upper_camel_case(#import_path::PostgresqlTypeWhere::new(
                                                                            and,
                                                                            multiple
                                                                        ))
                                                                    )
                                                                );
                                                            }
                                                        }
                                                    }
                                                    let value_3e75a2f2 = #import_path::SingleOrMultiple::Single(
                                                        #ident_where_upper_camel_case::#field_ident_upper_camel_case(#import_path::PostgresqlTypeWhere::try_new(
                                                            and,
                                                            value_a2900ac9.into_vec().into_iter().flat_map(|element_9efefcdc| match element_9efefcdc {
                                                                #import_path::SingleOrMultiple::Single(single) => {
                                                                    std::iter::once(single).collect()
                                                                }
                                                                #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                            })
                                                            .fold(Vec::new(), |mut acc_be2a6606, element_7ae146ee| {
                                                                if !acc_be2a6606.contains(&element_7ae146ee) {
                                                                    acc_be2a6606.push(element_7ae146ee);
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
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_a94bd7fb = Vec::new();
                                                #(#content_token_stream)*
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
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let initialization_token_stream = vec_syn_field.iter().map(|element_3fde3bb4| {
                                            let field_ident = &element_3fde3bb4.field_ident;
                                            let field_type = &element_3fde3bb4.field_type;
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            quote::quote! {
                                                let #field_ident = #field_type_as_postgresql_json_type_test_cases_token_stream::#method_name_token_stream(
                                                    read_only_ids_629675e2.0.#value_snake_case.#field_ident,
                                                    create_82796400.#field_ident
                                                );
                                            }
                                        });
                                        let if_some_content_token_stream = {
                                            let (last, rest) = vec_syn_field.split_last().expect("a8e7b6d6-d46c-4d15-880d-c5c14723966c");
                                            let generate_field_ident_is_some_token_stream = |field_ident: &syn::Ident|quote::quote!{#field_ident.is_some()};
                                            let rest_token_stream = rest.iter().map(|element_cd54f3c6| {
                                                let field_ident_is_some_token_stream = generate_field_ident_is_some_token_stream(&element_cd54f3c6.field_ident);
                                                quote::quote!{#field_ident_is_some_token_stream || }
                                            });
                                            let last_token_stream = generate_field_ident_is_some_token_stream(&last.field_ident);
                                            quote::quote! {#(#rest_token_stream)* #last_token_stream}
                                        };
                                        let content_token_stream = vec_syn_field.iter().map(|element_dbdd7930| {
                                            let field_ident = &element_dbdd7930.field_ident;
                                            let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                            quote::quote! {
                                                if let Some(value_f190793e) = #field_ident {
                                                    for element_22ac4087 in value_f190793e.clone().into_vec() {
                                                        let current_where = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                            match element_22ac4087 {
                                                                #import_path::SingleOrMultiple::Single(single) => #import_path::PostgresqlTypeWhere::try_new(
                                                                    and,
                                                                    vec![single]
                                                                ).expect("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd"),
                                                                #import_path::SingleOrMultiple::Multiple(multiple) => #import_path::PostgresqlTypeWhere::new(
                                                                    and,
                                                                    multiple.clone()
                                                                )
                                                            }
                                                        );
                                                        all_fields_acc.push(current_where.clone());
                                                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                                                            #id_snake_case.clone(),
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
                                                        value_f190793e.into_vec().into_iter().flat_map(|element_6df4f0be| match element_6df4f0be {
                                                            #import_path::SingleOrMultiple::Single(single) => {
                                                                std::iter::once(single).collect()
                                                            }
                                                            #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                        })
                                                        .fold(Vec::new(), |mut acc_01265629, element_9a7c960d| {
                                                            if !acc_01265629.contains(&element_9a7c960d) {
                                                                acc_01265629.push(element_9a7c960d);
                                                            }
                                                            acc_01265629
                                                        })
                                                    ) {
                                                        Ok(value_a4000d70) => {
                                                            let value_d6218307 = #ident_where_upper_camel_case::#element_field_ident_upper_camel_case(
                                                                #import_path::PostgresqlTypeWhere::new(
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
                                        quote::quote! {
                                            match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_359c0b3f = Vec::new();
                                                for (read_only_ids_629675e2, create_82796400) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()) {
                                                    let and = #import_path::LogicalOperator::And;
                                                    let #id_snake_case = #ident_where_upper_camel_case::ElementId(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            and,
                                                            vec![
                                                                #uuid_uuid_as_not_null_jsonb_string_where_upper_camel_case::Equal(#import_path::PostgresqlJsonTypeWhereEqual {
                                                                    logical_operator: #import_path::LogicalOperator::Or,
                                                                    #value_snake_case: #uuid_uuid_as_not_null_jsonb_string_table_type_declaration_upper_camel_case::new(
                                                                        read_only_ids_629675e2.0.#value_snake_case.#id_snake_case.0.#value_snake_case
                                                                    ),
                                                                })
                                                            ],
                                                        )
                                                        .expect("31db8e1e-28cd-44f7-9f32-a41cc6675660"), 
                                                    );
                                                    #(#initialization_token_stream)*
                                                    if #if_some_content_token_stream {
                                                        let mut all_fields_acc = vec![];
                                                        #(#content_token_stream)*
                                                        match #import_path::NotEmptyUniqueVec::try_new({
                                                            all_fields_acc.push(#id_snake_case);
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
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let current_ident_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&generate_ident_upper_camel_case(&match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        PostgresqlJsonObjectTypePattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote::quote! {
                                        match (#read_only_ids_snake_case.0.value, #create_snake_case.0) {
                                            (Some(read_only_ids_3e2e30c8), Some(create_79039a2f)) => #current_ident_token_stream::#method_name_token_stream(
                                                read_only_ids_3e2e30c8,
                                                create_79039a2f
                                            ).map_or_else(|| None, |value_35662b3a| match #import_path::NotEmptyUniqueVec::try_new({
                                                let mut acc_e0d72451 = vec![];
                                                for element_4632f100 in value_35662b3a.into_vec() {
                                                    match element_4632f100 {
                                                        #import_path::SingleOrMultiple::Single(single) => match #import_path::NotEmptyUniqueVec::try_new(vec![single]) {
                                                            Ok(value_4ce6ecd3) => {
                                                                acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_4ce6ecd3))));
                                                            }
                                                            Err(error) => match error {
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => (),
                                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("626ffa77-f81a-46ce-b5a0-44663fe1f182"),
                                                            },
                                                        },
                                                        #import_path::SingleOrMultiple::Multiple(multiple) => {
                                                            acc_e0d72451.push(#import_path::SingleOrMultiple::Single(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(multiple))));
                                                        }
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
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case
                                ),
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_snake_case
                                ),
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_snake_case
                                ),
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_snake_case
                                ),
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_snake_case
                                ),
                                generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_token_stream(
                                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_snake_case
                                )
                            )
                        };
                        postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_upper_camel_case,
                            &ident,
                            &option_vec_create_token_stream,
                            &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
                            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                            &read_only_ids_into_option_value_read_inner_token_stream,
                            &update_to_read_only_ids_token_stream,
                            &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            &previous_read_merged_with_option_update_into_read_token_stream,
                            &read_only_ids_merged_with_create_into_read_token_stream,
                            &read_only_ids_merged_with_create_into_option_value_read_token_stream,
                            &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
                            &read_only_ids_merged_with_create_into_where_equal_token_stream,
                            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
                            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
                            &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
                            &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream,
                        )
                    },
                    {
                        let option_vec_create_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case()};
                        let read_only_ids_to_two_dimensional_vec_read_inner_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(#read_only_ids_snake_case)};
                        let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case)};
                        let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case)};
                        let update_to_read_only_ids_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(#value_snake_case)};
                        let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case(#value_snake_case)};
                        let previous_read_merged_with_option_update_into_read_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#previous_read_merged_with_option_update_into_read_snake_case(#read_snake_case, #option_update_snake_case)};
                        let read_only_ids_merged_with_create_into_read_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_read_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_option_value_read_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_table_type_declaration_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_where_equal_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_where_equal_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = quote::quote! {#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream = quote::quote!{Some(#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        ))};
                        let create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream = &none_token_stream;
                        let postgresql_type_option_vec_where_greater_than_test_token_stream = &none_token_stream;
                        let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream = &none_token_stream;

                        let (
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                            read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream
                        ) = {
                            let generate_dimension_equal_handle_token_stream = |dimension: &postgresql_crud_macros_common::Dimension|{
                                let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case();
                                quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                    #read_only_ids_snake_case,
                                    #create_snake_case
                                )}
                            };
                            (
                                generate_dimension_equal_handle_token_stream(&postgresql_crud_macros_common::Dimension::One),
                                generate_dimension_equal_handle_token_stream(&postgresql_crud_macros_common::Dimension::Two),
                                generate_dimension_equal_handle_token_stream(&postgresql_crud_macros_common::Dimension::Three),
                                generate_dimension_equal_handle_token_stream(&postgresql_crud_macros_common::Dimension::Four)
                            )
                        };
                        let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
                            #create_snake_case
                        )};
                        let create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream = quote::quote!{#self_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )};
                        postgresql_crud_macros_common::generate_impl_postgresql_type_test_cases_for_ident_token_stream(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_upper_camel_case,
                            &ident,
                            &option_vec_create_token_stream,
                            &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
                            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                            &update_to_read_only_ids_token_stream,
                            &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            &previous_read_merged_with_option_update_into_read_token_stream,
                            &read_only_ids_merged_with_create_into_read_token_stream,
                            &read_only_ids_merged_with_create_into_option_value_read_token_stream,
                            &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
                            &read_only_ids_merged_with_create_into_where_equal_token_stream,
                            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
                            &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream,
                            &create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream,
                            &postgresql_type_option_vec_where_greater_than_test_token_stream,
                            &read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
                            &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
                            &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream,
                            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream,
                        )
                    },
                )
            };
            let impl_postgresql_type_not_primary_key_for_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_type_not_primary_key_for_ident_token_stream(&import_path, &ident);
            let generated = quote::quote! {
                #ident_token_stream
                #ident_table_type_declaration_token_stream
                #ident_create_token_stream
                #ident_create_for_query_token_stream
                #ident_select_token_stream
                #ident_where_token_stream
                #ident_read_token_stream
                #ident_read_only_ids_token_stream
                #ident_read_inner_token_stream
                #ident_update_token_stream
                #ident_update_for_query_token_stream
                #impl_postgresql_crud_postgresql_json_type_for_ident_token_stream
                #maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
                #impl_postgresql_json_type_test_cases_for_ident_token_stream
                #impl_postgresql_type_test_cases_for_ident_token_stream
                #impl_postgresql_type_not_primary_key_for_ident_token_stream
            };
            (
                {
                    let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().expect("7f9a06a5-db0f-420d-ae83-581ccc02c99f");
                    quote::quote! {
                        pub #field_ident: #ident,
                    }
                },
                generated,
            )
        })
        .collect::<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)>();
    macros_helpers::maybe_write_token_stream_into_file(
        generate_postgresql_json_object_type_config
            .postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_object_types,
        "postgresql_table_columns_using_postgresql_json_object_types",
        &quote::quote! {
            pub struct PostgresqlTableColumnsContentWriteIntoPostgresqlTableColumnsUsingPostgresqlJsonObjectTypes {
                #(#fields_token_stream)*
            }
        },
        &macros_helpers::FormatWithCargofmt::True,
    );
    let generated: proc_macro2::TokenStream =
        quote::quote! {#(#postgresql_json_object_type_array)*};
    macros_helpers::maybe_write_token_stream_into_file(
        generate_postgresql_json_object_type_config
            .whole_content_write_into_generate_postgresql_json_object_type,
        "generate_postgresql_json_object_type",
        &generated,
        &macros_helpers::FormatWithCargofmt::True,
    );
    generated
}
