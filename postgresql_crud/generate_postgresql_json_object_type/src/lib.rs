//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
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
    let syn_derive_input: syn::DeriveInput = syn::parse(input_token_stream.clone()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let postgresql_json_object_type_record_vec = {
        if false {
            PostgresqlJsonObjectTypeRecord::all()
        } else {
            let postgresql_json_object_type_record = serde_json::from_str::<PostgresqlJsonObjectTypeRecord>(
                &macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
                    &{
                        let syn_derive_input: syn::DeriveInput = syn::parse(input_token_stream).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
                        syn_derive_input.attrs
                    },
                    &"postgresql_crud::postgresql_json_object_type_pattern".to_string(),
                )
                .to_string()
            ).expect("failed to get Config for generate_postgresql_json_object_type");
            match (&postgresql_json_object_type_record.not_null_or_nullable, &postgresql_json_object_type_record.postgresql_json_object_type_pattern) {
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
                    PostgresqlJsonObjectTypeRecord {
                        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::Nullable,
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
    let postgresql_json_object_type_array = postgresql_json_object_type_record_vec
    .into_iter()
    .map(|element|{
        let not_null_or_nullable = &element.not_null_or_nullable;
        let postgresql_json_object_type_pattern = &element.postgresql_json_object_type_pattern;
        let trait_gen = &element.trait_gen;

        let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrud;

        let create_snake_case = naming::CreateSnakeCase;
        let update_snake_case = naming::UpdateSnakeCase;
        let delete_snake_case = naming::DeleteSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let as_upper_camel_case = naming::AsUpperCamelCase;
        let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
        let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
        let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let increments_snake_case = naming::IncrementsSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let field_ident_snake_case = naming::FieldIdentSnakeCase;
        let id_snake_case = naming::IdSnakeCase;
        let fields_snake_case = naming::FieldsSnakeCase;
        let self_upper_camel_case = naming::SelfUpperCamelCase;
        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
        let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
        let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
        let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
        let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
        let reference_std_primitive_str_token_stream = token_patterns::RefStdPrimitiveStr;
        let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
        let column_name_and_maybe_field_getter_for_error_message_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;

        let std_string_string_token_stream = token_patterns::StdStringString;
        let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;

        let query_postgres_arguments_token_stream = quote::quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
        let reference_mut_std_primitive_u64_token_stream = {
            let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
            quote::quote! {&mut #std_primitive_u64_token_stream}
        };
        let import_path_query_part_error_named_token_stream = {
            let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
            quote::quote!{#import_path::#query_part_error_named_upper_camel_case}
        };
        let import_path_query_part_error_named_checked_add_initialization_token_stream = quote::quote!{
            #import_path_query_part_error_named_token_stream::#checked_add_upper_camel_case {
                code_occurence: error_occurence_lib::code_occurence!()
            }
        };

        // let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        // let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

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
        let postgresql_json_type_snake_case = naming::PostgresqlJsonTypeSnakeCase;
        let uuid_uuid_as_not_null_jsonb_string_upper_camel_case = naming::UuidUuidAsNotNullJsonbStringUpperCamelCase;
        let import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_token_stream = quote::quote!{#import_path::#postgresql_json_type_snake_case::#uuid_uuid_as_not_null_jsonb_string_upper_camel_case};
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
                    segments: macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[import_path.to_path(), &postgresql_json_type_snake_case.to_string(), &uuid_uuid_as_not_null_jsonb_string_upper_camel_case.to_string()]),
                },
            }),
        };
        let vec_syn_field_with_id = {
            let mut acc = vec![&id_syn_field];
            for element in &vec_syn_field {
                acc.push(element);
            }
            acc
        };
        #[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum IsStandartWithId {
            False,
            True
        }
        let get_vec_syn_field = |is_standart_with_id: &IsStandartWithId| -> &std::vec::Vec<&syn::Field>{
            match &is_standart_with_id {
                IsStandartWithId::False => &vec_syn_field,
                IsStandartWithId::True => &vec_syn_field_with_id
            }
        };
        enum IdentPattern {
            NotNullStandartWithoutId,
            NotNullStandartWithId,
            NullableStandartWithoutId,
            NotNullArrayWithId,
            NullableArrayWithId,
        }
        let generate_ident_upper_camel_case = |ident_pattern: &IdentPattern|{
            let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
            let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
            let jsonb_object_upper_camel_case = naming::JsonbObjectUpperCamelCase;
            let with_id_upper_camel_case = naming::WithIdUpperCamelCase;
            let syn_derive_input_ident_stringified = syn_derive_input_ident.to_string();
            let jsonb_object_upper_camel_case_stringified = jsonb_object_upper_camel_case.to_string();
            let vec_of_syn_derive_input_ident_with_id = format!("{vec_of_upper_camel_case}{syn_derive_input_ident}{with_id_upper_camel_case}");
            let array_of_not_null_jsonb_object_with_id = format!(
                "{array_of_upper_camel_case}{}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}",
                postgresql_crud_macros_common::NotNullOrNullable::NotNull
            );
            let (rust_part, postgresql_part, current_not_null_or_nullable) = match &ident_pattern {
                IdentPattern::NotNullStandartWithoutId => {
                    (
                        syn_derive_input_ident_stringified,
                        jsonb_object_upper_camel_case_stringified,
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    )
                },
                IdentPattern::NotNullStandartWithId => {
                    (
                        format!("{syn_derive_input_ident}{with_id_upper_camel_case}"),
                        format!("{jsonb_object_upper_camel_case}{with_id_upper_camel_case}"),
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull
                    )
                },
                IdentPattern::NullableStandartWithoutId => {
                    (
                        syn_derive_input_ident_stringified,
                        jsonb_object_upper_camel_case_stringified,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable
                    )
                },
                IdentPattern::NotNullArrayWithId => {
                    (
                        vec_of_syn_derive_input_ident_with_id,
                        array_of_not_null_jsonb_object_with_id,
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull
                    )
                },
                IdentPattern::NullableArrayWithId => {
                    (
                        vec_of_syn_derive_input_ident_with_id,
                        array_of_not_null_jsonb_object_with_id,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable
                    )
                },
            };
            let current_not_null_or_nullable_rust = current_not_null_or_nullable.rust();
            format!("{current_not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}")
            .parse::<proc_macro2::TokenStream>().unwrap()
        };

        let ident = &generate_ident_upper_camel_case(
            &match (&not_null_or_nullable, &postgresql_json_object_type_pattern) {
                (
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    PostgresqlJsonObjectTypePattern::Standart
                ) => IdentPattern::NotNullStandartWithoutId,
                (
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    PostgresqlJsonObjectTypePattern::Array
                ) => IdentPattern::NotNullArrayWithId,
                (
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable,
                    PostgresqlJsonObjectTypePattern::Standart
                ) => IdentPattern::NullableStandartWithoutId,
                (
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable,
                    PostgresqlJsonObjectTypePattern::Array
                ) => IdentPattern::NullableArrayWithId,
            }
        );
        let ident_standart_not_null_upper_camel_case = &generate_ident_upper_camel_case(
            &IdentPattern::NotNullStandartWithoutId
        );
        let ident_with_id_standart_not_null_upper_camel_case = &generate_ident_upper_camel_case(
            &IdentPattern::NotNullStandartWithId
        );
        let ident_with_id_array_not_null_upper_camel_case = &generate_ident_upper_camel_case(
            &IdentPattern::NotNullArrayWithId
        );
        let is_standart_not_null = matches!(
            (&not_null_or_nullable, postgresql_json_object_type_pattern),
            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonObjectTypePattern::Standart)
        );
        let ident_token_stream = {
            let generate_struct_ident_token_stream = |ident: &dyn quote::ToTokens|{
                quote::quote! {
                    #[derive(Debug)]
                    pub struct #ident;
                }
            };
            let ident_token_stream = generate_struct_ident_token_stream(&ident);
            let maybe_ident_with_id_standart_not_null_token_stream = if is_standart_not_null {
                generate_struct_ident_token_stream(&&ident_with_id_standart_not_null_upper_camel_case)
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_token_stream
                #maybe_ident_with_id_standart_not_null_token_stream
            }
        };
        #[derive(Debug, strum_macros::Display)]
        enum PostgresqlJsonTypeSubtype {
            TableTypeDeclaration,
            Create,
            Select,
            WhereElement,
            Read,
            Update
        }
        impl quote::ToTokens for PostgresqlJsonTypeSubtype {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .unwrap()
                .to_tokens(tokens);
            }
        }
        let generate_type_as_postgresql_json_type_subtype_token_stream = |
            type_token_stream: &dyn quote::ToTokens,
            postgresql_json_type_subtype: &PostgresqlJsonTypeSubtype
        |{
            quote::quote! {<#type_token_stream as #import_path::PostgresqlJsonType>::#postgresql_json_type_subtype}
        };
        let generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
            let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
            quote::quote! {<#value_token_stream as #import_path::#postgresql_json_type_upper_camel_case>::}
        };
        let generate_field_type_as_crud_postgresql_json_type_from_field_token_stream = |field: &syn::Field| generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream(&field.ty);
        let generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
            macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                &proc_macro2::TokenStream::new(),
                &ident_token_stream,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self:?}")}
            )
        };
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
        let self_value_token_stream = quote::quote!{Self(#value_snake_case)};

        let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
        let ident_with_id_table_type_declaration_standart_not_null_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
        let ident_with_id_create_standart_not_null_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let generate_ident_table_type_declaration_or_ident_create_common_token_stream = |postgresql_json_type_subtype_table_type_declaration_or_create: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate|{
            let ident_table_type_declaration_or_ident_create_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_table_type_declaration_upper_camel_case,
                PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_create_upper_camel_case,
            };
            let generate_ident_table_type_declaration_or_create_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                    pub struct #ident_token_stream #content_token_stream
                }
            };
            enum StructDeclarationOrNewType {
                StructDeclaration,
                NewType
            }
            let generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream = |
                is_standart_with_id: &IsStandartWithId,
                postgresql_json_type_subtype_table_type_declaration_or_create: &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate,
                struct_declaration_or_new_type: &StructDeclarationOrNewType,
            |{
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let type_as_postgresql_json_type_subtype_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &element.ty,
                        &PostgresqlJsonTypeSubtype::from(postgresql_json_type_subtype_table_type_declaration_or_create)
                    );
                    quote::quote! {
                        #field_ident: #type_as_postgresql_json_type_subtype_table_type_declaration_token_stream
                    }
                });
                let fields_content_token_stream = quote::quote!{#(#content_token_stream),*};
                match &struct_declaration_or_new_type {
                    StructDeclarationOrNewType::StructDeclaration => quote::quote!{{#fields_content_token_stream}},
                    StructDeclarationOrNewType::NewType => fields_content_token_stream
                }
            };
            let generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream = |tokens: &dyn quote::ToTokens|{
                generate_type_as_postgresql_json_type_subtype_token_stream(
                    &tokens,
                    &match &postgresql_json_type_subtype_table_type_declaration_or_create {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => PostgresqlJsonTypeSubtype::TableTypeDeclaration,
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => PostgresqlJsonTypeSubtype::Create,
                    }
                )
            };
            let ident_table_type_declaration_or_ident_create_token_stream = generate_ident_table_type_declaration_or_create_token_stream(
                &ident_table_type_declaration_or_ident_create_upper_camel_case,
                &{
                    let wrap_into_scopes_token_stream = |content: &dyn quote::ToTokens|{
                        quote::quote! {(#content);}
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(
                                &IsStandartWithId::False,
                                postgresql_json_type_subtype_table_type_declaration_or_create,
                                &StructDeclarationOrNewType::StructDeclaration,
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                    &generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(ident_standart_not_null_upper_camel_case)
                                )
                            ),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => wrap_into_scopes_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(
                                &ident_with_id_standart_not_null_upper_camel_case
                            ))),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                &generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(&ident_with_id_array_not_null_upper_camel_case)
                            )),
                        },
                    }
                }
            );
            let generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let content_token_stream = get_vec_syn_field(&is_standart_with_id).iter().map(|element| {
                    element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    })
                });
                quote::quote!{Self {#(#content_token_stream),*}}
            };
            let impl_new_for_ident_table_type_declaration_or_ident_create_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                &ident_table_type_declaration_or_ident_create_upper_camel_case,
                &{
                    let generate_wrap_into_value_parameter_token_stream = |type_token_stream: &dyn quote::ToTokens|{
                        quote::quote!{value: #type_token_stream}
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(
                                &IsStandartWithId::False,
                                postgresql_json_type_subtype_table_type_declaration_or_create,
                                &StructDeclarationOrNewType::NewType,
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(ident_standart_not_null_upper_camel_case))),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(
                                &ident_with_id_standart_not_null_upper_camel_case
                            ))),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_wrap_into_value_parameter_token_stream(&postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                &postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                                    &generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(&ident_with_id_standart_not_null_upper_camel_case)
                                )
                            )),
                        },
                    }
                },
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(
                            &IsStandartWithId::False
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_value_token_stream.clone(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let ident_not_null_array_with_id_postfix_upper_camel_case = generate_type_as_postgresql_json_type_subtype_table_type_declaration_or_create_token_stream(&generate_ident_upper_camel_case(
                                &IdentPattern::NotNullArrayWithId
                            ));
                            quote::quote!{Self(
                                match #value_snake_case {
                                    Some(value) => Some(#ident_not_null_array_with_id_postfix_upper_camel_case::new(value)),
                                    None => None
                                }

                            )}
                        },
                    }
                }
            );
            let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream = |
                ident_token_stream: &dyn quote::ToTokens,
                content_token_stream: &dyn quote::ToTokens,
            |{
                postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {Self #content_token_stream}
                )
            };
            let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = |
                is_standart_with_id: &IsStandartWithId
            |{
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
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream(
                &IsStandartWithId::False
            );
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_ident_create_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream(
                &ident_table_type_declaration_or_ident_create_upper_camel_case,
                &match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream.clone(),
                        PostgresqlJsonObjectTypePattern::Array => quote::quote!{(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])}
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                        (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                    },
                }
            );
            let maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = if is_standart_not_null {
                let ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => &ident_with_id_table_type_declaration_standart_not_null_upper_camel_case,
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => &ident_with_id_create_standart_not_null_upper_camel_case,
                };
                let current_is_standart_with_id = match &postgresql_json_type_subtype_table_type_declaration_or_create {
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => IsStandartWithId::True,
                    PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => IsStandartWithId::False,
                };
                let ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = generate_ident_table_type_declaration_or_create_token_stream(
                    &ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_upper_camel_case,
                    &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(
                        &current_is_standart_with_id,
                        &postgresql_json_type_subtype_table_type_declaration_or_create,
                        &StructDeclarationOrNewType::StructDeclaration,
                    ),
                );
                let impl_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                    &ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_upper_camel_case,
                    &generate_ident_table_type_declaration_or_create_or_ident_with_id_table_type_declaration_or_create_standart_not_null_content_token_stream(
                        &current_is_standart_with_id,
                        &postgresql_json_type_subtype_table_type_declaration_or_create,
                        &StructDeclarationOrNewType::NewType,
                    ),
                    &generate_self_content_for_ident_or_ident_with_id_table_type_declaration_or_create_token_stream(
                        &current_is_standart_with_id
                    ),
                );
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_token_stream(
                    &ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_upper_camel_case,
                    &match &postgresql_json_type_subtype_table_type_declaration_or_create {
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration => generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream(
                            &IsStandartWithId::True
                        ),
                        PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create => impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_create_standart_not_null_content_token_stream,
                    }
                );
                quote::quote! {
                    #ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                    #impl_new_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_table_type_declaration_or_ident_create_token_stream
                #impl_new_for_ident_table_type_declaration_or_ident_create_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_or_ident_create_token_stream
                #maybe_ident_with_id_table_type_declaration_or_ident_with_id_create_standart_not_null_token_stream
            }
        };

        let ident_table_type_declaration_token_stream = {
            let ident_table_type_declaration_common_token_stream = generate_ident_table_type_declaration_or_ident_create_common_token_stream(
                &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::TableTypeDeclaration
            );
            let generate_impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let ident_token_stream = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_table_type_declaration_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_table_type_declaration_standart_not_null_upper_camel_case
                };
                postgresql_crud_macros_common::generate_create_table_column_query_part_token_stream(
                    &ident_token_stream,
                    postgresql_crud_macros_common::IsPrimaryKeyUsed::False,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_string());
                        quote::quote! {
                            format!(#format_handle_token_stream, serde_json::to_string(&schemars::schema_for!(#ident_token_stream)).unwrap())
                        }
                    }
                )
            };
            let impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream = generate_impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream(
                &IsStandartWithId::False,
            );
            let maybe_ident_with_id_table_type_declaration_standart_not_null_token_stream = if is_standart_not_null {
                let impl_create_table_column_query_part_for_ident_with_id_table_type_declaration_standart_not_null_token_stream = generate_impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream(
                    &IsStandartWithId::True
                );
                quote::quote! {
                    #impl_create_table_column_query_part_for_ident_with_id_table_type_declaration_standart_not_null_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_table_type_declaration_common_token_stream
                #impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream
                #maybe_ident_with_id_table_type_declaration_standart_not_null_token_stream
            }
        };
        let ident_create_token_stream = {
            let ident_create_common_token_stream = generate_ident_table_type_declaration_or_ident_create_common_token_stream(
                &PostgresqlJsonTypeSubtypeTableTypeDeclarationOrCreate::Create
            );
            let generate_impl_std_fmt_display_for_ident_create_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                macros_helpers::generate_impl_std_fmt_display_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &ident_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {write!(formatter, "{:?}", self)}
                )
            };
            let impl_std_fmt_display_for_ident_create_token_stream = generate_impl_std_fmt_display_for_ident_create_token_stream(
                &ident_create_upper_camel_case
            );
            let impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(
                &ident_create_upper_camel_case
            );
            let generate_standart_not_null_create_query_part_content_token_stream = |is_standart_with_id: IsStandartWithId|{
                let ok_value_token_stream = match &is_standart_with_id {
                    IsStandartWithId::False => quote::quote! {format!("{increments}")},
                    IsStandartWithId::True => quote::quote! {format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}")}
                };
                let create_query_part_fields_token_stream = vec_syn_field.iter().map(|element| {
                    let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                    let postgresql_crud_wrap_into_jsonb_build_object_token_stream = {
                        let wrap_into_jsonb_build_object_snake_case = naming::WrapIntoJsonbBuildObjectSnakeCase;
                        quote::quote! {#import_path::#wrap_into_jsonb_build_object_snake_case}
                    };
                    quote::quote! {
                        match #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_part_snake_case(&self.#element_field_ident, #increment_snake_case) {
                            Ok(value) => {
                                #increments_snake_case.push_str(&#postgresql_crud_wrap_into_jsonb_build_object_token_stream(#element_field_ident_double_quotes_token_stream, &value));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    }
                });
                quote::quote! {
                    let mut #increments_snake_case = std::string::String::from("");
                    #(#create_query_part_fields_token_stream)*
                    let _ = #increments_snake_case.pop();
                    let _ = #increments_snake_case.pop();
                    Ok(#ok_value_token_stream)
                }
            };
            let standart_not_null_create_query_bind_content_token_stream = {
                let create_query_bind_fields_token_stream = vec_syn_field.iter().map(|element| {
                    let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                    quote::quote! {
                        #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_bind_snake_case(self.#element_field_ident, #query_snake_case);
                    }
                });
                quote::quote! {
                    #(#create_query_bind_fields_token_stream)*
                    #query_snake_case
                }
            };
            let generate_create_query_part_and_create_query_bind_token_stream = |
                ident_token_stream: &dyn quote::ToTokens,
                create_query_part_content_token_stream: &dyn quote::ToTokens,
                is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable,
                create_query_bind_content_token_stream: &dyn quote::ToTokens,
            |{
                quote::quote!{
                    impl #ident_token_stream {
                        fn #create_query_part_snake_case(
                            &self,
                            #increment_snake_case: #reference_mut_std_primitive_u64_token_stream
                        ) -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                            #create_query_part_content_token_stream
                        }
                        fn #create_query_bind_snake_case(
                            self,
                            #is_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
                        ) -> #query_postgres_arguments_token_stream {
                            #create_query_bind_content_token_stream
                        }
                    }
                }
            };
            let impl_ident_create_token_stream = generate_create_query_part_and_create_query_bind_token_stream(
                &ident_create_upper_camel_case,
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_standart_not_null_create_query_part_content_token_stream(IsStandartWithId::False),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match &self.0 {
                                //todo maybe rewrite like this <VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as #import_path::PostgresqlJsonType>::create_query_part(#value_snake_case, #increment_snake_case)
                                Some(#value_snake_case) => #value_snake_case.#create_query_part_snake_case(#increment_snake_case),
                                None => match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        Ok(format!("${increment}"))
                                    },
                                    None => Err(#import_path_query_part_error_named_checked_add_initialization_token_stream),
                                }
                            }
                        },
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                            let mut acc = std::string::String::default();
                            for element in &self.0 {
                                match element.#create_query_part_snake_case(#increment_snake_case) {
                                    Ok(#value_snake_case) => {
                                        acc.push_str(&format!("{value},"));
                                    },
                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                            let _ = acc.pop();
                            Ok(format!("jsonb_build_array({acc})"))
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match &self.0 {
                                Some(value) => value.create_query_part(increment),
                                None => match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        Ok(format!("${increment}"))
                                    },
                                    None => Err(#import_path_query_part_error_named_checked_add_initialization_token_stream),
                                }
                            }
                        },
                    },
                },
                match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsQueryBindMutable::True,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsQueryBindMutable::False,
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsQueryBindMutable::True,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsQueryBindMutable::False,
                    },
                },
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#standart_not_null_create_query_bind_content_token_stream},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let ident_standart_not_null_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &ident_standart_not_null_upper_camel_case,
                                &PostgresqlJsonTypeSubtype::Create
                            );
                            quote::quote! {
                                match self.0 {
                                    Some(#value_snake_case) => #value_snake_case.#create_query_bind_snake_case(#query_snake_case),
                                    None => #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<
                                        #ident_standart_not_null_as_postgresql_json_type_create_token_stream
                                    >>))
                                }
                            }
                        }
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                            for element in self.0 {
                                #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                            }
                            #query_snake_case
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let ident_with_id_array_not_null_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &ident_with_id_array_not_null_upper_camel_case,
                                &PostgresqlJsonTypeSubtype::Create
                            );
                            quote::quote! {
                                match self.0 {
                                    Some(value) => value.create_query_bind(query),
                                    None => query.bind(sqlx::types::Json(None::<std::option::Option<#ident_with_id_array_not_null_as_postgresql_json_type_create_token_stream>>))
                                }
                            }
                        }
                    },
                }
            );
            let maybe_ident_with_id_create_standart_not_null_token_stream = if is_standart_not_null {
                let impl_std_fmt_display_for_ident_with_id_create_standart_not_null_token_stream = generate_impl_std_fmt_display_for_ident_create_token_stream(
                    &ident_with_id_create_standart_not_null_upper_camel_case
                );
                let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(
                    &ident_with_id_create_standart_not_null_upper_camel_case
                );
                let impl_ident_with_id_create_standart_not_null_token_stream = generate_create_query_part_and_create_query_bind_token_stream(
                    &ident_with_id_create_standart_not_null_upper_camel_case,
                    &generate_standart_not_null_create_query_part_content_token_stream(IsStandartWithId::True),
                    postgresql_crud_macros_common::IsQueryBindMutable::True,
                    &quote::quote!{#standart_not_null_create_query_bind_content_token_stream},
                );
                quote::quote! {
                    #impl_std_fmt_display_for_ident_with_id_create_standart_not_null_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_create_standart_not_null_token_stream
                    #impl_ident_with_id_create_standart_not_null_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_create_common_token_stream
                #impl_std_fmt_display_for_ident_create_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream
                #impl_ident_create_token_stream
                #maybe_ident_with_id_create_standart_not_null_token_stream
            }
        };
        let create_query_part_token_stream = quote::quote!{#value_snake_case.#create_query_part_snake_case(#increment_snake_case)};
        let create_query_bind_token_stream = quote::quote!{#value_snake_case.#create_query_bind_snake_case(#query_snake_case)};
        let generate_sqlx_types_json_type_declaration_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
            postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                &ident_token_stream,
                &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(
                    &self_upper_camel_case,
                )
            )
        };
        let generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
            postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                &ident_token_stream,
                &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(
                    &self_upper_camel_case,
                ),
                &quote::quote! {Ok(value.0)}
            )
        };
        let generate_value_type_token_stream = |type_token_stream: &dyn quote::ToTokens|{
            quote::quote!{#value_snake_case: #type_token_stream}
        };
        let generate_unique_vec_wrapper_token_stream = |type_token_stream: &dyn quote::ToTokens|{
            quote::quote!{#import_path::UniqueVec<#type_token_stream>}
        };
        let postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
            &import_path_postgresql_json_type_uuid_uuid_as_not_null_jsonb_string_token_stream,
            &PostgresqlJsonTypeSubtype::Update
        );
        enum ShouldDeriveDefault {
            True,
            False,
        }
        impl quote::ToTokens for ShouldDeriveDefault {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                if let Self::True = &self {
                    quote::quote!{Default,}.to_tokens(tokens);
                }
            }
        }
        let self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote!{
            Self(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
        };
        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_with_id_select_standart_not_null_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let ident_select_token_stream = {
            let ident_with_id_select_standart_not_null_snake_case = naming::parameter::SelfSelectSnakeCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_select_element_standart_not_null_upper_camel_case = naming::parameter::SelfSelectElementUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_select_element_standart_not_null_upper_camel_case = naming::parameter::SelfSelectElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let generate_pub_struct_ident_select_token_stream = |
                ident_token_stream: &dyn quote::ToTokens,
                should_derive_default: &ShouldDeriveDefault,
                content_token_stream: &dyn quote::ToTokens,
            |{
                quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        #should_derive_default
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                        utoipa::ToSchema,
                        schemars::JsonSchema,
                    )]
                    pub struct #ident_token_stream #content_token_stream
                }
            };
            let generate_ident_select_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let ident_select_standart_not_null_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
                generate_pub_struct_ident_select_token_stream(
                    match &is_standart_with_id {
                        IsStandartWithId::False => &ident_select_standart_not_null_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_select_standart_not_null_upper_camel_case
                    },
                    &ShouldDeriveDefault::True,
                    &{
                        let type_token_stream = generate_unique_vec_wrapper_token_stream(match &is_standart_with_id {
                            IsStandartWithId::False => &ident_select_element_standart_not_null_upper_camel_case,
                            IsStandartWithId::True => &ident_with_id_select_element_standart_not_null_upper_camel_case
                        });
                        quote::quote!{(#type_token_stream);}
                    }
                )
            };
            let dimension1_pagination_token_stream = quote::quote!{dimension1_pagination};
            let import_path_pagination_token_stream = quote::quote!{#import_path::Pagination};
            let ident_select_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_select_standart_not_null_token_stream(&IsStandartWithId::False),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_struct_ident_select_token_stream(
                        &ident_select_upper_camel_case,
                        &ShouldDeriveDefault::False,
                        &{
                            let type_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                &generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_standart_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Select
                                )
                            );
                            quote::quote!{(#type_token_stream);}
                        }
                    ),
                },
                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_pub_struct_ident_select_token_stream(
                        &ident_select_upper_camel_case,
                        &ShouldDeriveDefault::True,
                        &{
                            let ident_with_id_standart_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &ident_with_id_standart_not_null_upper_camel_case,
                                &PostgresqlJsonTypeSubtype::Select
                            );
                            quote::quote!{{
                                #ident_with_id_select_standart_not_null_snake_case: #ident_with_id_standart_not_null_as_postgresql_json_type_select_token_stream,
                                #dimension1_pagination_token_stream: #import_path_pagination_token_stream
                            }}
                        }
                    ),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_pub_struct_ident_select_token_stream(
                        &ident_select_upper_camel_case,
                        &ShouldDeriveDefault::False,
                        &{
                            let ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &ident_with_id_array_not_null_upper_camel_case,
                                &PostgresqlJsonTypeSubtype::Select
                            );
                            quote::quote!{(std::option::Option<#ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream>);}
                        }
                    )
                },
            };
            let impl_new_for_ident_select_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                &ident_select_upper_camel_case,
                &{
                    let unique_vec_ident_select_element_standart_not_null_token_stream = generate_unique_vec_wrapper_token_stream(&ident_select_element_standart_not_null_upper_camel_case);
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_value_type_token_stream(
                                &unique_vec_ident_select_element_standart_not_null_token_stream
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_type_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                    &unique_vec_ident_select_element_standart_not_null_token_stream
                                )
                            ),
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let ident_with_id_standart_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_with_id_standart_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Select
                                );
                                quote::quote!{
                                    #ident_with_id_select_standart_not_null_snake_case: #ident_with_id_standart_not_null_as_postgresql_json_type_select_token_stream,
                                    #dimension1_pagination_token_stream: #import_path_pagination_token_stream
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_type_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                    &generate_type_as_postgresql_json_type_subtype_token_stream(
                                        &ident_with_id_array_not_null_upper_camel_case,
                                        &PostgresqlJsonTypeSubtype::Select
                                    )
                                )
                            ),
                        },
                    }
                },
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_value_token_stream.clone(),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let ident_standart_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                &ident_standart_not_null_upper_camel_case,
                                &PostgresqlJsonTypeSubtype::Select
                            );
                            quote::quote!{
                                Self(match #value_snake_case {
                                    Some(#value_snake_case) => Some(#ident_standart_not_null_as_postgresql_json_type_select_token_stream::new(#value_snake_case)),
                                    None => None
                                })
                            }
                        },
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            quote::quote!{Self {
                                #ident_with_id_select_standart_not_null_snake_case,
                                #dimension1_pagination_token_stream,
                            }}
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_value_token_stream.clone(),
                    },
                },
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_select_upper_camel_case);
            let impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_select_upper_camel_case);
            let maybe_impl_default_default_for_ident_select_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                //todo different realizations of standart and array //todo maybe not need for array nullable
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_std_default_default_token_stream::generate_std_default_default_token_stream(
                    &ident_select_upper_camel_case,
                    &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                )
            };
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream = quote::quote!{
                Self(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
            };
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_select_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.clone(),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{
                            Self {
                                #ident_with_id_select_standart_not_null_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                #dimension1_pagination_token_stream: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            }
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.clone(),
                    },
                }
            );
            let generate_select_query_part_content_for_ident_select_or_ident_with_id_select_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageFieldIdentSnakeCase;
                let column_name_and_maybe_field_getter_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterFieldIdentSnakeCase;
                let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    let variant_name_token_stream: &dyn quote::ToTokens = &naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                    let field_ident_double_quotes_token_stream: &dyn quote::ToTokens = &generate_quotes::double_quotes_token_stream(&field_ident_stringified);
                    let column_name_and_maybe_field_getter_token_stream: &dyn quote::ToTokens = &quote::quote! {&#column_name_and_maybe_field_getter_field_ident_snake_case};
                    let element_type: &dyn quote::ToTokens = &{
                        let element_type = &element.ty;
                        quote::quote! {#element_type}
                    };

                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream(&element_type);
                    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
                    let ident_select_element_or_ident_with_id_select_element_standart_not_null_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_select_element_standart_not_null_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_select_element_standart_not_null_upper_camel_case
                    };
                    quote::quote! {
                        #ident_select_element_or_ident_with_id_select_element_standart_not_null_upper_camel_case::#variant_name_token_stream(value) => #field_type_as_crud_postgresql_json_type_from_field_token_stream #select_query_part_snake_case(
                            &value,
                            #field_ident_double_quotes_token_stream,
                            #column_name_and_maybe_field_getter_token_stream,
                            &#column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case,
                            false,
                        )
                    }
                });
                let self_field_vec_token_stream = quote::quote! {.0.to_vec()};
                let maybe_pagination_start_end_initialization_token_stream = proc_macro2::TokenStream::new();
                let column_name_and_maybe_field_getter_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                    &format!("{{{}}}->'{{{field_ident_snake_case}}}'", naming::ColumnNameAndMaybeFieldGetterSnakeCase)
                );
                let column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                    &format!("{{{}}}.{{{field_ident_snake_case}}}", naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase)
                );
                let (if_postgresql_type_is_true_format_handle_double_quotes_token_stream, if_postgresql_type_is_false_format_handle_double_quotes_token_stream) = {
                    let wrap_into_jsonb_build_object_field_ident = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{{{field_ident_snake_case}}}', {value})");
                    let wrap_into_jsonb_build_object_value = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{value_snake_case}',{value})");
                    let acc_format_handle = {
                        let acc_snake_case = naming::AccSnakeCase;
                        format!("{{{acc_snake_case}}}")
                    };
                    let jsonb_build_object_value_acc_format_handle = wrap_into_jsonb_build_object_value(&acc_format_handle);
                    (acc_format_handle, wrap_into_jsonb_build_object_field_ident(&jsonb_build_object_value_acc_format_handle))
                };
                quote::quote! {
                    let mut acc = std::string::String::default();
                    let #column_name_and_maybe_field_getter_field_ident_snake_case = if is_postgresql_type {
                        column_name_and_maybe_field_getter.to_string()
                    } else {
                        format!(#column_name_and_maybe_field_getter_format_handle_token_stream)
                    };
                    let #column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = format!(#column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream);
                    for element in self #self_field_vec_token_stream {
                        acc.push_str(&format!(
                            "{}||",
                            match element {
                                #(#variants_token_stream),*
                            }
                        ));
                    }
                    let _ = acc.pop();
                    let _ = acc.pop();
                    #maybe_pagination_start_end_initialization_token_stream
                    if is_postgresql_type {
                        format!(#if_postgresql_type_is_true_format_handle_double_quotes_token_stream)
                    }
                    else {
                        format!(#if_postgresql_type_is_false_format_handle_double_quotes_token_stream)
                    }
                }
            };
            enum ParameterIsPostgresqlTypeUsed {
                True,
                False
            }
            impl quote::ToTokens for ParameterIsPostgresqlTypeUsed {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    match &self {
                        ParameterIsPostgresqlTypeUsed::True => quote::quote!{is_postgresql_type}.to_tokens(tokens),
                        ParameterIsPostgresqlTypeUsed::False => quote::quote!{_}.to_tokens(tokens),
                    }
                }
            }
            let generate_select_query_part_token_stream = |
                parameter_is_postgresql_type_used: ParameterIsPostgresqlTypeUsed,
                content_token_stream: &dyn quote::ToTokens
            |{
                quote::quote!{
                    fn #select_query_part_snake_case(
                        &self,
                        #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                        #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                        #column_name_and_maybe_field_getter_for_error_message_snake_case: #reference_std_primitive_str_token_stream,
                        #parameter_is_postgresql_type_used: #std_primitive_bool_token_stream,
                    ) -> #std_string_string_token_stream {
                        #content_token_stream
                    }
                }
            };
            let impl_ident_select_token_stream = {
                let select_query_part_token_stream = generate_select_query_part_token_stream(
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => ParameterIsPostgresqlTypeUsed::True,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => ParameterIsPostgresqlTypeUsed::False,
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => ParameterIsPostgresqlTypeUsed::False,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => ParameterIsPostgresqlTypeUsed::False,
                        }
                    },
                    &match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_select_query_part_content_for_ident_select_or_ident_with_id_select_standart_not_null_token_stream(&IsStandartWithId::False),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_standart_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_standart_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Select
                                );
                                quote::quote!{
                                    let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
                                    format!(
                                        "jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then null else ({}) end))",
                                        {
                                            let value = match &self.0 {
                                                Some(value) => value,
                                                None => &<#ident_standart_not_null_as_postgresql_json_type_select_token_stream as #import_path::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()//todo reuse
                                            };
                                            value.select_query_part(
                                                field_ident,
                                                &column_name_and_maybe_field_getter_field_ident,
                                                column_name_and_maybe_field_getter_for_error_message,//todo maybe wrong - add postfix field_ident or column_name_and_maybe_field_getter_field_ident
                                                true
                                            )
                                        }
                                    )
                                }
                            },
                        },
                        PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',case when (jsonb_array_length({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_select_standart_not_null_snake_case}}})) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{dimension1_start}} and {{dimension1_end}}) end ))"));
                                quote::quote!{
                                    let #ident_with_id_select_standart_not_null_snake_case = self.#ident_with_id_select_standart_not_null_snake_case.select_query_part(
                                        field_ident,
                                        &"value",
                                        &"value",
                                        true
                                    );
                                    let dimension1_start = self.#dimension1_pagination_token_stream.start();
                                    let dimension1_end = self.#dimension1_pagination_token_stream.end();
                                    format!(#format_handle_token_stream)
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_with_id_array_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Select
                                );
                                quote::quote!{
                                    format!("case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value',null)) else ({}) end", {
                                        let value = match &self.0 {
                                            Some(value) => value,
                                            None => &<#ident_with_id_array_not_null_as_postgresql_json_type_select_token_stream as #import_path::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                        };
                                        value.select_query_part(
                                            field_ident,
                                            &column_name_and_maybe_field_getter,
                                            column_name_and_maybe_field_getter_for_error_message, true
                                        )
                                    })
                                }
                            },
                        }
                    }
                );
                quote::quote!{
                    impl #ident_select_upper_camel_case {
                        #select_query_part_token_stream
                    }
                }
            };
            let generate_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let ident_select_element_or_ident_with_id_select_element_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_select_element_standart_not_null_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_select_element_standart_not_null_upper_camel_case
                };
                let ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream = {
                    let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let serialize_deserialize_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                        let field_type_as_json_type_select_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::Select
                        );
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
                let impl_error_occurence_lib_to_std_string_string_for_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(
                    &ident_select_element_or_ident_with_id_select_element_upper_camel_case
                );
                let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_select_element_or_ident_with_id_select_element_upper_camel_case,
                    &{
                        let vec_content_token_stream = {
                            let elements_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                                let field_ident = &element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                quote::quote! {
                                    #ident_select_element_or_ident_with_id_select_element_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                                }
                            });
                            quote::quote! {#(#elements_token_stream),*}
                        };
                        quote::quote! {vec![
                            #vec_content_token_stream
                        ]}
                    }
                );
                quote::quote! {
                    #ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream
                }
            };
            let maybe_ident_select_element_token_stream = if is_standart_not_null {
                generate_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream(&IsStandartWithId::False)
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let maybe_ident_with_id_select_standart_not_null_token_stream = if is_standart_not_null {
                let ident_with_id_select_standart_not_null_token_stream = generate_ident_select_standart_not_null_token_stream(&IsStandartWithId::True);
                let impl_new_for_ident_with_id_select_standart_not_null_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                    &ident_with_id_select_standart_not_null_upper_camel_case,
                    &generate_value_type_token_stream(&generate_unique_vec_wrapper_token_stream(&ident_with_id_select_element_standart_not_null_upper_camel_case)),
                    &self_value_token_stream
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_with_id_select_standart_not_null_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(
                    &ident_with_id_select_standart_not_null_upper_camel_case
                );
                let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_select_standart_not_null_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(
                    &ident_with_id_select_standart_not_null_upper_camel_case
                );
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_select_standart_not_null_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_with_id_select_standart_not_null_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_standart_not_null_content_token_stream
                );
                let impl_ident_with_id_select_standart_not_null_token_stream = {
                    let select_query_part_token_stream = generate_select_query_part_token_stream(
                        ParameterIsPostgresqlTypeUsed::True,
                        &generate_select_query_part_content_for_ident_select_or_ident_with_id_select_standart_not_null_token_stream(&IsStandartWithId::True)
                    );
                    quote::quote!{
                        impl #ident_with_id_select_standart_not_null_upper_camel_case {
                            #select_query_part_token_stream
                        }
                    }
                };
                let ident_with_id_select_element_token_stream = generate_ident_select_element_or_ident_with_id_select_element_standart_not_null_token_stream(&IsStandartWithId::True);
                quote::quote! {
                    #ident_with_id_select_standart_not_null_token_stream
                    #impl_new_for_ident_with_id_select_standart_not_null_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_with_id_select_standart_not_null_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_select_standart_not_null_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_select_standart_not_null_token_stream
                    #impl_ident_with_id_select_standart_not_null_token_stream
                    #ident_with_id_select_element_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_select_token_stream
                #impl_new_for_ident_select_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream
                #maybe_impl_default_default_for_ident_select_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
                #impl_ident_select_token_stream
                #maybe_ident_select_element_token_stream
                #maybe_ident_with_id_select_standart_not_null_token_stream
            }
        };
        let select_query_part_token_stream = quote::quote!{
            #value_snake_case.#select_query_part_snake_case(
                #field_ident_snake_case,
                #column_name_and_maybe_field_getter_snake_case,
                #column_name_and_maybe_field_getter_for_error_message_snake_case,
                is_postgresql_type,
            )
        };
        let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
        let ident_with_id_where_element_standart_not_null_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let ident_where_element_token_stream = match &not_null_or_nullable {
            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                use postgresql_crud_macros_common::NotNullOrNullable;
                let generate_ident_where_element_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
                    let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                        let field_ident_stringified = element.ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        let field_type_as_json_type_where_element_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::WhereElement
                        );
                        quote::quote! {
                            #field_ident_upper_camel_case_token_stream(#import_path::PostgresqlTypeWhere<
                                #field_type_as_json_type_where_element_token_stream
                            >)
                        }
                    });
                    quote::quote!{#(#variants_token_stream),*}
                };
                let generate_ident_where_element_token_stream = |
                    ident_token_stream:&dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens
                |{
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub enum #ident_token_stream {
                             #content_token_stream
                        }
                    }
                };
                let maybe_ident_where_element_token_stream = {
                    let generate_ident_where_element_wrapper_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                        generate_ident_where_element_token_stream(
                            &ident_where_element_upper_camel_case,
                            &content_token_stream
                        )
                    };
                    match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &postgresql_json_object_type_pattern {
                            PostgresqlJsonObjectTypePattern::Standart => generate_ident_where_element_wrapper_token_stream(
                                &generate_ident_where_element_content_token_stream(&IsStandartWithId::False)
                            ),
                            PostgresqlJsonObjectTypePattern::Array => generate_ident_where_element_wrapper_token_stream(&{
                                let equal_token_stream = {
                                    let ident_as_postgresql_json_type_table_type_declaration_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                                        &ident,
                                        &PostgresqlJsonTypeSubtype::TableTypeDeclaration
                                    );
                                    quote::quote! {
                                        Equal(#import_path::where_element_filters::PostgresqlJsonTypeWhereElementEqual<#ident_as_postgresql_json_type_table_type_declaration_token_stream>)
                                    }
                                };
                                //todo additional filters
                                quote::quote! {
                                    #equal_token_stream
                                }
                            })
                        },
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    }
                };
                let generate_where_filter_query_part_content_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId|{
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
                            Self::#field_ident_upper_camel_case_token_stream(value) => #import_path::PostgresqlTypeWhereFilter::query_part(
                                value,
                                increment,
                                &format!(#format_handle_token_stream),
                                is_need_to_add_logical_operator,
                            )
                        }
                    });
                    quote::quote! {
                        match &self {
                            #(#query_part_variants_token_stream),*
                        }
                    }
                };
                let generate_where_filter_query_bind_content_standart_not_null_token_stream = |is_standart_with_id: &IsStandartWithId|{
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
                            Self::#field_ident_upper_camel_case_token_stream(value) => #import_path::PostgresqlTypeWhereFilter::query_bind(value, query)
                        }
                    });
                    quote::quote! {
                        match self {
                            #(#query_bind_variants_token_stream),*
                        }
                    }
                };
                let generate_impl_postgresql_type_where_filter_token_stream = |
                    ident_token_stream: &dyn quote::ToTokens,
                    query_part_content_token_stream: &dyn quote::ToTokens,
                    is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable,
                    query_bind_content_token_stream: &dyn quote::ToTokens,
                |{
                    postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                        &quote::quote! {<'a>},
                        &ident_token_stream,
                        &proc_macro2::TokenStream::new(),
                        &query_part_content_token_stream,
                        &is_query_bind_mutable,
                        &query_bind_content_token_stream,
                        &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                    )
                };
                let maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream = {
                    let generate_impl_postgresql_type_where_filter_for_ident_token_stream = |
                        query_part_content_token_stream: &dyn quote::ToTokens,
                        is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable,
                        query_bind_content_token_stream: &dyn quote::ToTokens,
                    |{
                        generate_impl_postgresql_type_where_filter_token_stream(
                            &ident_where_element_upper_camel_case,
                            &query_part_content_token_stream,
                            is_query_bind_mutable,
                            &query_bind_content_token_stream,
                        )
                    };
                    match &postgresql_json_object_type_pattern {
                        PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_impl_postgresql_type_where_filter_for_ident_token_stream(
                                &generate_where_filter_query_part_content_standart_not_null_token_stream(&IsStandartWithId::False),
                                postgresql_crud_macros_common::IsQueryBindMutable::False,
                                &generate_where_filter_query_bind_content_standart_not_null_token_stream(&IsStandartWithId::False),
                            ),
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                        //todo different filters for not_null and nullable
                        PostgresqlJsonObjectTypePattern::Array => generate_impl_postgresql_type_where_filter_for_ident_token_stream(
                            &quote::quote!{
                                match &self {
                                    Self::Equal(value) => #import_path::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
                                }
                            },
                            postgresql_crud_macros_common::IsQueryBindMutable::False,
                            &quote::quote!{
                                match self {
                                    Self::Equal(value) => #import_path::PostgresqlTypeWhereFilter::query_bind(value, query),
                                }
                            },
                        )
                    }
                };
                let maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream = if let (
                    PostgresqlJsonObjectTypePattern::Standart, NotNullOrNullable::Nullable
                ) = (&postgresql_json_object_type_pattern, &not_null_or_nullable) {
                    proc_macro2::TokenStream::new()
                }
                else {
                    generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(
                        &ident_where_element_upper_camel_case
                    )
                };
                let generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element = |is_standart_with_id: &IsStandartWithId|{
                    let variants_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        quote::quote! {
                            Self::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                        }
                    });
                    quote::quote! {vec![#(#variants_token_stream),*]}
                };
                let maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &ident_where_element_upper_camel_case,
                            &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element(&IsStandartWithId::False)
                        ),
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    //todo diffrent filters for not_null and nullable
                    PostgresqlJsonObjectTypePattern::Array => postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_where_element_upper_camel_case,
                        &quote::quote!{
                            vec![Self::Equal(#import_path::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
                        }
                    ),
                };
                let maybe_ident_with_id_where_element_standart_not_null_token_stream = if is_standart_not_null {
                    let ident_with_id_where_element_standart_not_null_token_stream = generate_ident_where_element_token_stream(
                        &ident_with_id_where_element_standart_not_null_upper_camel_case,
                        &generate_ident_where_element_content_token_stream(&IsStandartWithId::True)
                    );
                    let impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_where_element_standart_not_null_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                        &ident_with_id_where_element_standart_not_null_upper_camel_case,
                        &generate_where_filter_query_part_content_standart_not_null_token_stream(&IsStandartWithId::True),
                        postgresql_crud_macros_common::IsQueryBindMutable::False,
                        &generate_where_filter_query_bind_content_standart_not_null_token_stream(&IsStandartWithId::True),
                    );
                    let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_where_element_standart_not_null_token_stream = generate_generate_impl_error_occurence_lib_to_std_string_string_wrapper_token_stream(
                        &ident_with_id_where_element_standart_not_null_upper_camel_case
                    );
                    let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_where_element_standart_not_null_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_with_id_where_element_standart_not_null_upper_camel_case,
                        &generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_content_standart_not_null_where_element(&IsStandartWithId::True)
                    );
                    quote::quote! {
                        #ident_with_id_where_element_standart_not_null_token_stream
                        #impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_with_id_where_element_standart_not_null_token_stream
                        #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_where_element_standart_not_null_token_stream
                        #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_where_element_standart_not_null_token_stream
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #maybe_ident_where_element_token_stream
                    #maybe_impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream
                    #maybe_impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream
                    #maybe_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream
                    #maybe_ident_with_id_where_element_standart_not_null_token_stream
                }
            },
            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                let ident_standart_or_ident_with_id_array_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => &ident_standart_not_null_upper_camel_case,
                    PostgresqlJsonObjectTypePattern::Array => &ident_with_id_array_not_null_upper_camel_case
                };
                quote::quote!{
                    pub type #ident_where_element_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<<#ident_standart_or_ident_with_id_array_upper_camel_case as #import_path::PostgresqlJsonType>::WhereElement>;
                }
            },
        };
        let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
            generate_quotes::double_quotes_token_stream(&value.ident.as_ref().unwrap_or_else(|| {
                panic!("{}", naming::FIELD_IDENT_IS_NONE);
            }))
        };
        enum ShouldDeriveSerdeDeserialize {
            True,
            False
        }
        impl quote::ToTokens for ShouldDeriveSerdeDeserialize {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                if let Self::True = &self {
                    quote::quote!{serde::Deserialize,}.to_tokens(tokens);
                }
            }
        }
        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
        let ident_with_id_read_standart_not_null_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let ident_read_token_stream = {
            let ident_read_standart_not_null_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_read_try_from_error_named_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident);
            let ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case = naming::parameter::SelfReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
            let ident_with_id_read_array_not_null_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_with_id_array_not_null_upper_camel_case);
            enum ShouldAddSerdeOptionIsNoneAnnotation {
                True,
                False
            }
            let generate_ident_read_or_ident_with_id_read_fields_declaration_token_stream = |
                is_standart_with_id: &IsStandartWithId,
                should_add_serde_option_is_none_annotation: &ShouldAddSerdeOptionIsNoneAnnotation
            | {
                let content_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                    let maybe_serde_skip_serializing_if_option_is_none_token_stream = match &should_add_serde_option_is_none_annotation {
                        ShouldAddSerdeOptionIsNoneAnnotation::True => quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                        ShouldAddSerdeOptionIsNoneAnnotation::False => proc_macro2::TokenStream::new()
                    };
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type_as_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &element.ty,
                        &PostgresqlJsonTypeSubtype::Read
                    );
                    quote::quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_token_stream
                        #field_ident: std::option::Option<#import_path::Value<
                            #field_type_as_json_type_read_token_stream
                        >>
                    }
                });
                quote::quote! {
                    #(#content_token_stream),*
                }
            };
            let generate_ident_read_token_stream = |
                ident_token_stream: &dyn quote::ToTokens,
                content_token_stream: &dyn quote::ToTokens,
                should_derive_default: &ShouldDeriveDefault,
                should_derive_serde_deserialize: &ShouldDeriveSerdeDeserialize,
            |{
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, #should_derive_default serde::Serialize, #should_derive_serde_deserialize utoipa::ToSchema, schemars::JsonSchema)]
                    pub struct #ident_token_stream #content_token_stream
                }
            };
            let ident_read_token_stream = {
                let generate_ident_read_wrapper_token_stream = |
                    content_token_stream: &dyn quote::ToTokens,
                    should_derive_default: &ShouldDeriveDefault,
                    should_derive_serde_deserialize: &ShouldDeriveSerdeDeserialize,
                |{
                    generate_ident_read_token_stream(
                        &ident_read_upper_camel_case,
                        &content_token_stream,
                        should_derive_default,
                        should_derive_serde_deserialize,
                    )
                };
                match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_read_wrapper_token_stream(
                            &{
                                let content_token_stream = generate_ident_read_or_ident_with_id_read_fields_declaration_token_stream(
                                    &IsStandartWithId::False,
                                    &ShouldAddSerdeOptionIsNoneAnnotation::True
                                );
                                quote::quote!{{#content_token_stream}}
                            },
                            &ShouldDeriveDefault::True,
                            &ShouldDeriveSerdeDeserialize::False,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_read_wrapper_token_stream(
                            &quote::quote!{(std::option::Option<#ident_read_standart_not_null_upper_camel_case>);},
                            &ShouldDeriveDefault::False,//todo maybe True ?
                            &ShouldDeriveSerdeDeserialize::True,
                        ),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_read_wrapper_token_stream(
                            &quote::quote!{(std::vec::Vec<#ident_with_id_read_standart_not_null_upper_camel_case>);},
                            &ShouldDeriveDefault::True,
                            &ShouldDeriveSerdeDeserialize::True,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_read_wrapper_token_stream(
                            &quote::quote!{(std::option::Option<#ident_with_id_read_array_not_null_upper_camel_case>);},
                            &ShouldDeriveDefault::False,
                            &ShouldDeriveSerdeDeserialize::True,
                        ),
                    },
                }
            };
            let all_fields_are_none_upper_camel_case = naming::AllFieldsAreNoneUpperCamelCase;
            let generate_ident_read_try_from_error_named_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
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
            let generate_impl_try_new_for_ident_read_try_from_error_named_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let ident_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_read_standart_not_null_upper_camel_case,
                };
                let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                let fields_declaration_token_stream = generate_ident_read_or_ident_with_id_read_fields_declaration_token_stream(
                    is_standart_with_id,
                    &ShouldAddSerdeOptionIsNoneAnnotation::False
                );
                let (fields_reference_token_stream, fields_token_stream) = {
                    enum WithReference {
                        True,
                        False
                    }
                    let generate_fields_token_stream = |with_reference: &WithReference| {
                        let maybe_reference_symbol_token_stream = match &with_reference {
                            WithReference::True => quote::quote! {&},
                            WithReference::False => proc_macro2::TokenStream::new()
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
                    (
                        generate_fields_token_stream(&WithReference::True),
                        generate_fields_token_stream(&WithReference::False)
                    )
                };
                let ident_read_try_from_error_named_or_ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_try_from_error_named_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case
                };
                let check_if_all_fields_are_none_token_stream = {
                    let current_vec_syn_field_len = current_vec_syn_field.len();
                    let nones_token_stream = {
                        let mut acc = vec![];
                        for _ in 0..current_vec_syn_field_len {
                            acc.push(quote::quote! {None});
                        }
                        acc
                    };
                    let maybe_wrap_into_braces_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        if current_vec_syn_field_len > 1 {
                            quote::quote!{(#content_token_stream)}
                        }
                        else {
                            quote::quote!{#content_token_stream}
                        }
                    };
                    let left_token_stream = maybe_wrap_into_braces_token_stream(&quote::quote!{#(#nones_token_stream),*});
                    let right_token_stream = maybe_wrap_into_braces_token_stream(&fields_reference_token_stream);
                    quote::quote! {
                        if let #left_token_stream = #right_token_stream {
                            return Err(#ident_read_try_from_error_named_or_ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case::#all_fields_are_none_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                    }
                };
                quote::quote! {
                    impl #ident_token_stream {
                        pub fn try_new(#fields_declaration_token_stream) -> Result<Self, #ident_read_try_from_error_named_or_ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case> {
                            #check_if_all_fields_are_none_token_stream
                            Ok(Self{#fields_token_stream})
                        }
                    }
                }
            };
            let impl_new_or_try_new_for_ident_read_try_from_error_named_token_stream = {
                let std_vec_vec_ident_with_id_read_standart_not_null_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(
                    &ident_with_id_read_standart_not_null_upper_camel_case
                );
                match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_try_new_for_ident_read_try_from_error_named_token_stream(&IsStandartWithId::False),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(
                            &ident_read_upper_camel_case,
                            &generate_value_type_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_standart_not_null_upper_camel_case)
                            ),
                            &self_value_token_stream,
                        ),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_impl_new_for_ident_token_stream(
                            &ident_read_upper_camel_case,
                            &generate_value_type_token_stream(&std_vec_vec_ident_with_id_read_standart_not_null_token_stream),
                            &self_value_token_stream,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(
                            &ident_read_upper_camel_case,
                            &generate_value_type_token_stream(
                                &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&std_vec_vec_ident_with_id_read_standart_not_null_token_stream)
                            ),
                            &quote::quote!{
                                Self(match #value_snake_case {
                                    Some(#value_snake_case) => Some(#ident_with_id_read_array_not_null_upper_camel_case::new(#value_snake_case)),
                                    None => None
                                })
                            },
                        ),
                    },
                }
            };
            let generate_impl_serde_deserialize_for_ident_read_token_stream = |is_standart_with_id: &IsStandartWithId| {
                let ident_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_read_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_read_standart_not_null_upper_camel_case,
                };
                let current_vec_syn_field = get_vec_syn_field(is_standart_with_id);
                let current_vec_syn_field_len = current_vec_syn_field.len();
                let field_enum_variants_token_stream = {
                    let mut vec = vec![];
                    for element in 0..current_vec_syn_field_len {
                        let value = format!("__{}{element}", naming::FieldSnakeCase);
                        vec.push(value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)));
                    }
                    vec
                };
                let generate_field_index_token_stream = |index: std::primitive::usize| {
                    let value = format!("__field{index}");
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let visit_u64_value_enum_variants_token_stream = {
                    let mut acc = vec![];
                    for index in 0..current_vec_syn_field_len {
                        let index_u64_token_stream = {
                            let value = format!("{index}u64");
                            value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        acc.push(quote::quote! {
                            #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
                        });
                    }
                    acc
                };
                let generate_field_ident_double_quotes_serde_private_ok_field_token_stream = |field_name_double_quotes_token_stream: &dyn quote::ToTokens, index: std::primitive::usize| {
                    let field_index_token_stream = generate_field_index_token_stream(index);
                    quote::quote! {#field_name_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
                };
                let visit_str_value_enum_variants_token_stream = {
                    let visit_str_value_enum_variants_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        }));
                        generate_field_ident_double_quotes_serde_private_ok_field_token_stream(&field_name_double_quotes_token_stream, index)
                    });
                    quote::quote! {
                        #(#visit_str_value_enum_variants_token_stream),*,
                    }
                };
                let visit_bytes_value_enum_variants_token_stream = {
                    let visit_bytes_value_enum_variants_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let b_field_name_double_quotes_token_stream = {
                            let element_ident_double_quotes_stringified = generate_quotes::double_quotes_stringified(
                                &element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string(),
                            );
                            let value = format!("b{element_ident_double_quotes_stringified}");
                            value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        generate_field_ident_double_quotes_serde_private_ok_field_token_stream(&b_field_name_double_quotes_token_stream, index)
                    });
                    quote::quote! {
                        #(#visit_bytes_value_enum_variants_token_stream),*,
                    }
                };
                let struct_ident_options_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_token_stream}"));
                let struct_ident_options_with_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_token_stream} with {current_vec_syn_field_len} elements"));
                let visit_seq_fields_initialization_token_stream = {
                    let generate_serde_de_seq_access_next_element_token_stream = |index: std::primitive::usize, type_read_token_stream: &dyn quote::ToTokens| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            let #field_index_token_stream = match serde::de::SeqAccess::next_element::<
                                std::option::Option<#import_path::Value<#type_read_token_stream>>,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &#struct_ident_options_with_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                        }
                    };
                    let visit_seq_fields_initialization_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_serde_de_seq_access_next_element_token_stream(
                            index,
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    quote::quote! {
                        #(#visit_seq_fields_initialization_token_stream)*
                    }
                };
                let match_try_new_in_deserialize_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(
                    &ident_token_stream,
                    &{
                        let fields_token_stream = {
                            let mut acc = vec![];
                            for element in 0..current_vec_syn_field_len {
                                let field_index_token_stream = generate_field_index_token_stream(element);
                                acc.push(quote::quote! {#field_index_token_stream});
                            }
                            acc
                        };
                        quote::quote! {#(#fields_token_stream),*}
                    }
                );
                let visit_map_fields_initialization_token_stream = {
                    let generate_mut_field_index_serde_private_option_token_stream = |index: std::primitive::usize, type_token_stream: &dyn quote::ToTokens| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            let mut #field_index_token_stream: serde::__private::Option<
                                std::option::Option<#import_path::Value<#type_token_stream>>,
                            > = serde::__private::None;
                        }
                    };
                    let visit_map_fields_initialization_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_mut_field_index_serde_private_option_token_stream(
                            index,
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    quote::quote! {
                        #(#visit_map_fields_initialization_token_stream)*
                    }
                };
                let visit_map_match_variants_token_stream = {
                    let generate_field_initialization_token_stream = |index: std::primitive::usize, field_ident_double_quotes_token_stream: &dyn quote::ToTokens, type_token_stream: &dyn quote::ToTokens| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            __Field::#field_index_token_stream => {
                                if serde::__private::Option::is_some(&#field_index_token_stream) {
                                    return serde::__private::Err(
                                        <__A::Error as serde::de::Error>::duplicate_field(#field_ident_double_quotes_token_stream),
                                    );
                                }
                                #field_index_token_stream = serde::__private::Some(
                                    serde::de::MapAccess::next_value::<std::option::Option<#import_path::Value<#type_token_stream>>>(&mut __map)?,
                                );
                            }
                        }
                    };
                    let visit_map_match_variants_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_field_initialization_token_stream(
                            index,
                            &generate_field_ident_double_quotes_token_stream(element),
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    quote::quote! {
                        #(#visit_map_match_variants_token_stream)*
                    }
                };
                let visit_map_missing_fields_check_token_stream = {
                    let visit_map_missing_fields_check_token_stream = current_vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        quote::quote! {
                            let #field_index_token_stream = match #field_index_token_stream {
                                serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                                serde::__private::None => {
                                    serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
                                }
                            };
                        }
                    });
                    quote::quote! {
                        #(#visit_map_missing_fields_check_token_stream)*
                    }
                };
                let fields_array_elements_token_stream = {
                    let fields_array_elements_token_stream = current_vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(element));
                    quote::quote! {
                        #(#fields_array_elements_token_stream),*
                    }
                };
                let ident_read_or_ident_with_id_read_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_token_stream);
                quote::quote! {
                    impl<'de> serde::Deserialize<'de> for #ident_token_stream {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                #(#field_enum_variants_token_stream),*,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl serde::de::Visitor<'_> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __f: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __f,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> serde::__private::Result<Self::Value, __E>
                                where
                                    __E: serde::de::Error,
                                {
                                    match __value {
                                        #(#visit_u64_value_enum_variants_token_stream),*,
                                        _ => serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> serde::__private::Result<Self::Value, __E>
                                where
                                    __E: serde::de::Error,
                                {
                                    match __value {
                                        #visit_str_value_enum_variants_token_stream
                                        _ => serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> serde::__private::Result<Self::Value, __E>
                                where
                                    __E: serde::de::Error,
                                {
                                    match __value {
                                        #visit_bytes_value_enum_variants_token_stream
                                        _ => serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> serde::__private::Result<Self, __D::Error>
                                where
                                    __D: serde::Deserializer<'de>,
                                {
                                    serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<
                                    #ident_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #ident_token_stream;
                                fn expecting(
                                    &self,
                                    __f: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __f,
                                        #struct_ident_options_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: serde::de::SeqAccess<'de>,
                                {
                                    #visit_seq_fields_initialization_token_stream
                                    #match_try_new_in_deserialize_token_stream
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: serde::de::MapAccess<'de>,
                                {
                                    #visit_map_fields_initialization_token_stream
                                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            #visit_map_match_variants_token_stream
                                            _ => {
                                                let _ = serde::de::MapAccess::next_value::<
                                                    serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    #visit_map_missing_fields_check_token_stream
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[#fields_array_elements_token_stream];
                            serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #ident_read_or_ident_with_id_read_upper_camel_case_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #ident_token_stream,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let maybe_impl_serde_deserialize_for_ident_read_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_serde_deserialize_for_ident_read_token_stream(&IsStandartWithId::False),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                },
                PostgresqlJsonObjectTypePattern::Array => proc_macro2::TokenStream::new(),
            };
            let maybe_impl_impl_std_default_default_for_ident_read_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_std_default_default_token_stream::generate_std_default_default_token_stream(
                    &ident_read_upper_camel_case,
                    &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                ),
            };
            let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = |is_standart_with_id: &IsStandartWithId|{
                postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &match &is_standart_with_id {
                        IsStandartWithId::False => &ident_read_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_read_standart_not_null_upper_camel_case
                    },
                    &proc_macro2::TokenStream::new(),
                    &{
                        let fields_token_stream = get_vec_syn_field(is_standart_with_id).iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            quote::quote! {
                                #field_ident: Some(#import_path::Value {
                                    value: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                })
                            }
                        });
                        quote::quote! {Self{#(#fields_token_stream),*}}
                    },
                )
            };
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream(&IsStandartWithId::False),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_read_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    ),
                },
                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_read_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &quote::quote! {
                            Self(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])
                        },
                    ),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_read_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    ),
                },
            };
            let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_read_upper_camel_case);
            let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(&ident_read_upper_camel_case);
            let maybe_ident_with_id_read_token_stream = if is_standart_not_null {
                let ident_with_id_read_standart_not_null_token_stream = generate_ident_read_token_stream(
                    &ident_with_id_read_standart_not_null_upper_camel_case,
                    &{
                        let content_token_stream = generate_ident_read_or_ident_with_id_read_fields_declaration_token_stream(
                            &IsStandartWithId::True,
                            &ShouldAddSerdeOptionIsNoneAnnotation::True
                        );
                        quote::quote!{{#content_token_stream}}
                    },
                    &ShouldDeriveDefault::True,
                    &ShouldDeriveSerdeDeserialize::False
                );
                let ident_with_id_read_try_from_error_named_standart_not_null_token_stream = generate_ident_read_try_from_error_named_token_stream(&ident_with_id_read_try_from_error_named_standart_not_null_upper_camel_case);
                let impl_try_new_for_ident_with_id_read_try_from_error_named_standart_not_null_token_stream = generate_impl_try_new_for_ident_read_try_from_error_named_token_stream(&IsStandartWithId::True);
                let impl_serde_deserialize_for_ident_with_id_read_standart_not_null_token_stream = generate_impl_serde_deserialize_for_ident_read_token_stream(&IsStandartWithId::True);
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_read_standart_not_null_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream(&IsStandartWithId::True);
                let impl_sqlx_type_sqlx_postgres_for_ident_with_id_read_standart_not_null_token_stream = generate_sqlx_types_json_type_declaration_wrapper_token_stream(&ident_with_id_read_standart_not_null_upper_camel_case);
                let impl_sqlx_decode_sqlx_postgres_for_ident_with_id_read_standart_not_null_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_ident_wrapper_token_stream(
                    &ident_with_id_read_standart_not_null_upper_camel_case
                );
                quote::quote! {
                    #ident_with_id_read_standart_not_null_token_stream
                    #ident_with_id_read_try_from_error_named_standart_not_null_token_stream
                    #impl_try_new_for_ident_with_id_read_try_from_error_named_standart_not_null_token_stream
                    #impl_serde_deserialize_for_ident_with_id_read_standart_not_null_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_read_standart_not_null_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_with_id_read_standart_not_null_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_with_id_read_standart_not_null_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_read_token_stream
                #maybe_ident_read_try_from_error_named_token_stream
                #impl_new_or_try_new_for_ident_read_try_from_error_named_token_stream
                #maybe_impl_serde_deserialize_for_ident_read_token_stream
                #maybe_impl_impl_std_default_default_for_ident_read_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream
                #maybe_ident_with_id_read_token_stream
            }
        };
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_with_id_update_standart_not_null_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
        let ident_update_token_stream = {
            let ident_update_element_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident);
            let ident_update_standart_not_null_upper_camel_case = &naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_update_element_standart_not_null_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_with_id_update_element_standart_not_null_upper_camel_case = &naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);
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
            let generate_ident_update_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
                generate_unique_vec_wrapper_token_stream(match &is_standart_with_id {
                    IsStandartWithId::False => &ident_update_element_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_update_element_standart_not_null_upper_camel_case
                })
            };
            enum ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation {
                True,
                False
            }
            let generate_create_update_delete_fields_token_stream = |should_add_serde_skip_serializing_if_vec_is_empty_annotation: &ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation|{
                let maybe_serde_skip_serializing_if_vec_is_empty_token_stream = match &should_add_serde_skip_serializing_if_vec_is_empty_annotation {
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True => quote::quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
                    ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False => proc_macro2::TokenStream::new()
                };
                let ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                    &ident_with_id_standart_not_null_upper_camel_case,
                    &PostgresqlJsonTypeSubtype::Create
                );
                quote::quote!{
                    #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                    #create_snake_case: std::vec::Vec<#ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream>,
                    #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                    #update_snake_case: std::vec::Vec<#ident_with_id_update_element_standart_not_null_upper_camel_case>,
                    #maybe_serde_skip_serializing_if_vec_is_empty_token_stream
                    #delete_snake_case: std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>,
                }
            };

            let ident_update_token_stream = {
                let generate_ident_update_token_stream = |
                    should_derive_default: &ShouldDeriveDefault,
                    should_derive_serde_deserialize: &ShouldDeriveSerdeDeserialize,
                    content_token_stream: &dyn quote::ToTokens
                |{
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, #should_derive_default serde::Serialize, #should_derive_serde_deserialize utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_update_upper_camel_case #content_token_stream
                    }
                };
                let generate_std_option_option_ident_type_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                    let std_option_option_ident_token_stream = &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_token_stream);
                    quote::quote!{(#std_option_option_ident_token_stream);}
                };
                match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(
                            &ShouldDeriveDefault::True,
                            &ShouldDeriveSerdeDeserialize::True,
                            &{
                                let content_token_stream = generate_ident_update_standart_not_null_content_token_stream(&IsStandartWithId::False);
                                quote::quote!{(#content_token_stream);}//todo maybe reuse scope wrapper
                            }
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(
                            &ShouldDeriveDefault::False,
                            &ShouldDeriveSerdeDeserialize::True,
                            &generate_std_option_option_ident_type_token_stream(&ident_update_standart_not_null_upper_camel_case),
                        ),
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_update_token_stream(
                            &ShouldDeriveDefault::True,
                            &ShouldDeriveSerdeDeserialize::False,
                            &{
                                let fields_token_stream = generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::True);
                                quote::quote!{{#fields_token_stream}}
                            },
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_update_token_stream(
                            &ShouldDeriveDefault::False,
                            &ShouldDeriveSerdeDeserialize::True,
                            &generate_std_option_option_ident_type_token_stream(
                                &generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_with_id_array_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Update
                                )
                            ),
                        ),
                    },
                }
            };

            let not_unique_id_in_json_update_array_upper_camel_case = naming::NotUniqueIdInJsonUpdateArrayUpperCamelCase;
            let not_unique_id_in_json_delete_array_upper_camel_case = naming::NotUniqueIdInJsonDeleteArrayUpperCamelCase;
            let not_unique_id_in_json_update_and_delete_arrays_upper_camel_case = naming::NotUniqueIdInJsonUpdateAndDeleteArraysUpperCamelCase;

            let ident_update_try_new_error_named_upper_camel_case = &naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
            let maybe_ident_update_try_new_error_named_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => proc_macro2::TokenStream::new(),
                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_update_try_new_error_named_upper_camel_case {
                            #not_unique_id_in_json_update_array_upper_camel_case {
                                #[eo_to_std_string_string_serialize_deserialize]
                                error: std::string::String,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                            #not_unique_id_in_json_delete_array_upper_camel_case {
                                #[eo_to_std_string_string_serialize_deserialize]
                                error: std::string::String,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                            #not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                #[eo_to_std_string_string_serialize_deserialize]
                                error: std::string::String,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                }
            };
            let maybe_impl_new_or_try_new_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_update_upper_camel_case,
                        &generate_value_type_token_stream(
                            &generate_unique_vec_wrapper_token_stream(&ident_update_element_standart_not_null_upper_camel_case)
                        ),
                        &self_value_token_stream,
                    ),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_update_upper_camel_case,
                        &generate_value_type_token_stream(
                            &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_update_standart_not_null_upper_camel_case)
                        ),
                        &self_value_token_stream,
                    ),
                },
                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                        let fields_token_stream = generate_create_update_delete_fields_token_stream(&ShouldAddSerdeSkipSerializingIfVecIsEmptyAnnotation::False);
                        let custom_serde_error_deserializing_ident_update_stringified = format!("custom serde error deserializing {ident_update_upper_camel_case}");
                        let check_not_unique_id_token_stream = {
                            let check_not_unique_id_in_update_array_token_stream = {
                                let not_unique_id_in_json_update_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique id in json update array: {{}}")
                                );
                                quote::quote! {
                                    let update_acc = {
                                        let mut update_acc = vec![];
                                        for element in &update {
                                            let #id_snake_case = &element.#id_snake_case;
                                            if update_acc.contains(&#id_snake_case) {
                                                return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_update_array_upper_camel_case {
                                                    error: format!(#not_unique_id_in_json_update_array_double_quotes_token_stream, #id_snake_case.get_inner()),
                                                    code_occurence: error_occurence_lib::code_occurence!()
                                                });
                                            } else {
                                                update_acc.push(#id_snake_case);
                                            }
                                        }
                                        update_acc
                                    };
                                }
                            };
                            let check_not_unique_id_in_delete_aray_token_stream = {
                                let not_unique_id_in_json_delete_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json delete array: {{}}")
                                );
                                quote::quote! {
                                    let delete_acc = {
                                        let mut delete_acc = vec![];
                                        for element in &delete {
                                            if delete_acc.contains(&element) {
                                                return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_delete_array_upper_camel_case {
                                                    error: format!(#not_unique_id_in_json_delete_array_double_quotes_token_stream, element.get_inner()),
                                                    code_occurence: error_occurence_lib::code_occurence!()
                                                });
                                            } else {
                                                delete_acc.push(element);
                                            }
                                        }
                                        delete_acc
                                    };
                                }
                            };
                            let check_not_unique_id_in_update_and_delete_arrays_token_stream = {
                                let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &format!("{custom_serde_error_deserializing_ident_update_stringified}: not unique {id_snake_case} in json update and delete arrays: {{}}")
                                );
                                quote::quote! {
                                    for element in update_acc {
                                        if delete_acc.contains(&&element) {
                                            return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                                error: format!(#not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream, element.get_inner()),
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
                        quote::quote! {
                            impl #ident_update_upper_camel_case {
                                pub fn try_new(#fields_token_stream) -> Result<Self, #ident_update_try_new_error_named_upper_camel_case> {
                                    #check_not_unique_id_token_stream
                                    Ok(Self {
                                        #create_snake_case,
                                        #update_snake_case,
                                        #delete_snake_case
                                    })
                                }
                            }
                        }
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_impl_new_for_ident_token_stream(
                        &ident_update_upper_camel_case,
                        &generate_value_type_token_stream(
                            &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(
                                &generate_type_as_postgresql_json_type_subtype_token_stream(
                                    &ident_with_id_array_not_null_upper_camel_case,
                                    &PostgresqlJsonTypeSubtype::Update
                                )
                            )
                        ),
                        &self_value_token_stream,
                    ),
                }
            };
            let maybe_impl_serde_deserialize_for_ident_update_token_stream = match &postgresql_json_object_type_pattern {
                PostgresqlJsonObjectTypePattern::Standart => proc_macro2::TokenStream::new(),
                PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                        let tuple_struct_ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("tuple struct {ident_update_upper_camel_case}"));
                        let ident_update_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_update_upper_camel_case);
                        let match_try_new_in_deserialize_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(&ident_update_upper_camel_case, &quote::quote! {__field0, __field1, __field2});
                        let ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &ident_with_id_standart_not_null_upper_camel_case,
                            &PostgresqlJsonTypeSubtype::Create
                        );
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
                                            let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream>>(&mut __seq)? {
                                                serde::__private::Some(__value) => __value,
                                                serde::__private::None => {
                                                    vec![]
                                                }
                                            };
                                            let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_with_id_update_element_standart_not_null_upper_camel_case>>(&mut __seq)? {
                                                serde::__private::Some(__value) => __value,
                                                serde::__private::None => {
                                                    vec![]
                                                }
                                            };
                                            let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>>(&mut __seq)? {
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
                                            let mut __field0: serde::__private::Option<std::vec::Vec<#ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream>> = serde::__private::None;
                                            let mut __field1: serde::__private::Option<std::vec::Vec<#ident_with_id_update_element_standart_not_null_upper_camel_case>> = serde::__private::None;
                                            let mut __field2: serde::__private::Option<std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>> = serde::__private::None;
                                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if serde::__private::Option::is_some(&__field0) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                        }
                                                        __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_with_id_standart_not_null_as_postgresql_json_type_create_token_stream>>(&mut __map)?);
                                                    }
                                                    __Field::__field1 => {
                                                        if serde::__private::Option::is_some(&__field1) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                        }
                                                        __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_with_id_update_element_standart_not_null_upper_camel_case>>(&mut __map)?);
                                                    }
                                                    __Field::__field2 => {
                                                        if serde::__private::Option::is_some(&__field2) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                        }
                                                        __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>>(&mut __map)?);
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
                                                    vec![]
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
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new()
                },
            };
            let maybe_impl_default_default_for_ident_update_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_std_default_default_token_stream::generate_std_default_default_token_stream(
                    &ident_update_upper_camel_case,
                    &self_some_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                ),
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
                            #update_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                            #delete_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                        }},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            (Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))
                        },
                    },
                };
                quote::quote! {Self #value}
            });
            let generate_update_query_part_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
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
                    let ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                        IsStandartWithId::False => &ident_update_element_upper_camel_case,
                        IsStandartWithId::True => &ident_with_id_update_element_standart_not_null_upper_camel_case
                    };
                    quote::quote! {
                        #ident_token_stream::#variant_ident_upper_camel_case_token_stream(value) => {
                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
                                &value.value,
                                &#object_acc_snake_case,
                                &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                #field_ident_double_quotes_token_stream,
                                #increment_snake_case,
                            ) {
                                Ok(value) => {
                                    #object_acc_snake_case = value;
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                    }
                });
                let some_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end"));
                quote::quote! {
                    let mut #object_acc_snake_case = format!(#some_format_handle_token_stream);
                    #generate_jsonb_set_target_token_stream
                    for element in self.0.to_vec() {
                        match element {
                            #(#query_part_variants_token_stream),*
                        }
                    }
                    if #jsonb_set_accumulator_snake_case.is_empty() && #jsonb_set_path_snake_case.is_empty() {
                        Ok(#object_acc_snake_case)
                    }
                    else {
                        Ok(format!(#format_handle_token_stream))
                    }
                }
            };
            let generate_update_query_bind_standart_not_null_content_token_stream = |is_standart_with_id: &IsStandartWithId|{
                let ident_token_stream: &dyn quote::ToTokens = match &is_standart_with_id {
                    IsStandartWithId::False => &ident_update_element_upper_camel_case,
                    IsStandartWithId::True => &ident_with_id_update_element_standart_not_null_upper_camel_case
                };
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
                        #ident_token_stream::#variant_ident_upper_camel_case_token_stream(value) => {
                            #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                        }
                    }
                });
                quote::quote! {
                    for element in self.0.into_vec() {
                        match element {
                            #(#update_query_bind_variants_token_stream),*
                        }
                    }
                    #query_snake_case
                }
            };
            let generate_impl_ident_update_token_stream = |
                ident_token_stream: &dyn quote::ToTokens,
                update_query_part_content_token_stream: &dyn quote::ToTokens,
                is_query_bind_mutable: postgresql_crud_macros_common::IsQueryBindMutable,
                update_query_bind_content_token_stream: &dyn quote::ToTokens,
            |{
                quote::quote!{
                    impl #ident_token_stream {
                        fn #update_query_part_snake_case(
                            &self,
                            #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                            #jsonb_set_target_snake_case: #reference_std_primitive_str_token_stream,
                            #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                            #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
                        ) -> Result<#std_string_string_token_stream, #import_path_query_part_error_named_token_stream> {
                            #update_query_part_content_token_stream
                        }
                        fn #update_query_bind_snake_case(
                            self,
                            #is_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
                        ) -> #query_postgres_arguments_token_stream {
                            #update_query_bind_content_token_stream
                        }
                    }
                }

            };
            let impl_ident_update_token_stream = generate_impl_ident_update_token_stream(
                &ident_update_upper_camel_case,
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_update_query_part_standart_not_null_content_token_stream(&IsStandartWithId::False),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let none_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})"));
                            quote::quote!{
                                match &self.0 {
                                    Some(#value_snake_case) => #value_snake_case.#update_query_part_snake_case(
                                        jsonb_set_accumulator,
                                        jsonb_set_target,
                                        jsonb_set_path,
                                        increment,
                                    ),
                                    None => {
                                        match #increment_snake_case.checked_add(1) {
                                            Some(value) => {
                                                *#increment_snake_case = value;
                                                Ok(format!(#none_format_handle_token_stream))
                                            },
                                            None => Err(#import_path_query_part_error_named_checked_add_initialization_token_stream)
                                        }
                                    }
                                }
                            }
                        },
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            quote::quote! {
                                let update_query_part_acc = {
                                    if self.update.is_empty() {
                                        std::string::String::from("elem")
                                    } else {
                                        let mut update_query_part_acc = std::string::String::default();
                                        for element_handle in &self.update {
                                            match element_handle.update_query_part(&"", &"elem", &"", increment) {
                                                Ok(value) => {
                                                    update_query_part_acc.push_str(&value);
                                                }
                                                Err(error) => {
                                                    return Err(error);
                                                }
                                            }
                                        }
                                        let _ = update_query_part_acc.pop();
                                        format!("case {update_query_part_acc} else elem end")
                                    }
                                };
                                let delete_query_part_acc = {
                                    let mut delete_query_part_acc = std::string::String::default();
                                    for _ in &self.delete {
                                        match increment.checked_add(1) {
                                            Some(value) => {
                                                *increment = value;
                                                let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                                                delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                                            }
                                            None => {
                                                return Err(#import_path_query_part_error_named_checked_add_initialization_token_stream);
                                            }
                                        }
                                    }
                                    delete_query_part_acc
                                };
                                let create_query_part_acc = {
                                    let mut create_query_part_acc = std::string::String::default();
                                    for element in &self.create {
                                        match element.create_query_part(increment) {
                                            Ok(value) => {
                                                create_query_part_acc.push_str(&format!("{value},"));
                                            }
                                            Err(error) => {
                                                return Err(error);
                                            }
                                        }
                                    }
                                    let _ = create_query_part_acc.pop();
                                    create_query_part_acc
                                };
                                let maybe_where = if self.delete.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
                                let maybe_jsonb_build_array = if self.create.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
                                Ok (format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"))
                            }
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match &self.0 {
                                Some(value) => value.update_query_part(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
                                None => match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                                    }
                                    None => Err(#import_path_query_part_error_named_checked_add_initialization_token_stream),
                                },
                            }
                        },
                    },
                },
                match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsQueryBindMutable::True,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsQueryBindMutable::False,
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsQueryBindMutable::True,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsQueryBindMutable::False,
                    },
                },
                &match &postgresql_json_object_type_pattern {
                    PostgresqlJsonObjectTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_update_query_bind_standart_not_null_content_token_stream(&IsStandartWithId::False),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                            match self.0 {
                                Some(#value_snake_case) => #value_snake_case.#update_query_bind_snake_case(query),
                                None => #query_snake_case.bind(sqlx::types::Json(#ident_update_upper_camel_case(None))),
                            }
                        },
                    },
                    PostgresqlJsonObjectTypePattern::Array => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                            for element in self.#update_snake_case {
                                #query_snake_case = element.update_query_bind(#query_snake_case);
                            }
                            for element in self.delete {
                                #query_snake_case = element.query_bind_as_postgresql_text(#query_snake_case);
                            }
                            for element in self.create {
                                #query_snake_case = element.create_query_bind(#query_snake_case);
                            }
                            #query_snake_case
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match self.0 {
                                Some(#value_snake_case) => #value_snake_case.update_query_bind(#query_snake_case),
                                None => #query_snake_case.bind(sqlx::types::Json(Self(None))),
                            }
                        },
                    },
                }
            );
            let maybe_ident_with_id_update_token_stream = if is_standart_not_null {
                quote::quote! {
                    pub type #ident_with_id_update_standart_not_null_upper_camel_case = #ident_update_standart_not_null_upper_camel_case;
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let maybe_ident_update_element_token_stream = if is_standart_not_null {
                let ident_update_element_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        let field_type_as_json_type_update_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::Update
                        );
                        quote::quote! {
                            #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                            #variant_ident_upper_camel_case_token_stream(#import_path::Value<
                                #field_type_as_json_type_update_token_stream
                            >)
                        }
                    });
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub enum #ident_update_element_standart_not_null_upper_camel_case {
                            #(#variants_token_stream),*
                        }
                    }
                };
                let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_update_element_standart_not_null_upper_camel_case,
                    &{
                        let elements_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            quote::quote! {
                                #ident_update_element_standart_not_null_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#import_path::Value {
                                    value: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                })
                            }
                        });
                        quote::quote! {vec![#(#elements_token_stream),*]}
                    }
                );
                quote::quote! {
                    #ident_update_element_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let maybe_ident_with_id_update_element_standart_not_null_token_stream = if is_standart_not_null {
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
                let ident_with_id_update_element_standart_not_null_fields_declaration_token_stream = {
                    let fields_type_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &ident_standart_not_null_upper_camel_case,
                        &PostgresqlJsonTypeSubtype::Update
                    );
                    quote::quote!{
                        #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream,
                        #fields_snake_case: #fields_type_token_stream
                    }
                };
                let ident_with_id_update_element_standart_not_null_token_stream = quote::quote! {
                    #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                    pub struct #ident_with_id_update_element_standart_not_null_upper_camel_case {
                        #ident_with_id_update_element_standart_not_null_fields_declaration_token_stream
                    }
                };
                let impl_new_for_ident_with_id_update_element_standart_not_null_token_stream = macros_helpers::generate_impl_new_for_ident_token_stream(
                    &ident_with_id_update_element_standart_not_null_upper_camel_case,
                    &ident_with_id_update_element_standart_not_null_fields_declaration_token_stream,
                    &quote::quote!{Self {
                        #id_snake_case,
                        #fields_snake_case
                    }},
                );
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_update_element_standart_not_null_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_with_id_update_element_standart_not_null_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {Self {
                        #id_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                        #fields_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    }}
                );
                let impl_ident_with_id_update_element_standart_not_null_token_stream = generate_impl_ident_update_token_stream(
                    &ident_with_id_update_element_standart_not_null_upper_camel_case,
                    &quote::quote!{
                        let id_increment = match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                increment.to_string()
                            }
                            None => {
                                return Err(#import_path_query_part_error_named_checked_add_initialization_token_stream);
                            }
                        };
                        match self.fields.#update_query_part_snake_case(&jsonb_set_accumulator, &jsonb_set_target, &jsonb_set_path, increment) {
                            Ok(value) => Ok(format!("when {jsonb_set_target}->>'id' = ${id_increment} then {value} ")),
                            Err(error) => Err(error)
                        }
                    },
                    postgresql_crud_macros_common::IsQueryBindMutable::True,
                    &quote::quote!{
                        query = self.id.query_bind_as_postgresql_text(query);
                        query = self.fields.#update_query_bind_snake_case(query);
                        query
                    },
                );
                quote::quote! {
                    #ident_with_id_update_element_standart_not_null_token_stream
                    #impl_new_for_ident_with_id_update_element_standart_not_null_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_update_element_standart_not_null_token_stream
                    #impl_ident_with_id_update_element_standart_not_null_token_stream
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_update_token_stream
                #maybe_ident_update_try_new_error_named_token_stream
                #maybe_impl_new_or_try_new_for_ident_update_token_stream
                #maybe_impl_serde_deserialize_for_ident_update_token_stream
                #maybe_impl_default_default_for_ident_update_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
                #impl_ident_update_token_stream
                #maybe_ident_with_id_update_token_stream
                #maybe_ident_update_element_token_stream
                #maybe_ident_with_id_update_element_standart_not_null_token_stream
            }
        };
        let update_query_part_token_stream = quote::quote!{
            #value_snake_case.#update_query_part_snake_case(
                #jsonb_set_accumulator_snake_case,
                #jsonb_set_target_snake_case,
                #jsonb_set_path_snake_case,
                #increment_snake_case,
            )
        };
        let update_query_bind_token_stream = quote::quote!{#value_snake_case.#update_query_bind_snake_case(#query_snake_case)};
        let (
            maybe_impl_postgresql_crud_postgresql_json_type_for_ident_token_stream,
            maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
        ) = {
            let impl_postgresql_crud_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                &ident,
                &ident_table_type_declaration_upper_camel_case,
                &ident_create_upper_camel_case,
                &create_query_part_token_stream,
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                &create_query_bind_token_stream,
                &ident_select_upper_camel_case,
                &ident_read_upper_camel_case,
                &select_query_part_token_stream,
                &ident_where_element_upper_camel_case,
                &ident_update_upper_camel_case,
                &update_query_part_token_stream,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::False,
                &update_query_bind_token_stream,
            );
            let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_type_for_ident_token_stream(
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                &ident,
                &ident_table_type_declaration_upper_camel_case,
                &ident_create_upper_camel_case,
                &create_query_part_token_stream,
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                &create_query_bind_token_stream,
                &ident_select_upper_camel_case,
                &quote::quote!{
                    format!(
                        "{} as {column}",
                        {
                            let field_ident = column;
                            let column_name_and_maybe_field_getter = column;
                            let column_name_and_maybe_field_getter_for_error_message = column;
                            let is_postgresql_type = true;
                            #select_query_part_token_stream
                        }
                    )
                },
                &ident_where_element_upper_camel_case,
                &ident_read_upper_camel_case,
                &ident_update_upper_camel_case,
                &update_query_part_token_stream,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::False,
                &update_query_bind_token_stream
            );
            match &trait_gen {
                TraitGen::PostgresqlTypeAndPostgresqlJsonType => (
                    impl_postgresql_crud_postgresql_json_type_for_ident_token_stream,
                    impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
                ),
                TraitGen::PostgresqlJsonType => (
                    impl_postgresql_crud_postgresql_json_type_for_ident_token_stream,
                    proc_macro2::TokenStream::new(),
                ),
            }
        };
        let maybe_impl_postgresql_crud_postgresql_json_type_for_ident_with_id_not_null_token_stream = if is_standart_not_null {
            postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                &ident_with_id_standart_not_null_upper_camel_case,
                &ident_with_id_table_type_declaration_standart_not_null_upper_camel_case,
                &ident_with_id_create_standart_not_null_upper_camel_case,
                &create_query_part_token_stream,
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                &create_query_bind_token_stream,
                &ident_with_id_select_standart_not_null_upper_camel_case,
                &ident_with_id_read_standart_not_null_upper_camel_case,
                &select_query_part_token_stream,
                &ident_with_id_where_element_standart_not_null_upper_camel_case,
                &ident_with_id_update_standart_not_null_upper_camel_case,
                &update_query_part_token_stream,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::False,
                &update_query_bind_token_stream,
            )
        }
        else {
            proc_macro2::TokenStream::new()
        };
        //todo impl new or try new without inner wrapper types. (vec<Option<bool>> instead of VecOfOptionBool(OptionBool(Bool)))
        let generated = quote::quote! {
            #ident_token_stream
            #ident_table_type_declaration_token_stream
            #ident_create_token_stream
            #ident_select_token_stream
            #ident_where_element_token_stream
            #ident_read_token_stream
            #ident_update_token_stream
            #maybe_impl_postgresql_crud_postgresql_json_type_for_ident_token_stream
            #maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
            #maybe_impl_postgresql_crud_postgresql_json_type_for_ident_with_id_not_null_token_stream
        };
        // if let (
        //     postgresql_crud_macros_common::NotNullOrNullable::NotNull,
        //     // postgresql_crud_macros_common::NotNullOrNullable::Nullable,

        //     PostgresqlJsonObjectTypePattern::Standart,
        //     // PostgresqlJsonObjectTypePattern::Array,

        //     TraitGen::PostgresqlJsonType,
        //     // TraitGen::PostgresqlTypeAndPostgresqlJsonType,
        // ) = (
        //     &not_null_or_nullable,
        //     &postgresql_json_object_type_pattern,
        //     &trait_gen,
        // ) {
        //     if syn_derive_input_ident == "Animal" {//"Animal" // "Doggie"
        //         macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //             "GeneratePostgresqlJsonObjectType",
        //             &generated,
        //         );
        //     }
        // }
        generated
    });
    let generated = quote::quote!{#(#postgresql_json_object_type_array)*};
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlJsonObjectTypeTokens",
    //     &generated,
    // );
    generated.into()
}
