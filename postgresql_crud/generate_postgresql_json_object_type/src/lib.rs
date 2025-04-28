//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
#[proc_macro_attribute]
pub fn postgresql_json_object_type_pattern(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_derive(GeneratePostgresqlJsonObjectType)]
pub fn generate_postgresql_json_object_type(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    enum TraitGen {
        PostgresqlType,
        PostgresqlJsonType,
        PostgresqlTypeAndPostgresqlJsonType
    }
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct PostgresqlJsonObjectTypeRecord {
        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern,
        trait_gen: TraitGen,
    }
    impl PostgresqlJsonObjectTypeRecord {
        fn all() -> std::vec::Vec<Self> {
            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().fold(vec![], |mut acc, not_null_or_nullable| {
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::all_variants().into_iter().for_each(|postgresql_json_type_pattern| {
                    acc.push(PostgresqlJsonObjectTypeRecord {
                        not_null_or_nullable: not_null_or_nullable.clone(),
                        postgresql_json_type_pattern,
                        trait_gen: TraitGen::PostgresqlTypeAndPostgresqlJsonType
                    });
                });
                acc
            })
        }
    }
    let postgresql_json_object_type_record_vec = {
        if true {
            PostgresqlJsonObjectTypeRecord::all()
        }
        else {
            let vec = serde_json::from_str::<std::vec::Vec<PostgresqlJsonObjectTypeRecord>>(
                &macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
                    &{
                        let syn_derive_input: syn::DeriveInput = syn::parse(input_token_stream.clone()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
                        syn_derive_input.attrs
                    },
                    &"postgresql_crud::postgresql_json_object_type_pattern".to_string()
                ).to_string()
            )
            .expect("failed to get Config for generate_postgresql_json_object_type");
            let mut acc = vec![];
            for element in &vec {
                if acc.contains(&element) {
                    panic!("not unique postgersql type provided: {element:#?}");
                }
                else {
                    acc.push(&element);
                }
            }
            vec
        }
    }
    .into_iter()
    .filter(|element|{
        use postgresql_crud_macros_common::NotNullOrNullable;
        let not_null_or_nullable_filter = match &element.not_null_or_nullable {
            NotNullOrNullable::NotNull => true,
            NotNullOrNullable::Nullable => false,
        };
        let postgresql_json_type_pattern_filter = match &element.postgresql_json_type_pattern {
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => true,
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable,
            } => match &dimension1_not_null_or_nullable {
                NotNullOrNullable::NotNull => false,
                NotNullOrNullable::Nullable => false,
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                dimension1_not_null_or_nullable,
                dimension2_not_null_or_nullable,
            } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                dimension1_not_null_or_nullable,
                dimension2_not_null_or_nullable,
                dimension3_not_null_or_nullable,
            } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                dimension1_not_null_or_nullable,
                dimension2_not_null_or_nullable,
                dimension3_not_null_or_nullable,
                dimension4_not_null_or_nullable,
            } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            }
        };
        let trait_gen_filter = match &element.trait_gen {
            TraitGen::PostgresqlType => true,
            TraitGen::PostgresqlJsonType => true,
            TraitGen::PostgresqlTypeAndPostgresqlJsonType => true
        };
        not_null_or_nullable_filter && postgresql_json_type_pattern_filter && trait_gen_filter
    })
    .collect::<std::vec::Vec<PostgresqlJsonObjectTypeRecord>>();
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonObjectTypeJsonVariants",
    //     &serde_json::to_string(&postgresql_json_type_record_vec).unwrap(),
    // );
    use rayon::iter::ParallelIterator;
    use rayon::iter::IntoParallelRefIterator;
    let postgresql_json_object_type_array = postgresql_json_object_type_record_vec
    .into_iter().map(|element|(element, input_token_stream.to_string())).collect::<std::vec::Vec<(PostgresqlJsonObjectTypeRecord, std::string::String)>>()
    .par_iter()
    // .into_iter()//just for console prints ordering
    .map(|(element, input_token_stream_stringified)|{
        let not_null_or_nullable = &element.not_null_or_nullable;
        let postgresql_json_type_pattern = &element.postgresql_json_type_pattern;
        let trait_gen = &element.trait_gen;

        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

        let value_snake_case = naming::ValueSnakeCase;
        let as_upper_camel_case = naming::AsUpperCamelCase;
        let none_upper_camel_case = naming::NoneUpperCamelCase;

        let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

        let syn_derive_input: syn::DeriveInput = syn::parse(
            <proc_macro::TokenStream as std::str::FromStr>::from_str(&input_token_stream_stringified).unwrap()
        ).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
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

        enum StandartWithId {
            True,
            False
        }
        let generate_ident_token_stream = |
            not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
            postgresql_json_type_pattern: &postgresql_crud_macros_common::PostgresqlJsonTypePattern,
            standart_with_id: StandartWithId,
        |{
            let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
            let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
            let jsonb_object_upper_camel_case = naming::JsonbObjectUpperCamelCase;
            let with_id_upper_camel_case = naming::WithIdUpperCamelCase;
            let not_null_or_nullable_rust = not_null_or_nullable.rust();
            let (rust_part, postgresql_part) = match &postgresql_json_type_pattern {
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => {
                    let maybe_with_id: &dyn std::fmt::Display  = match &standart_with_id {
                        StandartWithId::True => &naming::WithIdUpperCamelCase,
                        StandartWithId::False => &"",
                    };
                    (
                        format!("{syn_derive_input_ident}"),
                        format!("{jsonb_object_upper_camel_case}")
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                    dimension1_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{syn_derive_input_ident}{with_id_upper_camel_case}"),
                        format!("{array_of_upper_camel_case}{d1}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}")
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{syn_derive_input_ident}{with_id_upper_camel_case}"),
                        format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}")
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{syn_derive_input_ident}{with_id_upper_camel_case}"),
                        format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}")
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    let d4 = dimension4_not_null_or_nullable;
                    let d4_rust = dimension4_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{syn_derive_input_ident}{with_id_upper_camel_case}"),
                        format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{jsonb_object_upper_camel_case}{with_id_upper_camel_case}")
                    )
                },
            };
            format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{not_null_or_nullable}{postgresql_part}")
            .parse::<proc_macro2::TokenStream>().unwrap()
        };
        let ident = &generate_ident_token_stream(&not_null_or_nullable, &postgresql_json_type_pattern, StandartWithId::False);
        let ident_with_id_upper_camel_case = naming::parameter::SelfWithIdUpperCamelCase::from_tokens(&ident);
        let ident_standart_not_null_upper_camel_case = &generate_ident_token_stream(
            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            &postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart,
            StandartWithId::False,
        );
        let ident_with_id_standart_not_null_upper_camel_case = naming::parameter::SelfWithIdUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
        let ident_with_id_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_with_id_upper_camel_case);
        let ident_origin_standart_not_null_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_with_id_origin_standart_not_null_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_with_id_standart_not_null_upper_camel_case);


        let ident_to_create_with_generated_id_upper_camel_case = naming::parameter::SelfToCreateWithGeneratedIdUpperCamelCase::from_tokens(&ident);
        // let ident_to_create_without_generated_id_upper_camel_case = naming::parameter::SelfToCreateWithoutGeneratedIdUpperCamelCase::from_tokens(&ident);

        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);

        let ident_select_without_id_upper_camel_case = naming::parameter::SelfSelectWithoutIdUpperCamelCase::from_tokens(&ident);
        let ident_select_with_id_upper_camel_case = naming::parameter::SelfSelectWithIdUpperCamelCase::from_tokens(&ident);

        // let ident_read_without_id_upper_camel_case = naming::parameter::SelfReadWithoutIdUpperCamelCase::from_tokens(&ident);
        // let ident_read_with_id_upper_camel_case = naming::parameter::SelfReadWithIdUpperCamelCase::from_tokens(&ident);

        let ident_update_element_upper_camel_case = naming::parameter::SelfUpdateElementUpperCamelCase::from_tokens(&ident);

        let ident_update_with_id_upper_camel_case = naming::parameter::SelfUpdateWithIdUpperCamelCase::from_tokens(&ident);

        let value_snake_case = naming::ValueSnakeCase;

        let postgresql_crud_path_token_stream = {
            let postgresql_crud_snake_case = naming::PostgresqlCrudSnakeCase;
            quote::quote! {#postgresql_crud_snake_case::}
        };
        #[derive(Debug, strum_macros::Display)]
        enum PostgresqlJsonTypeSubtype {
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
                .to_tokens(tokens)
            }
        }
        let generate_type_as_postgresql_json_type_subtype_token_stream = |
            type_token_stream: &dyn quote::ToTokens,
            postgresql_json_type_subtype: &PostgresqlJsonTypeSubtype
        |{
            quote::quote! {<#type_token_stream as postgresql_crud::PostgresqlJsonType>::#postgresql_json_type_subtype}
        };

        let (
            postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
            postgresql_crud_path_postgresql_json_type_uuid_uuid_select_token_stream,
            postgresql_crud_path_postgresql_json_type_uuid_uuid_read_token_stream,
            postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream
        ) = {
            let postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream = quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type:: UuidUuidAsNotNullJsonbString};
            (
                postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream.clone(),
                generate_type_as_postgresql_json_type_subtype_token_stream(
                    &postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
                    &PostgresqlJsonTypeSubtype::Select
                ),
                generate_type_as_postgresql_json_type_subtype_token_stream(
                    &postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
                    &PostgresqlJsonTypeSubtype::Read
                ),
                generate_type_as_postgresql_json_type_subtype_token_stream(
                    &postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
                    &PostgresqlJsonTypeSubtype::Update
                ),
            )
        };

        let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        // let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

        fn generate_supported_generics_template_struct_token_stream(is_pub: std::primitive::bool, struct_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            let maybe_pub_token_stream = if is_pub {
                quote::quote! {pub}
            } else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                #maybe_pub_token_stream struct #struct_ident_token_stream #content_token_stream
            }
        }
        fn generate_match_try_new_in_deserialize_token_stream(ident_token_stream: &dyn quote::ToTokens, initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {
                match #ident_token_stream::try_new(#initialization_token_stream) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }

        let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
        let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let increments_snake_case = naming::IncrementsSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;

        let postgresql_crud_wrap_into_jsonb_build_object_token_stream = {
            let wrap_into_jsonb_build_object_snake_case = naming::WrapIntoJsonbBuildObjectSnakeCase;
            quote::quote! {#postgresql_crud_path_token_stream #wrap_into_jsonb_build_object_snake_case}
        };
        let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
            generate_quotes::double_quotes_token_stream(&value.ident.as_ref().unwrap_or_else(|| {
                panic!("{}", naming::FIELD_IDENT_IS_NONE);
            }))
        };
        let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
        let generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
            quote::quote! {<#value_token_stream as #postgresql_crud_path_token_stream #postgresql_json_type_upper_camel_case>::}
        };
        let generate_field_type_as_crud_postgresql_json_type_from_field_token_stream = |field: &syn::Field| generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream(&field.ty);
        let id_snake_case = naming::IdSnakeCase;
        let id_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&id_snake_case);

        enum IsPub {
            True,
            False
        }
        let generate_fields_declaration_create_token_stream = |is_pub: IsPub|{
            let maybe_pub_token_stream = match &is_pub {
                IsPub::True => quote::quote!{pub},
                IsPub::False => proc_macro2::TokenStream::new(),
            };
            let value = vec_syn_field.iter().map(|element| {
                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                });
                let type_as_postgresql_json_type_subtype_create_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                    &element.ty,
                    &PostgresqlJsonTypeSubtype::Create
                );
                quote::quote! {
                    #maybe_pub_token_stream #field_ident: #type_as_postgresql_json_type_subtype_create_token_stream
                }
            });
            quote::quote!{{#(#value),*}}
        };

        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_fields_token_stream = {
            let fields_token_stream = vec_syn_field.iter().map(|element| {
                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                });
                quote::quote! {
                    #field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }
            });
            quote::quote! {{#(#fields_token_stream),*}}
        };

        enum PostgresqlJsonObjectType {
            WithoutId,
            WithId,
        }

        let pub_field_idents_field_types_token_stream = {
            let fields_token_stream = vec_syn_field.iter().map(|element| {
                let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                });
                let element_type = &element.ty;
                quote::quote! {pub #element_ident: #element_type}
            });
            quote::quote! {#(#fields_token_stream),*}
        };
        let ident_token_stream = quote::quote! {
            #[derive(Debug)]
            pub struct #ident;
        };

        let generate_field_type_handle = |standart_not_null_fields_initialization_token_stream: &dyn quote::ToTokens, prefix_wrapper: fn(&dyn quote::ToTokens) -> proc_macro2::TokenStream|{
            let wrap_into_scopes_pub_token_stream = |content: &dyn quote::ToTokens|{
                quote::quote! {(pub #content);}
            };
            let generate_current_ident_prefix = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &postgresql_crud_macros_common::PostgresqlJsonTypePattern|{
                let value = prefix_wrapper(&generate_ident_token_stream(
                    &current_not_null_or_nullable,
                    &current_postgresql_json_type_pattern,
                    StandartWithId::True,
                ));
                let content = match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value)
                };
                wrap_into_scopes_pub_token_stream(&content)
            };
            match &postgresql_json_type_pattern {
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#standart_not_null_fields_initialization_token_stream},
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => wrap_into_scopes_pub_token_stream(
                        &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&prefix_wrapper(ident_standart_not_null_upper_camel_case))
                    ),
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                    dimension1_not_null_or_nullable,
                } => {
                    let (
                        current_not_null_or_nullable,
                        current_postgresql_json_type_pattern,
                    ): (
                        &postgresql_crud_macros_common::NotNullOrNullable,
                        &postgresql_crud_macros_common::PostgresqlJsonTypePattern,
                    ) = match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            &dimension1_not_null_or_nullable,
                            &postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            &postgresql_json_type_pattern,
                        )
                    };
                    generate_current_ident_prefix(
                        &current_not_null_or_nullable,
                        &current_postgresql_json_type_pattern
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                } => {
                    let (
                        current_not_null_or_nullable,
                        current_postgresql_json_type_pattern,
                    ): (
                        &postgresql_crud_macros_common::NotNullOrNullable,
                        &postgresql_crud_macros_common::PostgresqlJsonTypePattern,
                    ) = match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            &dimension1_not_null_or_nullable,
                            &postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                            },
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            &postgresql_json_type_pattern,
                        )
                    };
                    generate_current_ident_prefix(
                        &current_not_null_or_nullable,
                        &current_postgresql_json_type_pattern
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => {
                    let (
                        current_not_null_or_nullable,
                        current_postgresql_json_type_pattern,
                    ): (
                        &postgresql_crud_macros_common::NotNullOrNullable,
                        &postgresql_crud_macros_common::PostgresqlJsonTypePattern,
                    ) = match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            &dimension1_not_null_or_nullable,
                            &postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                            },
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            &postgresql_json_type_pattern,
                        )
                    };
                    generate_current_ident_prefix(
                        &current_not_null_or_nullable,
                        &current_postgresql_json_type_pattern
                    )
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => {
                    let (
                        current_not_null_or_nullable,
                        current_postgresql_json_type_pattern,
                    ): (
                        &postgresql_crud_macros_common::NotNullOrNullable,
                        &postgresql_crud_macros_common::PostgresqlJsonTypePattern,
                    ) = match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            &dimension1_not_null_or_nullable,
                            &postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                            },
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            &postgresql_json_type_pattern,
                        )
                    };
                    generate_current_ident_prefix(
                        &current_not_null_or_nullable,
                        &current_postgresql_json_type_pattern
                    )
                }
            }
        };
        let field_type_handle: &dyn quote::ToTokens = &generate_field_type_handle(
            &quote::quote!{{#pub_field_idents_field_types_token_stream}},
            |tokens|{quote::quote!{#tokens}}
        );

        let common_token_stream = {
            let read_token_stream = {
                let (ident_select_token_stream, ident_with_id_select_token_stream) = {
                    let generate_template_select_struct_token_stream = |tokens_select_token_stream: &dyn quote::ToTokens, additional_content_token_stream: &dyn quote::ToTokens| {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
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
                            pub enum #tokens_select_token_stream {
                                #additional_content_token_stream
                                #(#variants_token_stream),*
                            }
                        }
                    };
                    let ident_select_without_id_token_stream = generate_template_select_struct_token_stream(&ident_select_without_id_upper_camel_case, &proc_macro2::TokenStream::new());
                    let ident_select_with_id_token_stream = generate_template_select_struct_token_stream(
                        &ident_select_with_id_upper_camel_case,
                        &quote::quote! {
                            #[serde(rename(serialize = #id_snake_case_double_quotes_token_stream, deserialize = #id_snake_case_double_quotes_token_stream))]
                             Id(#postgresql_crud_path_postgresql_json_type_uuid_uuid_select_token_stream),
                        },
                    );
                    (ident_select_without_id_token_stream, ident_select_with_id_token_stream)
                };
                let (impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream, impl_error_occurence_lib_to_std_string_string_for_ident_with_id_select_token_stream) = {
                    let content_token_stream = quote::quote! {format!("{self:?}")};
                    (
                        macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_select_without_id_upper_camel_case, &proc_macro2::TokenStream::new(), &content_token_stream),
                        macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_select_with_id_upper_camel_case, &proc_macro2::TokenStream::new(), &content_token_stream),
                    )
                };
                let (
                    impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream,
                    impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_select_token_stream,
                ) = {
                    let generate_vec_content_token_stream = |enum_ident_token_stream: &dyn quote::ToTokens| {
                        let elements_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = &element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            quote::quote! {
                                #enum_ident_token_stream::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                            }
                        });
                        quote::quote! {#(#elements_token_stream),*}
                    };
                    let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_without_id_token_stream =
                        postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_without_id_upper_camel_case, &{
                            let vec_content_token_stream = generate_vec_content_token_stream(&ident_select_without_id_upper_camel_case);
                            quote::quote! {vec![#vec_content_token_stream]}
                        });
                    // impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    //     &ident_select_without_id_upper_camel_case,
                    //     &generate_vec_content_token_stream(&ident_select_without_id_upper_camel_case)
                    // );
                    let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_with_id_token_stream =
                        postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_with_id_upper_camel_case, &{
                            let other_variants_token_stream = generate_vec_content_token_stream(&ident_select_with_id_upper_camel_case);
                            quote::quote! {
                                vec![
                                    #ident_select_with_id_upper_camel_case::Id(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                    #other_variants_token_stream
                                ]
                            }
                        });
                    (
                        impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_without_id_token_stream,
                        impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_with_id_token_stream,
                    )
                };
                quote::quote! {
                    #ident_select_token_stream
                    #ident_with_id_select_token_stream

                    #impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream

                    #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_select_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_with_id_select_token_stream
                }
            };
            let update_token_stream = {
                let ident_update_element_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        //todo maybe rename type_path to tokens for standart naming convention
                        let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        let field_type_as_json_type_update_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                            &element.ty,
                            &PostgresqlJsonTypeSubtype::Update
                        );
                        quote::quote! {
                            #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                            #variant_ident_upper_camel_case_token_stream(#postgresql_crud_path_token_stream Value<
                                #field_type_as_json_type_update_token_stream
                            >)
                        }
                    });
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub enum #ident_update_element_upper_camel_case {
                            #(#variants_token_stream),*
                        }
                    }
                };
                let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_element_upper_camel_case, &{
                        let elements_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            quote::quote! {
                                #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#postgresql_crud_path_token_stream Value {
                                    value: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                })
                            }
                        });
                        quote::quote! {vec![#(#elements_token_stream),*]}
                    });
                let ident_update_with_id_token_stream = {
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_update_with_id_upper_camel_case {
                            pub #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream,
                            pub fields: postgresql_crud::UniqueVec<#ident_update_element_upper_camel_case>
                        }
                    }
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_with_id_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_update_with_id_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &quote::quote! {Self {
                            #id_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            fields: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                        }},
                    );
                quote::quote! {
                    #ident_update_element_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_element_token_stream

                    #ident_update_with_id_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_with_id_token_stream
                }
            };
            quote::quote! {
                #read_token_stream
                #update_token_stream
            }
        };
        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
        let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
        let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
        let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
        let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
        let (generate_tuple_struct_tokens_double_quotes_token_stream, generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream) = {
            const TUPLE_STRUCT_SPACE_STRINGIFIED: &std::primitive::str = "tuple struct ";
            let generate_tuple_struct_tokens_double_quotes_token_stream = |value: &dyn std::fmt::Display| generate_quotes::double_quotes_token_stream(&format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value}"));
            let generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream = |value: &dyn std::fmt::Display| generate_quotes::double_quotes_token_stream(&format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value} with 1 element"));
            (generate_tuple_struct_tokens_double_quotes_token_stream, generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream)
        };
        // let field0_token_stream = quote::quote! {__field0};
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
        let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
        let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
        // let column_name_and_maybe_field_getter_handle_snake_case = naming::ColumnNameAndMaybeFieldGetterHandleSnakeCase;
        let column_name_and_maybe_field_getter_for_error_message_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;
        // let object_with_id_ident_token_stream = {
        //     let object_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
        //         true,
        //         &object_with_id_ident_upper_camel_case,
        //         &{
        //             quote::quote!{{
        //                 pub #id_snake_case: #postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream,
        //                 #pub_field_idents_field_types_token_stream
        //             }}
        //         }
        //     );
        //     let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_object_with_id_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
        //         &object_with_id_ident_upper_camel_case,
        //         &proc_macro2::TokenStream::new(),
        //         &{
        //             let fields_token_stream = vec_syn_field.iter().map(|element| {
        //                 let field_ident = element
        //                     .ident
        //                     .as_ref()
        //                     .unwrap_or_else(|| {
        //                         panic!("{}", naming::FIELD_IDENT_IS_NONE);
        //                     });
        //                 quote::quote!{
        //                     #field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
        //                 }
        //             });
        //             quote::quote!{Self {
        //                 #id_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
        //                 #(#fields_token_stream),*
        //             }}
        //         },
        //     );
        //     quote::quote!{
        //         #object_with_id_ident_token_stream
        //         #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_object_with_id_ident_token_stream
        //     }
        // };
        
        
        

        let ident_without_id_read_upper_camel_case = naming::parameter::SelfWithoutIdReadUpperCamelCase::from_tokens(&ident);
        let ident_with_id_read_upper_camel_case = naming::parameter::SelfWithIdReadUpperCamelCase::from_tokens(&ident);
        let ident_without_id_read_try_from_error_named_upper_camel_case = naming::parameter::SelfWithoutIdReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident);
        // let ident_with_id_read_try_from_error_named_upper_camel_case = naming::parameter::SelfWithIdReadTryFromErrorNamedUpperCamelCase::from_tokens(&ident);

        let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);

        let std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case = naming::parameter::StdVecVecObjectWithIdSelfJsonArrayChangeUpperCamelCase::from_tokens(&ident);
        let std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case = naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfJsonArrayChangeUpperCamelCase::from_tokens(&ident);
            
        let field_ident_snake_case = naming::FieldIdentSnakeCase;




        
        let postgresql_json_object_type = PostgresqlJsonObjectType::WithoutId;//todo


        let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
        let ident_create_token_stream = {
            let ident_create_token_stream = generate_supported_generics_template_struct_token_stream(
                true,
                &ident_create_upper_camel_case,
                &generate_field_type_handle(
                    &generate_fields_declaration_create_token_stream(IsPub::True),
                    |tokens|{
                        let content = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&tokens);
                        quote::quote!{#content}
                    }
                )
            );
            let impl_std_fmt_display_for_ident_create_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(
                &proc_macro2::TokenStream::new(),
                &ident_create_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {write!(formatter, "{:?}", self)}
            );
            let impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                &proc_macro2::TokenStream::new(),
                &ident_create_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self}")}
            );
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_create_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let content_token_stream = match &postgresql_json_type_pattern {
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_fields_token_stream},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{(Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                        },
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {..}
                        | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {..}
                        | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {..}
                        | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {..}
                        => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{(vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{(Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                        }
                    };
                    quote::quote! {Self #content_token_stream}
                }
            );
            quote::quote! {
                #ident_create_token_stream
                #impl_std_fmt_display_for_ident_create_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_create_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
            }
        };
        let create_query_part_token_stream = match &postgresql_json_type_pattern {
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                    let postgresql_json_object_type = PostgresqlJsonObjectType::WithoutId;//todo WithId
                    let ok_value_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => quote::quote! {format!("{increments}")},
                        PostgresqlJsonObjectType::WithId => quote::quote! {format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}")}
                    };
                    let create_query_part_fields_token_stream = vec_syn_field.iter().map(|element| {
                        let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
                        let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote! {
                            match #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_part_snake_case(&value.#element_field_ident, #increment_snake_case) {
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
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    match &#value_snake_case.0 {
                        Some(#value_snake_case) => match #value_snake_case.#create_query_part_snake_case(#increment_snake_case) {
                            Ok(#value_snake_case) => Ok(#value_snake_case),
                            Err(error) => Err(error)
                        },
                        //maybe not use null here and use increment logic
                        None => Ok(std::string::String::from("null"))
                    }
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: _,
            } => 
            //todo
            match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                        let mut acc = std::string::String::default();
                        for element in &#value_snake_case.0 {
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
                        match &#value_snake_case.0 {
                            Some(#value_snake_case) => {
                                let mut acc = std::string::String::default();
                                for element in #value_snake_case {
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
                            None => Ok(std::string::String::from("null"))
                        }
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
                dimension4_not_null_or_nullable: _,
            } => todo!()
        };
        let create_query_bind_token_stream = match &postgresql_json_type_pattern {
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                    let create_query_bind_fields_token_stream = vec_syn_field.iter().map(|element| {
                        let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote! {
                            #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_bind_snake_case(value.#element_field_ident, #query_snake_case);
                        }
                    });
                    quote::quote! {
                        #(#create_query_bind_fields_token_stream)*
                        #query_snake_case
                    }
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    if let Some(#value_snake_case) = #value_snake_case.0 {
                        #query_snake_case = #value_snake_case.#create_query_bind_snake_case(#query_snake_case);
                    }
                    #query_snake_case
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: _,
            } => 
            //todo
            match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                    for element in #value_snake_case.0 {
                        #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                    }
                    #query_snake_case
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    if let Some(#value_snake_case) = #value_snake_case.0 {
                        for element in #value_snake_case {
                            #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                        }
                    }
                    #query_snake_case
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
                dimension4_not_null_or_nullable: _,
            } => todo!()
        };
        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_select_token_stream = {
            let ident_select_token_stream = {
                //todo not trivial to write api for get by id, select fields in last child dimension object and iumpl pagination api at the same time
                // quote::quote! {postgresql_crud::UniqueVec<#ident_select_without_id_upper_camel_case>}
                // quote::quote! {postgresql_crud::UniqueVec<#ident_select_with_id_upper_camel_case>}
                let content_token_stream = match postgresql_crud_macros_common::ArrayDimension::try_from(postgresql_json_type_pattern) {
                    Ok(array_dimension) => {
                        let mut arguments_token_stream = vec![];
                        for element in 1..=array_dimension.to_usize() {
                            let dimension_number_pagination_token_stream = format!("dimension{element}_pagination")
                            .parse::<proc_macro2::TokenStream>().unwrap();
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: crate::Pagination
                            });
                        }
                        quote::quote! {{
                            #(#arguments_token_stream),*
                        }}
                    },
                    Err(_) => quote::quote! {(pub postgresql_crud::UniqueVec<#ident_select_without_id_upper_camel_case>);}
                };
                quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        Default,
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                        utoipa::ToSchema,
                        schemars::JsonSchema,
                    )]
                    pub struct #ident_select_upper_camel_case #content_token_stream
                }
            };
            let sqlx_types_json_unique_vec_ident_select_without_id_token_stream = postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(
                &quote::quote!{postgresql_crud::UniqueVec<#ident_select_without_id_upper_camel_case>}
            );
            let std_option_option_sqlx_types_json_unique_vec_ident_select_without_id_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&sqlx_types_json_unique_vec_ident_select_without_id_token_stream);
            let impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                &ident_select_upper_camel_case,
                //todo
                &sqlx_types_json_unique_vec_ident_select_without_id_token_stream
                // &match &postgresql_type {
                //     PostgresqlType::JsonbNotNull => &sqlx_types_json_unique_vec_ident_select_without_id_token_stream,
                //     PostgresqlType::JsonbNullable => &std_option_option_sqlx_types_json_unique_vec_ident_select_without_id_token_stream,
                // },
            );
            let impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                &ident_select_upper_camel_case,
                //todo
                &sqlx_types_json_unique_vec_ident_select_without_id_token_stream,
                // &match &postgresql_type {
                //     PostgresqlType::JsonbNotNull => &sqlx_types_json_unique_vec_ident_select_without_id_token_stream,
                //     PostgresqlType::JsonbNullable => &std_option_option_sqlx_types_json_unique_vec_ident_select_without_id_token_stream,
                // },
                //todo
                &quote::quote! {Ok(Self(value.0))},
                // &match &postgresql_type {
                //     PostgresqlType::JsonbNotNull => quote::quote! {Ok(Self(value.0))},
                //     PostgresqlType::JsonbNullable => quote::quote! {
                //         match value {
                //             Some(value) => Ok(Self(Some(value.0))),
                //             None => Ok(Self(None)),
                //         }
                //     },
                // }
            );
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &proc_macro2::TokenStream::new(), &{
                    //todo
                    let value = quote::quote! {(
                        #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    )};
                    // match &postgresql_type {
                    //     PostgresqlType::JsonbNotNull => quote::quote! {(
                    //         #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    //     )},
                    //     PostgresqlType::JsonbNullable => quote::quote! {(Some(
                    //         #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    //     ))},
                    // };
                    quote::quote! {Self #value}
                });
            quote::quote! {
                #ident_select_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_select_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_ident_select_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
            }
        };
        let generate_select_query_part_token_stream = |postgresql_type_or_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrJsonType|{
            let column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageFieldIdentSnakeCase;
            let generate_acc_push_str_variant_logic_token_stream =
                |variant_name_token_stream: &dyn quote::ToTokens, field_ident_double_quotes_token_stream: &dyn quote::ToTokens, column_name_and_maybe_field_getter_token_stream: &dyn quote::ToTokens, element_type: &dyn quote::ToTokens| {
                    let tokens_select_with_or_without_id_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type_pattern {
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => &ident_select_without_id_upper_camel_case,
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable: _,
                        } => 
                        //todo
                        &ident_select_with_id_upper_camel_case,
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                        } => todo!(),
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                            dimension3_not_null_or_nullable: _,
                        } => todo!(),
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                            dimension3_not_null_or_nullable: _,
                            dimension4_not_null_or_nullable: _,
                        } => todo!()
                    };
                    let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream(&element_type);
                    quote::quote! {
                        #tokens_select_with_or_without_id_upper_camel_case_token_stream::#variant_name_token_stream(value) => #field_type_as_crud_postgresql_json_type_from_field_token_stream #select_query_part_snake_case(
                            &value,
                            #field_ident_double_quotes_token_stream,
                            #column_name_and_maybe_field_getter_token_stream,
                            &#column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case,
                            false,
                        )
                    }
                };
            let column_name_and_maybe_field_getter_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterFieldIdentSnakeCase;
            let value_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&value_snake_case);
            let (maybe_id_variant_token_stream, variants_token_stream) = {
                let maybe_id_variant_token_stream = match &postgresql_json_type_pattern {
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable: _,
                    } => {
                        let id_upper_camel_case = naming::IdUpperCamelCase;
                        let id_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&naming::IdSnakeCase);
                        let value = generate_acc_push_str_variant_logic_token_stream(
                            &quote::quote! {#id_upper_camel_case},
                            &id_snake_case_double_quotes_token_stream,
                            &value_snake_case_double_quotes_token_stream,
                            &postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
                        );
                        quote::quote! {#value,}
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                        dimension4_not_null_or_nullable: _,
                    } => todo!()
                };
                let variants_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    generate_acc_push_str_variant_logic_token_stream(
                        &naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified),
                        &generate_quotes::double_quotes_token_stream(&field_ident_stringified),
                        &match &postgresql_json_type_pattern {
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => quote::quote! {&#column_name_and_maybe_field_getter_field_ident_snake_case},
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                                dimension1_not_null_or_nullable: _,
                            } => quote::quote! {#value_snake_case_double_quotes_token_stream},
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable: _,
                                dimension2_not_null_or_nullable: _,
                            } => todo!(),
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable: _,
                                dimension2_not_null_or_nullable: _,
                                dimension3_not_null_or_nullable: _,
                            } => todo!(),
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable: _,
                                dimension2_not_null_or_nullable: _,
                                dimension3_not_null_or_nullable: _,
                                dimension4_not_null_or_nullable: _,
                            } => todo!()
                        },
                        &{
                            let element_type = &element.ty;
                            quote::quote! {#element_type}
                        },
                    )
                });
                (maybe_id_variant_token_stream, variants_token_stream)
            };
            let self_field_vec_token_stream = match &postgresql_json_type_pattern {
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => quote::quote! {.0.to_vec()},
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                    dimension1_not_null_or_nullable: _,
                } => quote::quote! {.field_vec},
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                } => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                    dimension3_not_null_or_nullable: _,
                } => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                    dimension3_not_null_or_nullable: _,
                    dimension4_not_null_or_nullable: _,
                } => todo!()
            };
            let maybe_pagination_start_end_initialization_token_stream = match &postgresql_json_type_pattern {
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                    dimension1_not_null_or_nullable: _,
                } => macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&naming::SelectSnakeCase),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                } => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                    dimension3_not_null_or_nullable: _,
                } => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable: _,
                    dimension2_not_null_or_nullable: _,
                    dimension3_not_null_or_nullable: _,
                    dimension4_not_null_or_nullable: _,
                } => todo!()
            };
            let column_name_and_maybe_field_getter_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{{{field_ident_snake_case}}}'"));
            let column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{column_name_and_maybe_field_getter_for_error_message_snake_case}}}.{{{field_ident_snake_case}}}"));
            let (if_postgresql_type_is_true_format_handle_double_quotes_token_stream, if_postgresql_type_is_false_format_handle_double_quotes_token_stream) = {
                let wrap_into_jsonb_build_object_field_ident = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{{{field_ident_snake_case}}}', {value})");
                let wrap_into_jsonb_build_object_value = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{value_snake_case}',{value})");
                let acc_format_handle = {
                    let acc_snake_case = naming::AccSnakeCase;
                    format!("{{{acc_snake_case}}}")
                };
                let jsonb_build_object_value_acc_format_handle = wrap_into_jsonb_build_object_value(&acc_format_handle);
                let null_snake_case = naming::NullSnakeCase;
                match &postgresql_json_type_pattern {
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (acc_format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&jsonb_build_object_value_acc_format_handle)),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else {acc_format_handle} end"),
                            wrap_into_jsonb_build_object_field_ident(&{
                                let jsonb_build_object_value_null = wrap_into_jsonb_build_object_value(&null_snake_case);
                                format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {jsonb_build_object_value_null} else {jsonb_build_object_value_acc_format_handle} end")
                            }),
                        ),
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable: _,
                    } => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let format_handle = format!("(select jsonb_agg({{acc}}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}})");
                            (format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&wrap_into_jsonb_build_object_value(&format_handle)))
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let format_handle = format!(
                                "case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else (select jsonb_agg({acc_format_handle}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}}) end"
                            );
                            (format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&wrap_into_jsonb_build_object_value(&format_handle)))
                        },
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                        dimension4_not_null_or_nullable: _,
                    } => todo!()
                }
            };
            let maybe_declare_variables_token_stream = match &postgresql_type_or_json_type {
                postgresql_crud_macros_common::PostgresqlTypeOrJsonType::PostgresqlType => quote::quote! {
                    let field_ident = column;
                    let column_name_and_maybe_field_getter = column;
                    let column_name_and_maybe_field_getter_for_error_message = column;
                    let is_postgresql_type = true;
                },
                postgresql_crud_macros_common::PostgresqlTypeOrJsonType::PostgresqlJsonType => proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_declare_variables_token_stream
                let mut acc = std::string::String::default();
                let #column_name_and_maybe_field_getter_field_ident_snake_case = if is_postgresql_type {
                    column_name_and_maybe_field_getter.to_string()
                } else {
                    format!(#column_name_and_maybe_field_getter_format_handle_token_stream)
                };
                let #column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = format!(#column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream);
                for element in #value_snake_case #self_field_vec_token_stream {
                    acc.push_str(&format!(
                        "{}||",
                        match element {
                            #maybe_id_variant_token_stream
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
        let ident_where_element_token_stream = {
            let ident_where_element_token_stream = {
                let variants_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
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
                        #field_ident_upper_camel_case_token_stream(postgresql_crud::PostgresqlTypeWhere<
                            #field_type_as_json_type_where_element_token_stream
                        >)
                    }
                });
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                    pub enum #ident_where_element_upper_camel_case {
                         #(#variants_token_stream),*
                    }
                }
            };
            let impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream = postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                &quote::quote! {<'a>},
                &ident_where_element_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let query_part_variants_token_stream = vec_syn_field.iter().map(|element| {
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
                            Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
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
                },
                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                &{
                    let query_bind_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        quote::quote! {
                            Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
                        }
                    });
                    quote::quote! {
                        match self {
                            #(#query_bind_variants_token_stream),*
                        }
                    }
                },
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
            );
            let impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                &proc_macro2::TokenStream::new(),
                &ident_where_element_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self:#?}")}
            );
            let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_where_element_upper_camel_case, &{
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
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
                });
            quote::quote! {
                #ident_where_element_token_stream
                #impl_postgresql_crud_postgresql_type_postgresql_type_where_filter_for_ident_where_element_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_where_element_token_stream
                #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_element_token_stream
            }
        };
        let ident_read_token_stream = {
            let ident_without_id_or_with_id_read_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_object_type {
                PostgresqlJsonObjectType::WithoutId => &ident_without_id_read_upper_camel_case,
                PostgresqlJsonObjectType::WithId => &ident_with_id_read_upper_camel_case
            };
            let ident_without_id_or_with_id_read_try_from_error_named_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_json_object_type {
                PostgresqlJsonObjectType::WithoutId => &ident_without_id_read_try_from_error_named_upper_camel_case,
                PostgresqlJsonObjectType::WithId => &ident_without_id_read_try_from_error_named_upper_camel_case,
            };
            enum ShouldAddSerdeOptionIsNoneAnnotation {
                True,
                False
            }
            let generate_ident_without_id_or_with_id_read_fields_declaration_token_stream = |
                postgresql_json_object_type: &PostgresqlJsonObjectType,
                should_add_serde_option_is_none_annotation: &ShouldAddSerdeOptionIsNoneAnnotation
            | {
                let maybe_serde_skip_serializing_if_option_is_none_token_stream = match &should_add_serde_option_is_none_annotation {
                    ShouldAddSerdeOptionIsNoneAnnotation::True => quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                    ShouldAddSerdeOptionIsNoneAnnotation::False => proc_macro2::TokenStream::new()
                };
                let maybe_id_token_stream = match &postgresql_json_object_type {
                    PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                    PostgresqlJsonObjectType::WithId => quote::quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_token_stream
                        #id_snake_case: std::option::Option<#postgresql_crud_path_token_stream Value<#postgresql_crud_path_postgresql_json_type_uuid_uuid_read_token_stream>>,
                    }
                };
                let fields_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                    });
                    let field_type_as_json_type_read_token_stream = generate_type_as_postgresql_json_type_subtype_token_stream(
                        &element.ty,
                        &PostgresqlJsonTypeSubtype::Read
                    );
                    quote::quote! {
                        #maybe_serde_skip_serializing_if_option_is_none_token_stream
                        #field_ident: std::option::Option<#postgresql_crud_path_token_stream Value<
                            #field_type_as_json_type_read_token_stream
                        >>
                    }
                });
                quote::quote! {
                    #maybe_id_token_stream
                    #(#fields_token_stream),*
                }
            };
            let ident_without_id_or_with_id_read_token_stream = {
                let content_token_stream = generate_ident_without_id_or_with_id_read_fields_declaration_token_stream(
                    &postgresql_json_object_type,
                    &ShouldAddSerdeOptionIsNoneAnnotation::True
                );
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
                    pub struct #ident_without_id_or_with_id_read_upper_camel_case {
                        #content_token_stream
                    }
                }
            };
            let all_fields_are_none_upper_camel_case = naming::AllFieldsAreNoneUpperCamelCase;
            let ident_without_id_or_with_id_read_try_from_error_named_token_stream = quote::quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_without_id_or_with_id_read_try_from_error_named_upper_camel_case {
                    #all_fields_are_none_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            };
            let impl_try_new_for_ident_without_id_or_with_id_read_try_from_error_named_token_stream = {
                let ident_without_id_or_with_id_read_fields_declaration_token_stream = generate_ident_without_id_or_with_id_read_fields_declaration_token_stream(
                    &postgresql_json_object_type,
                    &ShouldAddSerdeOptionIsNoneAnnotation::False
                );
                let (ident_without_id_or_with_id_read_fields_reference_token_stream, ident_without_id_or_with_id_read_fields_token_stream) = {
                    enum WithReference {
                        True,
                        False
                    }
                    let generate_ident_without_id_or_with_id_read_fields_token_stream = |with_reference: &WithReference| {
                        let maybe_reference_symbol_token_stream = match &with_reference {
                            WithReference::True => quote::quote! {&},
                            WithReference::False => proc_macro2::TokenStream::new()
                        };
                        let maybe_id_token_stream = match &postgresql_json_object_type {
                            PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                            PostgresqlJsonObjectType::WithId => quote::quote! {#maybe_reference_symbol_token_stream #id_snake_case,}
                        };
                        let fields_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            quote::quote! {#maybe_reference_symbol_token_stream #field_ident}
                        });
                        quote::quote! {
                            #maybe_id_token_stream
                            #(#fields_token_stream),*
                        }
                    };
                    (
                        generate_ident_without_id_or_with_id_read_fields_token_stream(&WithReference::True),
                        generate_ident_without_id_or_with_id_read_fields_token_stream(&WithReference::False)
                    )
                };
                let ident_without_id_or_with_id_read_check_if_all_fields_are_none_token_stream = {
                    let nones_token_stream = {
                        let range_end = {
                            let vec_syn_field_len = vec_syn_field.len();
                            match &postgresql_json_object_type {
                                PostgresqlJsonObjectType::WithoutId => vec_syn_field_len,
                                PostgresqlJsonObjectType::WithId => vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                            }
                        };
                        let mut acc = vec![];
                        for _ in 0..range_end {
                            acc.push(quote::quote! {None});
                        }
                        acc
                    };
                    quote::quote! {
                        if let (#(#nones_token_stream),*) = (#ident_without_id_or_with_id_read_fields_reference_token_stream) {
                            return Err(#ident_without_id_or_with_id_read_try_from_error_named_upper_camel_case::#all_fields_are_none_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                    }
                };
                quote::quote! {
                    impl #ident_without_id_or_with_id_read_upper_camel_case {
                        pub fn try_new(#ident_without_id_or_with_id_read_fields_declaration_token_stream) -> Result<Self, #ident_without_id_or_with_id_read_try_from_error_named_upper_camel_case> {
                            #ident_without_id_or_with_id_read_check_if_all_fields_are_none_token_stream
                            Ok(Self{#ident_without_id_or_with_id_read_fields_token_stream})
                        }
                    }
                }
            };
            let impl_serde_deserialize_for_ident_without_id_or_with_id_read_token_stream = {
                let range_end = {
                    let vec_syn_field_len = vec_syn_field.len();
                    match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => vec_syn_field_len,
                        PostgresqlJsonObjectType::WithId => vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                    }
                };
                let field_enum_variants_token_stream = {
                    let mut vec = vec![];
                    for element in 0..range_end {
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
                    for index in 0..range_end {
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
                let generate_index = |index: std::primitive::usize| {
                    match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => index,
                        PostgresqlJsonObjectType::WithId => index.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                    }
                };
                let visit_str_value_enum_variants_token_stream = {
                    let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let index = generate_index(index);
                        let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        }));
                        generate_field_ident_double_quotes_serde_private_ok_field_token_stream(&field_name_double_quotes_token_stream, index)
                    });
                    let maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => {
                            let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(&id_snake_case_double_quotes_token_stream, 0);
                            quote::quote! {#value_token_stream,}
                        }
                    };
                    quote::quote! {
                        #maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream
                        #(#visit_str_value_enum_variants_token_stream),*,
                    }
                };
                let visit_bytes_value_enum_variants_token_stream = {
                    let visit_bytes_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let index = generate_index(index);
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
                    let maybe_b_field_ident_double_quotes_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => {
                            let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                                &{
                                    let value = format!("b{id_snake_case_double_quotes_token_stream}");
                                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                },
                                0,
                            );
                            quote::quote! {#value_token_stream,}
                        }
                    };
                    quote::quote! {
                        #maybe_b_field_ident_double_quotes_token_stream
                        #(#visit_bytes_value_enum_variants_token_stream),*,
                    }
                };
                let struct_ident_options_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_read_upper_camel_case}"));
                let struct_ident_options_with_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_read_upper_camel_case} with {range_end} elements"));
                let visit_seq_fields_initialization_token_stream = {
                    let generate_serde_de_seq_access_next_element_token_stream = |index: std::primitive::usize, type_read_token_stream: &dyn quote::ToTokens| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            let #field_index_token_stream = match serde::de::SeqAccess::next_element::<
                                std::option::Option<#postgresql_crud_path_token_stream Value<#type_read_token_stream>>,
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
                    let visit_seq_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_serde_de_seq_access_next_element_token_stream(
                            generate_index(index),
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    let maybe_id_serde_de_seq_access_next_element_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => generate_serde_de_seq_access_next_element_token_stream(
                            0,
                            &postgresql_crud_path_postgresql_json_type_uuid_uuid_read_token_stream,
                        )
                    };
                    quote::quote! {
                        #maybe_id_serde_de_seq_access_next_element_token_stream
                        #(#visit_seq_fields_initialization_token_stream)*
                    }
                };
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &ident_without_id_or_with_id_read_upper_camel_case,
                    &{
                        let fields_token_stream = {
                            let mut acc = vec![];
                            for element in 0..range_end {
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
                                std::option::Option<#postgresql_crud_path_token_stream Value<#type_token_stream>>,
                            > = serde::__private::None;
                        }
                    };
                    let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_mut_field_index_serde_private_option_token_stream(
                            generate_index(index),
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    let maybe_id_mut_field_index_serde_private_option_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => generate_mut_field_index_serde_private_option_token_stream(
                            0,
                            &postgresql_crud_path_postgresql_json_type_uuid_uuid_read_token_stream,
                        )
                    };
                    quote::quote! {
                        #maybe_id_mut_field_index_serde_private_option_token_stream
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
                                        <__A::Error as serde::de::Error>::duplicate_field(
                                            #field_ident_double_quotes_token_stream,
                                        ),
                                    );
                                }
                                #field_index_token_stream = serde::__private::Some(
                                    serde::de::MapAccess::next_value::<
                                        std::option::Option<#postgresql_crud_path_token_stream Value<#type_token_stream>>,
                                    >(&mut __map)?,
                                );
                            }
                        }
                    };
                    let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        generate_field_initialization_token_stream(
                            generate_index(index),
                            &generate_field_ident_double_quotes_token_stream(element),
                            &generate_type_as_postgresql_json_type_subtype_token_stream(
                                &element.ty,
                                &PostgresqlJsonTypeSubtype::Read
                            )
                        )
                    });
                    let id_field_initialization_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => generate_field_initialization_token_stream(
                            0,
                            &id_snake_case_double_quotes_token_stream,
                            &postgresql_crud_path_postgresql_json_type_uuid_uuid_read_token_stream,
                        )
                    };
                    quote::quote! {
                        #id_field_initialization_token_stream
                        #(#visit_map_match_variants_token_stream)*
                    }
                };
                let visit_map_missing_fields_check_token_stream = {
                    let generate_missing_field_token_stream = |index: std::primitive::usize, field_ident_double_quotes_token_stream: &dyn quote::ToTokens| {
                        let field_index_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            let #field_index_token_stream = match #field_index_token_stream {
                                serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                                serde::__private::None => {
                                    serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
                                }
                            };
                        }
                    };
                    let visit_map_missing_fields_check_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                        let index = generate_index(index);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        generate_missing_field_token_stream(index, &field_ident_double_quotes_token_stream)
                    });
                    let maybe_id_missing_field_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => generate_missing_field_token_stream(0, &id_snake_case_double_quotes_token_stream)
                    };
                    quote::quote! {
                        #maybe_id_missing_field_token_stream
                        #(#visit_map_missing_fields_check_token_stream)*
                    }
                };
                let fields_array_elements_token_stream = {
                    let fields_array_elements_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(element));
                    let maybe_id_double_quotes_comma_token_stream = match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => proc_macro2::TokenStream::new(),
                        PostgresqlJsonObjectType::WithId => quote::quote! {#id_snake_case_double_quotes_token_stream,}
                    };
                    quote::quote! {
                        #maybe_id_double_quotes_comma_token_stream
                        #(#fields_array_elements_token_stream),*
                    }
                };
                let ident_without_id_or_with_id_read_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_without_id_or_with_id_read_upper_camel_case);
                quote::quote! {
                    impl<'de> serde::Deserialize<'de> for #ident_without_id_or_with_id_read_upper_camel_case {
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
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
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
                                    #ident_without_id_or_with_id_read_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #ident_without_id_or_with_id_read_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
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
                                #ident_without_id_or_with_id_read_upper_camel_case_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #ident_without_id_or_with_id_read_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_without_id_or_with_id_read_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_without_id_or_with_id_read_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let generate_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = |field_ident: &syn::Ident| {
                        quote::quote! {
                            #field_ident: Some(#postgresql_crud_path_token_stream Value {
                                value: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            })
                        }
                    };
                    let content_token_stream = &match &postgresql_json_object_type {
                        PostgresqlJsonObjectType::WithoutId => {
                            let fields_token_stream = vec_syn_field.iter().map(|element| {
                                generate_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(element.ident.as_ref().unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                }))
                            });
                            quote::quote!{#(#fields_token_stream),*}
                        },
                        PostgresqlJsonObjectType::WithId => {
                            let fields_token_stream = {
                                let fields_token_stream = vec_syn_field.iter().map(|element| {
                                    generate_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(element.ident.as_ref().unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    }))
                                });
                                let id_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                                    generate_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(&syn::Ident::new(&naming::IdSnakeCase.to_string(), proc_macro2::Span::call_site()));
                                quote::quote! {
                                    #id_field_ident_some_value_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                    #(#fields_token_stream),*
                                }
                            };
                            quote::quote!{#fields_token_stream}
                        }
                    };
                    quote::quote! {Self{#content_token_stream}}
                },
            );
            let impl_sqlx_type_sqlx_postgres_and_impl_sqlx_decode_sqlx_postgres_token_stream = {
                let sqlx_types_json_ident_without_id_or_with_id_read_token_stream = postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_without_id_or_with_id_read_upper_camel_case);
                let std_option_option_sqlx_types_json_ident_without_id_or_with_id_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&sqlx_types_json_ident_without_id_or_with_id_read_token_stream);
                let impl_sqlx_type_sqlx_postgres_for_ident_without_id_or_with_id_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_without_id_or_with_id_read_upper_camel_case,
                    //todo
                    &sqlx_types_json_ident_without_id_or_with_id_read_token_stream
                    // &match &postgresql_type {
                    //     PostgresqlType::JsonbNotNull => &sqlx_types_json_ident_without_id_or_with_id_read_token_stream,
                    //     PostgresqlType::JsonbNullable => &std_option_option_sqlx_types_json_ident_without_id_or_with_id_read_token_stream,
                    // },
                );
                let impl_sqlx_decode_sqlx_postgres_for_ident_without_id_or_with_id_read_token_stream = {
                    postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                        &ident_without_id_or_with_id_read_upper_camel_case,
                        //todo
                         &sqlx_types_json_ident_without_id_or_with_id_read_token_stream,
                        // &match &postgresql_type {
                        //     PostgresqlType::JsonbNotNull => &sqlx_types_json_ident_without_id_or_with_id_read_token_stream,
                        //     PostgresqlType::JsonbNullable => &std_option_option_sqlx_types_json_ident_without_id_or_with_id_read_token_stream,
                        // },
                        //todo
                        &quote::quote! {Ok(value.0)},
                        // &match &postgresql_type {
                        //     PostgresqlType::JsonbNotNull => quote::quote! {Ok(value.0)},
                        //     PostgresqlType::JsonbNullable => quote::quote! {
                        //         match value {
                        //             Some(value) => Ok(Some(value.0)),
                        //             None => Ok(None),
                        //         }
                        //     },
                        // },
                    )
                };
                quote::quote! {
                    #impl_sqlx_type_sqlx_postgres_for_ident_without_id_or_with_id_read_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_without_id_or_with_id_read_token_stream
                }
            };
            quote::quote! {
                #ident_without_id_or_with_id_read_token_stream
                #ident_without_id_or_with_id_read_try_from_error_named_token_stream
                #impl_try_new_for_ident_without_id_or_with_id_read_try_from_error_named_token_stream
                #impl_serde_deserialize_for_ident_without_id_or_with_id_read_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_without_id_or_with_id_read_token_stream
                #impl_sqlx_type_sqlx_postgres_and_impl_sqlx_decode_sqlx_postgres_token_stream
            }
        };
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_update_token_stream = {
            let std_vec_vec_object_with_id_or_std_option_option_std_vec_vec_object_with_id_ident_token_stream = {
                let generate_ident_json_array_change_token_stream = |struct_ident_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens, struct_ident_try_new_error_named: &dyn quote::ToTokens, is_nullable: std::primitive::bool| {
                    let create_snake_case = naming::CreateSnakeCase;
                    let update_snake_case = naming::UpdateSnakeCase;
                    let delete_snake_case = naming::DeleteSnakeCase;
                    //todo move this logic into library as type with generic
                    let ident_json_array_change_token_stream = {
                        let serde_skip_serializing_if_vec_is_empty_token_stream = quote::quote! {#[serde(skip_serializing_if = "Vec::is_empty")]};
                        quote::quote! {
                            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub struct #struct_ident_token_stream {
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #create_snake_case: std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>,
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #update_snake_case: std::vec::Vec<#ident_update_with_id_upper_camel_case>,
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #delete_snake_case: std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>,
                            }
                        }
                    };
                    let custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified = format!("custom serde error deserializing {struct_ident_token_stream}");
                    let impl_try_new_for_ident_json_array_change_token_stream = {
                        let create_update_delete_check_fields_are_empty_upper_camel_case = naming::CreateUpdateDeleteCheckFieldsAreEmptyUpperCamelCase;
                        let not_unique_id_in_json_update_array_upper_camel_case = naming::NotUniqueIdInJsonUpdateArrayUpperCamelCase;
                        let not_unique_id_in_json_delete_array_upper_camel_case = naming::NotUniqueIdInJsonDeleteArrayUpperCamelCase;
                        let not_unique_id_in_json_update_and_delete_arrays_upper_camel_case = naming::NotUniqueIdInJsonUpdateAndDeleteArraysUpperCamelCase;
                        let try_new_error_named_token_stream = {
                            let maybe_create_update_delete_check_fields_are_empty_variant_token_stream = if is_nullable {
                                proc_macro2::TokenStream::new()
                            } else {
                                quote::quote! {
                                    #create_update_delete_check_fields_are_empty_upper_camel_case {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                }
                            };
                            quote::quote! {
                                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                                pub enum #struct_ident_try_new_error_named {
                                    #maybe_create_update_delete_check_fields_are_empty_variant_token_stream
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
                            }
                        };
                        let impl_pub_fn_try_new_token_stream = {
                            let maybe_check_create_update_delete_check_fields_are_empty_token_stream = if is_nullable {
                                proc_macro2::TokenStream::new()
                            } else {
                                quote::quote! {
                                    if #create_snake_case.is_empty() && #update_snake_case.is_empty() && #delete_snake_case.is_empty() {
                                        return Err(#struct_ident_try_new_error_named::#create_update_delete_check_fields_are_empty_upper_camel_case {
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            };
                            let check_not_unique_id_token_stream = {
                                let check_not_unique_id_in_update_array_token_stream = {
                                    let not_unique_id_in_json_update_array_double_quotes_token_stream =
                                        generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique id in json update array: {{}}"));
                                    quote::quote! {
                                        let update_acc = {
                                            let mut update_acc = vec![];
                                            for element in &update {
                                                let #id_snake_case = &element.#id_snake_case;
                                                if update_acc.contains(&#id_snake_case) {
                                                    return Err(#struct_ident_try_new_error_named::#not_unique_id_in_json_update_array_upper_camel_case {
                                                        error: format!(#not_unique_id_in_json_update_array_double_quotes_token_stream, #id_snake_case.0),
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
                                    let not_unique_id_in_json_delete_array_double_quotes_token_stream =
                                        generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique {id_snake_case} in json delete array: {{}}"));
                                    quote::quote! {
                                        let delete_acc = {
                                            let mut delete_acc = vec![];
                                            for element in &delete {
                                                if delete_acc.contains(&element) {
                                                    return Err(#struct_ident_try_new_error_named::#not_unique_id_in_json_delete_array_upper_camel_case {
                                                        error: format!(#not_unique_id_in_json_delete_array_double_quotes_token_stream, element.0),
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
                                    let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream =
                                        generate_quotes::double_quotes_token_stream(&format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique {id_snake_case} in json update and delete arrays: {{}}"));
                                    quote::quote! {
                                        for element in update_acc {
                                            if delete_acc.contains(&&element) {
                                                return Err(#struct_ident_try_new_error_named::#not_unique_id_in_json_update_and_delete_arrays_upper_camel_case {
                                                    error: format!(#not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream, element.0),
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
                                impl #struct_ident_token_stream {
                                    pub fn try_new(
                                        #create_snake_case: std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>,
                                        #update_snake_case: std::vec::Vec<#ident_update_with_id_upper_camel_case>,
                                        #delete_snake_case: std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>,
                                    ) -> Result<Self, #struct_ident_try_new_error_named> {
                                        #maybe_check_create_update_delete_check_fields_are_empty_token_stream
                                        #check_not_unique_id_token_stream
                                        Ok(Self {
                                            #create_snake_case,
                                            #update_snake_case,
                                            #delete_snake_case
                                        })
                                    }
                                }
                            }
                        };
                        quote::quote! {
                            #try_new_error_named_token_stream
                            #impl_pub_fn_try_new_token_stream
                        }
                    };
                    let impl_serde_deserialize_for_ident_json_array_change_token_stream = {
                        let tuple_struct_struct_ident_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&struct_ident_token_stream);
                        let struct_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&struct_ident_token_stream);
                        let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(&struct_ident_token_stream, &quote::quote! {__field0, __field1, __field2});
                        quote::quote! {
                            impl<'de> serde::Deserialize<'de> for #struct_ident_token_stream {
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
                                        fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                            serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                                        marker: serde::__private::PhantomData<#struct_ident_token_stream>,
                                        lifetime: serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = #struct_ident_token_stream;
                                        fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                            serde::__private::Formatter::write_str(__formatter, #tuple_struct_struct_ident_double_quotes_token_stream)
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>>(&mut __seq)? {
                                                serde::__private::Some(__value) => __value,
                                                serde::__private::None => {
                                                    vec![]
                                                }
                                            };
                                            let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_update_with_id_upper_camel_case>>(&mut __seq)? {
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
                                            let mut __field0: serde::__private::Option<std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>> = serde::__private::None;
                                            let mut __field1: serde::__private::Option<std::vec::Vec<#ident_update_with_id_upper_camel_case>> = serde::__private::None;
                                            let mut __field2: serde::__private::Option<std::vec::Vec<#postgresql_crud_path_postgresql_json_type_uuid_uuid_update_token_stream>> = serde::__private::None;
                                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if serde::__private::Option::is_some(&__field0) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                                        }
                                                        __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>>(&mut __map)?);
                                                    }
                                                    __Field::__field1 => {
                                                        if serde::__private::Option::is_some(&__field1) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                                                        }
                                                        __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_update_with_id_upper_camel_case>>(&mut __map)?);
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
                                        #struct_ident_double_quotes_token_stream,
                                        FIELDS,
                                        __Visitor {
                                            marker: serde::__private::PhantomData::<#struct_ident_token_stream>,
                                            lifetime: serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    };
                    let impl_ident_json_array_change_methods_token_stream = {
                        let update_query_part_token_stream = {
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
                                    #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        match #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
                                            &value.value,
                                            &element_acc,
                                            &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                            #field_ident_double_quotes_token_stream,
                                            #increment_snake_case,
                                        ) {
                                            Ok(value) => {
                                                element_acc = value;
                                            }
                                            Err(error) => {
                                                return Err(error);
                                            }
                                        }
                                    }
                                }
                            });
                            let ok_format_handle_token_stream = if is_nullable {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                                    "jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}', case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'array' then case when jsonb_array_length({{{jsonb_set_target_snake_case}}}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({{update_query_part_acc}}) from jsonb_array_elements({{{jsonb_set_target_snake_case}}}) as elem {{maybe_where}}), '[]'::jsonb)) end else '[]'::jsonb end {{maybe_jsonb_build_array}})"
                                ));
                                quote::quote! {
                                    Ok(format!(#format_handle_token_stream))
                                }
                            } else {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                                    "jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}', case when jsonb_array_length({{{jsonb_set_target_snake_case}}}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({{update_query_part_acc}}) from jsonb_array_elements({{{jsonb_set_target_snake_case}}}) as elem {{maybe_where}}), '[]'::jsonb)) end {{maybe_jsonb_build_array}})"
                                ));
                                quote::quote! {
                                    Ok(format!(#format_handle_token_stream))
                                }
                            };
                            let delete_query_part_acc_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{maybe_space_and_space}}elem->>'{id_snake_case}' <> ${{{increment_snake_case}}}"));
                            let update_push_str_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("when elem ->> '{id_snake_case}' = ${{id_increment}} then {{element_acc}} "));
                            quote::quote! {
                                let update_query_part_acc = {
                                    let mut element_acc = std::string::String::from("elem");
                                    if self.#update_snake_case.is_empty() {
                                        element_acc
                                    }
                                    else {
                                        let mut update_query_part_acc = std::string::String::default();
                                        #generate_jsonb_set_target_token_stream
                                        for element_handle in &self.#update_snake_case {
                                            let id_increment = match #increment_snake_case.checked_add(1) {
                                                Some(value) => {
                                                    *#increment_snake_case = value;
                                                    #increment_snake_case.to_string()
                                                }
                                                None => {
                                                    return Err(
                                                        postgresql_crud::QueryPartErrorNamed::#checked_add_upper_camel_case {
                                                            code_occurence: error_occurence_lib::code_occurence!()
                                                        }
                                                    );
                                                }
                                            };
                                            for element in &element_handle.fields.0 {
                                                match element {
                                                    #(#query_part_variants_token_stream),*
                                                }
                                            }
                                            update_query_part_acc.push_str(&format!(#update_push_str_format_handle_token_stream));
                                        }
                                        let _ = update_query_part_acc.pop();
                                        format!("case {update_query_part_acc} else elem end")
                                    }
                                };
                                let delete_query_part_acc = {
                                    let mut delete_query_part_acc = std::string::String::default();
                                    for _ in &self.#delete_snake_case {
                                        match #increment_snake_case.checked_add(1) {
                                            Some(value) => {
                                                *#increment_snake_case = value;
                                                let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                                                delete_query_part_acc.push_str(&format!(#delete_query_part_acc_format_handle_token_stream));
                                            }
                                            None => {
                                                return Err(
                                                    postgresql_crud::QueryPartErrorNamed::#checked_add_upper_camel_case {
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    }
                                                );
                                            }
                                        }
                                    }
                                    delete_query_part_acc
                                };
                                let create_query_part_acc = {
                                    let mut create_query_part_acc = std::string::String::default();
                                    for element in &self.#create_snake_case {
                                        match element.#create_query_part_snake_case(#increment_snake_case) {
                                            Ok(value) => {
                                                create_query_part_acc.push_str(&format!("{value},"));
                                            }
                                            Err(error) => {
                                                return Err(
                                                    postgresql_crud::QueryPartErrorNamed::#checked_add_upper_camel_case {
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    }
                                                );
                                            }
                                        }
                                    }
                                    let _ = create_query_part_acc.pop();
                                    create_query_part_acc
                                };
                                let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
                                let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
                                #ok_format_handle_token_stream
                            }
                        };
                        let update_query_bind_token_stream = {
                            let update_query_bind_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote! {
                                    #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                                    }
                                }
                            });
                            quote::quote! {
                                for element_handle in self.#update_snake_case {
                                    #query_snake_case = #query_snake_case.bind(element_handle.#id_snake_case.0.to_string());//postgresql: error returned from database: operator does not exist: text = jsonb
                                    for element in element_handle.fields.0 {
                                        match element {
                                            #(#update_query_bind_variants_token_stream),*
                                        }
                                    }
                                }
                                for element in self.#delete_snake_case {
                                    #query_snake_case = #query_snake_case.bind(element.0.to_string());//postgresql: error returned from database: operator does not exist: text <> jsonb
                                }
                                for element in self.#create_snake_case {
                                    #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                                }
                                #query_snake_case
                            }
                        };
                        quote::quote! {
                            impl #struct_ident_token_stream {
                                fn #update_query_part_snake_case(
                                    &self,
                                    #jsonb_set_accumulator_snake_case: &std::primitive::str,
                                    #jsonb_set_target_snake_case: &std::primitive::str,
                                    #jsonb_set_path_snake_case: &std::primitive::str,
                                    #increment_snake_case: &mut std::primitive::u64,
                                ) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
                                    #update_query_part_token_stream
                                }
                                fn #update_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                    #update_query_bind_token_stream
                                }
                            }
                        }
                    };
                    let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream =
                        postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                            &struct_ident_token_stream,
                            &proc_macro2::TokenStream::new(),
                            &quote::quote! {Self {
                                #create_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                                #update_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                                #delete_snake_case: vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream],
                            }},
                        );
                    quote::quote! {
                        #ident_json_array_change_token_stream
                        #impl_try_new_for_ident_json_array_change_token_stream
                        #impl_serde_deserialize_for_ident_json_array_change_token_stream
                        #impl_ident_json_array_change_methods_token_stream
                        #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream
                    }
                };
                match &postgresql_json_type_pattern {
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable: _,
                    } => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_ident_json_array_change_token_stream(
                        &std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case,
                            &naming::parameter::StdVecVecObjectWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                            false,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_json_array_change_token_stream(
                            &std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case,
                            &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                            true,
                        ),
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                        dimension4_not_null_or_nullable: _,
                    } => todo!()
                }
            };
            let tokens_update_token_stream = {
                let self_type_content_token_stream = match &postgresql_json_type_pattern {
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {pub postgresql_crud::UniqueVec<#ident_update_element_upper_camel_case>},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {pub std::option::Option<postgresql_crud::UniqueVec<#ident_update_element_upper_camel_case>>},
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable: _,
                    } => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {pub #std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {pub std::option::Option<#std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case>},
                    },
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                    } => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable: _,
                        dimension2_not_null_or_nullable: _,
                        dimension3_not_null_or_nullable: _,
                        dimension4_not_null_or_nullable: _,
                    } => todo!()
                };
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                    pub struct #ident_update_upper_camel_case(#self_type_content_token_stream);
                }
            };
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_update_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_upper_camel_case, &proc_macro2::TokenStream::new(), &{
                    let value = match &postgresql_json_type_pattern {
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                        },
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable: _,
                        } => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                        },
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                        } => todo!(),
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                            dimension3_not_null_or_nullable: _,
                        } => todo!(),
                        postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable: _,
                            dimension2_not_null_or_nullable: _,
                            dimension3_not_null_or_nullable: _,
                            dimension4_not_null_or_nullable: _,
                        } => todo!()
                    };
                    quote::quote! {Self #value}
                });
            quote::quote! {
                #std_vec_vec_object_with_id_or_std_option_option_std_vec_vec_object_with_id_ident_token_stream
                #tokens_update_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_update_token_stream
            }
        };
        let update_query_part_token_stream = match &postgresql_json_type_pattern {
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
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
                            #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
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
                    // let none_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                    //     &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})")
                    // );
                    quote::quote! {
                        let mut #object_acc_snake_case = format!(#some_format_handle_token_stream);
                        #generate_jsonb_set_target_token_stream
                        for element in #value_snake_case.0.to_vec() {
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
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let std_option_option_object_acc_snake_case = naming::StdOptionOptionObjectAccSnakeCase;
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',{{{std_option_option_object_acc_snake_case}}})"));
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
                            #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                match #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
                                    &value.value,
                                    &#std_option_option_object_acc_snake_case,
                                    &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                    #field_ident_double_quotes_token_stream,
                                    #increment_snake_case,
                                ) {
                                    Ok(value) => {
                                        #std_option_option_object_acc_snake_case = value;
                                    }
                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                        }
                    });
                    let some_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end"));
                    let none_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})"));
                    quote::quote! {
                        match &#value_snake_case.0 {
                            Some(value) => {
                                let mut #std_option_option_object_acc_snake_case = format!(#some_format_handle_token_stream);
                                #generate_jsonb_set_target_token_stream
                                for element in value.0.to_vec() {
                                    match element {
                                        #(#query_part_variants_token_stream),*
                                    }
                                }
                                if #jsonb_set_accumulator_snake_case.is_empty() && #jsonb_set_path_snake_case.is_empty() {
                                    Ok(#std_option_option_object_acc_snake_case)
                                }
                                else {
                                    Ok(format!(#format_handle_token_stream))
                                }
                            },
                            None => match #increment_snake_case.checked_add(1) {
                                Some(value) => {
                                    *#increment_snake_case = value;
                                    Ok(format!(#none_format_handle_token_stream))
                                },
                                None => Err(error)
                            }
                        }
                    }
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: _,
            } => match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                    match #value_snake_case.0.#update_query_part_snake_case(
                        #jsonb_set_accumulator_snake_case,
                        #jsonb_set_target_snake_case,
                        #jsonb_set_path_snake_case,
                        #increment_snake_case,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(error)
                    }
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})"));
                    quote::quote! {
                        match &#value_snake_case.0 {
                            Some(value) => {
                                match value.#update_query_part_snake_case(
                                    #jsonb_set_accumulator_snake_case,
                                    #jsonb_set_target_snake_case,
                                    #jsonb_set_path_snake_case,
                                    #increment_snake_case,
                                ) {
                                    Ok(value) => Ok(value),
                                    Err(error) => Err(error)
                                }
                            }
                            None => match #increment_snake_case.checked_add(1) {
                                Some(value) => {
                                    *#increment_snake_case = value;
                                    if #jsonb_set_accumulator_snake_case.is_empty() && #jsonb_set_path_snake_case.is_empty() {
                                        Ok(format!("${increment}"))
                                    }
                                    else {
                                        Ok(format!(#format_handle_token_stream))
                                    }
                                }
                                None => Err(error)
                            },
                        }
                    }
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
                dimension4_not_null_or_nullable: _,
            } => todo!()
        };
        let update_query_bind_token_stream = match &postgresql_json_type_pattern {
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
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
                            #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                            }
                        }
                    });
                    quote::quote! {
                        for element in #value_snake_case.0.into_vec() {
                            match element {
                                #(#update_query_bind_variants_token_stream),*
                            }
                        }
                        #query_snake_case
                    }
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
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
                            #ident_update_element_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                            }
                        }
                    });
                    quote::quote! {
                        match #value_snake_case.0 {
                            Some(value) => {
                                for element in value.0.into_vec() {
                                    match element {
                                        #(#update_query_bind_variants_token_stream),*
                                    }
                                }
                            },
                            None => {
                                #query_snake_case = #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<#ident_update_element_upper_camel_case>>>));
                            }
                        }
                        #query_snake_case
                    }
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: _,
            } => match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                    #query_snake_case = #value_snake_case.0.#update_query_bind_snake_case(#query_snake_case);
                    #query_snake_case
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    match #value_snake_case.0 {
                        Some(value) => {
                            #query_snake_case = value.#update_query_bind_snake_case(#query_snake_case);
                        }
                        None => {
                            #query_snake_case = #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<#std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case>>));
                        }
                    }
                    #query_snake_case
                },
            },
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
            } => todo!(),
            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
                dimension1_not_null_or_nullable: _,
                dimension2_not_null_or_nullable: _,
                dimension3_not_null_or_nullable: _,
                dimension4_not_null_or_nullable: _,
            } => todo!()
        };

        let impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream = {
            let postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                &ident,
                &ident_create_upper_camel_case,
                // &match &postgresql_json_type {
                //     PostgresqlJsonType::Object => {
                //         let postgresql_json_object_type = PostgresqlJsonObjectType::WithoutId;//todo WithId
                //         let ok_value_token_stream = match &postgresql_json_object_type {
                //             PostgresqlJsonObjectType::WithoutId => quote::quote! {format!("{increments}")},
                //             PostgresqlJsonObjectType::WithId => quote::quote! {format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}")}
                //         };
                //         let create_query_part_fields_token_stream = vec_syn_field.iter().map(|element| {
                //             let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                //                 panic!("{}", naming::FIELD_IDENT_IS_NONE);
                //             });
                //             let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
                //             let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                //             quote::quote! {
                //                 match #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_part_snake_case(&value.#element_field_ident, #increment_snake_case) {
                //                     Ok(value) => {
                //                         #increments_snake_case.push_str(&#postgresql_crud_wrap_into_jsonb_build_object_token_stream(#element_field_ident_double_quotes_token_stream, &value));
                //                     }
                //                     Err(error) => {
                //                         return Err(error);
                //                     }
                //                 }
                //             }
                //         });
                //         quote::quote! {
                //             let mut #increments_snake_case = std::string::String::from("");
                //             #(#create_query_part_fields_token_stream)*
                //             let _ = #increments_snake_case.pop();
                //             let _ = #increments_snake_case.pop();
                //             Ok(#ok_value_token_stream)
                //         }
                //     },
                //     PostgresqlJsonType::StdOptionOptionObject => quote::quote! {
                //         match &#value_snake_case.0 {
                //             Some(#value_snake_case) => match #value_snake_case.#create_query_part_snake_case(#increment_snake_case) {
                //                 Ok(#value_snake_case) => Ok(#value_snake_case),
                //                 Err(error) => Err(error)
                //             },
                //             //maybe not use null here and use increment logic
                //             None => Ok(std::string::String::from("null"))
                //         }
                //     },
                //     PostgresqlJsonType::StdVecVecObjectWithId => quote::quote! {
                //         let mut acc = std::string::String::default();
                //         for element in &#value_snake_case.0 {
                //             match element.#create_query_part_snake_case(#increment_snake_case) {
                //                 Ok(#value_snake_case) => {
                //                     acc.push_str(&format!("{value},"));
                //                 },
                //                 Err(error) => {
                //                     return Err(error);
                //                 }
                //             }
                //         }
                //         let _ = acc.pop();
                //         Ok(format!("jsonb_build_array({acc})"))
                //     },
                //     PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote! {
                //         match &#value_snake_case.0 {
                //             Some(#value_snake_case) => {
                //                 let mut acc = std::string::String::default();
                //                 for element in #value_snake_case {
                //                     match element.#create_query_part_snake_case(#increment_snake_case) {
                //                         Ok(#value_snake_case) => {
                //                             acc.push_str(&format!("{value},"));
                //                         },
                //                         Err(error) => {
                //                             return Err(error);
                //                         }
                //                     }
                //                 }
                //                 let _ = acc.pop();
                //                 Ok(format!("jsonb_build_array({acc})"))
                //             },
                //             None => Ok(std::string::String::from("null"))
                //         }
                //     },
                // },
                &create_query_part_token_stream,
                // &match &postgresql_json_type {
                //     PostgresqlJsonType::Object => {
                //         let create_query_bind_fields_token_stream = vec_syn_field.iter().map(|element| {
                //             let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                //                 panic!("{}", naming::FIELD_IDENT_IS_NONE);
                //             });
                //             let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_field_token_stream(element);
                //             quote::quote! {
                //                 #query_snake_case = #field_type_as_crud_postgresql_json_type_from_field_token_stream #create_query_bind_snake_case(value.#element_field_ident, #query_snake_case);
                //             }
                //         });
                //         quote::quote! {
                //             #(#create_query_bind_fields_token_stream)*
                //             #query_snake_case
                //         }
                //     },
                //     PostgresqlJsonType::StdOptionOptionObject => quote::quote! {
                //         if let Some(#value_snake_case) = #value_snake_case.0 {
                //             #query_snake_case = #value_snake_case.#create_query_bind_snake_case(#query_snake_case);
                //         }
                //         #query_snake_case
                //     },
                //     PostgresqlJsonType::StdVecVecObjectWithId => quote::quote! {
                //         for element in #value_snake_case.0 {
                //             #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                //         }
                //         #query_snake_case
                //     },
                //     PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote! {
                //         if let Some(#value_snake_case) = #value_snake_case.0 {
                //             for element in #value_snake_case {
                //                 #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                //             }
                //         }
                //         #query_snake_case
                //     },
                // },
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                &create_query_bind_token_stream,
                &ident_select_upper_camel_case,
                // &tokens_read_upper_camel_case,
                &ident_without_id_read_upper_camel_case,
                // &tokens_with_id_read_upper_camel_case,

                // &{
                //     let column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageFieldIdentSnakeCase;
                //     let generate_acc_push_str_variant_logic_token_stream =
                //         |variant_name_token_stream: &dyn quote::ToTokens, field_ident_double_quotes_token_stream: &dyn quote::ToTokens, column_name_and_maybe_field_getter_token_stream: &dyn quote::ToTokens, element_type: &dyn quote::ToTokens| {
                //             let tokens_select_with_or_without_id_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type {
                //                 PostgresqlJsonType::Object | PostgresqlJsonType::StdOptionOptionObject => &ident_select_without_id_upper_camel_case,
                //                 PostgresqlJsonType::StdVecVecObjectWithId | PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &ident_select_with_id_upper_camel_case,
                //             };
                //             let field_type_as_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_crud_postgresql_json_type_from_to_tokens_token_stream(&element_type);
                //             quote::quote! {
                //                 #tokens_select_with_or_without_id_upper_camel_case_token_stream::#variant_name_token_stream(value) => #field_type_as_crud_postgresql_json_type_from_field_token_stream #select_query_part_snake_case(
                //                     &value,
                //                     #field_ident_double_quotes_token_stream,
                //                     #column_name_and_maybe_field_getter_token_stream,
                //                     &#column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case,
                //                     false,
                //                 )
                //             }
                //         };
                //     let column_name_and_maybe_field_getter_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterFieldIdentSnakeCase;
                //     let value_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&value_snake_case);
                //     let (maybe_id_variant_token_stream, variants_token_stream) = {
                //         let maybe_id_variant_token_stream = match &postgresql_json_type {
                //             PostgresqlJsonType::Object | PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                //             PostgresqlJsonType::StdVecVecObjectWithId | PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                //                 let id_upper_camel_case = naming::IdUpperCamelCase;
                //                 let id_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&naming::IdSnakeCase);
                //                 let value = generate_acc_push_str_variant_logic_token_stream(
                //                     &quote::quote! {#id_upper_camel_case},
                //                     &id_snake_case_double_quotes_token_stream,
                //                     &value_snake_case_double_quotes_token_stream,
                //                     &postgresql_crud_path_postgresql_json_type_uuid_uuid_token_stream,
                //                 );
                //                 quote::quote! {#value,}
                //             }
                //         };
                //         let variants_token_stream = vec_syn_field.iter().map(|element| {
                //             let field_ident_stringified = element
                //                 .ident
                //                 .as_ref()
                //                .unwrap_or_else(|| {
                //                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                //                 })
                //                 .to_string();
                //             generate_acc_push_str_variant_logic_token_stream(
                //                 &naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified),
                //                 &generate_quotes::double_quotes_token_stream(&field_ident_stringified),
                //                 &match &postgresql_json_type {
                //                     PostgresqlJsonType::Object | PostgresqlJsonType::StdOptionOptionObject => quote::quote! {&#column_name_and_maybe_field_getter_field_ident_snake_case},
                //                     PostgresqlJsonType::StdVecVecObjectWithId | PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote! {#value_snake_case_double_quotes_token_stream},
                //                 },
                //                 &{
                //                     let element_type = &element.ty;
                //                     quote::quote! {#element_type}
                //                 },
                //             )
                //         });
                //         (maybe_id_variant_token_stream, variants_token_stream)
                //     };
                //     let self_field_vec_token_stream = match &postgresql_json_type {
                //         PostgresqlJsonType::Object | PostgresqlJsonType::StdOptionOptionObject => quote::quote! {.0.to_vec()},
                //         PostgresqlJsonType::StdVecVecObjectWithId | PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote! {.field_vec},
                //     };
                //     let maybe_pagination_start_end_initialization_token_stream = match &postgresql_json_type {
                //         PostgresqlJsonType::Object | PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                //         PostgresqlJsonType::StdVecVecObjectWithId | PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&naming::SelectSnakeCase),
                //     };
                //     let column_name_and_maybe_field_getter_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{{{field_ident_snake_case}}}'"));
                //     let column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{column_name_and_maybe_field_getter_for_error_message_snake_case}}}.{{{field_ident_snake_case}}}"));
                //     let (if_postgresql_type_is_true_format_handle_double_quotes_token_stream, if_postgresql_type_is_false_format_handle_double_quotes_token_stream) = {
                //         let wrap_into_jsonb_build_object_field_ident = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{{{field_ident_snake_case}}}', {value})");
                //         let wrap_into_jsonb_build_object_value = |value: &dyn std::fmt::Display| format!("jsonb_build_object('{value_snake_case}',{value})");
                //         let acc_format_handle = {
                //             let acc_snake_case = naming::AccSnakeCase;
                //             format!("{{{acc_snake_case}}}")
                //         };
                //         let jsonb_build_object_value_acc_format_handle = wrap_into_jsonb_build_object_value(&acc_format_handle);
                //         let null_snake_case = naming::NullSnakeCase;
                //         match postgresql_json_type {
                //             PostgresqlJsonType::Object => (acc_format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&jsonb_build_object_value_acc_format_handle)),
                //             PostgresqlJsonType::StdOptionOptionObject => (
                //                 format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else {acc_format_handle} end"),
                //                 wrap_into_jsonb_build_object_field_ident(&{
                //                     let jsonb_build_object_value_null = wrap_into_jsonb_build_object_value(&null_snake_case);
                //                     format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {jsonb_build_object_value_null} else {jsonb_build_object_value_acc_format_handle} end")
                //                 }),
                //             ),
                //             PostgresqlJsonType::StdVecVecObjectWithId => {
                //                 let format_handle = format!("(select jsonb_agg({{acc}}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}})");
                //                 (format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&wrap_into_jsonb_build_object_value(&format_handle)))
                //             }
                //             PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                //                 let format_handle = format!(
                //                     "case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else (select jsonb_agg({acc_format_handle}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}}) end"
                //                 );
                //                 (format_handle.clone(), wrap_into_jsonb_build_object_field_ident(&wrap_into_jsonb_build_object_value(&format_handle)))
                //             }
                //         }
                //     };
                //     quote::quote! {
                //         let mut acc = std::string::String::default();
                //         let #column_name_and_maybe_field_getter_field_ident_snake_case = if is_postgresql_type {
                //             column_name_and_maybe_field_getter.to_string()
                //         } else {
                //             format!(#column_name_and_maybe_field_getter_format_handle_token_stream)
                //         };
                //         let #column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = format!(#column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream);
                //         for element in #value_snake_case #self_field_vec_token_stream {
                //             acc.push_str(&format!(
                //                 "{}||",
                //                 match element {
                //                     #maybe_id_variant_token_stream
                //                     #(#variants_token_stream),*
                //                 }
                //             ));
                //         }
                //         let _ = acc.pop();
                //         let _ = acc.pop();
                //         #maybe_pagination_start_end_initialization_token_stream
                //         if is_postgresql_type {
                //             format!(#if_postgresql_type_is_true_format_handle_double_quotes_token_stream)
                //         }
                //         else {
                //             format!(#if_postgresql_type_is_false_format_handle_double_quotes_token_stream)
                //         }
                //     }
                // },
                &generate_select_query_part_token_stream(&postgresql_crud_macros_common::PostgresqlTypeOrJsonType::PostgresqlJsonType),
                &ident_where_element_upper_camel_case,
                &ident_update_upper_camel_case,
                &update_query_part_token_stream,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                &update_query_bind_token_stream,
            );
            quote::quote! {
                #postgresql_json_type_for_ident_token_stream
            }
        };
        let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream = {
            //todo
            let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
            let ident_table_type_declaration_token_stream = {
                let ident_table_type_declaration_token_stream = generate_supported_generics_template_struct_token_stream(
                    true,
                    &ident_table_type_declaration_upper_camel_case,
                    &generate_field_type_handle(
                        &generate_fields_declaration_create_token_stream(IsPub::False),
                        |tokens|{
                            let content = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&tokens);
                            quote::quote!{#content}
                        }
                    ),
                );
                let impl_std_fmt_display_for_ident_table_type_declaration_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &ident_table_type_declaration_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {write!(formatter, "{:?}", self)}
                );
                let impl_error_occurence_lib_to_std_string_string_for_ident_table_type_declaration_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &ident_table_type_declaration_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {format!("{self}")}
                );
                let impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_create_table_column_query_part_token_stream(
                    &ident_table_type_declaration_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let jsonb = "jsonb";
                        //todo
                        let type_stringified: &dyn std::fmt::Display = &format!("{jsonb} not null");
                        // match &postgresql_type {
                        //     PostgresqlType::JsonbNotNull => &format!("{jsonb} not null"),
                        //     PostgresqlType::JsonbNullable => &jsonb,
                        // };
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{column}} {type_stringified} check (jsonb_matches_schema('{{}}', {{column}}))"));
                        quote::quote! {
                            format!(#format_handle_token_stream, serde_json::to_string(&schemars::schema_for!(#ident_table_type_declaration_upper_camel_case)).unwrap())
                        }
                    }
                );
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_table_type_declaration_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let content_token_stream = match &postgresql_json_type_pattern {
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{#impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_fields_token_stream},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{(Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                            },
                            postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {..}
                            | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {..}
                            | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {..}
                            | postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {..}
                            => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{(vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{(Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))},
                            }
                        };
                        quote::quote! {Self #content_token_stream}
                    }
                );
                quote::quote! {
                    #ident_table_type_declaration_token_stream
                    #impl_std_fmt_display_for_ident_table_type_declaration_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_table_type_declaration_token_stream
                    #impl_create_table_column_query_part_for_ident_table_type_declaration_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream
                }
            };
            let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_type_for_ident_token_stream(
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
                &ident,
                &ident_table_type_declaration_upper_camel_case,
                &ident_create_upper_camel_case,
                // &{
                //     let generate_ok_try_generate_create_token_stream = |is_self_zero: std::primitive::bool| {
                //         let first_argument_token_stream: &dyn quote::ToTokens = if is_self_zero { &quote::quote! {&value.0.0} } else { &value_snake_case };
                //         //todo remove .unwrap()
                //         quote::quote! {Ok(#tokens_as_json_type_token_stream create_query_part(#first_argument_token_stream, increment).unwrap())}
                //     };
                //     match &postgresql_type {
                //         PostgresqlType::JsonbNotNull => generate_ok_try_generate_create_token_stream(true),
                //         PostgresqlType::JsonbNullable => {
                //             // let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
                //             // let postgresql_crud_postgresql_json_type_try_generate_postgresql_json_object_type_to_create_error_named_token_stream = quote::quote!{
                //             //     postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonObjectTypeToCreateErrorNamed
                //             // };
                //             let ok_as_json_type_token_stream = generate_ok_try_generate_create_token_stream(false);
                //             quote::quote! {
                //                 match &value.0 {
                //                     Some(#value_snake_case) => #ok_as_json_type_token_stream,
                //                     None => match increment.checked_add(1) {
                //                         Some(#value_snake_case) => {
                //                             *increment = #value_snake_case;
                //                             Ok(format!("${increment}"))
                //                         }
                //                         None =>
                //                         //- Err(#postgresql_crud_postgresql_json_type_try_generate_postgresql_json_object_type_to_create_error_named_token_stream::#checked_add_upper_camel_case {
                //                         //-     code_occurence: error_occurence_lib::code_occurence!()
                //                         //- })
                //                         todo!() //todo make generic error type instead of PostgresqlJsonTypeTryGeneratePostgresqlJsonObjectTypeToCreateErrorNamed
                //                         ,
                //                     }
                //                 }
                //             }
                //         }
                //     }
                // },
                &create_query_part_token_stream,
                // &{
                //     let query_snake_case = naming::QuerySnakeCase;
                //     let generate_create_query_bind_token_stream = |is_self_zero: std::primitive::bool| {
                //         let first_argument_token_stream: &dyn quote::ToTokens = if is_self_zero { &quote::quote! {value.0.0} } else { &value_snake_case };
                //         //todo reuse naming
                //         quote::quote! {#tokens_as_json_type_token_stream create_query_bind(#first_argument_token_stream, #query_snake_case)}
                //     };
                //     match &postgresql_type {
                //         PostgresqlType::JsonbNotNull => generate_create_query_bind_token_stream(true),
                //         PostgresqlType::JsonbNullable => {
                //             let bind_value_to_postgresql_query_part_to_create_token_stream = generate_create_query_bind_token_stream(false);
                //             quote::quote! {
                //                 #query_snake_case = match value.0 {
                //                     Some(#value_snake_case) => #bind_value_to_postgresql_query_part_to_create_token_stream,
                //                     None => #query_snake_case.bind(std::option::Option::<sqlx::types::Json<#ident_create_upper_camel_case>>::None)
                //                 };
                //                 #query_snake_case
                //             }
                //         }
                //     }
                // },
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                &create_query_bind_token_stream,
                &ident_select_upper_camel_case,
                // &{
                //     let format_value_token_stream = match &postgresql_type {
                //         PostgresqlType::JsonbNotNull => quote::quote! {
                //             #tokens_as_json_type_token_stream select_query_part(&#value_snake_case, &column, &column, &column, true)
                //         },
                //         PostgresqlType::JsonbNullable => quote::quote! {
                //             match &#value_snake_case.0 {
                //                 Some(#value_snake_case) => #tokens_as_json_type_token_stream select_query_part(value, &column, &column, &column, true),
                //                 None => "null".to_string()
                //             }
                //         },
                //     };
                //     quote::quote! {format!("{} as {column}", #format_value_token_stream)}
                // },
                &{
                    let common_select_query_part_token_stream = generate_select_query_part_token_stream(&postgresql_crud_macros_common::PostgresqlTypeOrJsonType::PostgresqlType);
                    quote::quote!{
                        let value = {
                            #common_select_query_part_token_stream
                        };
                        format!("{value} as {column}")
                    }
                },
                &ident_where_element_upper_camel_case,
                &ident_without_id_read_upper_camel_case,
                &ident_update_upper_camel_case,
                // &{
                    // //todo remove jsonb_ prefix (coz it can be json, jsonb, json not null, jsonb not null)
                    // let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                    // let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
                    // let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
                    // let increment_snake_case = naming::IncrementSnakeCase;
                    // let generate_ok_try_generate_postgresql_json_object_type_to_update_token_stream = |is_postgresql_type_self_to_update_zero: std::primitive::bool| {
                    //     let first_argument_token_stream: &dyn quote::ToTokens = if is_postgresql_type_self_to_update_zero { &quote::quote! {&#value_snake_case} } else { &value_snake_case };
                    //     //todo proper error handling - remove .unwrap()
                    //     quote::quote! {
                    //         Ok(#tokens_as_json_type_token_stream update_query_part(
                    //             #first_argument_token_stream,
                    //             #jsonb_set_accumulator_snake_case,
                    //             #jsonb_set_target_snake_case,
                    //             #jsonb_set_path_snake_case,
                    //             #increment_snake_case,
                    //         ).unwrap())
                    //     }
                    // };
                    // match &postgresql_type {
                    //     PostgresqlType::JsonbNotNull => generate_ok_try_generate_postgresql_json_object_type_to_update_token_stream(true),
                    //     PostgresqlType::JsonbNullable => {
                    //         let ok_try_generate_postgresql_json_object_type_to_update_token_stream = generate_ok_try_generate_postgresql_json_object_type_to_update_token_stream(false);
                    //         quote::quote! {
                    //             match &#value_snake_case.0 {
                    //                 Some(#value_snake_case) => #ok_try_generate_postgresql_json_object_type_to_update_token_stream,
                    //                 None => match increment.checked_add(1) {
                    //                     Some(#value_snake_case) => {
                    //                         *increment = #value_snake_case;
                    //                         Ok(format!("${increment}"))
                    //                     },
                    //                     //todo make generic error type instead of PostgresqlJsonTypeTryGeneratePostgresqlJsonObjectTypeToCreateErrorNamed
                    //                     None => Err(postgersql_crud::QueryPartErrorNamed::#checked_add_upper_camel_case {
                    //                         code_occurence: error_occurence_lib::code_occurence!()
                    //                     })
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                // },
                &update_query_part_token_stream,
                // &{
                    // match &postgresql_type {
                    //     PostgresqlType::JsonbNotNull => quote::quote! {
                    //         #tokens_as_json_type_token_stream update_query_bind(
                    //             #value_snake_case,
                    //             query
                    //         )
                    //     },
                    //     PostgresqlType::JsonbNullable => quote::quote! {
                    //         query = match #value_snake_case.0 {
                    //             Some(#value_snake_case) => #tokens_as_json_type_token_stream update_query_bind(
                    //                 #value_snake_case,
                    //                 query
                    //             ),
                    //             None => {
                    //                 #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<#ident_update_upper_camel_case>>))
                    //             }
                    //         };
                    //         query
                    //     },
                    // }
                // },
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                &update_query_bind_token_stream
            );
            // println!("{impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_ident_token_stream}");
            quote::quote! {
                #ident_table_type_declaration_token_stream
                #impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_ident_token_stream
            }
        };
        let (
            maybe_impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream,
            maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
        ) = match &trait_gen {
            TraitGen::PostgresqlType => (
                proc_macro2::TokenStream::new(),
                impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
            ),
            TraitGen::PostgresqlTypeAndPostgresqlJsonType => (
                impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream,
                impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
            ),
            TraitGen::PostgresqlJsonType => (
                impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream,
                proc_macro2::TokenStream::new(),
            ),
        };
        let generated = quote::quote! {
            #ident_token_stream
            // #ident_origin_token_stream
            #ident_create_token_stream
            #ident_select_token_stream
            #ident_where_element_token_stream
            #ident_read_token_stream
            #ident_update_token_stream

            #common_token_stream

            //todo return it later
            // #object_with_id_ident_token_stream

            #maybe_impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream
            #maybe_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_token_stream
        };
        // if let (
        //     postgresql_crud_macros_common::NotNullOrNullable::NotNull,
        //     // postgresql_crud_macros_common::NotNullOrNullable::Nullable,

        //     postgresql_crud_macros_common::PostgresqlJsonTypePattern::Standart,
        //     // postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension1 {
        //     //     dimension1_not_null_or_nullable,
        //     // },
        //     // postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension2 {
        //     //     dimension1_not_null_or_nullable,
        //     //     dimension2_not_null_or_nullable,
        //     // },
        //     // postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension3 {
        //     //     dimension1_not_null_or_nullable,
        //     //     dimension2_not_null_or_nullable,
        //     //     dimension3_not_null_or_nullable,
        //     // },
        //     // postgresql_crud_macros_common::PostgresqlJsonTypePattern::ArrayDimension4 {
        //     //     dimension1_not_null_or_nullable,
        //     //     dimension2_not_null_or_nullable,
        //     //     dimension3_not_null_or_nullable,
        //     //     dimension4_not_null_or_nullable,
        //     // },

        //     // TraitGen::PostgresqlType,
        //     // TraitGen::PostgresqlJsonType,
        //     TraitGen::PostgresqlTypeAndPostgresqlJsonType,
        // ) = (
        //     &not_null_or_nullable,
        //     &postgresql_json_type_pattern,
        //     &trait_gen,
        // ) {
        //     // use postgresql_crud_macros_common::NotNullOrNullable;
        //     // let d1 = match &dimension1_not_null_or_nullable {
        //     //     NotNullOrNullable::NotNull => true,
        //     //     NotNullOrNullable::Nullable => false,
        //     // };
        //     // let d2 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     // };
        //     // let d3 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     // };
        //     // let d4 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
        //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
        //     // };
        //     // if d1 {
        //         if syn_derive_input_ident == "Animal" {
        //             macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //                 "GeneratePostgresqlJsonObjectType",
        //                 &generated,
        //             );
        //         }
        //     // }
        // }
        generated.to_string()
    })
    .collect::<std::vec::Vec<String>>();
    let generated = {
        let postgresql_json_object_type_array = postgresql_json_object_type_array
        .into_iter()
        .map(|element|{
            element.parse::<proc_macro2::TokenStream>().unwrap()
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_json_object_type_array)*}
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlJsonObjectTypeTokens",
    //     &generated,
    // );
    generated.into()
}
//todo impl custom serde::Deserialize for PostgresqlTypeObjectAnimalAsJsonbNotNullWhere
