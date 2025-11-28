//todo generate authorization rights enum for json fields
//todo bug in update if updating array and creating element in jsonb array without anything - read_only_ids generation logic of vec returns wrong query part
#[proc_macro_attribute]
pub fn postgresql_json_object_type_pattern(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_derive(GeneratePostgresqlJsonObjectType)]
pub fn generate_postgresql_json_object_type(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    enum TraitGen {
        PostgresqlJsonType,
        PostgresqlTypeAndPostgresqlJsonType,
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
    impl PostgresqlJsonObjectTypeRecord {
        fn all() -> std::vec::Vec<Self> {
            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().fold(vec![], |mut acc, not_null_or_nullable| {
                for postgresql_json_object_type_pattern in PostgresqlJsonObjectTypePattern::into_array() {
                    acc.push(Self {
                        not_null_or_nullable,
                        postgresql_json_object_type_pattern,
                        trait_gen: TraitGen::PostgresqlTypeAndPostgresqlJsonType,
                    });
                }
                acc
            })
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlJsonObjectTypeConfig {
        All,
        Concrete(PostgresqlJsonObjectTypeRecord),
    }
    let syn_derive_input: syn::DeriveInput = syn::parse(input_token_stream.clone()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrud;
    let postgresql_json_object_type_record_vec = {
        let generate_postgresql_json_object_type_config = serde_json::from_str::<GeneratePostgresqlJsonObjectTypeConfig>(
            &macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
                &{
                    let syn_derive_input: syn::DeriveInput = syn::parse(input_token_stream).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
                    syn_derive_input.attrs
                },
                &format!("{}::postgresql_json_object_type_pattern", import_path.snake_case_std_primitive_str()),
            )
            .to_string()
        ).expect("failed to get Config for generate_postgresql_json_object_type");
        match generate_postgresql_json_object_type_config {
            GeneratePostgresqlJsonObjectTypeConfig::All => PostgresqlJsonObjectTypeRecord::all(),
            GeneratePostgresqlJsonObjectTypeConfig::Concrete(postgresql_json_object_type_record) => match (&postgresql_json_object_type_record.not_null_or_nullable, &postgresql_json_object_type_record.postgresql_json_object_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart) => vec![postgresql_json_object_type_record],
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Standart) => vec![
                    PostgresqlJsonObjectTypeRecord {
                        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                        postgresql_json_object_type_pattern: PostgresqlJsonObjectTypePattern::Standart,
                        trait_gen: postgresql_json_object_type_record.trait_gen.clone(),
                    },
                    postgresql_json_object_type_record
                ],
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
            },
        }
    }
    // .into_iter()
    // .filter(|element| {
    //     use postgresql_crud_macros_common::NotNullOrNullable;
    //     let not_null_or_nullable_filter = match &element.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => true,
    //         NotNullOrNullable::Nullable => true,
    //     };
    //     let postgresql_json_object_type_pattern_filter = match &element.postgresql_json_object_type_pattern {
    //         PostgresqlJsonObjectTypePattern::Standart => match &element.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //         PostgresqlJsonObjectTypePattern::Array => match &element.not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //     };
    //     let trait_gen_filter = match &element.trait_gen {
    //         TraitGen::PostgresqlJsonType => true,
    //         TraitGen::PostgresqlTypeAndPostgresqlJsonType => true,
    //     };
    //     not_null_or_nullable_filter && postgresql_json_object_type_pattern_filter && trait_gen_filter
    // })
    // .collect::<std::vec::Vec<PostgresqlJsonObjectTypeRecord>>()
    ;
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonObjectTypeJsonVariants",
    //     &serde_json::to_string(&postgresql_json_object_type_record_vec).unwrap(),
    // );

    // element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
    //     let element_snake_case_stringified = naming_common::AsRefStrToSnakeCaseStringified::case(element);
    //     if index == 0 {
    //         acc.push_str(&element_snake_case_stringified);
    //     } else {
    //         acc.push_str(&format!("_{element_snake_case_stringified}"));
    //     }
    //     acc
    // });
    // let postgresql_json_object_type_array
    let (fields_token_stream, postgresql_json_object_type_array) = postgresql_json_object_type_record_vec
        .into_iter()
        .enumerate()
        .map(|(index, element)| {
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_json_object_type_pattern = &element.postgresql_json_object_type_pattern;
            let trait_gen = &element.trait_gen;

            let create_snake_case = naming::CreateSnakeCase;
            let update_snake_case = naming::UpdateSnakeCase;
            let delete_snake_case = naming::DeleteSnakeCase;
            let value_upper_camel_case = naming::ValueUpperCamelCase;
            let value_snake_case = naming::ValueSnakeCase;
            let element_snake_case = naming::ElementSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let select_query_part_postgresql_type_snake_case = naming::SelectQueryPartPostgresqlTypeSnakeCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let id_snake_case = naming::IdSnakeCase;
            let acc_snake_case = naming::AccSnakeCase;
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
            let read_only_ids_merged_with_create_into_where_element_equal_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoWhereElementEqualSnakeCase;
            let read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereElementEqualUsingFieldsSnakeCase;
            let read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereElementEqualToJsonFieldSnakeCase;
            let read_only_ids_merged_with_create_into_table_type_declaration_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSnakeCase;
            let create_into_postgresql_json_type_option_vec_where_element_length_more_than_snake_case = naming::CreateIntoPostgresqlJsonTypeOptionVecWhereElementLengthMoreThanSnakeCase;
            let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementUpperCamelCase;
            let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;

            let std_string_string_token_stream = token_patterns::StdStringString;
            let self_field_vec_token_stream = quote::quote! {.0.to_vec()};
            let cfg_feature_test_utils = quote::quote! {#[cfg(feature = "test-utils")]};

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
            let import_path_default_but_option_is_always_some_call_token_stream = quote::quote!{
                #import_path::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
            };
            let generate_default_but_option_is_always_some_call_token_stream = |ident_token_stream: &dyn quote::ToTokens, |{
                quote::quote!{
                    <#ident_token_stream as #import_path::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case>::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                }
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
                    fields_named.named.iter().collect::<std::vec::Vec<&syn::Field>>()
                } else {
                    panic!("supports only syn::Fields::Named");
                }
            } else {
                panic!("does work only on structs!");
            };
            #[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
            enum IsStandartWithId {
                False,
                True,
            }
            let is_standart_with_id_false = IsStandartWithId::False;
            let is_standart_with_id_true = IsStandartWithId::True;
            enum IdentPattern {
                StandartNotNullWithoutId,
                StandartNotNullWithId,
                StandartNullableWithoutId,
                ArrayNotNullWithId,
                ArrayNullableWithId,
            }
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
                    IdentPattern::ArrayNullableWithId => (vec_of_syn_derive_input_ident_with_id, array_of_not_null_jsonb_object_with_id, postgresql_crud_macros_common::NotNullOrNullable::Nullable),
                };
                let current_not_null_or_nullable_rust = current_not_null_or_nullable.rust();
                format!("{current_not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().unwrap()
            };

            let ident = &generate_ident_upper_camel_case(&match (&not_null_or_nullable, &postgresql_json_object_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart) => IdentPattern::StandartNotNullWithoutId,
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Array) => IdentPattern::ArrayNotNullWithId,
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Standart) => IdentPattern::StandartNullableWithoutId,
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonObjectTypePattern::Array) => IdentPattern::ArrayNullableWithId,
            });
            let ident_standart_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::StandartNotNullWithoutId);
            let ident_standart_nullable_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::StandartNullableWithoutId);
            let ident_array_not_null_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId);
            let ident_array_nullable_upper_camel_case = &generate_ident_upper_camel_case(&IdentPattern::ArrayNullableWithId);
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
            let uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case = {
                let uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_display(&uuid_uuid_as_not_null_jsonb_string_upper_camel_case);
                quote::quote!{#import_path::#uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case}
            };
            let uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream = quote::quote!{
                #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream::Update
            };
            let uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream = quote::quote!{
                <#uuid_uuid_as_not_null_jsonb_string_token_stream as #import_path::PostgresqlJsonTypeObjectVecElementId>
            };
            let id_syn_field = syn::Field {
                attrs: vec![],
                vis: syn::Visibility::Public(syn::token::Pub { span: proc_macro2::Span::call_site() }),
                mutability: syn::FieldMutability::None,
                ident: Some(syn::Ident::new(&id_snake_case.to_string(), proc_macro2::Span::call_site())),
                colon_token: Some(syn::token::Colon { spans: [proc_macro2::Span::call_site()] }),
                ty: syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &[import_path.to_path(), &uuid_uuid_as_not_null_jsonb_string_upper_camel_case.to_string()]
                        ),
                    },
                }),
            };
            let vec_syn_field_with_id = vec_syn_field.iter().fold(vec![&id_syn_field], |mut acc, element| {
                acc.push(element);
                acc
            });
            let get_vec_syn_field = |is_standart_with_id: &IsStandartWithId| -> &std::vec::Vec<&syn::Field> {
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
            let ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident);
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
            let ident_with_id_standart_not_null_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_token_stream = {
                let generate_struct_ident_token_stream = |ident: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug)]
                        pub struct #ident;
                    }
                };
                let ident_token_stream = generate_struct_ident_token_stream(&ident);
                let maybe_ident_with_id_standart_not_null_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_token_stream = generate_struct_ident_token_stream(&ident_with_id_standart_not_null_upper_camel_case);
                    let cfg_feature_test_utils_impl_ident_with_id_standart_not_null_token_stream = {
                        let read_only_ids_merged_with_create_into_where_element_equal_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_where_element_equal_token_stream(
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
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
                                let ident_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    generate_token_stream(
                                        &field_ident,
                                        &element.ty,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    #ident_with_id_standart_not_null_where_element_upper_camel_case::#equal_upper_camel_case(postgresql_crud::PostgresqlJsonTypeWhereElementEqual {
                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                        #value_snake_case: #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                            #ident_token_stream,
                                            #(#content_token_stream),*
                                        ),
                                    })
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_token_stream(
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
                            &{
                                let generate_token_stream = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_token_stream: &dyn quote::ToTokens,
                                |{
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                    quote::quote!{
                                        #ident_with_id_standart_not_null_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                postgresql_crud::LogicalOperator::And,
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #second_argument_token_stream
                                                ),
                                            )
                                            .expect("error f8c3796f-7574-468b-ac60-12a620c0917d"),
                                        )
                                    }
                                };
                                let id_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    generate_token_stream(
                                        &field_ident,
                                        &element.ty,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    vec![
                                        #id_token_stream,
                                        #(#content_token_stream),*
                                    ]
                                }
                            },
                        );
                        let read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_token_stream = postgresql_crud_macros_common::generate_read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_token_stream(
                            &ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                            &ident_with_id_standart_not_null_create_upper_camel_case,
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
                            &{
                                let generate_token_stream = |
                                    field_ident: &dyn quote::ToTokens,
                                    field_type: &dyn quote::ToTokens,
                                    second_argument_token_stream: &dyn quote::ToTokens,
                                |{
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                    quote::quote!{
                                        #acc_snake_case.push(#ident_with_id_standart_not_null_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                            #import_path::PostgresqlTypeWhere::try_new(
                                                #import_path::LogicalOperator::Or,
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #second_argument_token_stream
                                                ),
                                            )
                                            .expect("error 187ece1f-7c99-437b-80a3-ed1a416731a3"),
                                        ));
                                    }
                                };
                                let id_token_stream = generate_token_stream(
                                    &id_snake_case,
                                    &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                    &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                );
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    generate_token_stream(
                                        &field_ident,
                                        &element.ty,
                                        &quote::quote!{#create_snake_case.#field_ident}
                                    )
                                });
                                quote::quote!{
                                    let mut #acc_snake_case = vec![];
                                    #id_token_stream
                                    #(#content_token_stream)*
                                    #acc_snake_case
                                }
                            },
                        );
                        quote::quote! {
                            #[cfg(feature = "test-utils")]
                            impl #ident_with_id_standart_not_null_upper_camel_case {
                                #read_only_ids_merged_with_create_into_where_element_equal_token_stream
                                #read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_token_stream
                                #read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_token_stream
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
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresqlJsonTypeSubtype {
                TableTypeDeclaration,
                Create,
                CreateForQuery,
                Select,
                WhereElement,
                Read,
                ReadOnlyIds,
                ReadInner,
                Update,
                UpdateForQuery,
            }
            impl quote::ToTokens for PostgresqlJsonTypeSubtype {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    self.to_string().parse::<proc_macro2::TokenStream>().unwrap().to_tokens(tokens);
                }
            }
            #[derive(Debug, Clone, strum_macros::Display)]
            enum PostgresqlTypeSubtype {
                // TableTypeDeclaration,
                // Create,
                // Select,
                // WhereElement,
                Read,
                // ReadOnlyIds,
                // ReadInner,
                Update,
            }
            impl quote::ToTokens for PostgresqlTypeSubtype {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    self.to_string().parse::<proc_macro2::TokenStream>().unwrap().to_tokens(tokens);
                }
            }
            let ident_array_not_null_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_array_not_null_upper_camel_case);
            let ident_with_id_array_not_null_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident_with_id_array_not_null_upper_camel_case);
            let postgresql_json_type_subtype_table_type_declaration = PostgresqlJsonTypeSubtype::TableTypeDeclaration;
            let postgresql_json_type_subtype_create = PostgresqlJsonTypeSubtype::Create;
            let postgresql_json_type_subtype_create_for_query = PostgresqlJsonTypeSubtype::CreateForQuery;
            let postgresql_json_type_subtype_select = PostgresqlJsonTypeSubtype::Select;
            let postgresql_json_type_subtype_where_element = PostgresqlJsonTypeSubtype::WhereElement;
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
            let generate_field_type_as_crud_postgresql_json_type_from_field_token_stream = |field: &syn::Field| generate_type_as_postgresql_json_type_token_stream(&field.ty);
            let generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens| macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_token_stream, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:?}")});
            let ident_as_postgresql_json_type_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&ident, &postgresql_json_type_subtype_table_type_declaration);
            enum PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate {
                TableTypeDeclaration,
                Create,
            }
            impl std::convert::From<&PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate> for PostgresqlJsonTypeSubtype {
                fn from(value: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate) -> Self {
                    match &value {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => Self::TableTypeDeclaration,
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => Self::Create,
                    }
                }
            }
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
                let generate_ident_table_type_declaration_or_create_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_token_stream #content_token_stream
                    }
                };
                enum StructDeclarationOrNewType {
                    StructDeclaration,
                    NewType,
                }
                let struct_declaration_or_new_type_struct_declaration = StructDeclarationOrNewType::StructDeclaration;
                let struct_declaration_or_new_type_new_type = StructDeclarationOrNewType::NewType;
                let generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId, postgresql_json_type_subtype_table_type_declaration_or_create: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate, struct_declaration_or_new_type: &StructDeclarationOrNewType| {
                    let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let type_as_postgresql_json_type_subtype_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&element.ty, &PostgresqlJsonTypeSubtype::from(postgresql_json_type_subtype_table_type_declaration_or_create));
                        quote::quote! {
                            #field_ident: #type_as_postgresql_json_type_subtype_table_type_declaration_token_stream
                        }
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
                    let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                        element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        })
                    });
                    quote::quote! {Self {#(#content_token_stream),*}}
                };
                let impl_new_for_ident_table_type_declaration_or_ident_create_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &{
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
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(&is_standart_with_id_false),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_value_token_stream.clone(),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_array_not_null_with_id_postfix_upper_camel_case = generate_tokens_table_type_declaration_or_create_token_stream(&generate_ident_upper_camel_case(&IdentPattern::ArrayNotNullWithId));
                                quote::quote! {Self(
                                    match #value_snake_case {
                                        Some(#value_snake_case) => Some(#ident_array_not_null_with_id_postfix_upper_camel_case::new(#value_snake_case)),
                                        None => None
                                    }

                                )}
                            }
                        },
                    },
                );
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream = |
                    ident_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens
                | postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {Self #content_token_stream}
                );
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        quote::quote! {
                            #field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        }
                    });
                    quote::quote! {{
                        #(#content_token_stream),*
                    }}
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream(&is_standart_with_id_false);
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_ident_create_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream.clone(),
                            PostgresqlJsonObjectTypePattern::Array => quote::quote! {(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                        },
                    },
                );
                let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &quote::quote!{sqlx::types::Json(#self_snake_case)}
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_or_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_table_type_declaration_or_ident_create_upper_camel_case,
                    &quote::quote!{sqlx::types::Json<#ident_table_type_declaration_or_ident_create_upper_camel_case>}
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
                        &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(&current_is_standart_with_id, &postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_struct_declaration),
                    );
                    let impl_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_with_id_table_type_declaration_or_ident_with_id_standart_not_null_create_upper_camel_case,
                        &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(&current_is_standart_with_id, &postgresql_json_type_subtype_table_type_declaration_or_create, &struct_declaration_or_new_type_new_type),
                        &generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(&current_is_standart_with_id),
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
                        #impl_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_table_type_declaration_or_ident_create_token_stream
                    #impl_new_for_ident_table_type_declaration_or_ident_create_token_stream
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
                let generate_impl_std_fmt_display_for_ident_create_token_stream = |ident_token_stream: &dyn quote::ToTokens| macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_token_stream, &proc_macro2::TokenStream::new(), &quote::quote! {write!(formatter, "{:?}", self)});
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
            let ident_standart_not_null_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_create_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&ident_standart_not_null_upper_camel_case);
            let ident_array_not_null_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&ident_array_not_null_upper_camel_case);
            let ident_array_not_null_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_array_not_null_upper_camel_case);
            let postgresql_crud_path_postgresql_json_type_uuid_uuid_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
            let generate_debug_clone_partialeq_serialize_pub_struct_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                    pub struct #ident_token_stream #content_token_stream
                }
            };
            let ident_create_for_query_token_stream = {
                let generate_struct_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
                    let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let type_as_postgresql_json_type_subtype_crate_for_query_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::CreateForQuery
                        );
                        quote::quote! {
                            #field_ident: #type_as_postgresql_json_type_subtype_crate_for_query_token_stream
                        }
                    });
                    quote::quote! {{#(#content_token_stream),*}}
                };
                let impl_std_convert_from_standart_not_null_without_id_content_token_stream = {
                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let type_as_postgresql_json_type_subtype_crate_for_query_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::CreateForQuery
                        );
                        quote::quote! {
                            #field_ident: #type_as_postgresql_json_type_subtype_crate_for_query_token_stream::from(#value_snake_case.#field_ident)
                        }
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
                    let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
                        &ident_create_for_query_upper_camel_case,
                        &quote::quote!{sqlx::types::Json(#self_snake_case)}
                    );
                    let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                        &ident_create_for_query_upper_camel_case,
                        &quote::quote!{sqlx::types::Json<#ident_create_for_query_upper_camel_case>}
                    );
                    let impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                        &ident_create_upper_camel_case,
                        &ident_create_for_query_upper_camel_case,
                        &{
                            let content_token_stream = match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {{#impl_std_convert_from_standart_not_null_without_id_content_token_stream}},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                        quote::quote!{(
                                            match #value_snake_case.0 {
                                                Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_json_type_create_for_query_token_stream::from(#value_snake_case)),
                                                None => None
                                            }
                                        )}
                                    },
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{(
                                        #value_snake_case.0.into_iter().map(|#element_snake_case|#ident_with_id_standart_not_null_create_for_query_upper_camel_case::from(
                                            #element_snake_case
                                        )).collect()
                                    )},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{(
                                        match #value_snake_case.0 {
                                            Some(#value_snake_case) => Some(#ident_array_not_null_as_postgresql_json_type_create_for_query_token_stream::from(#value_snake_case)),
                                            None => None
                                        }
                                    )},
                                },
                            };
                            quote::quote! {Self #content_token_stream}
                        }
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
                    let impl_std_convert_from_ident_with_id_standart_not_null_create_for_ident_with_id_standart_not_null_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
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
            let generate_sqlx_types_json_type_declaration_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_token_stream, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&self_upper_camel_case));
            let generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_token_stream, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&self_upper_camel_case), &quote::quote! {Ok(value.0)});
            let generate_value_type_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#value_snake_case: #type_token_stream}
            };
            let generate_pub_new_value_type_content_self_value_token_stream = |content_token_stream: &dyn quote::ToTokens|macros_helpers::generate_pub_new_token_stream(
                &generate_value_type_token_stream(&content_token_stream),
                &self_value_token_stream
            );
            let generate_unique_vec_wrapper_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#import_path::NotEmptyUniqueEnumVec<#type_token_stream>}
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
            let ident_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&ident);
            let ident_as_postgresql_json_type_update_token_stream = generate_type_as_postgresql_json_type_update_token_stream(&ident);
            let ident_as_postgresql_json_type_create_for_query_token_stream = generate_type_as_postgresql_json_type_create_for_query_token_stream(&ident);
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
                is_standart_with_id: &IsStandartWithId,
                in_token_stream: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_field_ident_token_stream: &dyn quote::ToTokens,
                column_name_and_maybe_field_getter_for_error_message_field_ident_token_stream: &dyn quote::ToTokens,
            |{
                let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    let variant_name_token_stream: &dyn quote::ToTokens = &naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                    let field_ident_double_quotes_token_stream: &dyn quote::ToTokens = &generate_quotes::double_quotes_token_stream(&field_ident_stringified);
                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                    let ident_or_ident_with_id_standart_not_null_select_element_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::True => &ident_with_id_standart_not_null_select_element_upper_camel_case,
                        IsStandartWithId::False => &ident_standart_not_null_select_element_upper_camel_case
                    };
                    quote::quote! {
                        #ident_or_ident_with_id_standart_not_null_select_element_upper_camel_case::#variant_name_token_stream(#value_snake_case) => #field_type_as_crud_postgresql_json_type_from_field_token_stream::#select_query_part_snake_case(
                            &#value_snake_case,
                            #field_ident_double_quotes_token_stream,
                            &#column_name_and_maybe_field_getter_field_ident_token_stream,
                            &#column_name_and_maybe_field_getter_for_error_message_field_ident_token_stream,
                            false,
                        )
                    }
                });
                quote::quote!{
                    for #element_snake_case in #in_token_stream #self_field_vec_token_stream {
                        #acc_snake_case.push_str(&format!(
                            "{}||",
                            match #element_snake_case {
                                #(#content_token_stream),*
                            }
                        ));
                    }
                }
            };
            let ident_select_token_stream = {
                let generate_pub_struct_ident_select_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            serde::Serialize,
                            serde::Deserialize,
                            utoipa::ToSchema,
                            schemars::JsonSchema,
                        )]
                        pub struct #ident_token_stream #content_token_stream
                    }
                };
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
                    let pub_new_token_stream = macros_helpers::generate_pub_new_token_stream(
                        &{
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
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                    Self(match #value_snake_case {
                                        Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_json_type_select_token_stream::new(#value_snake_case)),
                                        None => None
                                    })
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
                        }
                    );
                    let maybe_select_query_part_token_stream = if let PostgresqlJsonObjectTypePattern::Standart = &postgresql_json_object_type_pattern &&
                    let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable {
                        let select_query_part_for_loop_token_stream = generate_select_query_part_for_loop_token_stream(
                            &is_standart_with_id_false,
                            &self_snake_case,
                            &quote::quote!{column_name_and_maybe_field_getter},
                            &quote::quote!{column_name_and_maybe_field_getter_for_error_message},
                        );
                        quote::quote! {
                            fn #select_query_part_snake_case(
                                &self,
                                column_name_and_maybe_field_getter: &std::primitive::str,
                                column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
                            ) -> #std_string_string_token_stream {
                                let mut #acc_snake_case = #std_string_string_token_stream::default();
                                #select_query_part_for_loop_token_stream
                                let _ = #acc_snake_case.pop();
                                let _ = #acc_snake_case.pop();
                                format!("{acc}")
                            }
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    };
                    let select_query_part_postgresql_type_token_stream = {
                        let content_token_stream = match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#self_snake_case.#select_query_part_snake_case(
                                    &#column_snake_case,
                                    &#column_snake_case,
                                )},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(&match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_as_postgresql_json_type_select_token_stream,
                                        PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream,
                                    });
                                    quote::quote! {
                                        format!(
                                            "case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({}) end",
                                            {
                                                let #value_snake_case = match &self.0 {
                                                    Some(#value_snake_case) => #value_snake_case,
                                                    None => &#default_but_option_is_always_some_call_token_stream,
                                                };
                                                #value_snake_case.#select_query_part_postgresql_type_snake_case(#column_snake_case)
                                            }
                                        )
                                    }
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let select_query_part_for_loop_token_stream = generate_select_query_part_for_loop_token_stream(
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
                                            let mut #acc_snake_case = #std_string_string_token_stream::default();
                                            #select_query_part_for_loop_token_stream
                                            let _ = #acc_snake_case.pop();
                                            let _ = #acc_snake_case.pop();
                                            #acc_snake_case
                                        };
                                        let dimension1_start = self.#dimension1_pagination_token_stream.start();
                                        let dimension1_end = self.#dimension1_pagination_token_stream.end();
                                        format!(#format_handle_token_stream)
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({}) end");
                                    let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream);
                                    quote::quote! {
                                        format!(#format_handle_token_stream, {
                                            let #value_snake_case = match &self.0 {
                                                Some(#value_snake_case) => #value_snake_case,
                                                None => &#default_but_option_is_always_some_call_token_stream,
                                            };
                                            #value_snake_case.#select_query_part_postgresql_type_snake_case(column)
                                        })
                                    }
                                },
                            },
                        };
                        quote::quote! {
                            fn #select_query_part_postgresql_type_snake_case(
                                &self,
                                #column_snake_case: &std::primitive::str,
                            ) -> #std_string_string_token_stream {
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
                        let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let serialize_deserialize_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let field_type_as_json_type_select_token_stream = generate_type_as_postgresql_json_type_select_token_stream(&element.ty);
                            quote::quote! {
                                #[serde(rename(serialize = #serialize_deserialize_field_ident_double_quotes_token_stream, deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream))]
                                #variant_ident_upper_camel_case_token_stream(#field_type_as_json_type_select_token_stream)
                            }
                        });
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub enum #ident_select_element_or_ident_with_id_select_element_upper_camel_case {
                                #(#content_token_stream),*
                            }
                        }
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
                                let elements_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                                    let field_ident = &element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    quote::quote! {
                                        #ident_select_element_or_ident_with_id_select_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(#content_token_stream)
                                    }
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
                        let impl_new_for_ident_with_id_standart_not_null_select_token_stream = generate_pub_new_value_type_content_self_value_token_stream(
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
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let generate_ident_where_element_field_variants_token_stream = |is_standart_with_id: &IsStandartWithId| {
                        let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                            let field_type_as_json_type_where_element_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&element.ty, &postgresql_json_type_subtype_where_element);
                            quote::quote! {
                                #field_ident_upper_camel_case_token_stream(#import_path::PostgresqlTypeWhere<
                                    #field_type_as_json_type_where_element_token_stream
                                >)
                            }
                        });
                        quote::quote! {#(#variants_token_stream),*}
                    };
                    let generate_ident_where_element_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub enum #ident_token_stream {
                                #content_token_stream
                            }
                        }
                    };
                    let equal_variant_ident_token_stream = quote::quote! {#equal_upper_camel_case(#import_path::PostgresqlJsonTypeWhereElementEqual<#ident_as_postgresql_json_type_table_type_declaration_token_stream>)};
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
                    let maybe_ident_where_element_token_stream = {
                        let generate_ident_where_element_wrapper_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_ident_where_element_token_stream(
                            &ident_where_element_upper_camel_case,
                            &content_token_stream
                        );
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => generate_ident_where_element_wrapper_token_stream(&{
                                    let ident_where_element_field_variants_token_stream = generate_ident_where_element_field_variants_token_stream(&is_standart_with_id_false);
                                    quote::quote!{
                                        #ident_where_element_field_variants_token_stream,
                                        #equal_variant_ident_token_stream,
                                    }
                                }),
                                PostgresqlJsonObjectTypePattern::Array => generate_ident_where_element_wrapper_token_stream(&{
                                    let dimension_one_equal_token_stream = quote::quote! {
                                        DimensionOneEqual(#import_path::PostgresqlJsonTypeWhereElementDimensionOneEqual<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let length_equal_token_stream = quote::quote! {
                                        LengthEqual(#import_path::PostgresqlJsonTypeWhereElementLengthEqual),
                                    };
                                    let length_more_than_token_stream = quote::quote! {
                                        LengthMoreThan(#import_path::PostgresqlJsonTypeWhereElementLengthMoreThan),
                                    };
                                    let in_token_stream = quote::quote! {
                                        In(#import_path::PostgresqlJsonTypeWhereElementIn<#ident_as_postgresql_json_type_table_type_declaration_token_stream>),
                                    };
                                    let dimension_one_in_token_stream = quote::quote! {
                                        DimensionOneIn(#import_path::PostgresqlJsonTypeWhereElementDimensionOneIn<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let contains_all_elements_of_array_token_stream = quote::quote! {
                                        ContainsAllElementsOfArray(#import_path::PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let overlaps_with_array_token_stream = quote::quote! {
                                        OverlapsWithArray(#import_path::PostgresqlJsonTypeWhereElementOverlapsWithArray<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),
                                    };
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        let element_type_as_postgresql_json_type_where_element_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&element.ty, &PostgresqlJsonTypeSubtype::WhereElement);
                                        quote::quote! {
                                            #element_field_ident_upper_camel_case(#import_path::PostgresqlTypeWhere<
                                                #element_type_as_postgresql_json_type_where_element_token_stream
                                            >)
                                        }
                                    });
                                    quote::quote! {
                                        #equal_variant_ident_token_stream,
                                        #dimension_one_equal_token_stream
                                        #length_equal_token_stream
                                        #length_more_than_token_stream
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
                        let query_part_variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
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
                        let query_bind_variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                            quote::quote! {
                                Self::#field_ident_upper_camel_case_token_stream(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream
                            }
                        });
                        quote::quote! {#(#query_bind_variants_token_stream),*}
                    };
                    let generate_impl_postgresql_type_where_filter_token_stream = |ident_token_stream: &dyn quote::ToTokens, query_part_content_token_stream: &dyn quote::ToTokens, is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable, query_bind_content_token_stream: &dyn quote::ToTokens| {
                        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                            &quote::quote! {<'a>},
                            &ident_token_stream,
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
                    let maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream = {
                        let generate_impl_postgresql_type_where_filter_for_ident_token_stream = |query_part_content_token_stream: &dyn quote::ToTokens, is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable, query_bind_content_token_stream: &dyn quote::ToTokens| generate_impl_postgresql_type_where_filter_token_stream(&ident_where_element_upper_camel_case, &query_part_content_token_stream, is_query_bind_mutable, &query_bind_content_token_stream);
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
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        quote::quote! {
                                            Self::#element_field_ident_upper_camel_case(#value_snake_case) => generate_element_query(
                                                #value_snake_case.get_logical_operator(),
                                                #value_snake_case,
                                                &#field_ident_double_quotes_token_stream
                                            )
                                        }
                                    });
                                    quote::quote! {
                                        let mut generate_element_query = |
                                            logical_operator: &#import_path::LogicalOperator,
                                            #value_snake_case: &dyn #import_path::PostgresqlTypeWhereFilter<'_>,
                                            field: &std::primitive::str
                                        | -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                                            let logical_operator_query_part = logical_operator.to_query_part(is_need_to_add_logical_operator);
                                            let elem = "elem";
                                            let #value_snake_case = match #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
                                                #value_snake_case,
                                                #increment_snake_case,
                                                &format!("{elem}->'{field}'"),
                                                false
                                            ) {
                                                Ok(#value_snake_case) => #value_snake_case,
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            };
                                            Ok(format!("{logical_operator_query_part}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {value}))"))
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
                                            Self::LengthMoreThan(#value_snake_case) => #import_path::PostgresqlTypeWhereFilter::#query_part_snake_case(
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
                                    let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                        quote::quote! {Self::#element_field_ident_upper_camel_case(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream}
                                    });
                                    quote::quote! {
                                        match self {
                                            Self::Equal(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::DimensionOneEqual(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::LengthEqual(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
                                            Self::LengthMoreThan(#value_snake_case) => #postgresql_type_where_filter_query_bind_value_query_token_stream,
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
                    let maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream = if let (PostgresqlJsonObjectTypePattern::Standart, NotNullOrNullable::Nullable) = (&postgresql_json_object_type_pattern, &not_null_or_nullable) {
                        proc_macro2::TokenStream::new()
                    } else {
                        generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_where_element_upper_camel_case)
                    };
                    let generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element = |is_standart_with_id: &IsStandartWithId| {
                        let generate_self_variant_default_some_one_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{
                            Self::#content_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                        };
                        let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            generate_self_variant_default_some_one_token_stream(&naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified))
                        });
                        let self_equal_default_some_one_token_stream = generate_self_variant_default_some_one_token_stream(&equal_upper_camel_case);
                        quote::quote! {vec![
                            #(#variants_token_stream),*,
                            #self_equal_default_some_one_token_stream
                        ]}
                    };
                    let maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_where_element_upper_camel_case, &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element(&is_standart_with_id_false)),
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                        PostgresqlJsonObjectTypePattern::Array => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_where_element_upper_camel_case, &{
                            let element_filters_token_stream = vec_syn_field_with_id.iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                quote::quote! {
                                    Self::#element_field_ident_upper_camel_case(#import_path_default_but_option_is_always_some_call_token_stream)
                                }
                            });
                            quote::quote! {
                                vec![
                                    Self::Equal(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::DimensionOneEqual(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::LengthEqual(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::LengthMoreThan(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::In(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::DimensionOneIn(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::ContainsAllElementsOfArray(#import_path_default_but_option_is_always_some_call_token_stream),
                                    Self::OverlapsWithArray(#import_path_default_but_option_is_always_some_call_token_stream),
                                    #(#element_filters_token_stream),*
                                ]
                            }
                        }),
                    };
                    let maybe_ident_with_id_standart_not_null_where_element_token_stream = if is_standart_not_null {
                        let ident_with_id_standart_not_null_where_element_token_stream = generate_ident_where_element_token_stream(
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
                            &{
                                let ident_where_element_field_variants_token_stream = generate_ident_where_element_field_variants_token_stream(&is_standart_with_id_true);
                                quote::quote!{
                                    #ident_where_element_field_variants_token_stream,
                                    #equal_upper_camel_case(#import_path::PostgresqlJsonTypeWhereElementEqual<#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case>),//todo maybe reuse? variant generation
                                }
                            }
                        );
                        let impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_standart_not_null_where_element_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
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
                        let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_element_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(&ident_with_id_standart_not_null_where_element_upper_camel_case);
                        let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_where_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_with_id_standart_not_null_where_element_upper_camel_case,
                            &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element(&is_standart_with_id_true)
                        );
                        quote::quote! {
                            #ident_with_id_standart_not_null_where_element_token_stream
                            #impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_standart_not_null_where_element_token_stream
                            #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_standart_not_null_where_element_token_stream
                            #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_where_element_token_stream
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    quote::quote! {
                        #maybe_ident_where_element_token_stream
                        #maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream
                        #maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream
                        #maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream
                        #maybe_ident_with_id_standart_not_null_where_element_token_stream
                    }
                }
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let ident_standart_or_ident_with_id_array_as_postgresql_json_type_where_element_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                            PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_upper_camel_case,
                        },
                        &postgresql_json_type_subtype_where_element
                    );
                    quote::quote! {
                        pub type #ident_where_element_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                            #ident_standart_or_ident_with_id_array_as_postgresql_json_type_where_element_token_stream
                        >;
                    }
                }
            };
            let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
                generate_quotes::double_quotes_token_stream(&value.ident.as_ref().unwrap_or_else(|| {
                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                }))
            };
            enum ShouldDeriveSerdeDeserialize {
                True,
                False,
            }
            impl quote::ToTokens for ShouldDeriveSerdeDeserialize {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    if let Self::True = &self {
                        quote::quote! {serde::Deserialize,}.to_tokens(tokens);
                    }
                }
            }
            let generate_type_as_postgresql_json_type_read_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_read);
            let generate_type_as_postgresql_json_type_read_inner_token_stream = |type_token_stream: &dyn quote::ToTokens| generate_type_as_postgresql_json_type_subtype_token_stream(&type_token_stream, &postgresql_json_type_subtype_read_inner);
            enum ReadOrReadInner {
                ReadWithSerdeOptionIsNoneAnnotation,
                ReadWithoutSerdeOptionIsNoneAnnotation,
                ReadInner,
            }
            let generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream = |is_standart_with_id: &IsStandartWithId, read_or_read_inner: &ReadOrReadInner| {
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                    let maybe_serde_skip_serializing_if_option_is_none_token_stream = match &read_or_read_inner {
                        ReadOrReadInner::ReadWithSerdeOptionIsNoneAnnotation => quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                        ReadOrReadInner::ReadWithoutSerdeOptionIsNoneAnnotation => proc_macro2::TokenStream::new(),
                        ReadOrReadInner::ReadInner => proc_macro2::TokenStream::new(),
                    };
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type_as_json_type_read_token_stream = match &read_or_read_inner {
                        ReadOrReadInner::ReadWithSerdeOptionIsNoneAnnotation | ReadOrReadInner::ReadWithoutSerdeOptionIsNoneAnnotation => generate_type_as_postgresql_json_type_read_token_stream(&element.ty),
                        ReadOrReadInner::ReadInner => generate_type_as_postgresql_json_type_read_inner_token_stream(&element.ty),
                    };
                    let std_option_option_value_field_type_as_json_type_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                        &wrap_into_value_declaration_token_stream(&field_type_as_json_type_read_token_stream)
                    );
                    quote::quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_token_stream
                        #field_ident: #std_option_option_value_field_type_as_json_type_read_token_stream
                    }
                });
                quote::quote! {
                    #(#content_token_stream),*
                }
            };
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_array_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_array_not_null_upper_camel_case);
            let ident_with_id_standart_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_with_id_standart_not_null_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_read_token_stream = {
                let ident_read_try_from_error_named_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident);
                let ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
                let ident_standart_not_null_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&ident_standart_not_null_upper_camel_case);
                let ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&ident_with_id_array_not_null_upper_camel_case);
                let generate_ident_read_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens, should_derive_serde_deserialize: &ShouldDeriveSerdeDeserialize| {
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, #should_derive_serde_deserialize utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_token_stream #content_token_stream
                    }
                };
                let ident_read_token_stream = {
                    let (content_token_stream, should_derive_serde_deserialize) = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                {
                                    let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(&is_standart_with_id_false, &ReadOrReadInner::ReadWithSerdeOptionIsNoneAnnotation);
                                    quote::quote! {{#content_token_stream}}
                                },
                                ShouldDeriveSerdeDeserialize::False,
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_as_postgresql_json_type_read_token_stream)), ShouldDeriveSerdeDeserialize::True),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case)), ShouldDeriveSerdeDeserialize::True),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream)), ShouldDeriveSerdeDeserialize::True),
                        },
                    };
                    generate_ident_read_token_stream(&ident_read_upper_camel_case, &content_token_stream, &should_derive_serde_deserialize)
                };
                let generate_fn_into_inner_token_stream = |ident_token_stream: &dyn quote::ToTokens, return_type_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl #ident_token_stream {
                            fn into_inner(self) -> #return_type_token_stream {
                                #content_token_stream
                            }
                        }
                    }
                };
                let impl_into_inner_for_ident_read_token_stream = generate_fn_into_inner_token_stream(
                    &ident_read_upper_camel_case,
                    &ident_read_inner_upper_camel_case,
                    &{
                        let generate_into_inner_token_stream = |ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens|{
                            quote::quote!{#ident_token_stream::into_inner(#parameters_token_stream)}
                        };
                        let generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                            let ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                                IsStandartWithId::False => &ident_read_inner_upper_camel_case,
                                IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                            };
                            let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let value_content_token_stream = wrap_into_value_initialization_token_stream(&generate_into_inner_token_stream(
                                    &generate_type_as_postgresql_json_type_token_stream(&element.ty),
                                    &quote::quote!{#value_snake_case.#value_snake_case},
                                ));
                                let parameter_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                                    IsStandartWithId::False => &self_snake_case,
                                    IsStandartWithId::True => &element_snake_case,
                                };
                                quote::quote! {
                                    #field_ident: match #parameter_token_stream.#field_ident {
                                        Some(#value_snake_case) => Some(#value_content_token_stream),
                                        None => None
                                    }
                                }
                            });
                            quote::quote! {
                                #ident_token_stream {
                                    #(#content_token_stream),*
                                }
                            }
                        };
                        let generate_match_option_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                            quote::quote! {
                                match self.0 {
                                    Some(#value_snake_case) => Some(#content_token_stream),
                                    None => None
                                }
                            }
                        };
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&IsStandartWithId::False),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&generate_into_inner_token_stream(
                                    &generate_type_as_postgresql_json_type_token_stream(&ident_standart_not_null_upper_camel_case),
                                    &value_snake_case
                                )),
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = generate_impl_into_inner_for_ident_read_or_ident_with_id_standart_not_null_read_token_stream(&IsStandartWithId::True);
                                    quote::quote! {self.0.into_iter().map(|#element_snake_case|#content_token_stream).collect()}
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&generate_into_inner_token_stream(
                                    &ident_array_not_null_read_upper_camel_case,
                                    &value_snake_case
                                ))
                            },
                        }
                    },
                );
                let all_fields_are_none_upper_camel_case = naming::AllFieldsAreNoneUpperCamelCase;
                let generate_ident_read_try_from_error_named_token_stream = |ident_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_token_stream {
                            #all_fields_are_none_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }
                    }
                };
                let maybe_ident_read_try_from_error_named_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_read_try_from_error_named_token_stream(&ident_read_try_from_error_named_upper_camel_case),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => proc_macro2::TokenStream::new(),
                };
                let generate_ident_read_or_ident_with_id_read_upper_camel_case = |is_standart_with_id: &IsStandartWithId| match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_upper_camel_case,
                };
                let generate_impl_try_new_for_ident_read_try_from_error_named_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let ident_read_try_from_error_named_or_ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_try_from_error_named_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case,
                    };
                    macros_helpers::generate_impl_try_new_for_ident_token_stream::generate_impl_try_new_for_ident_token_stream(
                        &generate_ident_read_or_ident_with_id_read_upper_camel_case(&is_standart_with_id),
                        &generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(is_standart_with_id, &ReadOrReadInner::ReadWithoutSerdeOptionIsNoneAnnotation),
                        &quote::quote!{Self},
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
                                    let fields_token_stream = current_vec_syn_field.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        quote::quote! {#maybe_reference_symbol_token_stream #field_ident}
                                    });
                                    quote::quote! {
                                        #(#fields_token_stream),*
                                    }
                                };
                                (generate_fields_token_stream(&WithReference::True), generate_fields_token_stream(&WithReference::False))
                            };
                            let check_if_all_fields_are_none_token_stream = {
                                let (left_token_stream, right_token_stream) = {
                                    let current_vec_syn_field_len = current_vec_syn_field.len();
                                    let maybe_wrap_into_braces_handle_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::maybe_wrap_into_braces_token_stream(content_token_stream, current_vec_syn_field_len > 1);
                                    (
                                        maybe_wrap_into_braces_handle_token_stream(&{
                                            let nones_token_stream = {
                                                let mut acc = vec![];
                                                for _ in 0..current_vec_syn_field_len {
                                                    acc.push(quote::quote! {None});
                                                }
                                                acc
                                            };
                                            quote::quote! {#(#nones_token_stream),*}
                                        }),
                                        maybe_wrap_into_braces_handle_token_stream(&fields_reference_token_stream),
                                    )
                                };
                                quote::quote! {
                                    if let #left_token_stream = #right_token_stream {
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
                let impl_new_or_try_new_for_ident_read_try_from_error_named_token_stream = {
                    let std_vec_vec_ident_with_id_standart_not_null_read_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_try_new_for_ident_read_try_from_error_named_token_stream(&is_standart_with_id_false),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(&ident_read_upper_camel_case, &generate_value_type_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_as_postgresql_json_type_read_token_stream)), &self_value_token_stream),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_impl_new_for_ident_token_stream(&ident_read_upper_camel_case, &generate_value_type_token_stream(&std_vec_vec_ident_with_id_standart_not_null_read_token_stream), &self_value_token_stream),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(
                                &ident_read_upper_camel_case,
                                &generate_value_type_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&std_vec_vec_ident_with_id_standart_not_null_read_token_stream)),
                                &quote::quote! {
                                    Self(match #value_snake_case {
                                        Some(#value_snake_case) => Some(#ident_with_id_array_not_null_as_postgresql_json_type_read_token_stream::new(#value_snake_case)),
                                        None => None
                                    })
                                },
                            ),
                        },
                    }
                };
                let generate_impl_serde_deserialize_for_ident_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                    postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(&generate_ident_read_or_ident_with_id_read_upper_camel_case(&is_standart_with_id), current_vec_syn_field.iter().map(|element| (element.ident.as_ref().unwrap(), &element.ty)).collect::<std::vec::Vec<(&syn::Ident, &syn::Type)>>(), current_vec_syn_field.len(), &|_: &syn::Ident, syn_type: &syn::Type| {
                        let type_read_token_stream = generate_type_as_postgresql_json_type_read_token_stream(&syn_type);
                        postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                            &wrap_into_value_declaration_token_stream(&type_read_token_stream)
                        )
                    })
                };
                let maybe_impl_serde_deserialize_for_ident_read_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_serde_deserialize_for_ident_read_token_stream(&is_standart_with_id_false),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => proc_macro2::TokenStream::new(),
                };
                let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&generate_ident_read_or_ident_with_id_read_upper_camel_case(&is_standart_with_id), &proc_macro2::TokenStream::new(), &{
                        let fields_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let value_content_token_stream = wrap_into_value_initialization_token_stream(
                                &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            );
                            quote::quote! {#field_ident: Some(#value_content_token_stream)}
                        });
                        quote::quote! {Self{#(#fields_token_stream),*}}
                    })
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream(&is_standart_with_id_false),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &proc_macro2::TokenStream::new(), &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_read_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &quote::quote! {
                                Self(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])
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
                            let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(&is_standart_with_id_true, &ReadOrReadInner::ReadWithSerdeOptionIsNoneAnnotation);
                            quote::quote! {{#content_token_stream}}
                        },
                        &ShouldDeriveSerdeDeserialize::False,
                    );
                    let ident_with_id_standart_not_null_read_try_from_error_named_token_stream = generate_ident_read_try_from_error_named_token_stream(&ident_with_id_standart_not_null_read_try_from_error_named_upper_camel_case);
                    let impl_try_new_for_ident_with_id_standart_not_null_read_try_from_error_named_token_stream = generate_impl_try_new_for_ident_read_try_from_error_named_token_stream(&is_standart_with_id_true);
                    let impl_serde_deserialize_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_serde_deserialize_for_ident_read_token_stream(&is_standart_with_id_true);
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream(&is_standart_with_id_true);
                    let impl_sqlx_type_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                    let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_standart_not_null_read_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_with_id_standart_not_null_read_upper_camel_case);
                    quote::quote! {
                        #ident_with_id_standart_not_null_read_token_stream
                        #ident_with_id_standart_not_null_read_try_from_error_named_token_stream
                        #impl_try_new_for_ident_with_id_standart_not_null_read_try_from_error_named_token_stream
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
                    #impl_into_inner_for_ident_read_token_stream
                    #maybe_ident_read_try_from_error_named_token_stream
                    #impl_new_or_try_new_for_ident_read_try_from_error_named_token_stream
                    #maybe_impl_serde_deserialize_for_ident_read_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
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
                let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type_as_postgresql_json_type_read_only_ids_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &element.ty,
                        &PostgresqlJsonTypeSubtype::ReadOnlyIds
                    );
                    quote::quote! {#field_ident: #field_type_as_postgresql_json_type_read_only_ids_token_stream}
                });
                quote::quote! {{#(#content_token_stream),*}}
            };
            let generate_impl_sqlx_decode_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                    &ident_token_stream,
                    &quote::quote!{sqlx::types::Json<Self>},
                    &quote::quote!{Ok(#value_snake_case.0)}
                )
            };
            let generate_impl_sqlx_type_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_token_stream,
                    &quote::quote!{sqlx::types::Json<Self>}
                )
            };
            let generate_fields_read_only_ids_into_option_value_read_inner_token_stream = |is_standart_with_id: &IsStandartWithId, parameters_token_stream: &dyn quote::ToTokens|{
                let ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                    IsStandartWithId::False => &ident_standart_not_null_read_inner_upper_camel_case
                };
                let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type = &element.ty;
                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&field_type);
                    let field_type_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&field_type, &PostgresqlJsonTypeSubtype::Read);
                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                        let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(
                            &field_type_as_postgresql_json_type_read_token_stream
                        );
                        quote::quote!{#field_type_as_postgresql_json_type_token_stream::into_inner(#default_but_option_is_always_some_call_token_stream)}
                    });
                    quote::quote! {
                        #field_ident: match #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                            #parameters_token_stream.0.#value_snake_case.#field_ident
                        ) {
                            Some(#value_snake_case) => Some(#value_snake_case),
                            None => Some(#value_content_token_stream)
                        }
                    }
                });
                let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                    #ident_token_stream {
                        #(#content_token_stream),*
                    }
                });
                quote::quote!{Some(#value_content_token_stream)}
            };
            let ident_read_only_ids_token_stream = {
                let maybe_ident_read_only_ids_handle_token_stream = if is_standart_not_null {
                    let content_token_stream = generate_ident_read_only_ids_or_ident_with_id_read_only_ids_content_token_stream(&IsStandartWithId::False);
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub struct #ident_read_only_ids_handle_upper_camel_case #content_token_stream
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                let ident_read_only_ids_token_stream = {
                    let content_token_stream = match &postgresql_json_object_type_pattern {
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
                    };
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub struct #ident_read_only_ids_upper_camel_case #content_token_stream
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream = generate_impl_sqlx_decode_token_stream(&ident_read_only_ids_upper_camel_case);
                let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream = generate_impl_sqlx_type_token_stream(&ident_read_only_ids_upper_camel_case);
                let maybe_ident_with_id_standart_not_null_read_only_ids_token_stream = if is_standart_not_null {
                    let ident_with_id_standart_not_null_read_only_ids_token_stream = {
                        let ident_with_id_standart_not_null_read_only_ids_handle_token_stream = {
                            let content_token_stream = generate_ident_read_only_ids_or_ident_with_id_read_only_ids_content_token_stream(&IsStandartWithId::True);
                            quote::quote! {
                                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                                pub struct #ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case #content_token_stream
                            }
                        };
                        let ident_with_id_standart_not_null_read_only_ids_token_stream = {
                            let value_ident_with_id_standart_not_null_read_only_ids_handle_token_stream = wrap_into_value_declaration_token_stream(&ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case);
                            quote::quote! {
                                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                                pub struct #ident_with_id_standart_not_null_read_only_ids_upper_camel_case(pub #value_ident_with_id_standart_not_null_read_only_ids_handle_token_stream);
                            }
                        };
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
                let generate_ident_read_inner_or_ident_with_id_standart_not_null_read_inner_token_stream = |is_standart_with_id: &IsStandartWithId| {
                    let ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_inner_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_standart_not_null_read_inner_upper_camel_case,
                    };
                    let content_token_stream = generate_ident_or_ident_with_id_read_or_read_inner_fields_declaration_token_stream(&is_standart_with_id, &ReadOrReadInner::ReadInner);
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq)]
                        pub struct #ident_token_stream {
                            #content_token_stream
                        }
                    }
                };
                let ident_read_inner_token_stream = {
                    let generate_pub_type_ident_read_inner_alias_token_stream = |content_token_stream: &dyn quote::ToTokens| macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_read_inner_upper_camel_case, &content_token_stream);
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
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{jsonb_set_target_snake_case}}}->'{{value}}'"));
                    quote::quote! {
                        let #generate_jsonb_set_target_snake_case = |value: &std::primitive::str|{
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
            enum ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation {
                True,
                False,
            }
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
                    let generate_ident_update_token_stream = |should_derive_serde_deserialize: &ShouldDeriveSerdeDeserialize, content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, serde::Serialize, #should_derive_serde_deserialize utoipa::ToSchema, schemars::JsonSchema)]
                            pub struct #ident_update_upper_camel_case #content_token_stream
                        }
                    };
                    let generate_std_option_option_ident_type_token_stream = |ident_token_stream: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_token_stream));
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(&ShouldDeriveSerdeDeserialize::True, &wrap_content_into_scopes_dot_comma_token_stream(&generate_ident_update_standart_not_null_content_token_stream(&is_standart_with_id_false))),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(&ShouldDeriveSerdeDeserialize::True, &generate_std_option_option_ident_type_token_stream(&ident_standart_not_null_as_postgresql_json_type_update_token_stream)),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(&ShouldDeriveSerdeDeserialize::False, &{
                                let fields_token_stream = generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                quote::quote! {{#fields_token_stream}}
                            }),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(&ShouldDeriveSerdeDeserialize::True, &generate_std_option_option_ident_type_token_stream(&ident_with_id_array_not_null_as_postgresql_json_type_update_token_stream)),
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
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                            #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                            pub enum #ident_update_try_new_error_named_upper_camel_case {
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
                            }
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                };
                let impl_ident_update_token_stream = {
                    let maybe_new_or_try_new_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => {
                            let (
                                parameters_token_stream,
                                content_token_stream
                            ): (
                                &dyn quote::ToTokens,
                                &dyn quote::ToTokens
                            ) = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                    &generate_value_type_token_stream(&generate_unique_vec_wrapper_token_stream(&ident_standart_not_null_update_element_upper_camel_case)),
                                    &self_value_token_stream
                                ),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                                    &generate_value_type_token_stream(
                                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                            &ident_standart_not_null_as_postgresql_json_type_update_token_stream
                                        )
                                    ),
                                    &self_value_token_stream
                                ),
                            };
                            macros_helpers::generate_pub_new_token_stream(
                                &parameters_token_stream,
                                &content_token_stream
                            )
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_impl_try_new_for_ident_token_stream::generate_pub_try_new_token_stream(
                                &generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False),
                                &quote::quote!{Self},
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
                                            let generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream = |update_or_delete: &UpdateOrDelete|{
                                                let content_token_stream: &dyn quote::ToTokens = match &update_or_delete {
                                                    UpdateOrDelete::Update => &quote::quote!{#element_snake_case.#id_snake_case},
                                                    UpdateOrDelete::Delete => &element_snake_case
                                                };
                                                quote::quote!{
                                                    <#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream as error_occurence_lib::ToStdStringString>::to_std_string_string(
                                                        &#content_token_stream
                                                    )
                                                }
                                            };
                                            (
                                                generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream(&UpdateOrDelete::Update),
                                                generate_uuid_as_postgresql_json_type_update_to_std_string_string_token_stream(&UpdateOrDelete::Delete)
                                            )
                                        };
                                        quote::quote!{{
                                            let mut #acc_snake_case = vec![];
                                            for #element_snake_case in update.to_vec() {
                                                if #acc_snake_case.contains(&&#element_snake_case.#id_snake_case) {
                                                    return Err(#ident_update_try_new_error_named_upper_camel_case::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgresql_json_type_update_to_std_string_string_element_id_token_stream,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                else {
                                                    #acc_snake_case.push(&#element_snake_case.#id_snake_case);
                                                }
                                            }
                                            for #element_snake_case in &delete {
                                                if #acc_snake_case.contains(&#element_snake_case) {
                                                    return Err(#ident_update_try_new_error_named_upper_camel_case::#ids_are_not_unique_uppper_camel_case {
                                                        duplicate: #uuid_as_postgresql_json_type_update_to_std_string_string_element_token_stream,
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    });
                                                }
                                                else {
                                                    #acc_snake_case.push(&#element_snake_case);
                                                }
                                            }
                                        }}
                                    };
                                    let check_not_unique_id_token_stream = {
                                        let check_not_unique_id_in_update_array_token_stream = quote::quote! {
                                            let update_acc = #update_snake_case.to_vec().iter()
                                            .map(|#element_snake_case|&#element_snake_case.#id_snake_case)
                                            .collect::<std::vec::Vec<&#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_update_token_stream>>();
                                        };
                                        let check_not_unique_id_in_delete_aray_token_stream = {
                                            let not_unique_id_in_json_delete_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json delete array: {{}}"));
                                            quote::quote! {
                                                let delete_acc = {
                                                    let mut delete_acc = vec![];
                                                    for #element_snake_case in &delete {
                                                        if delete_acc.contains(&#element_snake_case) {
                                                            return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_delete_array_upper_camel_case {
                                                                #error_snake_case: format!(
                                                                    #not_unique_id_in_json_delete_array_double_quotes_token_stream,
                                                                    #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(
                                                                        //// &#element_snake_case
                                                                        &#element_snake_case.clone().into()
                                                                    )
                                                                ),
                                                                code_occurence: error_occurence_lib::code_occurence!()
                                                            });
                                                        } else {
                                                            delete_acc.push(#element_snake_case);
                                                        }
                                                    }
                                                    delete_acc
                                                };
                                            }
                                        };
                                        let check_not_unique_id_in_update_and_delete_arrays_token_stream = {
                                            let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json update and delete arrays: {{}}"));
                                            quote::quote! {
                                                for #element_snake_case in update_acc {
                                                    if delete_acc.contains(&&#element_snake_case) {
                                                        return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                                            #error_snake_case: format!(
                                                                #not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream,
                                                                #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(
                                                                    //// &#element_snake_case
                                                                    &element.clone().into()
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
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_new_value_type_content_self_value_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                    &ident_with_id_array_not_null_as_postgresql_json_type_update_token_stream
                                )
                            )
                        },
                    };
                    quote::quote!{
                        impl #ident_update_upper_camel_case {
                            #maybe_new_or_try_new_for_ident_update_token_stream
                        }
                    }
                };
                let maybe_impl_serde_deserialize_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => proc_macro2::TokenStream::new(),
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let tuple_struct_ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("tuple struct {ident_update_upper_camel_case}"));
                            let ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_update_upper_camel_case);
                            let match_try_new_in_deserialize_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(&ident_update_upper_camel_case, &quote::quote! {__field0, __field1, __field2});
                            quote::quote! {
                                impl<'de> serde::Deserialize<'de> for #ident_update_upper_camel_case {
                                    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
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
                                            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                                serde::__private::Formatter::write_str(__f, "field identifier")
                                            }
                                            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => serde::__private::Ok(__Field::__field0),
                                                    1u64 => serde::__private::Ok(__Field::__field1),
                                                    2u64 => serde::__private::Ok(__Field::__field2),
                                                    _ => serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    "create" => serde::__private::Ok(__Field::__field0),
                                                    "update" => serde::__private::Ok(__Field::__field1),
                                                    "delete" => serde::__private::Ok(__Field::__field2),
                                                    _ => serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: serde::de::Error,
                                            {
                                                match __value {
                                                    b"create" => serde::__private::Ok(__Field::__field0),
                                                    b"update" => serde::__private::Ok(__Field::__field1),
                                                    b"delete" => serde::__private::Ok(__Field::__field2),
                                                    _ => serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: serde::Deserializer<'de>,
                                            {
                                                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                                            }
                                        }
                                        #[doc(hidden)]
                                        struct __Visitor<'de> {
                                            marker: serde::__private::PhantomData<#ident_update_upper_camel_case>,
                                            lifetime: serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #ident_update_upper_camel_case;
                                            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                                serde::__private::Formatter::write_str(__f, #tuple_struct_ident_update_double_quotes_token_stream)
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 = match serde::de::SeqAccess::next_element::<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream>(&mut __seq)? {
                                                    serde::__private::Some(__value) => __value,
                                                    serde::__private::None => {
                                                        vec![]
                                                    }
                                                };
                                                let __field1 = match serde::de::SeqAccess::next_element::<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream>(&mut __seq)? {
                                                    serde::__private::Some(__value) => __value,
                                                    serde::__private::None => {
                                                        #import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream::default()
                                                    }
                                                };
                                                let __field2 = match serde::de::SeqAccess::next_element::<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>(&mut __seq)? {
                                                    serde::__private::Some(__value) => __value,
                                                    serde::__private::None => {
                                                        vec![]
                                                    }
                                                };
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                            #[inline]
                                            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: serde::__private::Option<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream> = serde::__private::None;
                                                let mut __field1: serde::__private::Option<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream> = serde::__private::None;
                                                let mut __field2: serde::__private::Option<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream> = serde::__private::None;
                                                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if serde::__private::Option::is_some(&__field0) {
                                                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                            }
                                                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<#std_vec_vec_ident_with_id_standart_not_null_create_token_stream>(&mut __map)?);
                                                        }
                                                        __Field::__field1 => {
                                                            if serde::__private::Option::is_some(&__field1) {
                                                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                            }
                                                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<#import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream>(&mut __map)?);
                                                        }
                                                        __Field::__field2 => {
                                                            if serde::__private::Option::is_some(&__field2) {
                                                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                            }
                                                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<#std_vec_vec_postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>(&mut __map)?);
                                                        }
                                                        _ => {
                                                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    serde::__private::Some(__field0) => __field0,
                                                    serde::__private::None => {
                                                        vec![]
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    serde::__private::Some(__field1) => __field1,
                                                    serde::__private::None => {
                                                        #import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream::default()
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    serde::__private::Some(__field2) => __field2,
                                                    serde::__private::None => {
                                                        vec![]
                                                    }
                                                };
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
                                        serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_update_double_quotes_token_stream,
                                            FIELDS,
                                            __Visitor {
                                                marker: serde::__private::PhantomData::<#ident_update_upper_camel_case>,
                                                lifetime: serde::__private::PhantomData,
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
                                #create_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                                #update_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                #delete_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                            }},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                            },
                        },
                    };
                    quote::quote! {Self #value}
                });
                let maybe_ident_update_element_token_stream = if is_standart_not_null {
                    let ident_update_element_token_stream = {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                            let value_field_type_as_json_type_update_token_stream = wrap_into_value_declaration_token_stream(&generate_type_as_postgresql_json_type_update_token_stream(&element.ty));
                            quote::quote! {
                                #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                                #variant_ident_upper_camel_case_token_stream(#value_field_type_as_json_type_update_token_stream)
                            }
                        });
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub enum #ident_standart_not_null_update_element_upper_camel_case {
                                #(#variants_token_stream),*
                            }
                        }
                    };
                    let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_standart_not_null_update_element_upper_camel_case, &{
                        let elements_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let value_content_token_stream = wrap_into_value_initialization_token_stream(
                                &postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            );
                            quote::quote! {
                                #ident_standart_not_null_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_content_token_stream)
                            }
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
                    let ident_with_id_standart_not_null_update_element_token_stream = quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_with_id_standart_not_null_update_element_upper_camel_case {
                            #ident_with_id_standart_not_null_update_element_fields_declaration_token_stream
                        }
                    };
                    let impl_new_for_ident_with_id_standart_not_null_update_element_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_with_id_standart_not_null_update_element_upper_camel_case,
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
                        #impl_new_for_ident_with_id_standart_not_null_update_element_token_stream
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
                    let generate_std_option_option_ident_type_token_stream = |ident_token_stream: &dyn quote::ToTokens| wrap_content_into_scopes_dot_comma_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_token_stream));
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
                                    let match_variants_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(#value_snake_case) => {
                                                match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                    &#value_snake_case.#value_snake_case,
                                                    &#field_ident_double_quotes_token_stream,
                                                    &column_name_and_maybe_field_getter,
                                                    #increment_snake_case
                                                ) {
                                                    Ok(mut #value_snake_case) => {
                                                        let _ = #value_snake_case.pop();
                                                        #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    }
                                                }
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        let mut #acc_snake_case = #std_string_string_token_stream::default();
                                        for #element_snake_case in self.0.to_vec() {
                                            match &#element_snake_case {
                                                #(#match_variants_token_stream),*
                                            }
                                        }
                                        let _ = #acc_snake_case.pop();
                                        let _ = #acc_snake_case.pop();
                                        Ok(#acc_snake_case)
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let match_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(
                                                #value_snake_case
                                            ) => match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                &#value_snake_case.#value_snake_case,
                                                &#field_ident_double_quotes_token_stream,
                                                &column_name_and_maybe_field_getter,
                                                #increment_snake_case
                                            ) {
                                                Ok(mut #value_snake_case) => {
                                                    let _ = #value_snake_case.pop();
                                                    #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                                }
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        Ok(match &self.0 {
                                            Some(#value_snake_case) => {
                                                let mut #acc_snake_case = #std_string_string_token_stream::default();
                                                for #element_snake_case in #value_snake_case.0.to_vec() {
                                                    match &#element_snake_case {
                                                        #(#match_content_token_stream),*
                                                    }
                                                }
                                                let _ = #acc_snake_case.pop();
                                                let _ = #acc_snake_case.pop();
                                                format!("jsonb_build_object('value',{})", #acc_snake_case)
                                            },
                                            None => "'null'::jsonb".to_string()
                                        })
                                    }
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let match_variants_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                        quote::quote! {
                                            #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(#value_snake_case) => match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_part_snake_case(
                                                &#value_snake_case.#value_snake_case,
                                                &#field_ident_double_quotes_token_stream,
                                                &"elem",
                                                #increment_snake_case
                                            ) {
                                                Ok(mut #value_snake_case) => {
                                                    let _ = #value_snake_case.pop();
                                                    current_acc.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                                }
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    let select_only_created_ids_query_part_content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                                        let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                        quote::quote! {
                                            match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                                &#element_snake_case.#field_ident,
                                                &#field_ident_double_quotes_token_stream,
                                                &"elem",
                                                #increment_snake_case
                                            ) {
                                                Ok(mut #value_snake_case) => {
                                                    let _ = #value_snake_case.pop();
                                                     #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        Ok(format!(
                                            "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                                            {
                                                let mut #acc_snake_case = #std_string_string_token_stream::new();
                                                for #element_snake_case in self.#update_snake_case.to_vec() {
                                                    //todo maybe wrong for multiple updates by id?
                                                    let mut current_acc = #std_string_string_token_stream::new();
                                                    match #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream ::select_only_updated_ids_query_part(
                                                        &#element_snake_case.id,
                                                        &"id",
                                                        &"elem",
                                                        #increment_snake_case
                                                    ) {
                                                        Ok(mut #value_snake_case) => {
                                                            let _ = #value_snake_case.pop();
                                                            current_acc.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                                        }
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        }
                                                    }
                                                    for #element_snake_case in #element_snake_case.fields.0.to_vec() {
                                                        match &#element_snake_case {
                                                            #(#match_variants_token_stream),*
                                                        }
                                                    }
                                                    let _ = current_acc.pop();
                                                    let _ = current_acc.pop();
                                                    #acc_snake_case.push_str(&format!("{}||", current_acc));
                                                }
                                                for #element_snake_case in &self.create {
                                                    #(#select_only_created_ids_query_part_content_token_stream)*
                                                }
                                                let _ = #acc_snake_case.pop();
                                                let _ = #acc_snake_case.pop();
                                                format!("jsonb_build_object('value',{})", #acc_snake_case)
                                            },
                                            column_name_and_maybe_field_getter,
                                            {
                                                let mut #acc_snake_case = #std_string_string_token_stream::new();
                                                for _ in self.#update_snake_case.to_vec() {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                        Ok(#value_snake_case) => {
                                                             #acc_snake_case.push_str(&format!("${value},"));
                                                        },
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        },
                                                    }
                                                }
                                                for #element_snake_case in &self.#create_snake_case {
                                                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                        Ok(#value_snake_case) => {
                                                             #acc_snake_case.push_str(&format!("${value},"));
                                                        },
                                                        Err(#error_snake_case) => {
                                                            return Err(#error_snake_case);
                                                        },
                                                    }
                                                }
                                                let _ = #acc_snake_case.pop();
                                                #acc_snake_case
                                            }
                                        ))
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                    Ok(match &self.0 {
                                        Some(#value_snake_case) => format!(
                                            "jsonb_build_object('value',{})",
                                            match #ident_array_not_null_update_for_query_upper_camel_case::#select_only_updated_ids_query_part_snake_case(
                                                &#value_snake_case,
                                                &column_name_and_maybe_field_getter,
                                                #increment_snake_case
                                            ) {
                                                Ok(#value_snake_case) => #value_snake_case,
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        ),
                                        None => "'null'::jsonb".to_string(),
                                    })
                                },
                            },
                        };
                        quote::quote!{
                            fn #select_only_updated_ids_query_part_snake_case(
                                &self,
                                column_name_and_maybe_field_getter: &std::primitive::str,
                                #increment_snake_case: &mut std::primitive::u64
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
                let impl_std_convert_from_ident_standart_not_null_update_for_ident_standart_not_null_update_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &quote::quote!{#ident_as_import_path_postgresql_json_type_token_stream::Update},
                    &quote::quote!{#ident_as_import_path_postgresql_json_type_token_stream::UpdateForQuery},
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{
                                Self(#import_path::NotEmptyUniqueEnumVec::from_t1_impl_from_t2(#value_snake_case.0))
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                Self(match #value_snake_case.0 {
                                    Some(#value_snake_case) => Some(#ident_standart_not_null_as_import_path_postgresql_json_type_token_stream::UpdateForQuery::from(#value_snake_case)),
                                    None => None,
                                })
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{
                                Self {
                                    #create_snake_case: #value_snake_case.#create_snake_case.into_iter().map(|#element_snake_case|#ident_with_id_standart_not_null_create_for_query_upper_camel_case::from(
                                        #element_snake_case
                                    )).collect(),
                                    #update_snake_case: #import_path::UniqueVec::from_t1_impl_from_t2(#value_snake_case.#update_snake_case),
                                    #delete_snake_case: #value_snake_case.#delete_snake_case.into_iter().map(|#element_snake_case|#element_snake_case.into()).collect(),
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote::quote!{
                                    Self(match #value_snake_case.0 {
                                        Some(#value_snake_case) => Some(#ident_array_not_null_as_import_path_postgresql_json_type_token_stream::UpdateForQuery::from(#value_snake_case)),
                                        None => None,
                                    })
                                }
                            },
                        },
                    }
                );
                let maybe_ident_update_for_query_element_token_stream = if is_standart_not_null {
                    let ident_standart_not_null_update_for_query_element_token_stream = {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                            let value_field_type_as_json_type_update_for_query_token_stream = wrap_into_value_declaration_token_stream(&generate_type_as_postgresql_json_type_update_for_query_token_stream(&element.ty));
                            quote::quote! {
                                #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                                #variant_ident_upper_camel_case_token_stream(#value_field_type_as_json_type_update_for_query_token_stream)
                            }
                        });
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                            pub enum #ident_standart_not_null_update_for_query_element_upper_camel_case {
                                #(#variants_token_stream),*
                            }
                        }
                    };
                    let impl_std_convert_from_ident_standart_not_null_update_element_for_ident_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                        &ident_standart_not_null_update_element_upper_camel_case,
                        &ident_standart_not_null_update_for_query_element_upper_camel_case,
                        &{
                            let variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                                    let field_type_as_json_type_update_for_query_token_stream = generate_type_as_postgresql_json_type_update_for_query_token_stream(&element.ty);
                                    quote::quote!{
                                        #field_type_as_json_type_update_for_query_token_stream::from(#value_snake_case.#value_snake_case)
                                    }
                                });
                                quote::quote! {
                                    #ident_standart_not_null_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_snake_case) => #ident_standart_not_null_update_for_query_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_initialization_token_stream)
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
                    let impl_new_for_ident_with_id_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_with_id_standart_not_null_update_for_query_element_upper_camel_case,
                        &ident_with_id_standart_not_null_update_for_query_element_fields_declaration_token_stream,
                        &quote::quote! {Self {
                            #id_snake_case,
                            #fields_snake_case
                        }},
                    );
                    let impl_std_convert_from_ident_with_id_standart_not_null_update_element_for_ident_with_id_standart_not_null_update_for_query_element_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
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
                        #impl_new_for_ident_with_id_standart_not_null_update_for_query_element_token_stream
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
            let value_into_inner_token_stream = quote::quote! {#value_snake_case.into_inner()};
            let (maybe_impl_postgresql_crud_postgresql_json_type_for_ident_token_stream, maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream) = {
                let postgresql_type_or_postgresql_json_type_postgresql_type = postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType;
                let postgresql_type_or_postgresql_json_type_postgresql_json_type = postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType;
                let generate_update_query_part_standart_nullable_token_stream = |postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType|{
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{value_snake_case}}})"),
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => format!("${{{value_snake_case}}}"),
                    });
                    quote::quote! {
                        match &#value_snake_case.0 {
                            Some(#value_snake_case) => #ident_standart_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                #value_snake_case,
                                jsonb_set_accumulator,
                                jsonb_set_target,
                                jsonb_set_path,
                                increment,
                            ),
                            None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                Ok(#value_snake_case) => Ok(format!(#format_handle_token_stream)),
                                Err(#error_snake_case) => Err(#error_snake_case),
                            }
                        }
                    }
                };
                let generate_update_delete_create_array_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
                    quote::quote! {
                        let update_query_part_acc = {
                            if #value_snake_case.#update_snake_case.is_empty() {
                                #std_string_string_token_stream::from("elem")
                            } else {
                                let mut #acc_snake_case = #std_string_string_token_stream::default();
                                for #element_snake_case in #value_snake_case.#update_snake_case.to_vec() {
                                    let ident_with_id_handle = {
                                        let id_increment = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                            Ok(#value_snake_case) => #value_snake_case,
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        };
                                        match #ident_standart_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                            &#element_snake_case.fields,
                                            &"",
                                            &"elem",
                                            &"",
                                            #increment_snake_case
                                        ) {
                                            Ok(#value_snake_case) => Ok(format!("when elem->>'id' = ${id_increment} then {value} ")),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    };
                                    match ident_with_id_handle {
                                        Ok(#value_snake_case) => {
                                            #acc_snake_case.push_str(&#value_snake_case);
                                        }
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                let _ = #acc_snake_case.pop();
                                format!("case {acc} else elem end")
                            }
                        };
                        let delete_query_part_acc = {
                            let mut #acc_snake_case = #std_string_string_token_stream::default();
                            for _ in &#value_snake_case.#delete_snake_case {
                                let #increment_snake_case = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                    Ok(#value_snake_case) => #value_snake_case,
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                };
                                let maybe_space_and_space = if #acc_snake_case.is_empty() { "" } else { " and " };
                                #acc_snake_case.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                            }
                            #acc_snake_case
                        };
                        let create_query_part_acc = {
                            let mut #acc_snake_case = #std_string_string_token_stream::default();
                            for _ in &#value_snake_case.#create_snake_case {
                                let #increment_snake_case = match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::increment_checked_add_one(#increment_snake_case) {
                                    Ok(#value_snake_case) => #value_snake_case,
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                };
                                #acc_snake_case.push_str(&format!("${increment},"));
                            }
                            let _ = #acc_snake_case.pop();
                            #acc_snake_case
                        };
                        let maybe_where = if #value_snake_case.#delete_snake_case.is_empty() {
                            #std_string_string_token_stream::default()
                        } else {
                            format!(" where {delete_query_part_acc}")
                        };
                        let maybe_jsonb_build_array = if #value_snake_case.#create_snake_case.is_empty() {
                            #std_string_string_token_stream::default()
                        } else {
                            format!(" || jsonb_build_array({create_query_part_acc})")
                        };
                        Ok (format!(#format_handle_token_stream))
                    }
                };
                let generate_update_query_part_array_not_null_token_stream = |postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType|{
                    generate_update_delete_create_array_token_stream(&generate_quotes::double_quotes_token_stream(&match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})",
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => "((select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) {maybe_jsonb_build_array})",
                    }))
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
                                let acc = value.select_query_part(
                                    &if is_postgresql_type {
                                        column_name_and_maybe_field_getter.to_string()
                                    } else {
                                        format!("{column_name_and_maybe_field_getter}->'{field_ident}'")
                                    },
                                    &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
                                );
                                if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',{acc}))") }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(
                                    &ident_standart_not_null_as_postgresql_json_type_select_token_stream
                                );
                                quote::quote! {
                                    let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
                                    format!(
                                        "jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({}) end))",
                                        {
                                            let #value_snake_case = match &#value_snake_case.0 {
                                                Some(#value_snake_case) => #value_snake_case,
                                                None => &#default_but_option_is_always_some_call_token_stream,
                                            };
                                            #ident_standart_not_null_as_postgresql_json_type_token_stream::select_query_part(
                                                value,
                                                field_ident,
                                                &column_name_and_maybe_field_getter_field_ident,
                                                column_name_and_maybe_field_getter_for_error_message,
                                                true
                                            )
                                        }
                                    )
                                }
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let select_query_part_for_loop_token_stream = {
                                    let value_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&value_snake_case);
                                    generate_select_query_part_for_loop_token_stream(
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
                                        let mut #acc_snake_case = #std_string_string_token_stream::default();
                                        #select_query_part_for_loop_token_stream
                                        let _ = #acc_snake_case.pop();
                                        let _ = #acc_snake_case.pop();
                                        #acc_snake_case
                                    };
                                    let dimension1_start = #value_snake_case.#dimension1_pagination_token_stream.start();
                                    let dimension1_end = #value_snake_case.#dimension1_pagination_token_stream.end();
                                    format!(#format_handle_token_stream)
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({}) end");
                                let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(
                                    &ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream
                                );
                                quote::quote! {
                                    format!(
                                        #format_handle_token_stream,
                                        {
                                            let #value_snake_case = match &#value_snake_case.0 {
                                                Some(#value_snake_case) => #value_snake_case,
                                                None => &#default_but_option_is_always_some_call_token_stream,
                                            };
                                            #ident_with_id_array_not_null_as_postgresql_json_type_token_stream::select_query_part(
                                                #value_snake_case,
                                                field_ident,
                                                &column_name_and_maybe_field_getter,
                                                column_name_and_maybe_field_getter_for_error_message,
                                                true
                                            )
                                        }
                                    )
                                }
                            }
                        },
                    },
                    &ident_where_element_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &ident_read_only_ids_upper_camel_case,
                    &{
                        let generate_select_only_ids_query_part_token_stream = |is_standart_with_id: &IsStandartWithId, postgresql_json_object_type_pattern: &PostgresqlJsonObjectTypePattern| {
                            let acc_push_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{field_ident}',{{}})||"));
                                let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                let content_token_stream = match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let select_only_ids_query_part_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{column_name_and_maybe_field_getter}}->'{field_ident}'"));
                                        quote::quote! {
                                            format!(#select_only_ids_query_part_format_handle_token_stream)
                                        }
                                    },
                                    PostgresqlJsonObjectTypePattern::Array => generate_quotes::double_quotes_token_stream(&format!("elem->'{field_ident}'")),
                                };
                                quote::quote! {
                                    #acc_snake_case.push_str(&format!(
                                        #format_handle_token_stream,
                                        #field_type_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(&#content_token_stream)
                                    ));
                                }
                            });
                            quote::quote! {
                                let mut #acc_snake_case = #std_string_string_token_stream::default();
                                #(#acc_push_token_stream)*
                                let _ = #acc_snake_case.pop();
                                let _ = #acc_snake_case.pop();
                                format!("jsonb_build_object('value',{})",#acc_snake_case)
                            }
                        };
                        let case_null_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('value',case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}})='null' then 'null'::jsonb else {{}} end)"));
                        match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_select_only_ids_query_part_token_stream(&is_standart_with_id_false, &postgresql_json_object_type_pattern),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {format!(
                                        #case_null_format_handle_token_stream,
                                        #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(#column_name_and_maybe_field_getter_snake_case),
                                    )}
                                },
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('value',(select jsonb_agg({{}}) from jsonb_array_elements({{{column_name_and_maybe_field_getter_snake_case}}}) as elem))"));
                                    let content_token_stream = &generate_select_only_ids_query_part_token_stream(&is_standart_with_id_true, &postgresql_json_object_type_pattern);
                                    quote::quote! {format!(#format_handle_token_stream, {#content_token_stream})}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {format!(
                                        #case_null_format_handle_token_stream,
                                        #ident_with_id_array_not_null_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(column_name_and_maybe_field_getter),
                                    )}
                                },
                            },
                        }
                    },
                    &ident_read_inner_upper_camel_case,
                    &value_into_inner_token_stream,
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let object_acc_snake_case = naming::StdOptionOptionObjectAccSnakeCase;
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',{{{object_acc_snake_case}}})"));
                                let query_part_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                                    quote::quote! {
                                        #ident_update_for_query_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream::#update_query_part_snake_case(
                                                &#value_snake_case.#value_snake_case,
                                                &#object_acc_snake_case,
                                                &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                                #field_ident_double_quotes_token_stream,
                                                #increment_snake_case,
                                            ) {
                                                Ok(#value_snake_case) => {
                                                    #object_acc_snake_case = #value_snake_case;
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
                                    for #element_snake_case in #value_snake_case.0.to_vec() {
                                        match #element_snake_case {
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
                                    Some(#value_snake_case) => #ident_array_not_null_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
                                        #value_snake_case,
                                        jsonb_set_accumulator,
                                        jsonb_set_target,
                                        jsonb_set_path,
                                        #increment_snake_case,
                                    ),
                                    None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                        Ok(#value_snake_case) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${value})")),
                                        Err(#error_snake_case) => Err(#error_snake_case)
                                    }
                                }
                            },
                        },
                    },
                    &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::True,
                    &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                        },
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let update_query_bind_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                                    quote::quote! {
                                        #ident_update_for_query_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream::#update_query_bind_snake_case(
                                                #value_snake_case.#value_snake_case,
                                                #query_snake_case
                                            ) {
                                                Ok(#value_snake_case) => {
                                                    #query_snake_case = #value_snake_case;
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(#error_snake_case);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote::quote! {
                                    for #element_snake_case in #value_snake_case.0.into_vec() {
                                        match #element_snake_case {
                                            #(#update_query_bind_variants_token_stream),*
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                match #value_snake_case.0 {
                                    Some(#value_snake_case) => #ident_standart_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        #value_snake_case,
                                        #query_snake_case
                                    ),
                                    None => if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(#ident_as_postgresql_json_type_update_token_stream::new(None))) {
                                        return Err(#error_snake_case.to_string());
                                    }
                                    else {
                                        Ok(#query_snake_case)
                                    },
                                }
                            }
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                for #element_snake_case in #value_snake_case.#update_snake_case.into_vec() {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                        #element_snake_case.#id_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                    match #ident_standart_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        #element_snake_case.#fields_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                for #element_snake_case in #value_snake_case.delete {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                        #element_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(#error_snake_case);
                                        }
                                    }
                                }
                                for #element_snake_case in #value_snake_case.#create_snake_case {
                                    if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(
                                        #ident_with_id_standart_not_null_create_for_query_upper_camel_case::from(#element_snake_case)
                                    )) {
                                        return Err(#error_snake_case.to_string());
                                    }
                                }
                                Ok(#query_snake_case)
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                match #value_snake_case.0 {
                                    Some(#value_snake_case) => #ident_array_not_null_as_postgresql_json_type_token_stream::update_query_bind(
                                        #value_snake_case,
                                        #query_snake_case
                                    ),
                                    None => if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(#ident_as_postgresql_json_type_update_token_stream::new(None))) {
                                        return Err(#error_snake_case.to_string());
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
                            Ok(#value_snake_case) => Ok(format!("'{field_ident}',jsonb_build_object('value',{}),", #value_snake_case)),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let match_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    quote::quote! {
                                        #ident_standart_not_null_update_for_query_element_upper_camel_case::#field_ident_upper_camel_case(#value_snake_case) => {
                                            match #field_type_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                                &#value_snake_case.#value_snake_case,
                                                #query_snake_case
                                            ) {
                                                Ok(#value_snake_case) => {
                                                    #query_snake_case = #value_snake_case;
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(error);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for #element_snake_case in #value_snake_case.0.to_vec() {
                                        match #element_snake_case {
                                            #(#match_content_token_stream),*
                                        }
                                    }
                                    Ok(#query_snake_case)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                if let Some(#value_snake_case) = &#value_snake_case.0 {
                                    match #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(&#value_snake_case, #query_snake_case) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
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
                                let select_only_created_ids_query_bind_content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            &#element_snake_case.#field_ident,
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for #element_snake_case in #value_snake_case.#update_snake_case.to_vec() {
                                        match #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                            &#element_snake_case.#id_snake_case,
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                        match #ident_standart_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
                                            &#element_snake_case.fields,
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    for #element_snake_case in &#value_snake_case.create {
                                        #(#select_only_created_ids_query_bind_content_token_stream)*
                                    }
                                    for #element_snake_case in #value_snake_case.#update_snake_case.to_vec() {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_update_for_query(
                                            #element_snake_case.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                    for #element_snake_case in &#value_snake_case.#create_snake_case {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_create_for_query(
                                            #element_snake_case.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
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
                                if let Some(#value_snake_case) = &#value_snake_case.0 {
                                    match #ident_array_not_null_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(&#value_snake_case, #query_snake_case) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
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
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(
                                        &format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{field_ident}'")
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &#value_snake_case.#field_ident,
                                            &#field_ident_double_quotes_token_stream,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream),
                                            #increment_snake_case
                                        ) {
                                            Ok(mut #value_snake_case) => {
                                                let _ = #value_snake_case.pop();
                                                 #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
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
                                            let mut #acc_snake_case = #std_string_string_token_stream::new();
                                            #(#content_token_stream)*
                                            let _ = #acc_snake_case.pop();
                                            let _ = #acc_snake_case.pop();
                                            #acc_snake_case
                                        }
                                    ))
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    let column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(
                                        &format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{field_ident}'")
                                    );
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &#value_snake_case.#field_ident,
                                            &#field_ident_double_quotes_token_stream,
                                            &format!(#column_name_and_maybe_field_getter_field_ident_double_quotes_token_stream),
                                            #increment_snake_case
                                        ) {
                                            Ok(mut #value_snake_case) => {
                                                let _ = #value_snake_case.pop();
                                                #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
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
                                            Some(#value_snake_case) => format!(
                                                "jsonb_build_object('value',{})",
                                                {
                                                    let mut #acc_snake_case = #std_string_string_token_stream::new();
                                                    #(#content_token_stream)*
                                                    let _ = #acc_snake_case.pop();
                                                    let _ = #acc_snake_case.pop();
                                                    #acc_snake_case
                                                }
                                            ),
                                            None => "'null'::jsonb".to_string(),
                                        }
                                    ))
                                }
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &#element_snake_case.#field_ident,
                                            &#field_ident_double_quotes_token_stream,
                                            &"elem",
                                            #increment_snake_case
                                        ) {
                                            Ok(mut #value_snake_case) => {
                                                let _ = #value_snake_case.pop();
                                                #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    Ok(format!(
                                        "'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
                                        {
                                            let mut #acc_snake_case = #std_string_string_token_stream::new();
                                            for #element_snake_case in &#value_snake_case.0 {
                                                #(#content_token_stream)*
                                            }
                                            let _ = #acc_snake_case.pop();
                                            let _ = #acc_snake_case.pop();
                                            format!("jsonb_build_object('value',{})", #acc_snake_case)
                                        },
                                        &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                        {
                                            let mut #acc_snake_case = #std_string_string_token_stream::new();
                                            for _ in &#value_snake_case.0 {
                                                match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                    Ok(#value_snake_case) => {
                                                        #acc_snake_case.push_str(&format!("${value},"));
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    },
                                                }
                                            }
                                            let _ = #acc_snake_case.pop();
                                            #acc_snake_case
                                        }
                                    ))
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    let field_ident_double_quotes_token_stream = &generate_quotes::double_quotes_token_stream(&field_ident);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_part_snake_case(
                                            &#element_snake_case.#field_ident,
                                            &#field_ident_double_quotes_token_stream,
                                            &"elem",
                                            #increment_snake_case
                                        ) {
                                            Ok(mut #value_snake_case) => {
                                                let _ = #value_snake_case.pop();
                                                #acc_snake_case.push_str(&format!("jsonb_build_object({})||", #value_snake_case));
                                            }
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
                                            Some(#value_snake_case) => format!(
                                                "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                                                {
                                                    let mut #acc_snake_case = #std_string_string_token_stream::new();
                                                    for #element_snake_case in &#value_snake_case.0 {
                                                        #(#content_token_stream)*
                                                    }
                                                    let _ = #acc_snake_case.pop();
                                                    let _ = #acc_snake_case.pop();
                                                    format!("jsonb_build_object('value',{})", #acc_snake_case)
                                                },
                                                &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                                                {
                                                    let mut #acc_snake_case = #std_string_string_token_stream::new();
                                                    for _ in &#value_snake_case.0 {
                                                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                                            Ok(#value_snake_case) => {
                                                                #acc_snake_case.push_str(&format!("${value},"));
                                                            },
                                                            Err(#error_snake_case) => {
                                                                return Err(#error_snake_case);
                                                            },
                                                        }
                                                    }
                                                    let _ = #acc_snake_case.pop();
                                                    #acc_snake_case
                                                }
                                            ),
                                            None => "'null'::jsonb".to_string(),
                                        }
                                    ))
                                }
                            },
                        },
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True,
                        },
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            &#value_snake_case.#field_ident,
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
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
                                    if let Some(#value_snake_case) = &#value_snake_case.0 {
                                        match #ident_standart_not_null_as_import_path_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(
                                            &#value_snake_case,
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
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
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&element.ty);
                                    quote::quote! {
                                        match #field_type_as_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(&#element_snake_case.#field_ident, #query_snake_case) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
                                            }
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            }
                                        }
                                    }
                                });
                                quote::quote!{
                                    for #element_snake_case in &#value_snake_case.0 {
                                        #(#content_token_stream)*
                                    }
                                    for #element_snake_case in &#value_snake_case.0 {
                                        match #uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::query_bind_string_as_postgresql_text_create_for_query(
                                            #element_snake_case.#id_snake_case.clone(),
                                            #query_snake_case
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
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
                                    if let Some(#value_snake_case) = &#value_snake_case.0 {
                                        match #ident_array_not_null_as_import_path_postgresql_json_type_token_stream::#select_only_created_ids_query_bind_snake_case(&#value_snake_case, #query_snake_case) {
                                            Ok(#value_snake_case) => {
                                                #query_snake_case = #value_snake_case;
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
                    postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                    &{
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_string());
                        quote::quote! {
                            format!(#format_handle_token_stream, serde_json::to_string(&schemars::schema_for!(#ident_table_type_declaration_upper_camel_case)).unwrap())
                        }
                    },
                    &ident_create_upper_camel_case,
                    &postgresql_crud_macros_common::CreateQueryPartValueUnderscore::True,
                    &postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                    &quote::quote!{
                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(#value_snake_case) => Ok(format!("${value}")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                    &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                    &quote::quote!{
                        if let Err(#error_snake_case) = #query_snake_case.try_bind(
                            #ident_as_postgresql_json_type_create_for_query_token_stream::from(#value_snake_case)
                        ) {
                            return Err(#error_snake_case.to_string());
                        }
                        Ok(#query_snake_case)
                    },
                    &ident_select_upper_camel_case,
                    &postgresql_crud_macros_common::SelectQueryPartValueUnderscore::False,
                    &quote::quote! {format!("{} as {column}", #value_snake_case.#select_query_part_postgresql_type_snake_case(#column_snake_case))},
                    &ident_where_element_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &value_snake_case,
                    &ident_read_only_ids_upper_camel_case,
                    &quote::quote! {format!("{} as {column},", #ident_as_postgresql_json_type_token_stream::#select_only_ids_query_part_snake_case(&#column_snake_case))},
                    &ident_read_inner_upper_camel_case,
                    &value_into_inner_token_stream,
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &postgresql_crud_macros_common::UpdateQueryPartValueUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::False,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::False,
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#ident_as_postgresql_json_type_token_stream::#update_query_part_snake_case(
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
                                        Some(#value_snake_case) => {
                                            #content_token_stream
                                        },
                                        None => match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                            Ok(#value_snake_case) => Ok(format!("${value}")),
                                            Err(#error_snake_case) => Err(#error_snake_case)
                                        }
                                    }
                                }
                            },
                        },
                    },
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::False,
                    &quote::quote!{#ident_as_postgresql_json_type_token_stream::#update_query_bind_snake_case(
                        #value_snake_case,
                        #query_snake_case
                    )},
                    &quote::quote!{
                        match #value_snake_case.#select_only_updated_ids_query_part_snake_case(
                            &#column_snake_case,
                            #increment_snake_case
                        ) {
                            Ok(#value_snake_case) => Ok(format!("jsonb_build_object('value',{}) as {column},", #value_snake_case)),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                    &quote::quote!{#ident_as_postgresql_json_type_token_stream::#select_only_updated_ids_query_bind_snake_case(
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
                let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
                let acc_is_empty_none_or_some_acc_token_stream = quote::quote!{
                    if #acc_snake_case.is_empty() {
                        None
                    }
                    else {
                        Some(#acc_snake_case)
                    }
                };
                let generate_dimension_equal_token_stream = |dimension: &postgresql_crud_macros_common::Dimension|{
                    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case();
                    let generate_nullable_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote! {
                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                            (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => match <
                                #content_token_stream
                                as
                                #import_path::PostgresqlJsonTypeTestCases
                            >::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            ) {
                                Some(#value_snake_case) => if #value_snake_case.is_empty() {
                                    None
                                }
                                else {
                                    Some({
                                        let mut #acc_snake_case = vec![];
                                        for #element_snake_case in #value_snake_case.clone() {
                                            #acc_snake_case.push(
                                                #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                                    Some(
                                                        #import_path::NotEmptyUniqueEnumVec::try_new(
                                                            vec![#element_snake_case]
                                                        ).expect("error af515faf-8805-44c3-a5b6-f6e307b8b23a")
                                                    )
                                                )
                                            );
                                        }
                                        let whole = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                            Some(
                                                #import_path::NotEmptyUniqueEnumVec::try_new(
                                                    #value_snake_case
                                                ).expect("error 44d4e9b2-12e2-44c4-8c67-88cb7b8465ce")
                                            )
                                        );
                                        if !#acc_snake_case.contains(&whole) {
                                            #acc_snake_case.push(whole);
                                        }
                                        #acc_snake_case
                                    })
                                },
                                None => None
                            },
                            (Some(_), None) => panic!("error 6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98"),
                            (None, Some(_)) => panic!("error a2761cd2-27ff-4db0-ae81-948aa04573a6"),
                            (None, None) => None,
                        }
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                    quote::quote! {
                                        if let Some(#value_snake_case) = #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case(
                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                            #create_snake_case.#field_ident
                                        ) {
                                            for #element_snake_case in #value_snake_case.clone() {
                                                #acc_snake_case.push(
                                                    #ident_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![#element_snake_case]
                                                        ).expect("error 479db858-6f36-48ba-9ab0-741b7df7956c")
                                                    )
                                                );
                                            }
                                            let whole = #ident_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                                #import_path::PostgresqlTypeWhere::try_new(
                                                    #import_path::LogicalOperator::And,
                                                    #value_snake_case
                                                ).expect("error beaeb784-58ea-4836-9c54-73924493bfb4")
                                            );
                                            if !#acc_snake_case.contains(&whole) {
                                                #acc_snake_case.push(whole);
                                            }
                                        }
                                    }
                                });
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    #(#content_token_stream)*
                                    #acc_is_empty_none_or_some_acc_token_stream
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_standart_not_null_upper_camel_case)
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                    quote::quote! {
                                        if let Some(#value_snake_case) = #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case(
                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident.clone(),
                                            #create_snake_case.#field_ident.clone()
                                        ) {
                                            for #element_snake_case in #value_snake_case.clone() {
                                                let handle = #ident_where_element_upper_camel_case::#element_field_ident_upper_camel_case(
                                                    #import_path::PostgresqlTypeWhere::try_new(
                                                        #import_path::LogicalOperator::And,
                                                        vec![#element_snake_case]
                                                    ).expect("error 1f7ae335-461f-4215-8fb5-ee7cf2f32881")
                                                );
                                                if !#acc_snake_case.contains(&handle) {
                                                    #acc_snake_case.push(handle);
                                                }
                                            }
                                            let whole = #ident_where_element_upper_camel_case::#element_field_ident_upper_camel_case(
                                                #import_path::PostgresqlTypeWhere::try_new(
                                                    #import_path::LogicalOperator::And,
                                                    #value_snake_case
                                                ).expect("error 79634838-847d-4eeb-b199-4927d57b2e2c")
                                            );
                                            if !#acc_snake_case.contains(&whole) {
                                                #acc_snake_case.push(whole);
                                            }
                                        }
                                    }
                                });
                                let maybe_dimension_one_token_stream = match &dimension {
                                    postgresql_crud_macros_common::Dimension::One => {
                                        let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #create_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote!{
                                            #acc_snake_case.push(#ident_where_element_upper_camel_case::DimensionOneEqual(#import_path::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                                                logical_operator: #import_path::LogicalOperator::And,
                                                dimensions: #import_path::BoundedStdVecVec::try_from(
                                                    vec![
                                                        #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                            std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")
                                                        ).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")
                                                    ]
                                                ).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                                                #value_snake_case: #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                    <#uuid_uuid_as_not_null_jsonb_string_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                        #read_only_ids_snake_case.0.#value_snake_case.#id_snake_case,
                                                        #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                    ),
                                                    #(#content_token_stream),*
                                                ),
                                            }));
                                        }
                                    },
                                    postgresql_crud_macros_common::Dimension::Two => proc_macro2::TokenStream::new(),
                                    postgresql_crud_macros_common::Dimension::Three => proc_macro2::TokenStream::new(),
                                    postgresql_crud_macros_common::Dimension::Four => proc_macro2::TokenStream::new(),
                                };
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    for (index, (#read_only_ids_snake_case, #create_snake_case)) in #read_only_ids_snake_case.0.#value_snake_case.into_iter()
                                        .zip(#create_snake_case.0.into_iter())
                                        .collect::<std::vec::Vec<(#ident_with_id_standart_not_null_read_only_ids_upper_camel_case, #ident_with_id_standart_not_null_create_upper_camel_case)>>()
                                        .into_iter()
                                        .enumerate()
                                    {
                                        #(#content_token_stream)*
                                        #maybe_dimension_one_token_stream
                                    }
                                    #acc_is_empty_none_or_some_acc_token_stream
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_array_not_null_upper_camel_case)
                        },
                    }
                };
                let generate_create_into_postgresql_json_type_option_vec_where_element_length_more_than_token_stream = ||{
                    let generate_nullable_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote! {
                        match #create_snake_case.0 {
                            Some(#create_snake_case) => match <
                                #content_token_stream
                                as
                                #import_path::PostgresqlJsonTypeTestCases
                            >::#create_into_postgresql_json_type_option_vec_where_element_length_more_than_snake_case(
                                #create_snake_case
                            ) {
                                Some(#value_snake_case) => if #value_snake_case.is_empty() {
                                    None
                                }
                                else {
                                    Some({
                                        let mut #acc_snake_case = vec![];
                                        for #element_snake_case in #value_snake_case.clone() {
                                            #acc_snake_case.push(
                                                #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                                    Some(
                                                        #import_path::NotEmptyUniqueEnumVec::try_new(
                                                            vec![#element_snake_case]
                                                        ).expect("error af515faf-8805-44c3-a5b6-f6e307b8b23a")
                                                    )
                                                )
                                            );
                                        }
                                        let whole = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                            Some(
                                                #import_path::NotEmptyUniqueEnumVec::try_new(
                                                    #value_snake_case
                                                ).expect("error 44d4e9b2-12e2-44c4-8c67-88cb7b8465ce")
                                            )
                                        );
                                        if !#acc_snake_case.contains(&whole) {
                                            #acc_snake_case.push(whole);
                                        }
                                        #acc_snake_case
                                    })
                                },
                                None => None
                            },
                            None => None,
                        }
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                    quote::quote! {
                                        if let Some(#value_snake_case) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_element_length_more_than_snake_case(
                                            #create_snake_case.#field_ident
                                        ) {
                                            for #element_snake_case in #value_snake_case.clone() {
                                                #acc_snake_case.push(
                                                    #ident_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![#element_snake_case]
                                                        ).expect("error 479db858-6f36-48ba-9ab0-741b7df7956c")
                                                    )
                                                );
                                            }
                                            let whole = #ident_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                                #import_path::PostgresqlTypeWhere::try_new(
                                                    #import_path::LogicalOperator::And,
                                                    #value_snake_case
                                                ).expect("error beaeb784-58ea-4836-9c54-73924493bfb4")
                                            );
                                            if !#acc_snake_case.contains(&whole) {
                                                #acc_snake_case.push(whole);
                                            }
                                        }
                                    }
                                });
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    #(#content_token_stream)*
                                    #acc_is_empty_none_or_some_acc_token_stream
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_standart_not_null_upper_camel_case)
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let element_field_ident_upper_camel_case = naming::parameter::ElementSelfUpperCamelCase::from_tokens(&field_ident);
                                    let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                    quote::quote! {
                                        for (index, #create_snake_case) in #create_snake_case.0.clone().into_iter().into_iter().enumerate() {
                                            if let Some(#value_snake_case) = #field_type_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_element_length_more_than_snake_case(
                                                #create_snake_case.#field_ident
                                            ) {
                                                for #element_snake_case in #value_snake_case.clone() {
                                                    #acc_snake_case.push(#ident_where_element_upper_camel_case::#element_field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::And,
                                                            vec![#element_snake_case]
                                                        )
                                                        .expect("error 955c6c27-863d-4b9b-9d88-e71f11161b3e"),
                                                    ));
                                                }
                                                let whole = #ident_where_element_upper_camel_case::#element_field_ident_upper_camel_case(
                                                    #import_path::PostgresqlTypeWhere::try_new(
                                                        #import_path::LogicalOperator::And,
                                                        #value_snake_case
                                                    )
                                                    .expect("error 6ee7a510-c8be-41f5-8fe5-505893eca3cc"),
                                                );
                                                if !#acc_snake_case.contains(&whole) {
                                                    #acc_snake_case.push(whole);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    #(#content_token_stream)*
                                    #acc_snake_case.push(#ident_where_element_upper_camel_case::LengthEqual(
                                        #import_path::PostgresqlJsonTypeWhereElementLengthEqual {
                                            logical_operator: #import_path::LogicalOperator::And,
                                            #value_snake_case: #import_path::UnsignedPartOfStdPrimitiveI32::try_from(
                                                std::primitive::i32::try_from(#create_snake_case.0.len()).expect("error 1fbbd897-2fae-4271-9fac-4b4007aecf32")
                                            ).expect("error 0eb5d915-bbbe-44c0-9096-d3d858d3a937"),
                                        }
                                    ));
                                    Some(#acc_snake_case)
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(&ident_array_not_null_upper_camel_case)
                        },
                    }
                };
                (
                    postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
                        &cfg_feature_test_utils,
                        &import_path,
                        &ident_read_inner_upper_camel_case,
                        &ident,
                        &{
                            let content_token_stream = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            let parameters_token_stream = vec_syn_field.iter().map(|element| {
                                                let current_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                                });
                                                if field_ident == current_field_ident {
                                                    quote::quote! {
                                                        #element_snake_case
                                                    }
                                                } else {
                                                    quote::quote! {
                                                        #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                    }
                                                }
                                            });
                                            quote::quote! {
                                                if let Some(#value_snake_case) = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                                    for #element_snake_case in #value_snake_case {
                                                        let #value_snake_case = #ident_standart_not_null_as_postgresql_json_type_create_token_stream::new(
                                                            #(#parameters_token_stream),*
                                                        );
                                                        if !#acc_snake_case.contains(&#value_snake_case) {
                                                            #acc_snake_case.push(#value_snake_case);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                        quote::quote! {#(#content_token_stream)*}
                                    },
                                    PostgresqlJsonObjectTypePattern::Array => {
                                        let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            #[derive(Clone)]
                                            enum ShouldAddDotClone {
                                                True,
                                                False
                                            }
                                            let generate_parameters_token_stream = |should_add_dot_clone: ShouldAddDotClone|{
                                                let mut acc = vec![];
                                                for element in get_vec_syn_field(&is_standart_with_id_false) {
                                                    let current_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                                    });
                                                    acc.push(if field_ident == current_field_ident {
                                                        let maybe_dot_clone_token_stream = match should_add_dot_clone.clone() {
                                                            ShouldAddDotClone::True => quote::quote!{.clone()},
                                                            ShouldAddDotClone::False => proc_macro2::TokenStream::new()
                                                        };
                                                        quote::quote! {
                                                            #element_snake_case #maybe_dot_clone_token_stream
                                                        }
                                                    } else {
                                                        quote::quote! {
                                                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                        }
                                                    });
                                                }
                                                acc
                                            };
                                            let option_additional_parameters_token_stream = generate_parameters_token_stream(ShouldAddDotClone::True);
                                            let parameters_token_stream = generate_parameters_token_stream(ShouldAddDotClone::False);
                                            quote::quote! {
                                                if let Some(vec_create) = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                                    let mut inner_acc = vec![];
                                                    let option_additional = {
                                                        let mut option_additional = None;
                                                        for #element_snake_case in &vec_create {
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
                                                    let has_len_more_than_one = vec_create.len() > 1;
                                                    for #element_snake_case in vec_create {
                                                        inner_acc.push(#ident_with_id_standart_not_null_create_upper_camel_case::new(
                                                            #(#parameters_token_stream),*
                                                        ));
                                                    }
                                                    {
                                                        let #value_snake_case = #ident_create_upper_camel_case::new(inner_acc);
                                                        if !#acc_snake_case.contains(&#value_snake_case) {
                                                            #acc_snake_case.push(#value_snake_case);
                                                        }
                                                    }
                                                    if let Some(#value_snake_case) = option_additional {
                                                        if has_len_more_than_one {
                                                            let #value_snake_case = #value_snake_case.0;
                                                            if !#acc_snake_case.contains(&#value_snake_case) {
                                                                #acc_snake_case.push(#value_snake_case);
                                                            }
                                                        }
                                                        if !has_len_more_than_one {
                                                            let #value_snake_case = #value_snake_case.1;
                                                            if !#acc_snake_case.contains(&#value_snake_case) {
                                                                #acc_snake_case.push(#value_snake_case);
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
                                        current_ident_not_null,
                                        content_token_stream
                                    ): (
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens,
                                        &dyn quote::ToTokens
                                    ) = match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => (
                                            &ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream,
                                            &ident_standart_nullable_upper_camel_case,
                                            &proc_macro2::TokenStream::new()
                                        ),
                                        PostgresqlJsonObjectTypePattern::Array => (
                                            &ident_array_not_null_as_postgresql_json_type_test_cases_token_stream,
                                            &ident_array_nullable_upper_camel_case,
                                            &quote::quote!{.0}
                                        ),
                                    };
                                    let current_ident_not_null_as_import_path_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&current_ident_not_null);
                                    quote::quote! {
                                        if let Some(#value_snake_case) = #current_ident_not_null_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case() {
                                            for #element_snake_case in #value_snake_case {
                                                let #value_snake_case = #current_ident_not_null_as_import_path_postgresql_json_type_token_stream::Create::new(Some(#element_snake_case #content_token_stream));
                                                if !#acc_snake_case.contains(&#value_snake_case) {
                                                    #acc_snake_case.push(#value_snake_case);
                                                }
                                            }
                                        }
                                        {
                                            let #value_snake_case = #current_ident_not_null_as_import_path_postgresql_json_type_token_stream::Create::new(None);
                                            if !#acc_snake_case.contains(&#value_snake_case) {
                                                #acc_snake_case.push(#value_snake_case);
                                            }
                                        }
                                    }
                                }
                            };
                            quote::quote!{Some({
                                let mut #acc_snake_case = vec![];
                                #content_token_stream
                                #acc_snake_case
                            })}
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_last_initialization_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&field_ident);
                                        let field_type_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        quote::quote! {
                                            let mut #field_ident_last_snake_case = #field_type_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                read_only_ids.0.value.#field_ident.clone()
                                            );
                                        }
                                    });
                                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_current_snake_case = naming::parameter::SelfCurrentSnakeCase::from_display(&field_ident);
                                        let field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&field_ident);
                                        let fields_token_stream = vec_syn_field.iter().map(|element| {
                                            let current_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let current_field_ident_current_snake_case = naming::parameter::SelfCurrentSnakeCase::from_display(&current_field_ident);
                                            let current_field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&current_field_ident);
                                            if field_ident == current_field_ident {
                                                quote::quote! {
                                                    #current_field_ident: #current_field_ident_current_snake_case.clone()
                                                }
                                            } else {
                                                quote::quote! {
                                                    #current_field_ident: #current_field_ident_last_snake_case.clone()
                                                }
                                            }
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element1});
                                        quote::quote! {
                                            for element0 in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#read_only_ids_snake_case.0.value.#field_ident) {
                                                for element1 in element0 {
                                                    let #field_ident_current_snake_case = Some(#value_content_token_stream);
                                                    #field_ident_last_snake_case = #field_ident_current_snake_case.clone();
                                                    #acc_snake_case.push(
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
                                    //just for the linter to not show me error variable not read
                                    let drop_fields_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_last_snake_case = naming::parameter::SelfLastSnakeCase::from_display(&field_ident);
                                        quote::quote! {drop(#field_ident_last_snake_case);}
                                    });
                                    quote::quote! {
                                        let mut #acc_snake_case = vec![];
                                        #(#fields_last_initialization_token_stream)*
                                        #(#content_token_stream)*
                                        #(#drop_fields_token_stream)*
                                        #acc_snake_case
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        let mut #acc_snake_case = vec![];
                                        if let Some(#value_snake_case) = &#read_only_ids_snake_case.0.#value_snake_case {
                                            for element0 in #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#value_snake_case) {
                                                for element1 in element0 {
                                                    #acc_snake_case.push(vec![Some(element1)]);
                                                }
                                            }
                                        }
                                        #acc_snake_case.push(vec![None]);
                                        #acc_snake_case
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let fields_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let current_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            if field_ident == current_field_ident {
                                                let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element2});
                                                quote::quote! {
                                                    #current_field_ident: Some(#value_content_token_stream)
                                                }
                                            } else {
                                                let current_field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                                quote::quote! {
                                                    #current_field_ident: #current_field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                        element0.0.#value_snake_case.#current_field_ident.clone()
                                                    )
                                                }
                                            }
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{element0.0.#value_snake_case.#id_snake_case.0.#value_snake_case.clone()});
                                        quote::quote! {
                                            for element1 in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(
                                                &element0.0.#value_snake_case.#field_ident.clone()
                                            ) {
                                                for element2 in element1 {
                                                    #acc_snake_case.push(
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
                                        #read_only_ids_snake_case.0.#value_snake_case.iter().map(|element0|{
                                            let mut #acc_snake_case = vec![];
                                            #(#content_token_stream)*
                                            #acc_snake_case
                                        })
                                        .collect()
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        let mut #acc_snake_case = vec![];
                                        if let Some(#value_snake_case) = &#read_only_ids_snake_case.0.#value_snake_case {
                                            for element0 in #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#value_snake_case) {
                                                for element1 in element0 {
                                                    #acc_snake_case.push(vec![Some(element1)]);
                                                }
                                            }
                                        }
                                        #acc_snake_case.push(vec![None]);
                                        #acc_snake_case
                                    }
                                }
                            },
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    let parameters_token_stream = vec_syn_field.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case.#value_snake_case)
                                        });
                                        quote::quote! {
                                            match #value_snake_case.#field_ident {
                                                Some(#value_snake_case) => Some(#value_content_token_stream),
                                                None => None
                                            }
                                        }
                                    });
                                    quote::quote! {#self_element_as_postgresql_type_read_token_stream::try_new(#(#parameters_token_stream),*).unwrap()}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_read_token_stream::new(match #value_snake_case {
                                            Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case)),
                                            None => None
                                        })
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case.#value_snake_case)
                                        });
                                        quote::quote! {
                                            #field_ident: match #element_snake_case.#field_ident {
                                                Some(#value_snake_case) => Some(#value_content_token_stream),
                                                None => None,
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        #ident_read_upper_camel_case::new({
                                            let mut #acc_snake_case = vec![];
                                            for #element_snake_case in #value_snake_case {
                                                #acc_snake_case.push(
                                                    #ident_with_id_standart_not_null_read_upper_camel_case {
                                                        #(#content_token_stream),*
                                                    }
                                                );
                                            }
                                            #acc_snake_case
                                        })
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case.#value_snake_case.clone())
                                        });
                                        quote::quote! {
                                            #field_ident: match &#element_snake_case.#field_ident {
                                                Some(#value_snake_case) => Some(#value_content_token_stream),
                                                None => None
                                            }
                                        }
                                    });
                                    let self_element_as_postgresql_type_read_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Read);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_read_token_stream::new(match #value_snake_case {
                                            Some(#value_snake_case) => Some(
                                                #value_snake_case.into_iter().map(|#element_snake_case|{
                                                    #ident_with_id_standart_not_null_read_upper_camel_case {
                                                        #(#content_token_stream),*
                                                    }
                                                }).collect()
                                            ),
                                            None => None,
                                        })
                                    }
                                }
                            },
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let self_element_as_postgresql_type_update_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Update);
                                    let parameters_token_stream = vec_syn_field.iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case.#value_snake_case)
                                        });
                                        quote::quote! {
                                            if let Some(#value_snake_case) = #value_snake_case.#field_ident {
                                                #acc_snake_case.push(#ident_standart_not_null_update_element_upper_camel_case::#field_ident_upper_camel_case(#value_content_token_stream));
                                            }
                                        }
                                    });
                                    quote::quote! {
                                        #self_element_as_postgresql_type_update_token_stream::new(
                                            #import_path::NotEmptyUniqueEnumVec::try_new({
                                                let mut #acc_snake_case = vec![];
                                                #(#parameters_token_stream)*
                                                #acc_snake_case
                                            }).unwrap()
                                        )
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let self_element_as_postgresql_type_update_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Update);
                                    let ident_standart_not_null_as_postgresql_type_test_cases_token_stream = generate_type_as_postgresql_type_test_cases_token_stream(&ident_standart_not_null_upper_camel_case);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_update_token_stream::new(match #value_snake_case {
                                            Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case)),
                                            None => None
                                        })
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        quote::quote! {#field_ident: element.#field_ident}
                                    });
                                    quote::quote! {
                                        #ident_update_upper_camel_case::try_new(
                                            vec![],
                                            #import_path_unique_vec_ident_with_id_standart_not_null_update_element_token_stream::try_new(
                                                #value_snake_case.into_iter().map(|#element_snake_case| #ident_with_id_standart_not_null_update_element_upper_camel_case {
                                                    #id_snake_case: #uuid_uuid_as_not_null_jsonb_string_update_upper_camel_case::new(#element_snake_case.#id_snake_case.clone().unwrap().#value_snake_case),
                                                    fields: #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(
                                                        #ident_standart_not_null_read_inner_upper_camel_case {
                                                            #(#fields_token_stream),*
                                                        }
                                                    ),
                                                })
                                                .collect(),
                                            )
                                            .expect("error ca51d559-d3d1-4855-8d9a-0f799cccd3e4"),
                                            vec![],
                                        )
                                        .unwrap()
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let self_element_as_postgresql_type_update_token_stream = generate_type_as_postgresql_type_subtype_token_stream(&self_postgresql_json_type_token_stream, &PostgresqlTypeSubtype::Update);
                                    let ident_with_id_array_not_null_as_postgresql_type_test_cases_token_stream = generate_type_as_postgresql_type_test_cases_token_stream(&ident_with_id_array_not_null_upper_camel_case);
                                    quote::quote! {
                                        #self_element_as_postgresql_type_update_token_stream::new(match #value_snake_case {
                                            Some(#value_snake_case) => Some(#ident_with_id_array_not_null_as_postgresql_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case)),
                                            None => None,
                                        })
                                    }
                                }
                            },
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_fields_read_only_ids_into_option_value_read_inner_token_stream(&is_standart_with_id_false, &value_snake_case),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        match #value_snake_case.0.#value_snake_case {
                                            Some(#value_snake_case) => match #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(#value_snake_case) {
                                                Some(#value_snake_case) => Some(#value_snake_case.#value_snake_case),
                                                None => None //none or struct where all fields are none
                                            },
                                            None => None //none or struct where all fields are none
                                        }
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        let content_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type = &element.ty;
                                            let field_type_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(&field_type);
                                            let field_type_as_postgresql_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(&field_type, &PostgresqlJsonTypeSubtype::Read);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&field_type);
                                            let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                                let default_but_option_is_always_some_call_token_stream = generate_default_but_option_is_always_some_call_token_stream(
                                                    &field_type_as_postgresql_json_type_read_token_stream
                                                );
                                                quote::quote!{#field_type_as_postgresql_json_type_token_stream::into_inner(#default_but_option_is_always_some_call_token_stream)}
                                            });
                                            quote::quote! {
                                                #field_ident: match #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                    #element_snake_case.0.#value_snake_case.#field_ident
                                                ) {
                                                    Some(#value_snake_case) => Some(#value_snake_case),
                                                    None => Some(#value_content_token_stream)
                                                }
                                            }
                                        });
                                        quote::quote!{
                                            #value_snake_case.0.#value_snake_case.into_iter().fold(vec![], |mut #acc_snake_case, #element_snake_case| {
                                                #acc_snake_case.push(#ident_with_id_standart_not_null_read_inner_upper_camel_case {
                                                    #(#content_token_stream),*
                                                });
                                                #acc_snake_case
                                            })
                                        }
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let ident_array_not_null_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_array_not_null_upper_camel_case);
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        match #value_snake_case.0.#value_snake_case {
                                            Some(#value_snake_case) => match #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_into_option_value_read_inner_snake_case(
                                                #value_snake_case
                                            ) {
                                                Some(#value_snake_case) => Some(#value_snake_case.#value_snake_case),
                                                None => None,
                                            },
                                            None => None,
                                        }
                                    });
                                    quote::quote! {Some(#value_content_token_stream)}
                                }
                            },
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let fields_initialization_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        quote::quote! {
                                            let mut #field_ident = None;
                                        }
                                    });
                                    let match_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        quote::quote! {
                                            #ident_update_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                                #field_ident = Some(#field_type_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#value_snake_case.#value_snake_case));
                                            }
                                        }
                                    });
                                    let struct_fields_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        quote::quote! {#field_ident: #field_ident.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5")}
                                    });
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        #ident_read_only_ids_handle_upper_camel_case{
                                            #(#struct_fields_content_token_stream),*
                                        }
                                    });
                                    quote::quote! {
                                        #(#fields_initialization_content_token_stream)*
                                        for #element_snake_case in #value_snake_case.0.to_vec() {
                                            match #element_snake_case {
                                                #(#match_content_token_stream),*
                                            }
                                        }
                                        #ident_read_only_ids_upper_camel_case(#value_content_token_stream)
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        quote::quote!{
                                            match &#value_snake_case.0 {
                                                Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#value_snake_case)),
                                                None => None
                                            }
                                        }
                                    });
                                    quote::quote! {
                                        #ident_read_only_ids_upper_camel_case(#value_content_token_stream)
                                    }
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let initialization_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        quote::quote! {let mut #field_ident = None;}
                                    });
                                    let for_loop_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let match_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let current_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let current_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&current_field_ident);
                                            if &field_ident == &current_field_ident {
                                                quote::quote!{
                                                    #ident_standart_not_null_update_element_upper_camel_case::#current_field_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                                        #field_ident = Some(#value_snake_case.#value_snake_case.clone());
                                                    }
                                                }
                                            }
                                            else {
                                                quote::quote!{
                                                    #ident_standart_not_null_update_element_upper_camel_case::#current_field_ident_upper_camel_case_token_stream(_) => ()
                                                }
                                            }
                                        });
                                        quote::quote! {
                                            for element1 in element.fields.0.to_vec() {
                                                match &element1 {
                                                    #(#match_content_token_stream),*
                                                }
                                            }
                                        }
                                    });
                                    let fields_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        quote::quote! {
                                            #field_ident: #field_type_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#field_ident.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d"))
                                        }
                                    });
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&{
                                        let uuid_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&uuid_uuid_as_not_null_jsonb_string_token_stream);
                                        let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                            #ident_with_id_standart_not_null_read_only_ids_handle_upper_camel_case {
                                                #id_snake_case: #uuid_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#element_snake_case.#id_snake_case),
                                                #(#fields_token_stream),*
                                            }
                                        });
                                        quote::quote!{
                                            #value_snake_case.#update_snake_case.to_vec().iter().map(|#element_snake_case|{
                                                #(#initialization_token_stream)*
                                                #(#for_loop_token_stream)*
                                                #ident_with_id_standart_not_null_read_only_ids_upper_camel_case(#value_content_token_stream)
                                            }).collect()
                                        }
                                    });
                                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_content_token_stream)}
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let ident_array_not_null_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_array_not_null_upper_camel_case);
                                    let value_content_token_stream = wrap_into_value_initialization_token_stream(&quote::quote!{
                                        match &#value_snake_case.0 {
                                            Some(#value_snake_case) => Some(#ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(&#value_snake_case)),
                                            None => None,
                                        }
                                    });
                                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_content_token_stream)}
                                }
                            },
                        },
                        &{
                            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                    &#value_snake_case.0.#value_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::try_new(
                                                #(#parameters_token_stream),*
                                            ).expect("error 57820868-38ac-4f77-aa0f-dc734399d8b2")
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                        #ident_read_upper_camel_case::new(
                                            match &#value_snake_case.0.#value_snake_case {
                                                Some(#value_snake_case) => match #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(#value_snake_case) {
                                                    Some(#value_snake_case) => Some(#value_snake_case.#value_snake_case),
                                                    None => None,
                                                }
                                                None => None
                                            }
                                        )
                                    }
                                },
                                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_true).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            quote::quote! {
                                                #field_type_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                    &#element_snake_case.0.#value_snake_case.#field_ident
                                                )
                                            }
                                        });
                                        let import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream = generate_type_as_postgresql_json_type_token_stream(
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream
                                        );
                                        quote::quote! {
                                            #ident_read_upper_camel_case::new({
                                                let mut #acc_snake_case = #value_snake_case.0.#value_snake_case.clone().into_iter().map(|#element_snake_case|{
                                                    #ident_with_id_standart_not_null_read_upper_camel_case::try_new(
                                                        #(#parameters_token_stream),*
                                                    ).expect("error 8f6fb6b6-6e84-4819-b9c6-526d39e1ac88")
                                                }).collect::<std::vec::Vec<#ident_with_id_standart_not_null_read_upper_camel_case>>();
                                                #acc_snake_case.sort_by(|a, b| {
                                                    if let (Some(value_a), Some(value_b)) = (&a.id, &b.id) {
                                                        //maybe remove .clone - add .get by ref method
                                                        #import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::into_inner(
                                                            value_a.#value_snake_case.clone()
                                                        )
                                                        .cmp(&#import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_token_stream::into_inner(
                                                            value_b.#value_snake_case.clone()
                                                        ))
                                                    }
                                                    else {
                                                        panic!("error 0bdf0f44-0012-4cda-9a27-3a89804d871b");
                                                    }
                                                });
                                                #acc_snake_case
                                            })
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                        #ident_read_upper_camel_case::new(
                                            match &#value_snake_case.0.#value_snake_case {
                                                Some(#value_snake_case) => match #ident_array_not_null_as_postgresql_json_type_test_cases_token_stream::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                    #value_snake_case
                                                ) {
                                                    Some(#value_snake_case) => Some(#value_snake_case.#value_snake_case.0),
                                                    None => None,
                                                },
                                                None => None
                                            }
                                        )
                                    }
                                },
                            });
                            quote::quote!{Some(#value_initialization_token_stream)}
                        },
                        &{
                            let fields_initialization_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                quote::quote! {
                                    let mut #field_ident = None;
                                }
                            });
                            let match_content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                quote::quote! {
                                    #ident_standart_not_null_update_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(#value_snake_case) => {
                                        #field_ident = Some(#value_snake_case.#value_snake_case);
                                    }
                                }
                            });
                            let generate_struct_initialization_token_stream = |function: &dyn Fn(&dyn quote::ToTokens) -> proc_macro2::TokenStream|{//content_token_stream: &dyn quote::ToTokens
                                let token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                                        let content_token_stream = function(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
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
                            let generate_option_token_stream = |postgresql_json_object_type_pattern: &PostgresqlJsonObjectTypePattern|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                                    PostgresqlJsonObjectTypePattern::Array => &ident_array_not_null_upper_camel_case
                                });
                                quote::quote! {
                                    match #option_update_snake_case {
                                        Some(#value_snake_case) => #ident_read_upper_camel_case(
                                            match #value_snake_case.0 {
                                                Some(#value_snake_case) => Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::previous_read_merged_with_option_update_into_read(
                                                        match #read_snake_case.0 {
                                                            Some(#value_snake_case) => #value_snake_case,
                                                            None => #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                                        },
                                                        Some(#value_snake_case),
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
                                                #read_snake_case.#content_token_stream.expect("error 2e8229f7-38d6-48c1-93c9-e77631a3e155").#value_snake_case
                                            }
                                        });
                                        quote::quote!{
                                            match #option_update_snake_case {
                                                Some(#value_snake_case) => {
                                                    #(#fields_initialization_content_token_stream)*
                                                    for #element_snake_case in #value_snake_case.0.into_vec() {
                                                        match #element_snake_case {
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
                                                found_read_element.#content_token_stream.expect("error 2e8229f7-38d6-48c1-93c9-e77631a3e155").#value_snake_case
                                            }
                                        });
                                        quote::quote! {
                                            match #option_update_snake_case {
                                                Some(#value_snake_case) => #ident_read_upper_camel_case({
                                                    let mut #acc_snake_case = vec![];
                                                    for update_element in #value_snake_case.#update_snake_case.into_vec() {
                                                        let mut option_read_element = None;
                                                        for read_element in &#read_snake_case.0 {
                                                            if 
                                                                *#uuid_uuid_as_not_null_jsonb_string_as_postgresql_json_type_object_vec_element_id_token_stream::get_inner(&update_element.#id_snake_case.clone().into())
                                                                ==
                                                                #uuid_uuid_as_not_null_jsonb_string_as_import_path_postgresql_json_type_token_stream::into_inner(
                                                                    read_element.#id_snake_case.clone().expect("error df2413fe-e703-451b-ab75-add67da716f7").#value_snake_case
                                                                )
                                                            {
                                                                option_read_element = Some(read_element.clone());
                                                                break;
                                                            }
                                                        }
                                                        let found_read_element = option_read_element.expect("error 139882b9-d38f-4cb5-98ea-af0ab23ec474");
                                                        #(#fields_initialization_content_token_stream)*
                                                        for #element_snake_case in update_element.fields.0.into_vec() {
                                                            match #element_snake_case {
                                                                #(#match_content_token_stream),*
                                                            }
                                                        }
                                                        #acc_snake_case.push(#ident_with_id_standart_not_null_read_upper_camel_case {
                                                            #id_snake_case: found_read_element.#id_snake_case,
                                                            #struct_initializattion_token_stream
                                                        });
                                                    }
                                                    #acc_snake_case
                                                }),
                                                None => #read_snake_case
                                            }
                                        }
                                    },
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_option_token_stream(&postgresql_json_object_type_pattern)
                            }
                        },
                        &{
                            let generate_nullable_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_token_stream);
                                quote::quote! {
                                    #ident_read_upper_camel_case::new(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => {
                                                Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                                                        #read_only_ids_snake_case,
                                                        #create_snake_case
                                                    ).expect("error 56ac4450-0feb-4ea7-aca7-6f51c8f4893c").#value_snake_case #content_token_stream
                                                )
                                            },
                                            (Some(_), None) => panic!("error 75be9ae0-ca9f-4251-bfff-2156a90b10c6"),
                                            (None, Some(_)) => panic!("error 6a95d7ae-54f5-4e04-9217-223ba156b799"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
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
                                            ).expect("error 52ad3994-8392-4fc5-8427-4ca42d87d726")
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
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #content_token_stream,
                                                )
                                            }
                                        };
                                        let id_parameter_token_stream = generate_parameter_token_stream(
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &id_snake_case,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element|{
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            generate_parameter_token_stream(
                                                &element.ty,
                                                &field_ident,
                                                &quote::quote! {#create_snake_case.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_read_upper_camel_case::new({
                                                assert_eq!(#read_only_ids_snake_case.0.#value_snake_case.len(), #create_snake_case.0.len(), "error 90d33ddd-e08d-488c-8577-c75febe97301");
                                                let mut #acc_snake_case = vec![];
                                                for (#read_only_ids_snake_case, #create_snake_case) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()).collect::<std::vec::Vec<(
                                                    #ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                                                    #ident_with_id_standart_not_null_create_upper_camel_case,
                                                )>>() {
                                                    #acc_snake_case.push(#ident_with_id_standart_not_null_read_upper_camel_case::try_new(
                                                        #id_parameter_token_stream,
                                                        #(#parameters_token_stream),*
                                                    ).expect("error 1330ac8d-ebf2-4c79-b25e-6394d58de927"));
                                                }
                                                #acc_snake_case
                                            })
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_array_not_null_upper_camel_case,
                                        &quote::quote!{.0}
                                    )
                                },
                            }
                        },
                        &{
                            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote!{
                                <#ident as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_snake_case(
                                    #read_only_ids_snake_case,
                                    #create_snake_case
                                )
                            });
                            quote::quote!{Some(#value_initialization_token_stream)}
                        },
                        &{
                            let generate_nullable_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                                let current_ident_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&ident_token_stream);
                                quote::quote! {
                                    #ident_table_type_declaration_upper_camel_case::new(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => {
                                                Some(
                                                    #current_ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                        #read_only_ids_snake_case,
                                                        #create_snake_case
                                                    ) #content_token_stream
                                                )
                                            },
                                            (Some(_), None) => panic!("error 9349dcd5-3ed3-4157-b1ef-14c51d55262f"),
                                            (None, Some(_)) => panic!("error 45f8e70a-ffca-41b6-ac70-96f101ac3c80"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            };
                            match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
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
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #content_token_stream,
                                                )
                                            }
                                        };
                                        let id_parameter_token_stream = generate_parameter_token_stream(
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &id_snake_case,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element|{
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            generate_parameter_token_stream(
                                                &element.ty,
                                                &field_ident,
                                                &quote::quote! {#create_snake_case.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_table_type_declaration_upper_camel_case::new({
                                                assert_eq!(#read_only_ids_snake_case.0.#value_snake_case.len(), #create_snake_case.0.len(), "error 7776a146-06a8-4972-8e16-371d41ee164c");
                                                let mut #acc_snake_case = vec![];
                                                for (#read_only_ids_snake_case, #create_snake_case) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()).collect::<std::vec::Vec<(
                                                    #ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                                                    #ident_with_id_standart_not_null_create_upper_camel_case,
                                                )>>() {
                                                    #acc_snake_case.push(#ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                        #id_parameter_token_stream,
                                                        #(#parameters_token_stream),*
                                                    ));
                                                }
                                                #acc_snake_case
                                            })
                                        }
                                    }
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_nullable_token_stream(
                                        &ident_array_not_null_upper_camel_case,
                                        &quote::quote!{.0}
                                    )
                                },
                            }
                        },
                        &match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                PostgresqlJsonObjectTypePattern::Standart => {
                                    let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        quote::quote! {
                                            #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                                #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                #create_snake_case.#field_ident
                                            )
                                        }
                                    });
                                    quote::quote! {
                                        #ident_where_element_upper_camel_case::#equal_upper_camel_case(
                                            #import_path::PostgresqlJsonTypeWhereElementEqual {
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
                                                #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                #content_token_stream
                                            )
                                        }
                                    };
                                    let ident_token_stream = generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                        &id_snake_case,
                                        &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                        &import_path_default_but_option_is_always_some_call_token_stream
                                    );
                                    let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                            &field_ident,
                                            &element.ty,
                                            &quote::quote!{#create_snake_case.#field_ident}
                                        )
                                    });
                                    quote::quote! {
                                        #ident_where_element_upper_camel_case::#equal_upper_camel_case(
                                            #import_path::PostgresqlJsonTypeWhereElementEqual {
                                                logical_operator: #import_path::LogicalOperator::And,
                                                #value_snake_case: #ident_table_type_declaration_upper_camel_case::new({
                                                    let mut #acc_snake_case = vec![];
                                                    for (#read_only_ids_snake_case, #create_snake_case) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()).collect::<std::vec::Vec<(
                                                        #ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                                                        #ident_with_id_standart_not_null_create_upper_camel_case,
                                                    )>>() {
                                                        #acc_snake_case.push(
                                                            #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                                #ident_token_stream,
                                                                #(#parameters_token_stream),*
                                                            )
                                                        );
                                                    }
                                                    #acc_snake_case
                                                })
                                            }
                                        )
                                    }
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let content_token_stream = {
                                    let ident_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&generate_ident_upper_camel_case(&match &postgresql_json_object_type_pattern {
                                        PostgresqlJsonObjectTypePattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                        PostgresqlJsonObjectTypePattern::Array => IdentPattern::ArrayNotNullWithId,
                                    }));
                                    quote::quote!{
                                        vec![
                                            #ident_token_stream::#read_only_ids_merged_with_create_into_where_element_equal_snake_case(
                                                #read_only_ids_snake_case,
                                                #create_snake_case
                                            )
                                        ]
                                    }
                                };
                                quote::quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                        match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                            (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => Some(#import_path::NotEmptyUniqueEnumVec::try_new(#content_token_stream).expect("error 9f550fbd-2d60-4a8a-a67b-ab49f728c9d0")),
                                            (Some(_), None) => panic!("error 49e4c289-b37d-4365-96e3-5d896d6860f7"),
                                            (None, Some(_)) => panic!("error ad71caa2-2503-4f9a-952c-e796abf5bbbe"),
                                            (None, None) => None,
                                        }
                                    )
                                }
                            },
                        },
                        &{
                            let content_token_stream = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                                    PostgresqlJsonObjectTypePattern::Standart => {
                                        let elements_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                            let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                            quote::quote! {
                                                #ident_where_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(
                                                    #import_path::PostgresqlTypeWhere::try_new(
                                                        #import_path::LogicalOperator::And,
                                                        #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case(
                                                            #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                            #create_snake_case.#field_ident
                                                        )
                                                    )
                                                    .expect("error edacf099-3f54-41ab-980d-e1d8760e216f")
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
                                                    #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                    #content_token_stream
                                                )
                                            }
                                        };
                                        let ident_token_stream = generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                            &id_snake_case,
                                            &uuid_uuid_as_not_null_jsonb_string_token_stream,
                                            &import_path_default_but_option_is_always_some_call_token_stream
                                        );
                                        let parameters_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                            generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
                                                &field_ident,
                                                &element.ty,
                                                &quote::quote!{#create_snake_case.#field_ident}
                                            )
                                        });
                                        quote::quote! {
                                            #ident_where_element_upper_camel_case::#equal_upper_camel_case(
                                                #import_path::PostgresqlJsonTypeWhereElementEqual {
                                                    logical_operator: #import_path::LogicalOperator::And,
                                                    #value_snake_case: #ident_table_type_declaration_upper_camel_case::new({
                                                        let mut #acc_snake_case = vec![];
                                                        for (#read_only_ids_snake_case, #create_snake_case) in #read_only_ids_snake_case.0.#value_snake_case.into_iter().zip(#create_snake_case.0.into_iter()).collect::<std::vec::Vec<(
                                                            #ident_with_id_standart_not_null_read_only_ids_upper_camel_case,
                                                            #ident_with_id_standart_not_null_create_upper_camel_case,
                                                        )>>() {
                                                            #acc_snake_case.push(
                                                                #ident_with_id_standart_not_null_table_type_declaration_upper_camel_case::new(
                                                                    #ident_token_stream,
                                                                    #(#parameters_token_stream),*
                                                                )
                                                            );
                                                        }
                                                        #acc_snake_case
                                                    })
                                                }
                                            )
                                        }
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = {
                                        let ident_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&generate_ident_upper_camel_case(&match &postgresql_json_object_type_pattern {
                                            PostgresqlJsonObjectTypePattern::Standart => IdentPattern::StandartNotNullWithoutId,
                                            PostgresqlJsonObjectTypePattern::Array => IdentPattern::ArrayNotNullWithId,
                                        }));
                                        quote::quote! {
                                            #ident_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case(
                                                #read_only_ids_snake_case,
                                                #create_snake_case
                                            )
                                        }
                                    };
                                    quote::quote! {
                                        #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                            match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                                (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => Some(#import_path::NotEmptyUniqueEnumVec::try_new(#content_token_stream).expect("error 9f550fbd-2d60-4a8a-a67b-ab49f728c9d0")),
                                                (Some(_), None) => panic!("error 49e4c289-b37d-4365-96e3-5d896d6860f7"),
                                                (None, Some(_)) => panic!("error ad71caa2-2503-4f9a-952c-e796abf5bbbe"),
                                                (None, None) => None,
                                            }
                                        )
                                    }
                                },
                            };
                            quote::quote!{vec![#content_token_stream]}
                        },
                        &match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let content_token_stream = get_vec_syn_field(&is_standart_with_id_false).iter().map(|element| {
                                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        });
                                        let field_ident_upper_camel_case = &naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                        let field_type_as_postgresql_json_type_test_cases_token_stream = generate_type_as_postgresql_json_type_test_cases_token_stream(&element.ty);
                                        quote::quote! {
                                            for #element_snake_case in #field_type_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_snake_case(
                                                #read_only_ids_snake_case.0.#value_snake_case.#field_ident,
                                                #create_snake_case.#field_ident
                                            ) {
                                                #acc_snake_case.push(
                                                    #ident_where_element_upper_camel_case::#field_ident_upper_camel_case(
                                                        #import_path::PostgresqlTypeWhere::try_new(
                                                            #import_path::LogicalOperator::Or,
                                                            vec![#element_snake_case],
                                                        )
                                                        .expect("error 0c6ccad1-6ffc-451f-9b16-0731010fee9f"),
                                                    )
                                                );
                                            }
                                        }
                                    });
                                    quote::quote!{
                                        let mut #acc_snake_case = vec![];
                                        #(#content_token_stream)*
                                        #acc_snake_case
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                    let mut #acc_snake_case = vec![];
                                    match (#read_only_ids_snake_case.0.#value_snake_case, #create_snake_case.0) {
                                        (Some(#read_only_ids_snake_case), Some(#create_snake_case)) => {
                                            for #element_snake_case in #ident_standart_not_null_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_snake_case(
                                                #read_only_ids_snake_case,
                                                #create_snake_case
                                            ) {
                                                #acc_snake_case.push(
                                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                                                        #import_path::NotEmptyUniqueEnumVec::try_new(vec![#element_snake_case])
                                                        .expect("error 130eaf97-dd2b-440f-be51-8b0e9ec2d18d")
                                                    ))
                                                );
                                            }
                                        },
                                        (Some(_), None) => panic!("error b4507b4c-5282-4d91-9a50-190b2d789849"),
                                        (None, Some(_)) => panic!("error 8f458c1d-a286-404f-b3b7-cd8f7b4c8bed"),
                                        (None, None) => {
                                            #acc_snake_case.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(None));
                                        },
                                    }
                                    #acc_snake_case
                                }
                            },
                            PostgresqlJsonObjectTypePattern::Array => quote::quote!{
                                #ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case(
                                    #read_only_ids_snake_case,
                                    #create_snake_case
                                )
                            }
                        },
                        &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::One),
                        &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Two),
                        &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Three),
                        &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Four),
                        &quote::quote!{todo!()},
                        &generate_create_into_postgresql_json_type_option_vec_where_element_length_more_than_token_stream()
                    ),
                    {
                        let generate_dimension_equal_token_stream = |dimension: &postgresql_crud_macros_common::Dimension|{
                            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case();
                            quote::quote!{#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_number_equal_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )}
                        };
                        postgresql_crud_macros_common::generate_impl_postgresql_type_test_cases_for_ident_token_stream(
                            &cfg_feature_test_utils,
                            &import_path,
                            &ident_read_inner_upper_camel_case,
                            &ident,
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#option_vec_create_snake_case()},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#read_only_ids_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_read_with_new_or_try_new_unwraped_snake_case(#value_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#update_to_read_only_ids_snake_case(#value_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case(#value_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#previous_read_merged_with_option_update_into_read_snake_case(#read_snake_case, #option_update_snake_case)},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_read_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_option_value_read_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_where_element_equal_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )},
                            &quote::quote! {#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            )},
                            &quote::quote!{Some(#ident_as_postgresql_json_type_test_cases_token_stream::#read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field_snake_case(
                                #read_only_ids_snake_case,
                                #create_snake_case
                            ))},
                            &quote::quote!{None},
                            &quote::quote!{vec![]},
                            &quote::quote!{None},
                            &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::One),
                            &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Two),
                            &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Three),
                            &generate_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Four),
                            &quote::quote!{todo!()},
                            &quote::quote!{#ident_as_postgresql_json_type_test_cases_token_stream::#create_into_postgresql_json_type_option_vec_where_element_length_more_than_snake_case(
                                #create_snake_case
                            )},
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
                #ident_where_element_token_stream
                #ident_read_token_stream
                #ident_read_only_ids_token_stream
                #ident_read_inner_token_stream
                #ident_update_token_stream
                #ident_update_for_query_token_stream
                #maybe_impl_postgresql_crud_postgresql_json_type_for_ident_token_stream
                #maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
                #impl_postgresql_json_type_test_cases_for_ident_token_stream
                #impl_postgresql_type_test_cases_for_ident_token_stream
                #impl_postgresql_type_not_primary_key_for_ident_token_stream
            };
            // if let (
            //     postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            //     // postgresql_crud_macros_common::NotNullOrNullable::Nullable,

            //     // PostgresqlJsonObjectTypePattern::Standart,
            //     PostgresqlJsonObjectTypePattern::Array,

            //     TraitGen::PostgresqlJsonType,
            //     // TraitGen::PostgresqlTypeAndPostgresqlJsonType,
            // ) = (
            //     &not_null_or_nullable,
            //     &postgresql_json_object_type_pattern,
            //     &trait_gen,
            // ) {
            //     if syn_derive_input_ident == "Doggie" {//"Animal" // "Doggie"
            //         macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //             "GeneratePostgresqlJsonObjectType",
            //             &generated,
            //             &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
            //         );
            //     }
            // }
            (
                {
                    let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().unwrap();
                    quote::quote! {
                        pub #field_ident: #ident,
                    }
                },
                generated,
            )
        })
        .collect::<(std::vec::Vec<proc_macro2::TokenStream>, std::vec::Vec<proc_macro2::TokenStream>)>();
    if false {
        macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            "GeneratePostgresqlJsonObjectTypeExample",
            &quote::quote! {
                pub struct GeneratePostgresqlJsonObjectTypeExample {
                    #(#fields_token_stream)*
                }
            },
            &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
        );
    }
    let generated: proc_macro2::TokenStream = quote::quote! {#(#postgresql_json_object_type_array)*};
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlJsonObjectType",
    //     &generated,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    generated.into()
}