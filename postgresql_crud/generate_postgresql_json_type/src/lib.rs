//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
#[proc_macro_derive(GeneratePostgresqlJsonType)]
pub fn generate_postgresql_json_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().collect::<std::vec::Vec<&syn::Field>>()
        } else {
            panic!("supports only syn::Fields::Named");
        }
    } else {
        panic!("does work only on structs!");
    };
    let ident_to_create_with_generated_id_upper_camel_case = naming::parameter::SelfToCreateWithGeneratedIdUpperCamelCase::from_tokens(&ident);
    let ident_to_create_without_generated_id_upper_camel_case = naming::parameter::SelfToCreateWithoutGeneratedIdUpperCamelCase::from_tokens(&ident);


    let ident_options_to_read_upper_camel_case = naming::parameter::SelfOptionsToReadUpperCamelCase::from_tokens(&ident);
    let ident_field_to_update_upper_camel_case = naming::parameter::SelfFieldToUpdateUpperCamelCase::from_tokens(&ident);

    let ident_field_to_read_without_id_upper_camel_case = naming::parameter::SelfFieldToReadWithoutIdUpperCamelCase::from_tokens(&ident);
    let ident_field_to_read_with_id_upper_camel_case = naming::parameter::SelfFieldToReadWithIdUpperCamelCase::from_tokens(&ident);

    let ident_options_to_read_without_id_upper_camel_case = naming::parameter::SelfOptionsToReadWithoutIdUpperCamelCase::from_tokens(&ident);
    let ident_options_to_read_with_id_upper_camel_case = naming::parameter::SelfOptionsToReadWithIdUpperCamelCase::from_tokens(&ident);

    let ident_update_origin_upper_camel_case = naming::parameter::SelfUpdateOriginUpperCamelCase::from_tokens(&ident);
    
    let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
    let ident_options_to_update_upper_camel_case = naming::parameter::SelfOptionsToUpdateUpperCamelCase::from_tokens(&ident);

    let value_snake_case = naming::ValueSnakeCase;

    let ident_json_array_change_try_generate_error_named_upper_camel_case = naming::parameter::SelfJsonArrayChangeTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident);
    let postgresql_crud_path_token_stream = {
        let postgresql_crud_snake_case = naming::PostgresqlCrudSnakeCase;
        quote::quote!{#postgresql_crud_snake_case::}
    };

    let postgersql_crud_pagination_token_stream = quote::quote!{#postgresql_crud_path_token_stream Pagination};

    let postgresql_crud_uuid_update_token_stream = quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidUpdate};//todo reuse

    let postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
    let postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;

    let (
        generate_struct_tokens_double_quotes_token_stream,
        generate_struct_tokens_with_2_elements_double_quotes_token_stream
    ) = {
        const STRUCT_SPACE_STRINGIFIED: &str = "struct ";
        let generate_struct_tokens_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            generate_quotes::double_quotes_token_stream(&format!("{STRUCT_SPACE_STRINGIFIED}{value}"))
        };
        let generate_struct_tokens_with_2_elements_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            generate_quotes::double_quotes_token_stream(&format!("{STRUCT_SPACE_STRINGIFIED}{value} with 2 elements"))
        };
        (
            generate_struct_tokens_double_quotes_token_stream,
            generate_struct_tokens_with_2_elements_double_quotes_token_stream
        )
    };

    fn generate_supported_generics_template_struct_token_stream(is_pub: std::primitive::bool, struct_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let maybe_pub_token_stream = if is_pub {
            quote::quote!{pub}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            #maybe_pub_token_stream struct #struct_ident_token_stream #content_token_stream
        }
    }

    enum PostgresqlJsonTypeFieldReader {
        //todo why Ident variant here? maybe remove it?
        Ident,
        ObjectIdent,
        StdOptionOptionObjectIdent,
        StdVecVecObjectWithIdIdent,
        StdOptionOptionStdVecVecObjectWithIdIdent,
    }
    let generate_tokens_field_reader_token_stream = |postgresql_json_type_field_reader: &PostgresqlJsonTypeFieldReader|{
        let tokens_field_reader_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type_field_reader {
            PostgresqlJsonTypeFieldReader::Ident => &naming::parameter::SelfFieldReaderUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::ObjectIdent => &naming::parameter::ObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdOptionOptionObjectIdent => &naming::parameter::StdOptionOptionObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdVecVecObjectWithIdIdent => &naming::parameter::StdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdOptionOptionStdVecVecObjectWithIdIdent => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
        };
        let std_vec_vec_ident_field_to_read_without_id_upper_camel_case_token_stream = quote::quote!{std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case>};
        let field_vec_std_vec_vec_ident_field_to_read_with_id_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream = quote::quote!{field_vec: std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>, pagination: #postgersql_crud_pagination_token_stream};
        let content_token_stream = match &postgresql_json_type_field_reader {
            //todo option (null) must be an read option too
            PostgresqlJsonTypeFieldReader::Ident |
            PostgresqlJsonTypeFieldReader::ObjectIdent |
            PostgresqlJsonTypeFieldReader::StdOptionOptionObjectIdent => quote::quote!{(#std_vec_vec_ident_field_to_read_without_id_upper_camel_case_token_stream);},
            PostgresqlJsonTypeFieldReader::StdVecVecObjectWithIdIdent |
            PostgresqlJsonTypeFieldReader::StdOptionOptionStdVecVecObjectWithIdIdent => quote::quote!{
                {
                    #field_vec_std_vec_vec_ident_field_to_read_with_id_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream
                }
            },
        };
        let tokens_field_reader_try_new_error_named_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type_field_reader {
            PostgresqlJsonTypeFieldReader::Ident => &naming::parameter::SelfFieldReaderTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::ObjectIdent => &naming::parameter::ObjectSelfFieldReaderTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdOptionOptionObjectIdent => &naming::parameter::StdOptionOptionObjectSelfFieldReaderTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdVecVecObjectWithIdIdent => &naming::parameter::StdVecVecObjectWithIdSelfFieldReaderTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlJsonTypeFieldReader::StdOptionOptionStdVecVecObjectWithIdIdent => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfFieldReaderTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        let fields_filter_is_empty_upper_camel_case = naming::FieldsFilterIsEmptyUpperCamelCase;
        let not_unique_field_filter_upper_camel_case = naming::NotUniqueFieldFilterUpperCamelCase;
        let generate_impl_pub_fn_try_new_token_stream = |
            contains_id: std::primitive::bool,
            input_parameters_token_stream: &dyn quote::ToTokens,
            is_vec: std::primitive::bool,
        |{
            let field_type_token_stream: &dyn quote::ToTokens = if contains_id {
                &ident_field_to_read_with_id_upper_camel_case
            }
            else {
                &ident_field_to_read_without_id_upper_camel_case
            };
            
            let field_vec_snake_case = naming::FieldVecSnakeCase;
            let pagination_snake_case = naming::PaginationSnakeCase;
            let check_handle_token_stream = if is_vec {
                quote::quote!{#field_vec_snake_case}
            }
            else {
                quote::quote!{#value_snake_case}
            };
            let unique_snake_case = naming::UniqueSnakeCase;
            let self_initialization_token_stream = if is_vec {
                quote::quote!{{ #field_vec_snake_case: #unique_snake_case, #pagination_snake_case }}
            }
            else {
                quote::quote!{(#unique_snake_case)}
            };
            quote::quote!{
                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #tokens_field_reader_try_new_error_named_upper_camel_case_token_stream {
                    #fields_filter_is_empty_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #not_unique_field_filter_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #field_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
                impl #tokens_field_reader_upper_camel_case_token_stream {
                    pub fn try_new(#input_parameters_token_stream) -> Result<Self, #tokens_field_reader_try_new_error_named_upper_camel_case_token_stream> {
                        if #check_handle_token_stream.is_empty() {
                            return Err(#tokens_field_reader_try_new_error_named_upper_camel_case_token_stream::#fields_filter_is_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                        let mut #unique_snake_case = vec![];
                        for element in #check_handle_token_stream {
                            if #unique_snake_case.contains(&element) {
                                return Err(#tokens_field_reader_try_new_error_named_upper_camel_case_token_stream::#not_unique_field_filter_upper_camel_case {
                                    field: element,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            } else {
                                #unique_snake_case.push(element);
                            }
                        }
                        Ok(Self #self_initialization_token_stream)
                    }
                }
            }
        };
        let impl_pub_fn_try_new_token_stream = {
            let generate_value_input_parameter_type_token_stream = |value_token_stream: &dyn quote::ToTokens|{
                quote::quote!{#value_snake_case: #value_token_stream}
            };
            match &postgresql_json_type_field_reader {
                PostgresqlJsonTypeFieldReader::Ident |
                PostgresqlJsonTypeFieldReader::ObjectIdent |
                PostgresqlJsonTypeFieldReader::StdOptionOptionObjectIdent => generate_impl_pub_fn_try_new_token_stream(
                    false,
                    &generate_value_input_parameter_type_token_stream(&std_vec_vec_ident_field_to_read_without_id_upper_camel_case_token_stream),
                    false,
                ),
                PostgresqlJsonTypeFieldReader::StdVecVecObjectWithIdIdent |
                PostgresqlJsonTypeFieldReader::StdOptionOptionStdVecVecObjectWithIdIdent => generate_impl_pub_fn_try_new_token_stream(
                    true,
                    &field_vec_std_vec_vec_ident_field_to_read_with_id_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream,
                    true,
                )
            }
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #tokens_field_reader_upper_camel_case_token_stream #content_token_stream
            #impl_pub_fn_try_new_token_stream
        }
    };
    fn generate_match_try_new_in_deserialize_token_stream(ident_token_stream: &dyn quote::ToTokens, initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        quote::quote!{
            match #ident_token_stream::try_new(#initialization_token_stream) {
                Ok(value) => serde::__private::Ok(value),
                Err(error) => {
                    return Err(serde::de::Error::custom(format!("{error:?}")));
                }
            }
        }
    }
    
    let generate_postgresql_json_type_tokens_options_to_read_alias_token_stream = |postgresql_json_type_tokens_options_to_read_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool|{
        let postgresql_json_type_tokens_options_to_read_with_or_without_id_token_stream = if contains_id {
            quote::quote!{#ident_options_to_read_with_id_upper_camel_case}
        }
        else {
            quote::quote!{#ident_options_to_read_without_id_upper_camel_case}
        };
        macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(postgresql_json_type_tokens_options_to_read_token_stream, &postgresql_json_type_tokens_options_to_read_with_or_without_id_token_stream)
    };
    let postgresql_crud_create_query_part_error_named_token_stream = {
        let create_query_part_error_named_upper_camel_case = naming::CreateQueryPartErrorNamedUpperCamelCase;
        quote::quote!{#postgresql_crud_path_token_stream #create_query_part_error_named_upper_camel_case}
    };
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let increments_snake_case = naming::IncrementsSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let generate_tokens_to_create_methods_token_stream = |
        struct_ident_token_stream: &dyn quote::ToTokens,
        create_query_part_content_token_stream: &dyn quote::ToTokens,
        create_query_bind_content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl #struct_ident_token_stream {
                fn #create_query_part_snake_case(&self, #increment_snake_case: &mut std::primitive::u64) -> Result<std::string::String, #postgresql_crud_create_query_part_error_named_token_stream> {
                    #create_query_part_content_token_stream
                }
                fn #create_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #create_query_bind_content_token_stream
                }
            }
        }
    };

    let generate_tokens_try_generate_postgresql_json_type_error_named_token_stream = |
        struct_token_stream: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #struct_token_stream {
                #content_token_stream
            }
        }
    };

    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let (
        checked_add_variant_declaration_token_stream,
        checked_add_variant_initialization_token_stream
    ) = {
        let code_occurence_snake_case = naming::CodeOccurenceSnakeCase;
        let checked_add_variant_declaration_token_stream = quote::quote!{
            #checked_add_upper_camel_case {
                #code_occurence_snake_case: error_occurence_lib::code_occurence::CodeOccurence,
            }
        };
        let checked_add_variant_initialization_token_stream = quote::quote!{
            #checked_add_upper_camel_case {
                #code_occurence_snake_case: error_occurence_lib::code_occurence!()
            }
        };
        (
            checked_add_variant_declaration_token_stream,
            checked_add_variant_initialization_token_stream
        )
    };

    let generate_postgresql_json_type_tokens_options_to_read_token_stream = |
        postgresql_json_type_token_options_to_read_token_stream: &dyn quote::ToTokens,
        impl_serde_deserialize: std::primitive::bool,
        content_token_stream: &dyn quote::ToTokens,
    |{
        let maybe_impl_serde_deserialize_token_stream = if impl_serde_deserialize {
            quote::quote!{serde::Deserialize,}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, #maybe_impl_serde_deserialize_token_stream utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #postgresql_json_type_token_options_to_read_token_stream #content_token_stream
        }
    };
    let postgresql_crud_wrap_into_jsonb_build_object_token_stream = {
        let wrap_into_jsonb_build_object_snake_case = naming::WrapIntoJsonbBuildObjectSnakeCase;
        quote::quote!{#postgresql_crud_path_token_stream #wrap_into_jsonb_build_object_snake_case}
    };
    let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
        generate_quotes::double_quotes_token_stream(
            &value.ident
            .as_ref()
            .unwrap_or_else(|| {
                panic!("{}", naming::FIELD_IDENT_IS_NONE);
            })
        )
    };
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let generate_field_type_as_postgresql_crud_postgresql_json_type_from_to_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens|{
        quote::quote!{<#value_token_stream as #postgresql_crud_path_token_stream #postgresql_json_type_upper_camel_case>::}
    };
    let generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = |field: &syn::Field|{
        generate_field_type_as_postgresql_crud_postgresql_json_type_from_to_tokens_token_stream(&field.ty)
    };
    let id_snake_case = naming::IdSnakeCase;
    let id_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&id_snake_case);
    
    let ident_to_create_origin_upper_camel_case = naming::parameter::SelfToCreateOriginUpperCamelCase::from_tokens(&ident);

    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_fields_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                });
            quote::quote!{
                #field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            }
        });
        quote::quote!{{#(#fields_token_stream),*}}
    };
    
    let common_token_stream = {
        let create_token_stream = {
            let fields_declaration_token_stream = {
                let value = vec_syn_field.iter().map(|element| {
                    let field_ident = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                    let type_path_create_token_stream = naming::parameter::SelfCreateUpperCamelCase::from_type_last_segment(&element.ty);
                    quote::quote!{
                        #field_ident: #type_path_create_token_stream
                    }
                });
                quote::quote!{#(#value),*}
            };
            let ident_to_create_origin_token_stream = generate_supported_generics_template_struct_token_stream(
                false,
                &ident_to_create_origin_upper_camel_case,
                &quote::quote!{{ #fields_declaration_token_stream }}
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_origin_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_to_create_origin_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote!{Self #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_fields_token_stream}
            );
            let ident_to_create_origin_methods_token_stream = generate_tokens_to_create_methods_token_stream(
                &ident_to_create_origin_upper_camel_case,
                &{
                    let try_generate_postgresql_json_type_to_create_fields_token_stream = vec_syn_field.iter().map(|element| {
                        let element_field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                        let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
                        let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote!{
                            match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #create_query_part_snake_case(&self.#element_field_ident, #increment_snake_case) {
                                Ok(value) => {
                                    #increments_snake_case.push_str(&#postgresql_crud_wrap_into_jsonb_build_object_token_stream(#element_field_ident_double_quotes_token_stream, &value));
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                    });
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{increments_snake_case}}}"));
                    quote::quote!{
                        let mut #increments_snake_case = std::string::String::from("");
                        #(#try_generate_postgresql_json_type_to_create_fields_token_stream)*
                        let _ = #increments_snake_case.pop();
                        let _ = #increments_snake_case.pop();
                        Ok(format!(#format_handle_token_stream))
                    }
                },
                &{
                    let bind_value_to_postgresql_query_part_to_create_fields_token_stream = vec_syn_field.iter().map(|element| {
                        let element_field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                        let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote!{
                            #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #create_query_bind_snake_case(self.#element_field_ident, #query_snake_case);
                        }
                    });
                    quote::quote!{
                        #(#bind_value_to_postgresql_query_part_to_create_fields_token_stream)*
                        #query_snake_case
                    }
                },
            );
            let (
                ident_to_create_with_generated_id_token_stream,
                ident_to_create_without_generated_id_token_stream
            ) = {
                let content_token_stream = quote::quote!{(#ident_to_create_origin_upper_camel_case);};
                let ident_to_create_with_generated_id_token_stream = generate_supported_generics_template_struct_token_stream(
                    true,
                    &ident_to_create_with_generated_id_upper_camel_case,
                    &content_token_stream
                );
                let ident_to_create_without_generated_id_token_stream = generate_supported_generics_template_struct_token_stream(
                    true,
                    &ident_to_create_without_generated_id_upper_camel_case,
                    &content_token_stream
                );
                (
                    ident_to_create_with_generated_id_token_stream,
                    ident_to_create_without_generated_id_token_stream
                )
            };
            let (
                impl_new_for_ident_to_create_with_generated_id_token_stream,
                impl_new_for_ident_to_create_without_generated_id_token_stream
            ) = {
                let generate_impl_pub_new_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens|{
                    let fields_initialization_token_stream = vec_syn_field.iter().map(|element| {
                        element.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                    });
                    quote::quote!{
                        impl #struct_ident_token_stream {
                            pub fn new(#fields_declaration_token_stream) -> Self {
                                Self(#ident_to_create_origin_upper_camel_case {
                                    #(#fields_initialization_token_stream),*
                                })
                            }
                        }
                    }
                };
                let impl_new_for_ident_to_create_with_generated_id_token_stream = generate_impl_pub_new_token_stream(&ident_to_create_with_generated_id_upper_camel_case);
                let impl_new_for_ident_to_create_without_generated_id_token_stream = generate_impl_pub_new_token_stream(&ident_to_create_without_generated_id_upper_camel_case);
                (
                    impl_new_for_ident_to_create_with_generated_id_token_stream,
                    impl_new_for_ident_to_create_without_generated_id_token_stream
                )
            };
            let (
                impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_with_generated_id_token_stream,
                impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_without_generated_id_token_stream
            ) = {
                let initialization_token_stream = quote::quote!{Self(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)};
                let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_with_generated_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_to_create_with_generated_id_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &initialization_token_stream
                );
                let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_without_generated_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_to_create_without_generated_id_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &initialization_token_stream
                );
                (
                    impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_with_generated_id_token_stream,
                    impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_without_generated_id_token_stream
                )
            };
            let (
                impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_with_generated_id_token_stream,
                impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_without_generated_id_token_stream
            ) = {
                let generate_impl_json_create_tokens_to_create_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool|{
                    generate_tokens_to_create_methods_token_stream(
                        &struct_ident_token_stream,
                        &{
                            let ok_value_token_stream = if contains_id {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{id_snake_case}', to_jsonb(gen_random_uuid()))||{{value}}"));
                                quote::quote!{format!(#format_handle_token_stream)}
                            }
                            else {
                                quote::quote!{value}
                            };
                            quote::quote!{
                                match self.0.#create_query_part_snake_case(#increment_snake_case) {
                                    Ok(value) => Ok(#ok_value_token_stream),
                                    Err(error) => Err(error)
                                }
                            }
                        },
                        &{
                            quote::quote!{
                                #query_snake_case = self.0.#create_query_bind_snake_case(#query_snake_case);
                                #query_snake_case
                            }
                        },
                    )
                };
                let impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_with_generated_id_token_stream = generate_impl_json_create_tokens_to_create_token_stream(
                    &ident_to_create_with_generated_id_upper_camel_case,
                    true
                );
                let impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_without_generated_id_token_stream = generate_impl_json_create_tokens_to_create_token_stream(
                    &ident_to_create_without_generated_id_upper_camel_case,
                    false
                );
                (
                    impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_with_generated_id_token_stream,
                    impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_without_generated_id_token_stream
                )
            };
            //for new compatibility with GeneratePostgresqlCrud logic
            let 
            // (
            //     // impl_postgresql_crud_bind_query_second_for_ident_to_create_with_generated_id_token_stream,
            //     impl_postgresql_crud_bind_query_second_for_ident_to_create_without_generated_id_token_stream
            // )
            impl_postgresql_crud_bind_query_second_for_ident_to_create_without_generated_id_token_stream
            = {
                let generate_impl_postgresql_crud_bind_query_for_tokens_token_stream = |
                    ident_token_stream: &dyn quote::ToTokens,
                    try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
                    bind_value_to_query_token_stream: &dyn quote::ToTokens,
                |{
                    quote::quote!{
                        impl postgresql_crud::BindQuery for #ident_token_stream {
                            fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
                                #try_generate_bind_increments_token_stream
                            }
                            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                #bind_value_to_query_token_stream
                            }
                        }
                    }
                };
                // (
                //     // generate_impl_postgresql_crud_bind_query_for_tokens_token_stream(
                //     //     &ident_to_create_with_generated_id_upper_camel_case,
                //     //     &quote::quote!{todo!()},
                //     //     &quote::quote!{todo!()},
                //     // ),

                // )
                generate_impl_postgresql_crud_bind_query_for_tokens_token_stream(
                    &ident_to_create_without_generated_id_upper_camel_case,
                    &{
                        let try_generate_bind_increments_token_stream = vec_syn_field.iter().map(|element| {
                            let element_field_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
                            let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                            quote::quote!{
                                match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #create_query_part_snake_case(&self.0.#element_field_ident, #increment_snake_case) {
                                    Ok(value) => {
                                        #increments_snake_case.push_str(&#postgresql_crud_wrap_into_jsonb_build_object_token_stream(#element_field_ident_double_quotes_token_stream, &value));
                                    }
                                    Err(error) => {
                                        return Err(error.into());
                                    }
                                }
                            }
                        });
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{increments_snake_case}}}"));
                        quote::quote!{
                            let mut #increments_snake_case = std::string::String::from("");
                            #(#try_generate_bind_increments_token_stream)*
                            let _ = #increments_snake_case.pop();
                            let _ = #increments_snake_case.pop();
                            Ok(format!(#format_handle_token_stream))
                        }
                    },
                    &{
                        let bind_value_to_query_token_stream = vec_syn_field.iter().map(|element| {
                            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                            let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                            quote::quote!{
                                #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #create_query_bind_snake_case(self.0.#element_ident, #query_snake_case);
                            }
                        });
                        quote::quote!{
                            #(#bind_value_to_query_token_stream)*
                            #query_snake_case
                        }
                    }
                )
            };
            quote::quote!{
                #ident_to_create_origin_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_origin_token_stream
                #ident_to_create_origin_methods_token_stream

                #ident_to_create_with_generated_id_token_stream
                #ident_to_create_without_generated_id_token_stream

                #impl_new_for_ident_to_create_with_generated_id_token_stream
                #impl_new_for_ident_to_create_without_generated_id_token_stream
                
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_with_generated_id_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_without_generated_id_token_stream

                #impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_with_generated_id_token_stream
                #impl_postgresql_crud_json_create_postgresql_query_part_for_ident_to_create_without_generated_id_token_stream

                // #impl_postgresql_crud_bind_query_second_for_ident_to_create_with_generated_id_token_stream,
                #impl_postgresql_crud_bind_query_second_for_ident_to_create_without_generated_id_token_stream
            }
        };
        let (
            read_token_stream,
            update_token_stream
        ) = {
            let read_token_stream = {
                let (
                    ident_field_to_read_token_stream,
                    ident_with_id_field_to_read_token_stream
                ) = {
                    let generate_template_field_to_read_struct_token_stream = |
                        tokens_field_to_read_token_stream: &dyn quote::ToTokens,
                        additional_content_token_stream: &dyn quote::ToTokens,
                    |{
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                            let serialize_deserialize_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            let type_path_field_reader_token_stream = naming::parameter::SelfFieldReaderUpperCamelCase::from_type_last_segment(&element.ty);
                            quote::quote!{
                                #[serde(rename(serialize = #serialize_deserialize_field_ident_double_quotes_token_stream, deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream))]
                                #variant_ident_upper_camel_case_token_stream(#type_path_field_reader_token_stream)
                            }
                        });
                        quote::quote!{
                            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub enum #tokens_field_to_read_token_stream {
                                #additional_content_token_stream
                                #(#variants_token_stream),*
                            }
                        }
                    };
                    let ident_field_to_read_without_id_token_stream = generate_template_field_to_read_struct_token_stream(
                        &ident_field_to_read_without_id_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                    );
                    let ident_field_to_read_with_id_token_stream = generate_template_field_to_read_struct_token_stream(
                        &ident_field_to_read_with_id_upper_camel_case,
                        &quote::quote!{
                            #[serde(rename(serialize = #id_snake_case_double_quotes_token_stream, deserialize = #id_snake_case_double_quotes_token_stream))]
                             Id(#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidFieldReader),//todo reuse
                        },
                    );
                    (
                        ident_field_to_read_without_id_token_stream,
                        ident_field_to_read_with_id_token_stream
                    )
                };
                let (
                    impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream,
                    impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream
                ) = {
                    let generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream = |value: &dyn quote::ToTokens|{
                        quote::quote!{
                            impl error_occurence_lib::ToStdStringString for #value {
                                fn to_std_string_string(&self) -> std::string::String {
                                    format!("{self:?}")
                                }
                            }
                        }
                    };
                    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_without_id_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_field_to_read_without_id_upper_camel_case);
                    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_with_id_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_field_to_read_with_id_upper_camel_case);
                    (
                        impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_without_id_token_stream,
                        impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_with_id_token_stream
                    )
                };
                let (
                    impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream,
                    impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream
                ) = {
                    let generate_vec_content_token_stream = |enum_ident_token_stream: &dyn quote::ToTokens|{
                        let elements_token_stream = vec_syn_field.iter().map(|element|{
                            let field_ident = &element.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            quote::quote!{
                                #enum_ident_token_stream::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                            }
                        });
                        quote::quote!{#(#elements_token_stream),*}
                    };
                    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_without_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_field_to_read_without_id_upper_camel_case,
                        &{
                            let vec_content_token_stream = generate_vec_content_token_stream(&ident_field_to_read_without_id_upper_camel_case);
                            quote::quote!{vec![#vec_content_token_stream]}
                        },
                    );
                    // impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    //     &ident_field_to_read_without_id_upper_camel_case,
                    //     &generate_vec_content_token_stream(&ident_field_to_read_without_id_upper_camel_case)
                    // );
                    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_with_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_field_to_read_with_id_upper_camel_case,
                        &{
                            let other_variants_token_stream = generate_vec_content_token_stream(&ident_field_to_read_with_id_upper_camel_case);
                            quote::quote!{
                                vec![
                                    #ident_field_to_read_with_id_upper_camel_case::Id(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream),
                                    #other_variants_token_stream
                                ]
                            }
                        },
                    );
                    (
                        impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_without_id_token_stream,
                        impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_with_id_token_stream
                    )
                };
                let generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream = |
                    contains_id: std::primitive::bool,
                    add_serde_option_is_none_annotation: std::primitive::bool
                |{
                    let maybe_serde_skip_serializing_if_option_is_none_token_stream = if add_serde_option_is_none_annotation {
                        quote::quote!{#[serde(skip_serializing_if = "Option::is_none")]}
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    };
                    let maybe_id_token_stream = if contains_id {
                        quote::quote!{
                            #maybe_serde_skip_serializing_if_option_is_none_token_stream
                            #id_snake_case: std::option::Option<#postgresql_crud_path_token_stream Value<#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidOptionsToRead>>,//todo reuse
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    };
                    let fields_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                        let type_path_options_to_read_token_stream = naming::parameter::SelfOptionsToReadUpperCamelCase::from_type_last_segment(&element.ty);
                        quote::quote!{
                            #maybe_serde_skip_serializing_if_option_is_none_token_stream
                            #field_ident: std::option::Option<#postgresql_crud_path_token_stream Value<#type_path_options_to_read_token_stream>>
                        }
                    });
                    quote::quote!{
                        #maybe_id_token_stream
                        #(#fields_token_stream),*
                    }
                };
                let (
                    postgresql_json_type_ident_options_to_read_without_id_token_stream,
                    postgresql_json_type_ident_options_to_read_with_id_token_stream
                ) = {
                    let generate_struct_postgresql_json_type_tokens_options_to_read_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool|{
                        generate_postgresql_json_type_tokens_options_to_read_token_stream(
                            &struct_ident_token_stream,
                            false,
                            &{
                                let postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream = generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream(contains_id, true);
                                quote::quote!{{#postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream}}
                            },
                        )
                    };
                    let postgresql_json_type_ident_options_to_read_without_id_token_stream = generate_struct_postgresql_json_type_tokens_options_to_read_token_stream(&ident_options_to_read_without_id_upper_camel_case, false);
                    let postgresql_json_type_ident_options_to_read_with_id_token_stream = generate_struct_postgresql_json_type_tokens_options_to_read_token_stream(&ident_options_to_read_with_id_upper_camel_case, true);
                    (
                        postgresql_json_type_ident_options_to_read_without_id_token_stream,
                        postgresql_json_type_ident_options_to_read_with_id_token_stream
                    )
                };
                let (
                    postgresql_json_type_ident_options_to_read_with_or_without_id_try_from_error_named_token_stream,
                    impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                    impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                ) = {
                    let all_fields_are_none_upper_camel_case = naming::AllFieldsAreNoneUpperCamelCase;
                    let ident_options_to_read_with_or_without_id_try_from_error_named_upper_camel_case = naming::parameter::SelfOptionsToReadWithOrWithoutIdTryFromErrorNamedUpperCamelCase::from_tokens(&ident);
                    let postgresql_json_type_ident_options_to_read_with_or_without_id_try_from_error_named_token_stream = {
                        quote::quote!{
                            #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                            pub enum #ident_options_to_read_with_or_without_id_try_from_error_named_upper_camel_case {
                                #all_fields_are_none_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            }
                        }
                    };
                    let (
                        impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                        impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                    ) = {
                        let generate_impl_try_new_for_postgresql_json_type_ident_options_to_read_with_or_without_id_token_stream = |contains_id: std::primitive::bool|{
                            let postgresql_json_type_ident_options_to_read_with_or_without_id_token_stream: &dyn quote::ToTokens = if contains_id {
                                &ident_options_to_read_with_id_upper_camel_case
                            }
                            else {
                                &ident_options_to_read_without_id_upper_camel_case
                            };
                            let postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream = generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream(contains_id, false);
                            let (
                                postgresql_json_type_ident_options_to_read_with_or_without_id_fields_reference_token_stream,
                                postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream
                            ) = {
                                let generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream = |with_reference: std::primitive::bool|{
                                    let maybe_reference_symbol_token_stream = if with_reference {
                                        quote::quote!{&}
                                    }
                                    else {
                                        proc_macro2::TokenStream::new()
                                    };
                                    let maybe_id_token_stream = if contains_id {
                                        quote::quote!{#maybe_reference_symbol_token_stream #id_snake_case,}
                                    }
                                    else {
                                        proc_macro2::TokenStream::new()
                                    };
                                    let fields_token_stream = vec_syn_field.iter().map(|element| {
                                        let field_ident = element.ident.as_ref()
                                            .unwrap_or_else(|| {
                                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                            });
                                        quote::quote!{#maybe_reference_symbol_token_stream #field_ident}
                                    });
                                    quote::quote!{
                                        #maybe_id_token_stream
                                        #(#fields_token_stream),*
                                    }
                                };
                                let postgresql_json_type_ident_options_to_read_with_or_without_id_fields_reference_token_stream = generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream(true);
                                let postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream = generate_postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream(false);
                                (
                                    postgresql_json_type_ident_options_to_read_with_or_without_id_fields_reference_token_stream,
                                    postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream
                                )
                            };
                            let postgresql_json_type_ident_options_to_read_with_or_without_id_check_if_all_fields_are_none_token_stream = {
                                let nones_token_stream = {
                                    let range_end = {
                                        let vec_syn_field_len = vec_syn_field.len();
                                        if contains_id {
                                            vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                                        }
                                        else {
                                            vec_syn_field_len
                                        }
                                    };
                                    let mut acc = vec![];
                                    for _ in 0..range_end {
                                        acc.push(quote::quote!{None});
                                    }
                                    acc
                                };
                                quote::quote!{
                                    if let (#(#nones_token_stream),*) = (#postgresql_json_type_ident_options_to_read_with_or_without_id_fields_reference_token_stream) {
                                        return Err(#ident_options_to_read_with_or_without_id_try_from_error_named_upper_camel_case::#all_fields_are_none_upper_camel_case {
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            };
                            quote::quote!{
                                impl #postgresql_json_type_ident_options_to_read_with_or_without_id_token_stream {
                                    pub fn try_new(#postgresql_json_type_ident_options_to_read_with_or_without_id_fields_declaration_token_stream) -> Result<Self, #ident_options_to_read_with_or_without_id_try_from_error_named_upper_camel_case> {
                                        #postgresql_json_type_ident_options_to_read_with_or_without_id_check_if_all_fields_are_none_token_stream
                                        Ok(Self{#postgresql_json_type_ident_options_to_read_with_or_without_id_fields_token_stream})
                                    }
                                }
                            }
                        };
                        let impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream = generate_impl_try_new_for_postgresql_json_type_ident_options_to_read_with_or_without_id_token_stream(false);
                        let impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream = generate_impl_try_new_for_postgresql_json_type_ident_options_to_read_with_or_without_id_token_stream(true);
                        (
                            impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                            impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                        )
                    };
                    (
                        postgresql_json_type_ident_options_to_read_with_or_without_id_try_from_error_named_token_stream,
                        impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                        impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                    )
                };
                let (
                    impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                    impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                ) = {
                    let generate_impl_serde_deserialize_for_postgresql_json_type_options_to_read_token_stream = |
                        struct_ident_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
                        contains_id: std::primitive::bool,
                    |{
                        let range_end = {
                            let vec_syn_field_len = vec_syn_field.len();
                            if contains_id {
                                vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                            }
                            else {
                                vec_syn_field_len
                            }
                        };
                        let field_enum_variants_token_stream = {
                            let mut vec = vec![];
                            for element in 0..range_end {
                                let value = format!("__{}{element}", naming::FieldSnakeCase);
                                vec.push(
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                );
                            }
                            vec
                        };
                        let generate_field_index_token_stream = |index: std::primitive::usize| {
                            let value = format!("__field{index}");
                            value
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let visit_u64_value_enum_variants_token_stream = {
                            let mut acc = vec![];
                            for index in 0..range_end {
                                let index_u64_token_stream = {
                                    let value = format!("{index}u64");
                                    value
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                let field_index_token_stream = generate_field_index_token_stream(index);
                                acc.push(quote::quote! {
                                    #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
                                });
                            }
                            acc
                        };
                        let generate_field_ident_double_quotes_serde_private_ok_field_token_stream = |
                            field_name_double_quotes_token_stream: &dyn quote::ToTokens,
                            index: std::primitive::usize,
                        |{
                            let field_index_token_stream = generate_field_index_token_stream(index);
                            quote::quote!{#field_name_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
                        };
                        let generate_index = |index: std::primitive::usize|{
                            if contains_id {
                                index.checked_add(1).unwrap_or_else(|| panic!("vec_syn_field_len + 1 is None(int overflow)"))
                            }
                            else {
                                index
                            }
                        };
                        let visit_str_value_enum_variants_token_stream = {
                            let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                                let index = generate_index(index);
                                let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &element.ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                );
                                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                                    &field_name_double_quotes_token_stream,
                                    index,
                                )
                            });
                            let maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream = if contains_id {
                                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                                    &id_snake_case_double_quotes_token_stream,
                                    0,
                                );
                                quote::quote!{#value_token_stream,}
                            }
                            else {
                                proc_macro2::TokenStream::new()
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
                                    value
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                                    &b_field_name_double_quotes_token_stream,
                                    index,
                                )
                            });
                            let maybe_b_field_ident_double_quotes_token_stream = if contains_id {
                                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                                    &{
                                        let value = format!("b{id_snake_case_double_quotes_token_stream}");
                                        value.parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                    },
                                    0,
                                );
                                quote::quote!{#value_token_stream,}
                            }
                            else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #maybe_b_field_ident_double_quotes_token_stream
                                #(#visit_bytes_value_enum_variants_token_stream),*,
                            }
                        };
                        let struct_ident_options_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case}"));
                        let struct_ident_options_with_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case} with {range_end} elements"));
                        let visit_seq_fields_initialization_token_stream = {
                            let generate_serde_de_seq_access_next_element_token_stream = |
                                index: std::primitive::usize,
                                type_options_to_read_token_stream: &dyn quote::ToTokens,
                            |{
                                let field_index_token_stream = generate_field_index_token_stream(index);
                                quote::quote! {
                                    let #field_index_token_stream = match serde::de::SeqAccess::next_element::<
                                        std::option::Option<#postgresql_crud_path_token_stream Value<#type_options_to_read_token_stream>>,
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
                                let index = generate_index(index);
                                let type_options_to_read_token_stream = naming::parameter::SelfOptionsToReadUpperCamelCase::from_type_last_segment(&element.ty);
                                generate_serde_de_seq_access_next_element_token_stream(
                                    index,
                                    &type_options_to_read_token_stream,
                                )
                            });
                            let maybe_id_serde_de_seq_access_next_element_token_stream = if contains_id {
                                generate_serde_de_seq_access_next_element_token_stream(
                                    0,
                                    &quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidOptionsToRead},//todo reuse
                                )
                            }
                            else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #maybe_id_serde_de_seq_access_next_element_token_stream
                                #(#visit_seq_fields_initialization_token_stream)*
                            }
                        };
                        let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                            if contains_id {
                                &ident_options_to_read_with_id_upper_camel_case
                            }
                            else {
                                &ident_options_to_read_without_id_upper_camel_case
                            },
                            &{
                                let fields_token_stream = {
                                    let mut acc = vec![];
                                    for element in 0..range_end {
                                        let field_index_token_stream = generate_field_index_token_stream(element);
                                        acc.push(quote::quote!{#field_index_token_stream});
                                    }
                                    acc
                                };
                                quote::quote!{#(#fields_token_stream),*}
                            },
                        );
                        let visit_map_fields_initialization_token_stream = {
                            let generate_mut_field_index_serde_private_option_token_stream = |
                                index: std::primitive::usize,
                                type_token_stream: &dyn quote::ToTokens,
                            |{
                                let field_index_token_stream = generate_field_index_token_stream(index);
                                quote::quote! {
                                    let mut #field_index_token_stream: serde::__private::Option<
                                        std::option::Option<#postgresql_crud_path_token_stream Value<#type_token_stream>>,
                                    > = serde::__private::None;
                                }
                            };
                            let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                                let index = generate_index(index);
                                let type_options_to_read_token_stream = naming::parameter::SelfOptionsToReadUpperCamelCase::from_type_last_segment(&element.ty);
                                generate_mut_field_index_serde_private_option_token_stream(
                                    index,
                                    &type_options_to_read_token_stream,
                                )
                            });
                            let maybe_id_mut_field_index_serde_private_option_token_stream = if contains_id {
                                generate_mut_field_index_serde_private_option_token_stream(
                                    0,
                                    &quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidOptionsToRead},//todo
                                )
                            }
                            else {
                                quote::quote!{}
                            };
                            quote::quote! {
                                #maybe_id_mut_field_index_serde_private_option_token_stream
                                #(#visit_map_fields_initialization_token_stream)*
                            }
                        };
                        let visit_map_match_variants_token_stream = {
                            let generate_field_initialization_token_stream = |
                                index: std::primitive::usize,
                                field_ident_double_quotes_token_stream: &dyn quote::ToTokens,
                                type_token_stream: &dyn quote::ToTokens,
                            |{
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
                                let index = generate_index(index);
                                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                let type_options_to_read_token_stream = naming::parameter::SelfOptionsToReadUpperCamelCase::from_type_last_segment(&element.ty);
                                generate_field_initialization_token_stream(
                                    index,
                                    &field_ident_double_quotes_token_stream,
                                    &type_options_to_read_token_stream,
                                )
                            });
                            let id_field_initialization_token_stream = if contains_id {
                                generate_field_initialization_token_stream(
                                    0,
                                    &id_snake_case_double_quotes_token_stream,
                                    &quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type::UuidUuidOptionsToRead},//todo reuse
                                )
                            }
                            else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #id_field_initialization_token_stream
                                #(#visit_map_match_variants_token_stream)*
                            }
                        };
                        let visit_map_missing_fields_check_token_stream = {
                            let generate_missing_field_token_stream = |
                                index: std::primitive::usize,
                                field_ident_double_quotes_token_stream: &dyn quote::ToTokens
                            |{
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
                                generate_missing_field_token_stream(
                                    index,
                                    &field_ident_double_quotes_token_stream
                                )
                            });
                            let maybe_id_missing_field_token_stream = if contains_id {
                                generate_missing_field_token_stream(
                                    0,
                                    &id_snake_case_double_quotes_token_stream
                                )
                            }
                            else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #maybe_id_missing_field_token_stream
                                #(#visit_map_missing_fields_check_token_stream)*
                            }
                        };
                        let fields_array_elements_token_stream = {
                            let fields_array_elements_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(element));
                            let maybe_id_double_quotes_comma_token_stream = if contains_id {
                                quote::quote! {#id_snake_case_double_quotes_token_stream,}
                            }
                            else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #maybe_id_double_quotes_comma_token_stream
                                #(#fields_array_elements_token_stream),*
                            }
                        };
                        let postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_origin_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&struct_ident_token_stream);
                        quote::quote!{
                            impl<'de> serde::Deserialize<'de> for #struct_ident_token_stream {
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
                                            #struct_ident_token_stream,
                                        >,
                                        lifetime: serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = #struct_ident_token_stream;
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
                                        #postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_origin_double_quotes_token_stream,
                                        FIELDS,
                                        __Visitor {
                                            marker: serde::__private::PhantomData::<
                                                #struct_ident_token_stream,
                                            >,
                                            lifetime: serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    };
                    let impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_without_id_token_stream = generate_impl_serde_deserialize_for_postgresql_json_type_options_to_read_token_stream(
                        &ident_options_to_read_without_id_upper_camel_case,
                        false,
                    );
                    let impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_with_id_token_stream = generate_impl_serde_deserialize_for_postgresql_json_type_options_to_read_token_stream(
                        &ident_options_to_read_with_id_upper_camel_case,
                        true,
                    );
                    (
                        impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                        impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                    )
                };
                let (
                    impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                    impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                ) = {
                    let (
                        fields_some_value_self_options_to_read_initialization_content_token_stream,
                        fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
                    ) = {
                        let generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = |
                            field_ident: &syn::Ident,
                        |{
                            quote::quote!{
                                #field_ident: Some(#postgresql_crud_path_token_stream Value { 
                                    value: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                })
                            }
                        };
                        let fields_some_value_self_options_to_read_initialization_content_token_stream = {
                            let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                                element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                            ));
                            quote::quote!{Self{#(#fields_token_stream),*}}
                        };
                        let fields_with_id_some_value_self_options_to_read_initialization_content_token_stream = {
                            let fields_token_stream = {
                                let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                                    element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                ));
                                let id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                                    &syn::Ident::new(&naming::IdSnakeCase.to_string(), ident.span()),
                                );
                                quote::quote!{
                                    #id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                    #(#fields_token_stream),*
                                }
                            };
                            quote::quote!{Self{ #fields_token_stream }}
                        };
                        (
                            fields_some_value_self_options_to_read_initialization_content_token_stream,
                            fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
                        )
                    };
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_without_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_options_to_read_without_id_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &fields_some_value_self_options_to_read_initialization_content_token_stream,
                    );
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_with_id_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &ident_options_to_read_with_id_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
                    );
                    (
                        impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_without_id_token_stream,
                        impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                    )
                };
                quote::quote!{
                    #ident_field_to_read_token_stream
                    #ident_with_id_field_to_read_token_stream

                    #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream
                
                    #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream

                    #postgresql_json_type_ident_options_to_read_without_id_token_stream
                    #postgresql_json_type_ident_options_to_read_with_id_token_stream

                    #postgresql_json_type_ident_options_to_read_with_or_without_id_try_from_error_named_token_stream
                    #impl_try_new_for_postgresql_json_type_ident_options_to_read_without_id_token_stream
                    #impl_try_new_for_postgresql_json_type_ident_options_to_read_with_id_token_stream

                    #impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_without_id_token_stream
                    #impl_serde_deserialize_for_postgresql_json_type_ident_options_to_read_with_id_token_stream

                    #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_without_id_token_stream
                    #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_options_to_read_with_id_token_stream
                }
            };
            let update_token_stream = {
                let ident_field_to_update_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let element_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_ident);
                        let element_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
                        quote::quote! {
                            #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
                            #element_ident_upper_camel_case_token_stream
                        }
                    });
                    quote::quote!{
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub enum #ident_field_to_update_upper_camel_case {
                            #(#variants_token_stream),*
                        }
                    }
                };
                let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                        });
                        let element_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_ident);
                        let element_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
                        quote::quote! {
                            Self::#element_ident_upper_camel_case_token_stream => #element_ident_double_quotes_token_stream.to_owned()
                        }
                    });
                    quote::quote!{
                        impl error_occurence_lib::ToStdStringString for #ident_field_to_update_upper_camel_case {
                            fn to_std_string_string(&self) -> std::string::String {
                                match &self {
                                    #(#variants_token_stream),*
                                }
                            }
                        }
                    }
                };
                let postgresql_json_type_ident_option_to_update_origin_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                        //todo maybe rename type_path to tokens for standart naming convention
                        let type_path_update_token_stream = naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&element.ty);
                        let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        quote::quote!{
                            #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                            #variant_ident_upper_camel_case_token_stream(#postgresql_crud_path_token_stream Value<#type_path_update_token_stream>)
                        }
                    });
                    quote::quote!{
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub enum #ident_update_origin_upper_camel_case {
                            #(#variants_token_stream),*
                        }
                    }
                };
                let ident_json_array_change_try_generate_postgresql_json_type_error_named_token_stream = generate_tokens_try_generate_postgresql_json_type_error_named_token_stream(
                    &ident_json_array_change_try_generate_error_named_upper_camel_case,
                    &{
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                            let element_type_option_to_update_try_generate_error_named_upper_camel_case = naming::parameter::SelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_type_last_segment(&element.ty);
                            quote::quote!{
                                #variant_ident_upper_camel_case_token_stream {
                                    #[eo_error_occurence]
                                    error: #element_type_option_to_update_try_generate_error_named_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                }
                            }
                        });
                        quote::quote!{
                            #checked_add_variant_declaration_token_stream,
                            Create {
                                #[eo_error_occurence]
                                error: #postgresql_crud_create_query_part_error_named_token_stream,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                            #(#variants_token_stream),*
                        }
                    }
                );
                let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_option_to_update_origin_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_update_origin_upper_camel_case,
                    &{
                        let elements_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident = element.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                });
                            let variant_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                            quote::quote!{
                                #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(#postgresql_crud_path_token_stream Value {
                                    value: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                })
                            }
                        });
                        quote::quote!{vec![#(#elements_token_stream),*]}
                    },
                );
                let ident_options_to_update_token_stream = {
                    quote::quote!{
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
                        pub struct #ident_options_to_update_upper_camel_case {
                            pub #id_snake_case: #postgresql_crud_uuid_update_token_stream,
                            pub fields: #ident_update_upper_camel_case
                        }
                    }
                };
                let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream =        postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_options_to_update_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote!{Self {
                        #id_snake_case: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                        fields: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    }}
                );
                quote::quote!{
                    #ident_field_to_update_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream

                    #postgresql_json_type_ident_option_to_update_origin_token_stream
                    #ident_json_array_change_try_generate_postgresql_json_type_error_named_token_stream
                    #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_option_to_update_origin_token_stream

                    #ident_options_to_update_token_stream
                    #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream
                }
            };
            (
                read_token_stream,
                update_token_stream
            )
        };
        quote::quote!{
            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
    let generate_tokens_to_update_methods_token_stream = |
        struct_token_stream: &dyn quote::ToTokens,
        //todo rename
        tokens_try_generate_bind_increments_error_named_upper_camel_case_token_stream: &dyn quote::ToTokens,
        update_query_part_token_stream: &dyn quote::ToTokens,
        update_query_bind_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl #struct_token_stream {
                fn #update_query_part_snake_case(
                    &self,
                    #jsonb_set_accumulator_snake_case: &std::primitive::str,
                    #jsonb_set_target_snake_case: &std::primitive::str,
                    #jsonb_set_path_snake_case: &std::primitive::str,
                    #increment_snake_case: &mut std::primitive::u64,
                ) -> Result<std::string::String, #tokens_try_generate_bind_increments_error_named_upper_camel_case_token_stream> {
                    #update_query_part_token_stream
                }
                fn #update_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #update_query_bind_token_stream
                }
            }
        }
    };
    let (
        generate_tuple_struct_tokens_double_quotes_token_stream,
        generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream
    ) = {
        const TUPLE_STRUCT_SPACE_STRINGIFIED: &std::primitive::str = "tuple struct ";
        let generate_tuple_struct_tokens_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            generate_quotes::double_quotes_token_stream(&format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value}"))
        };
        let generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            generate_quotes::double_quotes_token_stream(&format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value} with 1 element"))
        };
        (
            generate_tuple_struct_tokens_double_quotes_token_stream,
            generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream
        )
    };
    let generate_tokens_reader_alias_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, struct_options_to_read_token_stream: &dyn quote::ToTokens|{
        macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(struct_ident_token_stream, struct_options_to_read_token_stream)
    };
    let generate_tokens_option_to_update_token_stream = |
        struct_ident_token_stream: &dyn quote::ToTokens,
        self_type_content_token_stream: &dyn quote::ToTokens,
        impl_deserialize: std::primitive::bool,
        is_pub: std::primitive::bool,
    |{
        let maybe_impl_deserialize_token_stream = if impl_deserialize {
            quote::quote!{serde::Deserialize,}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        let maybe_pub_token_stream = if is_pub {
            quote::quote!{pub}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, #maybe_impl_deserialize_token_stream utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #struct_ident_token_stream(#maybe_pub_token_stream #self_type_content_token_stream);
        }
    };
    let field0_token_stream = quote::quote!{__field0};
    let ident_option_to_update_try_generate_error_named_upper_camel_case = naming::parameter::SelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_update_try_new_error_named_upper_camel_case = naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let (
        generate_jsonb_set_target_snake_case,
        generate_jsonb_set_target_token_stream
    ) = {
        let generate_jsonb_set_target_snake_case = naming::GenerateJsonbSetTargetSnakeCase;
        let generate_jsonb_set_target_token_stream = {
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{jsonb_set_target_snake_case}}}->'{{value}}'"));
            quote::quote!{
                let #generate_jsonb_set_target_snake_case = |value: &std::primitive::str|{
                    format!(#format_handle_token_stream)
                };
            }
        };
        (
            generate_jsonb_set_target_snake_case,
            generate_jsonb_set_target_token_stream
        )
    };
    let field_reader_query_part_snake_case = naming::FieldReaderQueryPartSnakeCase;
    let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
    // let column_name_and_maybe_field_getter_handle_snake_case = naming::ColumnNameAndMaybeFieldGetterHandleSnakeCase;
    let column_name_and_maybe_field_getter_for_error_message_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;
    let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
    //its for GeneratePostgresqlCrud
    let ident_token_stream = {
        let impl_std_fmt_display_for_ident_token_stream = quote::quote!{
            impl std::fmt::Display for #ident {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", &self)
                }
            }
        };
        let create_token_stream = {
            let ident_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_create_upper_camel_case, &ident_to_create_without_generated_id_upper_camel_case);
            quote::quote!{
                #ident_create_alias_token_stream
            }
        };
        let read_token_stream = {
            let ident_field_to_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&naming::parameter::SelfFieldToReadUpperCamelCase::from_tokens(&ident), &ident_field_to_read_without_id_upper_camel_case);

            let ident_options_to_read_upper_camel_case = naming::parameter::SelfOptionsToReadUpperCamelCase::from_tokens(&ident);
            let ident_options_to_read_alias_token_stream = generate_postgresql_json_type_tokens_options_to_read_alias_token_stream(&ident_options_to_read_upper_camel_case, false);

            let ident_field_reader_upper_camel_case = naming::parameter::SelfFieldReaderUpperCamelCase::from_tokens(&ident);
            let postgresql_json_type_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(&PostgresqlJsonTypeFieldReader::Ident);
            let impl_serde_deserialize_for_postgresql_json_type_ident_field_reader_token_stream = {
                let tuple_struct_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&ident_field_reader_upper_camel_case);
                let tuple_struct_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&ident_field_reader_upper_camel_case);
                let ident_field_reader_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_field_reader_upper_camel_case);
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &ident_field_reader_upper_camel_case,
                    &field0_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #ident_field_reader_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#ident_field_reader_upper_camel_case>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #ident_field_reader_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_ident_field_reader_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_newtype_struct<__E>(
                                    self,
                                    __e: __E,
                                ) -> serde::__private::Result<Self::Value, __E::Error>
                                where
                                    __E: serde::Deserializer<'de>,
                                {
                                    let __field0: std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case> = <std::vec::Vec<
                                        #ident_field_to_read_without_id_upper_camel_case,
                                    > as serde::Deserialize>::deserialize(__e)?;
                                    #match_try_new_in_deserialize_token_stream
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                        std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<#ident_field_reader_upper_camel_case>,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_field_reader_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote!{Self(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            );

            //for compatibility with GeneratePostgresqlCrud logic
            let impl_postgresql_crud_generate_postgresql_json_type_to_read_ident_generate_postgresql_json_type_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
                let impl_postgresql_crud_generate_postgresql_json_type_to_read_ident_generate_postgresql_json_type_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote!{
                            #ident_field_to_read_without_id_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #field_reader_query_part_snake_case(
                                value,
                                #field_ident_double_quotes_token_stream,
                                #column_name_and_maybe_field_getter_snake_case,
                                #column_name_and_maybe_field_getter_for_error_message_snake_case,
                                false,
                            )
                        }
                    });
                    quote::quote!{
                        impl #postgresql_crud_path_token_stream GeneratePostgresqlJsonTypeToRead for #ident_field_to_read_without_id_upper_camel_case {
                            fn generate_postgresql_json_type_to_read_from_vec(
                                value: &std::vec::Vec<Self>,
                                #column_name_and_maybe_field_getter_snake_case: &std::primitive::str,
                                #column_name_and_maybe_field_getter_for_error_message_snake_case: &std::primitive::str,
                            ) -> std::string::String {
                                let mut acc = std::string::String::default();
                                for element in value {
                                    acc.push_str(&format!(
                                        "{}||",
                                        match element {
                                            #(#variants_token_stream),*
                                        }
                                    ));
                                }
                                let _ = acc.pop();
                                let _ = acc.pop();
                                format!("{acc}")
                            }
                        }
                    }
                };
                quote::quote!{
                    #impl_postgresql_crud_generate_postgresql_json_type_to_read_ident_generate_postgresql_json_type_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream
                }
            };

            let ident_reader_token_stream = generate_tokens_reader_alias_token_stream(
                &naming::parameter::SelfReaderUpperCamelCase::from_tokens(&ident),
                &ident_options_to_read_upper_camel_case
            );
            quote::quote!{
                #ident_field_to_read_alias_token_stream

                #ident_options_to_read_alias_token_stream

                #postgresql_json_type_ident_field_reader_token_stream
                #impl_serde_deserialize_for_postgresql_json_type_ident_field_reader_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream

                #impl_postgresql_crud_generate_postgresql_json_type_to_read_ident_generate_postgresql_json_type_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream

                #ident_reader_token_stream
            }
        };
        let update_token_stream = {
            let postgresql_json_type_ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &ident_update_upper_camel_case,
                &quote::quote!{std::vec::Vec<#ident_update_origin_upper_camel_case>},
                false,
                false,
            );
            // println!("{postgresql_json_type_ident_option_to_update_token_stream}");
            let impl_try_new_for_postgresql_json_type_ident_option_to_update_token_stream = {
                let fields_are_empty_upper_camel_case = naming::FieldsAreEmptyUpperCamelCase;
                let try_new_error_named_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            });
                        let not_unique_field_self_upper_camel_case_token_stream = naming::parameter::NotUniqueFieldSelfUpperCamelCase::from_tokens(&field_ident);
                        quote::quote!{
                            #not_unique_field_self_upper_camel_case_token_stream {
                                #[eo_to_std_string_string_serialize_deserialize]
                                error: std::string::String,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            }
                        }
                    });
                    quote::quote!{
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_update_try_new_error_named_upper_camel_case {
                            #fields_are_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                            #(#variants_token_stream),*
                        }
                    }
                };
                let impl_pub_fn_try_new_token_stream = {
                    let custom_checks_token_stream = {
                        let check_fields_are_empty_token_stream = {
                            quote::quote!{
                                if value.is_empty() {
                                    return Err(#ident_update_try_new_error_named_upper_camel_case::#fields_are_empty_upper_camel_case {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        };
                        let check_unique_fields_token_stream = {
                            let generate_not_unique_field_snake_case = naming::GenerateNotUniqueFieldSnakeCase;
                            let variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    });
                                let field_ident_stringified = field_ident.to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                let not_unique_field_self_upper_camel_case_token_stream = naming::parameter::NotUniqueFieldSelfUpperCamelCase::from_tokens(&field_ident);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(_) => {
                                        let value = #ident_field_to_update_upper_camel_case::#variant_ident_upper_camel_case_token_stream;
                                        if acc.contains(&value) {
                                            return Err(#ident_update_try_new_error_named_upper_camel_case::#not_unique_field_self_upper_camel_case_token_stream {
                                                error: #generate_not_unique_field_snake_case(#field_ident_double_quotes_token_stream),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                        else {
                                            acc.push(value);
                                        }
                                    }
                                }
                            });
                            quote::quote!{
                                {
                                    let mut acc = vec![];
                                    let #generate_not_unique_field_snake_case = |value: &std::primitive::str|{
                                        format!("not unique {value} field")
                                    };
                                    for element in &value {
                                        match element {
                                            #(#variants_token_stream),*
                                        }
                                    }
                                }
                            }
                        };
                        quote::quote!{
                            #check_fields_are_empty_token_stream
                            #check_unique_fields_token_stream
                        }
                    };
                    quote::quote!{
                        impl #ident_update_upper_camel_case {
                            pub fn try_new(value: std::vec::Vec<#ident_update_origin_upper_camel_case>) -> Result<Self, #ident_update_try_new_error_named_upper_camel_case> {
                                #custom_checks_token_stream
                                Ok(Self(value))
                            }
                        }
                    }
                };
                quote::quote!{
                    #try_new_error_named_token_stream
                    #impl_pub_fn_try_new_token_stream
                }
            };
            let impl_serde_deserialize_for_postgresql_json_type_ident_option_to_update_token_stream = {
                let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
                let tuple_struct_postgresql_json_type_ident_option_to_update_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&ident_update_upper_camel_case);
                let tuple_struct_postgresql_json_type_ident_option_to_update_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&ident_update_upper_camel_case);
                let ident_update_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_update_upper_camel_case);
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &ident_update_upper_camel_case,
                    &field0_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #ident_update_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#ident_update_upper_camel_case>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #ident_update_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_postgresql_json_type_ident_option_to_update_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_newtype_struct<__E>(
                                    self,
                                    __e: __E,
                                ) -> serde::__private::Result<Self::Value, __E::Error>
                                where
                                    __E: serde::Deserializer<'de>,
                                {
                                    let __field0: std::vec::Vec<#ident_update_origin_upper_camel_case> = <std::vec::Vec<
                                        #ident_update_origin_upper_camel_case,
                                    > as serde::Deserialize>::deserialize(__e)?;
                                    #match_try_new_in_deserialize_token_stream
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                        std::vec::Vec<#ident_update_origin_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_postgresql_json_type_ident_option_to_update_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #ident_update_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #ident_update_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_option_to_update_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_update_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote!{Self(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            );
            
            let postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_token_stream = generate_tokens_try_generate_postgresql_json_type_error_named_token_stream(
                &ident_option_to_update_try_generate_error_named_upper_camel_case,
                &{
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        let element_type_option_to_update_try_generate_error_named_upper_camel_case_token_stream = naming::parameter::SelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_type_last_segment(&element.ty);
                        quote::quote!{
                            #variant_ident_upper_camel_case_token_stream {
                                #[eo_error_occurence]
                                error: #element_type_option_to_update_try_generate_error_named_upper_camel_case_token_stream,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            }
                        }
                    });
                    quote::quote!{
                        #checked_add_variant_declaration_token_stream,
                        #(#variants_token_stream),*
                    }
                }
            );
            let impl_postgresql_json_type_ident_option_to_update_methods_token_stream = generate_tokens_to_update_methods_token_stream(
                &ident_update_upper_camel_case,
                &ident_option_to_update_try_generate_error_named_upper_camel_case,
                &{
                    let generate_jsonb_set_path_snake_case = naming::GenerateJsonbSetPathSnakeCase;
                    let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        //todo maybe reuse?
                        let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                        let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote!{
                            #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
                                &value.value,
                                &local_acc,
                                &#generate_jsonb_set_target_snake_case(#field_ident_double_quotes_token_stream),
                                &#generate_jsonb_set_path_snake_case(#field_ident_double_quotes_token_stream),
                                #increment_snake_case,
                            ) {
                                Ok(value) => {
                                    local_acc = value;
                                }
                                Err(error) => {
                                    return Err(#ident_option_to_update_try_generate_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!()
                                    });
                                }
                            }
                        }
                    });
                    let local_acc_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end)")
                    );
                    let is_empty_false_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{jsonb_set_path_snake_case}}},"));
                    quote::quote!{
                        #generate_jsonb_set_target_token_stream
                        let #generate_jsonb_set_path_snake_case = |value: &std::primitive::str|{
                            let previous = match #jsonb_set_path_snake_case.is_empty() {
                                true => std::string::String::default(),
                                false => format!(#is_empty_false_format_handle_token_stream),
                            };
                            format!("{previous}{value}")
                        };
                        let mut local_acc = format!(#local_acc_format_handle_token_stream);
                        for element in &self.0 {
                            match &element {
                                #(#try_generate_bind_increments_variants_token_stream),*
                            }
                        }
                        Ok(local_acc)
                    }
                },
                &{
                    let bind_value_to_postgresql_query_part_to_update_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{}", naming::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                        let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                        quote::quote!{
                            #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                            }
                        }
                    });
                    quote::quote!{
                        for element in self.0 {
                            match element {
                                #(#bind_value_to_postgresql_query_part_to_update_variants_token_stream),*
                            }
                        }
                        #query_snake_case
                    }
                },
            );
            //integration with GeneratePostgresqlCrud
            let impl_postgresql_crud_bind_query_second_for_postgresql_json_type_ident_option_to_update_token_stream = {
                quote::quote!{
                    impl postgresql_crud::BindQuery for #ident_update_upper_camel_case {
                        fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
                            todo!()
                        }
                        fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                            todo!()
                        }
                    }
                }
            };
            quote::quote!{
                #postgresql_json_type_ident_option_to_update_token_stream
                #impl_try_new_for_postgresql_json_type_ident_option_to_update_token_stream
                #impl_serde_deserialize_for_postgresql_json_type_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_option_to_update_token_stream
                #postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_token_stream
                #impl_postgresql_json_type_ident_option_to_update_methods_token_stream

                #impl_postgresql_crud_bind_query_second_for_postgresql_json_type_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #impl_std_fmt_display_for_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    let pub_field_idents_field_types_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{}", naming::FIELD_IDENT_IS_NONE);
            });
            let element_type = &element.ty;
            quote::quote!{pub #element_ident: #element_type}
        });
        quote::quote!{#(#fields_token_stream),*}
    };
    let object_with_id_ident_upper_camel_case = naming::parameter::ObjectWithIdSelfUpperCamelCase::from_tokens(&ident);
    // let object_with_id_ident_token_stream = {
    //     let object_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
    //         true,
    //         &object_with_id_ident_upper_camel_case,
    //         &{
    //             quote::quote!{{
    //                 pub #id_snake_case: #postgresql_crud_uuid_update_token_stream,
    //                 #pub_field_idents_field_types_token_stream
    //             }}
    //         }
    //     );
    //     let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_object_with_id_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
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
    //                     #field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
    //                 }
    //             });
    //             quote::quote!{Self {
    //                 #id_snake_case: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
    //                 #(#fields_token_stream),*
    //             }}
    //         },
    //     );
    //     quote::quote!{
    //         #object_with_id_ident_token_stream
    //         #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_object_with_id_ident_token_stream
    //     }
    // };
    let postgresql_type_traits_token_stream = {
        enum PostgresqlJsonType {
            Object,
            StdOptionOptionObject,
            StdVecVecObjectWithId,
            StdOptionOptionStdVecVecObjectWithId,
        }
        let generate_postgresql_type_traits_token_stream = |postgresql_json_type: &PostgresqlJsonType, need_to_generate_postgresql_crud_logic: bool|{
            let tokens_create_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                PostgresqlJsonType::Object => &naming::parameter::ObjectSelfCreateUpperCamelCase::from_tokens(&ident),
                PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfCreateUpperCamelCase::from_tokens(&ident),
                PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfCreateUpperCamelCase::from_tokens(&ident),
                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfCreateUpperCamelCase::from_tokens(&ident),
            };
            let generate_tokens_update_upper_camel_case_token_stream = |postgresql_json_type: &PostgresqlJsonType| {
                let value: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfUpdateUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfUpdateUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfUpdateUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfUpdateUpperCamelCase::from_tokens(&ident),
                };
                quote::quote!{#value}
            };
            let impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream = {
                let tokens_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfUpperCamelCase::from_tokens(&ident),
                };
                let tokens_token_stream = {
                    let tokens_token_stream = generate_supported_generics_template_struct_token_stream(
                        true,
                        &tokens_upper_camel_case,
                        &match &postgresql_json_type {
                            PostgresqlJsonType::Object => quote::quote!{{#pub_field_idents_field_types_token_stream}},
                            PostgresqlJsonType::StdOptionOptionObject => {
                                let object_ident_upper_camel_case = naming::parameter::ObjectSelfUpperCamelCase::from_tokens(&ident);
                                quote::quote!{(pub std::option::Option<#object_ident_upper_camel_case>);}
                            },
                            PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(std::vec::Vec<#object_with_id_ident_upper_camel_case>);},
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(std::option::Option<std::vec::Vec<#object_with_id_ident_upper_camel_case>>);},
                        }
                    );
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream =  postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &tokens_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let value = match &postgresql_json_type {
                                PostgresqlJsonType::Object => quote::quote!{#impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_fields_token_stream},
                                PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))},
                                PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))},
                            };
                            quote::quote!{Self #value}
                        }
                    );
                    quote::quote!{
                        #tokens_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream
                    }
                };
                let create_token_stream = {
                    let postgresql_json_type_tokens_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
                        true,
                        &tokens_create_upper_camel_case,
                        &match &postgresql_json_type {
                            PostgresqlJsonType::Object => quote::quote!{(pub #ident_to_create_without_generated_id_upper_camel_case);},
                            PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(pub std::option::Option<#ident_to_create_without_generated_id_upper_camel_case>);},
                            PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(pub std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>);},
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(pub std::option::Option<std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>>);},
                        }
                    );
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream =  postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &tokens_create_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let value = match &postgresql_json_type {
                                PostgresqlJsonType::Object => quote::quote!{(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
                                PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))},
                                PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))},
                            };
                            quote::quote!{Self #value}
                        }
                    );
                    quote::quote!{
                        #postgresql_json_type_tokens_to_create_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream
                    }
                };
                let tokens_options_to_read_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                };
                let tokens_field_reader_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                };
                let tokens_where_element_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfWhereElementUpperCamelCase::from_tokens(&ident),
                };
                let tokens_where_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfWhereUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfWhereUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfWhereUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfWhereUpperCamelCase::from_tokens(&ident),
                };
                let read_token_stream = {
                    let postgresql_json_type_tokens_options_to_read_token_stream = generate_postgresql_json_type_tokens_options_to_read_token_stream(
                        &tokens_options_to_read_upper_camel_case,
                        //todo refactor - instead of bool and impl serde deserialize token stream maybe use enum 
                        match &postgresql_json_type {
                            PostgresqlJsonType::Object |
                            PostgresqlJsonType::StdOptionOptionObject => true,
                            PostgresqlJsonType::StdVecVecObjectWithId |
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => false,
                        },
                        &match &postgresql_json_type {
                            PostgresqlJsonType::Object => quote::quote!{(pub #ident_options_to_read_without_id_upper_camel_case);},
                            PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(pub std::option::Option<#ident_options_to_read_without_id_upper_camel_case>);},
                            PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>);},
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(pub std::option::Option<std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>>);},
                        }
                    );
                    // println!("{postgresql_json_type_tokens_options_to_read_token_stream}");
                    //todo maybe all impl must be try_new ?
                    let maybe_impl_try_new_for_postgresql_json_type_tokens_options_to_read_token_stream = {
                        let not_unique_id_upper_camel_case = naming::NotUniqueIdUpperCamelCase;
                        let generate_impl_try_new_for_postgresql_json_type_tokens_options_to_read_token_stream = |
                            try_new_type_token_stream: &dyn quote::ToTokens,
                            content_token_stream: &dyn quote::ToTokens,
                        |{
                            let postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                                PostgresqlJsonType::Object |
                                PostgresqlJsonType::StdOptionOptionObject => todo!(),
                                PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                            };
                            let pub_enum_postgresql_json_type_tokens_options_to_read_try_new_error_named_token_stream = {
                                quote::quote!{
                                    #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                                    pub enum #postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case {
                                        #not_unique_id_upper_camel_case {
                                            #[eo_to_std_string_string_serialize_deserialize]
                                            error: std::string::String,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                        }
                                    }
                                }
                            };
                            let impl_postgresql_json_type_tokens_options_to_read_try_new_error_named_pub_fn_try_new_token_stream = {
                                quote::quote!{
                                    impl #tokens_options_to_read_upper_camel_case {
                                        pub fn try_new(value: #try_new_type_token_stream) -> Result<Self, #postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case> {
                                            #content_token_stream
                                        }
                                    }
                                }
                            };
                            quote::quote!{
                                #pub_enum_postgresql_json_type_tokens_options_to_read_try_new_error_named_token_stream
                                #impl_postgresql_json_type_tokens_options_to_read_try_new_error_named_pub_fn_try_new_token_stream
                            }
                        };
                        match &postgresql_json_type {
                            PostgresqlJsonType::Object |
                            PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                            PostgresqlJsonType::StdVecVecObjectWithId => generate_impl_try_new_for_postgresql_json_type_tokens_options_to_read_token_stream(
                                &quote::quote!{std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>},
                                &{
                                    let check_not_unique_id_token_stream = {
                                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("not unique {id_snake_case} {{}}"));
                                        //todo its duplicate - remove it
                                        let postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                                            PostgresqlJsonType::Object |
                                            PostgresqlJsonType::StdOptionOptionObject => todo!(),
                                            PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                                        };
                                        quote::quote!{
                                            {
                                                let mut acc = vec![];
                                                for element in &value {
                                                    if let Some(value) = &element.#id_snake_case {
                                                        if acc.contains(&&value.value) {
                                                            return Err(#postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case::#not_unique_id_upper_camel_case {
                                                                error: format!(#format_handle_token_stream, value.value.0),
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            });
                                                        }
                                                        else {
                                                            acc.push(&value.value);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    quote::quote!{
                                        #check_not_unique_id_token_stream
                                        Ok(Self(value))
                                    }
                                },
                            ),
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => generate_impl_try_new_for_postgresql_json_type_tokens_options_to_read_token_stream(
                                &quote::quote!{std::option::Option<std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>>},
                                &{
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("not unique {id_snake_case} {{}}"));
                                    //todo its duplicate -remove it
                                    let postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                                        PostgresqlJsonType::Object |
                                        PostgresqlJsonType::StdOptionOptionObject => todo!(),
                                        PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                                    };
                                    quote::quote!{
                                        match value {
                                            Some(value) => {
                                                let mut acc = vec![];
                                                for element in &value {
                                                    if let Some(value) = &element.#id_snake_case {
                                                        if acc.contains(&&value.value) {
                                                            return Err(#postgresql_json_type_tokens_options_to_read_try_new_error_named_upper_camel_case::#not_unique_id_upper_camel_case {
                                                                error: format!(#format_handle_token_stream, value.value.0),
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            });
                                                        }
                                                        else {
                                                            acc.push(&value.value);
                                                        }
                                                    }
                                                }
                                                Ok(Self(Some(value)))
                                            },
                                            None => Ok(Self(None))
                                        }
                                    }
                                },
                            ),
                        }
                    };
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_options_to_read_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &tokens_options_to_read_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let value = match &postgresql_json_type {
                                PostgresqlJsonType::Object => quote::quote!{(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
                                PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))},
                                PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))},
                            };
                            quote::quote!{Self #value}
                        }
                    );
                    let tokens_field_reader_token_stream = generate_tokens_field_reader_token_stream(&match &postgresql_json_type {
                        PostgresqlJsonType::Object => PostgresqlJsonTypeFieldReader::ObjectIdent,
                        PostgresqlJsonType::StdOptionOptionObject => PostgresqlJsonTypeFieldReader::StdOptionOptionObjectIdent,
                        PostgresqlJsonType::StdVecVecObjectWithId => PostgresqlJsonTypeFieldReader::StdVecVecObjectWithIdIdent,
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => PostgresqlJsonTypeFieldReader::StdOptionOptionStdVecVecObjectWithIdIdent,
                    });
                    let tokens_options_to_read_upper_camel_case_token_stream = {
                        let value: &dyn quote::ToTokens = match &postgresql_json_type {
                            PostgresqlJsonType::Object => &naming::parameter::ObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                        };
                        quote::quote!{#value}
                    };
                    let impl_serde_deserialize_for_postgresql_json_type_tokens_options_to_read_token_stream = match &postgresql_json_type {
                        //todo impl_serde_deserialize
                        PostgresqlJsonType::Object => proc_macro2::TokenStream::new(),
                        PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                        PostgresqlJsonType::StdVecVecObjectWithId => {
                            let tuple_struct_postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&tokens_options_to_read_upper_camel_case_token_stream);
                            let tuple_struct_postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&tokens_options_to_read_upper_camel_case_token_stream);
                            let postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                &tokens_options_to_read_upper_camel_case_token_stream
                            );
                            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                &tokens_options_to_read_upper_camel_case_token_stream,
                                &field0_token_stream,
                            );
                            quote::quote!{
                                impl<'de> serde::Deserialize<'de> for #tokens_options_to_read_upper_camel_case_token_stream {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: serde::Deserializer<'de>,
                                    {
                                        #[doc(hidden)]
                                        struct __Visitor<'de> {
                                            marker: serde::__private::PhantomData<
                                                #tokens_options_to_read_upper_camel_case_token_stream,
                                            >,
                                            lifetime: serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #tokens_options_to_read_upper_camel_case_token_stream;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut serde::__private::Formatter<'_>,
                                            ) -> serde::__private::fmt::Result {
                                                serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    #tuple_struct_postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_double_quotes_token_stream,
                                                )
                                            }
                                            #[inline]
                                            fn visit_newtype_struct<__E>(
                                                self,
                                                __e: __E,
                                            ) -> serde::__private::Result<Self::Value, __E::Error>
                                            where
                                                __E: serde::Deserializer<'de>,
                                            {
                                                let __field0: std::vec::Vec<
                                                    #ident_options_to_read_with_id_upper_camel_case,
                                                > = <std::vec::Vec<
                                                    #ident_options_to_read_with_id_upper_camel_case,
                                                > as serde::Deserialize>::deserialize(__e)?;
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 = match serde::de::SeqAccess::next_element::<
                                                    std::vec::Vec<
                                                        #ident_options_to_read_with_id_upper_camel_case,
                                                    >,
                                                >(&mut __seq)? {
                                                    serde::__private::Some(__value) => __value,
                                                    serde::__private::None => {
                                                        return serde::__private::Err(
                                                            serde::de::Error::invalid_length(
                                                                0usize,
                                                                &#tuple_struct_postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                                            ),
                                                        );
                                                    }
                                                };
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                        }
                                        serde::Deserializer::deserialize_newtype_struct(
                                            __deserializer,
                                            #postgresql_json_type_std_vec_vec_object_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream,
                                            __Visitor {
                                                marker: serde::__private::PhantomData::<
                                                    #tokens_options_to_read_upper_camel_case_token_stream,
                                                >,
                                                lifetime: serde::__private::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        },
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                            let tuple_struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&tokens_options_to_read_upper_camel_case_token_stream);
                            let tuple_struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&tokens_options_to_read_upper_camel_case_token_stream);
                            let postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                &tokens_options_to_read_upper_camel_case_token_stream
                            );
                            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                &tokens_options_to_read_upper_camel_case_token_stream,
                                &field0_token_stream,
                            );
                            quote::quote!{
                                impl<'de> serde::Deserialize<'de> for #tokens_options_to_read_upper_camel_case_token_stream {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: serde::Deserializer<'de>,
                                    {
                                        #[doc(hidden)]
                                        struct __Visitor<'de> {
                                            marker: serde::__private::PhantomData<
                                                #tokens_options_to_read_upper_camel_case_token_stream,
                                            >,
                                            lifetime: serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #tokens_options_to_read_upper_camel_case_token_stream;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut serde::__private::Formatter<'_>,
                                            ) -> serde::__private::fmt::Result {
                                                serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    #tuple_struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_double_quotes_token_stream,
                                                )
                                            }
                                            #[inline]
                                            fn visit_newtype_struct<__E>(
                                                self,
                                                __e: __E,
                                            ) -> serde::__private::Result<Self::Value, __E::Error>
                                            where
                                                __E: serde::Deserializer<'de>,
                                            {
                                                let __field0: std::option::Option<
                                                    std::vec::Vec<
                                                        #ident_options_to_read_with_id_upper_camel_case,
                                                    >,
                                                > = <std::option::Option<
                                                    std::vec::Vec<
                                                        #ident_options_to_read_with_id_upper_camel_case,
                                                    >,
                                                > as serde::Deserialize>::deserialize(__e)?;
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 = match serde::de::SeqAccess::next_element::<
                                                    std::option::Option<
                                                        std::vec::Vec<
                                                            #ident_options_to_read_with_id_upper_camel_case,
                                                        >,
                                                    >,
                                                >(&mut __seq)? {
                                                    serde::__private::Some(__value) => __value,
                                                    serde::__private::None => {
                                                        return serde::__private::Err(
                                                            serde::de::Error::invalid_length(
                                                                0usize,
                                                                &#tuple_struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                                            ),
                                                        );
                                                    }
                                                };
                                                #match_try_new_in_deserialize_token_stream
                                            }
                                        }
                                        serde::Deserializer::deserialize_newtype_struct(
                                            __deserializer,
                                            #postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream,
                                            __Visitor {
                                                marker: serde::__private::PhantomData::<
                                                    #tokens_options_to_read_upper_camel_case_token_stream,
                                                >,
                                                lifetime: serde::__private::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        },
                    };
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_field_reader_token_stream = 
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &tokens_field_reader_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let value = match &postgresql_json_type {
                                //todo option(null) must also be a read value
                                PostgresqlJsonType::Object |
                                PostgresqlJsonType::StdOptionOptionObject => quote::quote!{(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
                                PostgresqlJsonType::StdVecVecObjectWithId |
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{
                                    {
                                        field_vec: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                        pagination: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                    }
                                },
                            };
                            quote::quote!{Self #value}
                        }
                    );
                    let postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream = {
                        let value: &dyn quote::ToTokens = match &postgresql_json_type {
                            PostgresqlJsonType::Object => &naming::parameter::ObjectSelfFieldReaderUpperCamelCase::from_display(&ident),
                            PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident),
                        };
                        quote::quote!{#value}
                    };
                    let impl_serde_deserialize_for_postgresql_json_type_tokens_field_reader_token_stream = {
                        let field0_field1_token_stream = quote::quote!{__field0, __field1};
                        match &postgresql_json_type {
                            PostgresqlJsonType::Object => {
                                let object_ident_field_reader_upper_camel_case = naming::parameter::ObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident);
                                let tuple_struct_object_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&object_ident_field_reader_upper_camel_case);
                                let tuple_struct_object_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&object_ident_field_reader_upper_camel_case);
                                let object_ident_field_reader_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&object_ident_field_reader_upper_camel_case);
                                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                    &postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream,
                                    &field0_token_stream,
                                );
                                quote::quote!{
                                    impl<'de> serde::Deserialize<'de> for #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: serde::Deserializer<'de>,
                                        {
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: serde::__private::PhantomData<#postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream>,
                                                lifetime: serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut serde::__private::Formatter<'_>,
                                                ) -> serde::__private::fmt::Result {
                                                    serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #tuple_struct_object_ident_field_reader_double_quotes_token_stream,
                                                    )
                                                }
                                                #[inline]
                                                fn visit_newtype_struct<__E>(
                                                    self,
                                                    __e: __E,
                                                ) -> serde::__private::Result<Self::Value, __E::Error>
                                                where
                                                    __E: serde::Deserializer<'de>,
                                                {
                                                    let __field0: std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case> = <std::vec::Vec<
                                                        #ident_field_to_read_without_id_upper_camel_case,
                                                    > as serde::Deserialize>::deserialize(__e)?;
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                                        std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case>,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &#tuple_struct_object_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                            }
                                            serde::Deserializer::deserialize_newtype_struct(
                                                __deserializer,
                                                #object_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                                __Visitor {
                                                    marker: serde::__private::PhantomData::<#postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream>,
                                                    lifetime: serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                }
                            },
                            PostgresqlJsonType::StdOptionOptionObject => {
                                let postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case = naming::parameter::PostgresqlJsonTypeStdOptionOptionObjectSelfFieldReaderUpperCamelCase::from_tokens(&ident);
                                let tuple_struct_postgresql_json_type_std_option_option_object_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case);
                                let tuple_struct_postgresql_json_type_std_option_option_object_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case);
                                let postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case
                                );
                                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                    &postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case,
                                    &field0_token_stream,
                                );
                                quote::quote!{
                                    impl<'de> serde::Deserialize<'de> for #postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: serde::Deserializer<'de>,
                                        {
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: serde::__private::PhantomData<
                                                    #postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case,
                                                >,
                                                lifetime: serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut serde::__private::Formatter<'_>,
                                                ) -> serde::__private::fmt::Result {
                                                    serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #tuple_struct_postgresql_json_type_std_option_option_object_ident_field_reader_double_quotes_token_stream,
                                                    )
                                                }
                                                #[inline]
                                                fn visit_newtype_struct<__E>(
                                                    self,
                                                    __e: __E,
                                                ) -> serde::__private::Result<Self::Value, __E::Error>
                                                where
                                                    __E: serde::Deserializer<'de>,
                                                {
                                                    let __field0: std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case> = <std::vec::Vec<
                                                        #ident_field_to_read_without_id_upper_camel_case,
                                                    > as serde::Deserialize>::deserialize(__e)?;
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                                        std::vec::Vec<#ident_field_to_read_without_id_upper_camel_case>,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &#tuple_struct_postgresql_json_type_std_option_option_object_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                            }
                                            serde::Deserializer::deserialize_newtype_struct(
                                                __deserializer,
                                                #postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                                __Visitor {
                                                    marker: serde::__private::PhantomData::<
                                                        #postgresql_json_type_std_option_option_object_ident_field_reader_upper_camel_case,
                                                    >,
                                                    lifetime: serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                }
                            },
                            PostgresqlJsonType::StdVecVecObjectWithId => {
                                let postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case = naming::parameter::PostgresqlJsonTypeStdVecVecObjectWithIdSelfFieldReaderUpperCamelCase::from_tokens(&ident);
                                let struct_postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_double_quotes_token_stream = generate_struct_tokens_double_quotes_token_stream(&postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case);
                                let struct_postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = generate_struct_tokens_with_2_elements_double_quotes_token_stream(&postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case);
                                let postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case
                                );
                                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                    &postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case,
                                    &field0_field1_token_stream,
                                );
                                quote::quote!{
                                    impl<'de> serde::Deserialize<'de> for #postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: serde::Deserializer<'de>,
                                        {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __field1,
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
                                                        0u64 => serde::__private::Ok(__Field::__field0),
                                                        1u64 => serde::__private::Ok(__Field::__field1),
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
                                                        "field_vec" => serde::__private::Ok(__Field::__field0),
                                                        "pagination" => serde::__private::Ok(__Field::__field1),
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
                                                        b"field_vec" => serde::__private::Ok(__Field::__field0),
                                                        b"pagination" => serde::__private::Ok(__Field::__field1),
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
                                                    #postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case,
                                                >,
                                                lifetime: serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut serde::__private::Formatter<'_>,
                                                ) -> serde::__private::fmt::Result {
                                                    serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #struct_postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_double_quotes_token_stream,
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
                                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &#struct_postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match serde::de::SeqAccess::next_element::<
                                                        #postgersql_crud_pagination_token_stream,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &#struct_postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
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
                                                    let mut __field0: serde::__private::Option<
                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                    > = serde::__private::None;
                                                    let mut __field1: serde::__private::Option<#postgersql_crud_pagination_token_stream> = serde::__private::None;
                                                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if serde::__private::Option::is_some(&__field0) {
                                                                    return serde::__private::Err(
                                                                        <__A::Error as serde::de::Error>::duplicate_field(
                                                                            "field_vec",
                                                                        ),
                                                                    );
                                                                }
                                                                __field0 = serde::__private::Some(
                                                                    serde::de::MapAccess::next_value::<
                                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field1 => {
                                                                if serde::__private::Option::is_some(&__field1) {
                                                                    return serde::__private::Err(
                                                                        <__A::Error as serde::de::Error>::duplicate_field(
                                                                            "pagination",
                                                                        ),
                                                                    );
                                                                }
                                                                __field1 = serde::__private::Some(
                                                                    serde::de::MapAccess::next_value::<#postgersql_crud_pagination_token_stream>(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = serde::de::MapAccess::next_value::<
                                                                    serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        serde::__private::Some(__field0) => __field0,
                                                        serde::__private::None => {
                                                            serde::__private::de::missing_field("field_vec")?
                                                        }
                                                    };
                                                    let __field1 = match __field1 {
                                                        serde::__private::Some(__field1) => __field1,
                                                        serde::__private::None => {
                                                            serde::__private::de::missing_field("pagination")?
                                                        }
                                                    };
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
                                            serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                #postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                                FIELDS,
                                                __Visitor {
                                                    marker: serde::__private::PhantomData::<
                                                        #postgresql_json_type_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case,
                                                    >,
                                                    lifetime: serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                }
                            },
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                                let struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_double_quotes_token_stream = generate_struct_tokens_double_quotes_token_stream(&postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream);
                                let struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = generate_struct_tokens_with_2_elements_double_quotes_token_stream(&postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream);
                                let postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                    &postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream
                                );
                                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                                    &postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream,
                                    &field0_field1_token_stream,
                                );
                                quote::quote!{
                                    impl<'de> serde::Deserialize<'de> for #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: serde::Deserializer<'de>,
                                        {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __field1,
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
                                                        0u64 => serde::__private::Ok(__Field::__field0),
                                                        1u64 => serde::__private::Ok(__Field::__field1),
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
                                                        "field_vec" => serde::__private::Ok(__Field::__field0),
                                                        "pagination" => serde::__private::Ok(__Field::__field1),
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
                                                        b"field_vec" => serde::__private::Ok(__Field::__field0),
                                                        b"pagination" => serde::__private::Ok(__Field::__field1),
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
                                                    #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream,
                                                >,
                                                lifetime: serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut serde::__private::Formatter<'_>,
                                                ) -> serde::__private::fmt::Result {
                                                    serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_double_quotes_token_stream,
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
                                                    let __field0 = match serde::de::SeqAccess::next_element::<
                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &#struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match serde::de::SeqAccess::next_element::<
                                                        #postgersql_crud_pagination_token_stream,
                                                    >(&mut __seq)? {
                                                        serde::__private::Some(__value) => __value,
                                                        serde::__private::None => {
                                                            return serde::__private::Err(
                                                                serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &#struct_postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
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
                                                    let mut __field0: serde::__private::Option<
                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                    > = serde::__private::None;
                                                    let mut __field1: serde::__private::Option<#postgersql_crud_pagination_token_stream> = serde::__private::None;
                                                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if serde::__private::Option::is_some(&__field0) {
                                                                    return serde::__private::Err(
                                                                        <__A::Error as serde::de::Error>::duplicate_field(
                                                                            "field_vec",
                                                                        ),
                                                                    );
                                                                }
                                                                __field0 = serde::__private::Some(
                                                                    serde::de::MapAccess::next_value::<
                                                                        std::vec::Vec<#ident_field_to_read_with_id_upper_camel_case>,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field1 => {
                                                                if serde::__private::Option::is_some(&__field1) {
                                                                    return serde::__private::Err(
                                                                        <__A::Error as serde::de::Error>::duplicate_field(
                                                                            "pagination",
                                                                        ),
                                                                    );
                                                                }
                                                                __field1 = serde::__private::Some(
                                                                    serde::de::MapAccess::next_value::<#postgersql_crud_pagination_token_stream>(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = serde::de::MapAccess::next_value::<
                                                                    serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        serde::__private::Some(__field0) => __field0,
                                                        serde::__private::None => {
                                                            serde::__private::de::missing_field("field_vec")?
                                                        }
                                                    };
                                                    let __field1 = match __field1 {
                                                        serde::__private::Some(__field1) => __field1,
                                                        serde::__private::None => {
                                                            serde::__private::de::missing_field("pagination")?
                                                        }
                                                    };
                                                    #match_try_new_in_deserialize_token_stream
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
                                            serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                #postgresql_json_type_std_option_option_std_vec_vec_object_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                                FIELDS,
                                                __Visitor {
                                                    marker: serde::__private::PhantomData::<
                                                        #postgresql_json_type_tokens_field_reader_upper_camel_case_token_stream,
                                                    >,
                                                    lifetime: serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                }
                            },
                        }
                    };
                    let tokens_reader_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                        PostgresqlJsonType::Object => &naming::parameter::ObjectSelfReaderUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfReaderUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfReaderUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfReaderUpperCamelCase::from_tokens(&ident),
                    };
                    let tokens_reader_token_stream = generate_tokens_reader_alias_token_stream(
                        &tokens_reader_upper_camel_case,
                        &tokens_options_to_read_upper_camel_case
                    );
                    let tokens_where_element_token_stream = {
                        let tokens_where_element_token_stream = {
                            let variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_type_where_upper_camel_case_token_stream = naming::parameter::SelfWhereUpperCamelCase::from_type_last_segment(&element.ty);
                                quote::quote!{
                                    #field_ident_upper_camel_case_token_stream(#field_type_where_upper_camel_case_token_stream)
                                }
                            });
                            quote::quote!{
                                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                                pub enum #tokens_where_element_upper_camel_case {
                                     #(#variants_token_stream),*
                                }
                            }
                        };
                        let impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_where_element_token_stream = postgresql_crud_macros_common::impl_postgresql_type_self_where_filter_for_ident_token_stream(
                            &tokens_where_element_upper_camel_case,
                            &{
                                let where_query_part_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                        &format!("{{column}}->'{field_ident_stringified}'")
                                    );
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(
                                            value,
                                            increment,
                                            &format!(#format_handle_token_stream),
                                            is_need_to_add_logical_operator,
                                        )
                                    }
                                });
                                quote::quote!{
                                    match &self {
                                        #(#where_query_part_variants_token_stream),*
                                    }
                                }
                            },
                            &{
                                let where_query_bind_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query)
                                    }
                                });
                                quote::quote!{
                                    match self {
                                        #(#where_query_bind_variants_token_stream),*
                                    }
                                }
                            },
                            &postgresql_crud_macros_common::PostgresqlTypeSelfWhereFilterPath::PostgresqlCrud,
                        );
                        let impl_error_occurence_lib_to_std_string_string_for_tokens_where_element_token_stream = {
                            quote::quote!{
                                impl error_occurence_lib::ToStdStringString for #tokens_where_element_upper_camel_case {
                                    fn to_std_string_string(&self) -> std::string::String {
                                        format!("{self:#?}")
                                    }
                                }
                            }
                        };
                        let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_where_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &tokens_where_element_upper_camel_case,
                            &{
                                let variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                                    }
                                });
                                quote::quote!{vec![#(#variants_token_stream),*]}
                            },
                        );
                        quote::quote!{
                            #tokens_where_element_token_stream
                            #impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_where_element_token_stream
                            #impl_error_occurence_lib_to_std_string_string_for_tokens_where_element_token_stream
                            #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_where_element_token_stream
                        }
                    };
                    let postgresql_json_type_tokens_where_token_stream = {
                        //todo try_new and impl serde::Deserialize
                        let postgresql_json_type_tokens_where_token_stream = {
                            quote::quote!{
                                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]//, schemars :: JsonSchema
                                pub struct #tokens_where_upper_camel_case {
                                    logical_operator: postgresql_crud::LogicalOperator,
                                    value: std::vec::Vec<#tokens_where_element_upper_camel_case>,
                                }
                            }
                        };
                        let impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_tokens_where_token_stream = postgresql_crud_macros_common::impl_postgresql_type_self_where_filter_for_ident_token_stream(
                            &tokens_where_upper_camel_case,
                            &quote::quote!{
                                let mut acc = std::string::String::default();
                                let mut is_need_to_add_logical_operator_inner_handle = false;
                                for element in &self.value {
                                    match postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                                        Ok(value) => {
                                            acc.push_str(&format!("{value} "));
                                            is_need_to_add_logical_operator_inner_handle = true;
                                        }
                                        Err(error) => {
                                            return Err(error);
                                        }
                                    }
                                }
                                let _ = acc.pop();
                                Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                            },
                            &quote::quote!{
                                for element in self.value {
                                    query = postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(element, query);
                                }
                                query
                            },
                            &postgresql_crud_macros_common::PostgresqlTypeSelfWhereFilterPath::PostgresqlCrud,
                        );
                        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_where_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &tokens_where_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &quote::quote!{Self {
                                logical_operator: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                value: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            }},
                        );
                        quote::quote!{
                            #postgresql_json_type_tokens_where_token_stream
                            #impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_tokens_where_token_stream
                            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_where_token_stream
                        }
                    };
                    quote::quote!{
                        #postgresql_json_type_tokens_options_to_read_token_stream
                        #maybe_impl_try_new_for_postgresql_json_type_tokens_options_to_read_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_options_to_read_token_stream
                        #tokens_field_reader_token_stream
                        #impl_serde_deserialize_for_postgresql_json_type_tokens_options_to_read_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_field_reader_token_stream
                        #impl_serde_deserialize_for_postgresql_json_type_tokens_field_reader_token_stream
                        #tokens_reader_token_stream
                        #tokens_where_element_token_stream
                        #postgresql_json_type_tokens_where_token_stream
                    }
                };
                let std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case = naming::parameter::StdVecVecObjectWithIdSelfJsonArrayChangeUpperCamelCase::from_tokens(&ident);
                let std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case = naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfJsonArrayChangeUpperCamelCase::from_tokens(&ident);
                let generate_ident_json_array_change_token_stream = |
                    struct_ident_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
                    struct_ident_try_new_error_named: &dyn quote::ToTokens,
                    is_nullable: std::primitive::bool,
                |{
                    let create_snake_case = naming::CreateSnakeCase;
                    let update_snake_case = naming::UpdateSnakeCase;
                    let delete_snake_case = naming::DeleteSnakeCase;
                    let ident_json_array_change_token_stream = {
                        let serde_skip_serializing_if_vec_is_empty_token_stream = quote::quote!{#[serde(skip_serializing_if = "Vec::is_empty")]};
                        quote::quote!{
                            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
                            pub struct #struct_ident_token_stream {
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #create_snake_case: std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>,
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #update_snake_case: std::vec::Vec<#ident_options_to_update_upper_camel_case>,
                                #serde_skip_serializing_if_vec_is_empty_token_stream
                                #delete_snake_case: std::vec::Vec<#postgresql_crud_uuid_update_token_stream>,
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
                            }
                            else {
                                quote::quote!{
                                    #create_update_delete_check_fields_are_empty_upper_camel_case {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                }
                            };
                            quote::quote!{
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
                            }
                            else {
                                quote::quote!{
                                    if #create_snake_case.is_empty() && #update_snake_case.is_empty() && #delete_snake_case.is_empty() {
                                        return Err(#struct_ident_try_new_error_named::#create_update_delete_check_fields_are_empty_upper_camel_case {
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            };
                            let check_not_unique_id_token_stream = {
                                let check_not_unique_id_in_update_array_token_stream = {
                                    let not_unique_id_in_json_update_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                        &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique id in json update array: {{}}"),
                                    );
                                    quote::quote!{
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
                                    let not_unique_id_in_json_delete_array_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                        &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique {id_snake_case} in json delete array: {{}}"),
                                    );
                                    quote::quote!{
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
                                    let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                                        &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique {id_snake_case} in json update and delete arrays: {{}}")
                                    );
                                    quote::quote!{
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
                                quote::quote!{
                                    {
                                        #check_not_unique_id_in_update_array_token_stream
                                        #check_not_unique_id_in_delete_aray_token_stream
                                        #check_not_unique_id_in_update_and_delete_arrays_token_stream
                                    }
                                }
                            };
                            quote::quote!{
                                impl #struct_ident_token_stream {
                                    pub fn try_new(
                                        #create_snake_case: std::vec::Vec<#ident_to_create_with_generated_id_upper_camel_case>,
                                        #update_snake_case: std::vec::Vec<#ident_options_to_update_upper_camel_case>,
                                        #delete_snake_case: std::vec::Vec<#postgresql_crud_uuid_update_token_stream>,
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
                        quote::quote!{
                            #try_new_error_named_token_stream
                            #impl_pub_fn_try_new_token_stream
                        }
                    };
                    let impl_serde_deserialize_for_ident_json_array_change_token_stream = {
                        let tuple_struct_struct_ident_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&struct_ident_token_stream);
                        let struct_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&struct_ident_token_stream);
                        let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                            &struct_ident_token_stream,
                            &quote::quote!{__field0, __field1, __field2},
                        );
                        quote::quote!{
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
                                            let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_options_to_update_upper_camel_case>>(&mut __seq)? {
                                                serde::__private::Some(__value) => __value,
                                                serde::__private::None => {
                                                    vec![]
                                                }
                                            };
                                            let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#postgresql_crud_uuid_update_token_stream>>(&mut __seq)? {
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
                                            let mut __field1: serde::__private::Option<std::vec::Vec<#ident_options_to_update_upper_camel_case>> = serde::__private::None;
                                            let mut __field2: serde::__private::Option<std::vec::Vec<#postgresql_crud_uuid_update_token_stream>> = serde::__private::None;
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
                                                        __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_options_to_update_upper_camel_case>>(&mut __map)?);
                                                    }
                                                    __Field::__field2 => {
                                                        if serde::__private::Option::is_some(&__field2) {
                                                            return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                                                        }
                                                        __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#postgresql_crud_uuid_update_token_stream>>(&mut __map)?);
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
                    let impl_ident_json_array_change_methods_token_stream = generate_tokens_to_update_methods_token_stream(
                        &struct_ident_token_stream,
                        &ident_json_array_change_try_generate_error_named_upper_camel_case,
                        &{
                            let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
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
                                                return Err(#ident_json_array_change_try_generate_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                                    error,
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                        }
                                    }
                                }
                            });
                            let ok_format_handle_token_stream = if is_nullable {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                    &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}', case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'array' then case when jsonb_array_length({{{jsonb_set_target_snake_case}}}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({{update_query_part_acc}}) from jsonb_array_elements({{{jsonb_set_target_snake_case}}}) as elem {{maybe_where}}), '[]'::jsonb)) end else '[]'::jsonb end {{maybe_jsonb_build_array}})")
                                );
                                quote::quote!{
                                    Ok(format!(#format_handle_token_stream))
                                }
                            }
                            else {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                    &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}', case when jsonb_array_length({{{jsonb_set_target_snake_case}}}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({{update_query_part_acc}}) from jsonb_array_elements({{{jsonb_set_target_snake_case}}}) as elem {{maybe_where}}), '[]'::jsonb)) end {{maybe_jsonb_build_array}})")
                                );
                                quote::quote!{
                                    Ok(format!(#format_handle_token_stream))
                                }
                            };
                            let delete_query_part_acc_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("{{maybe_space_and_space}}elem->>'{id_snake_case}' <> ${{{increment_snake_case}}}")
                            );
                            let update_push_str_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("when elem ->> '{id_snake_case}' = ${{id_increment}} then {{element_acc}} ")
                            );
                            quote::quote!{
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
                                                    return Err(#ident_json_array_change_try_generate_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
                                                }
                                            };
                                            for element in &element_handle.fields.0 {
                                                match element {
                                                    #(#try_generate_bind_increments_variants_token_stream),*
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
                                                return Err(#ident_json_array_change_try_generate_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
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
                                                return Err(#ident_json_array_change_try_generate_error_named_upper_camel_case::Create {
                                                    error,
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
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
                        },
                        &{
                            let bind_value_to_postgresql_query_part_to_update_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident = element.ident.as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                                    }
                                }
                            });
                            quote::quote!{
                                for element_handle in self.#update_snake_case {
                                    #query_snake_case = #query_snake_case.bind(element_handle.#id_snake_case.0.to_string());//postgresql: error returned from database: operator does not exist: text = jsonb
                                    for element in element_handle.fields.0 {
                                        match element {
                                            #(#bind_value_to_postgresql_query_part_to_update_variants_token_stream),*
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
                        },
                    );
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream =       postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &struct_ident_token_stream,
                        &proc_macro2::TokenStream::new(),
                        &quote::quote!{Self {
                            #create_snake_case: vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream],
                            #update_snake_case: vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream],
                            #delete_snake_case: vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream],
                        }},
                    );
                    quote::quote!{
                        #ident_json_array_change_token_stream
                        #impl_try_new_for_ident_json_array_change_token_stream
                        #impl_serde_deserialize_for_ident_json_array_change_token_stream
                        #impl_ident_json_array_change_methods_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream
                    }
                };
                let tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::Object => &naming::parameter::ObjectSelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident),
                    PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_tokens(&ident),
                };
                let update_token_stream = {
                    let std_vec_vec_object_with_id_or_std_option_option_std_vec_vec_object_with_id_ident_token_stream = match &postgresql_json_type {
                        PostgresqlJsonType::Object |
                        PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                        PostgresqlJsonType::StdVecVecObjectWithId => generate_ident_json_array_change_token_stream(
                            &std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case,
                            &naming::parameter::StdVecVecObjectWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                            false,
                        ),
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => generate_ident_json_array_change_token_stream(
                            &std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case,
                            &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
                            true,
                        ),
                    };
                    let postgresql_json_type_tokens_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                        &generate_tokens_update_upper_camel_case_token_stream(postgresql_json_type),
                        &match &postgresql_json_type {
                            PostgresqlJsonType::Object => quote::quote!{pub #ident_update_upper_camel_case},
                            PostgresqlJsonType::StdOptionOptionObject => quote::quote!{pub std::option::Option<#ident_update_upper_camel_case>},
                            PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{#std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case},
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{std::option::Option<#std_option_option_std_vec_vec_object_with_id_ident_json_array_change_upper_camel_case>},
                        },
                        true,
                        match &postgresql_json_type {
                            PostgresqlJsonType::Object |
                            PostgresqlJsonType::StdOptionOptionObject => false,
                            PostgresqlJsonType::StdVecVecObjectWithId |
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => true,
                        }
                    );
                    let postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream = {
                        let generate_postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                            generate_tokens_try_generate_postgresql_json_type_error_named_token_stream(
                                &tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream,
                                &content_token_stream
                            )
                        };
                        let object_and_std_option_option_object_variants_token_stream = {
                            let variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let element_type_option_to_update_try_generate_error_named_upper_camel_case = naming::parameter::SelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase::from_type_last_segment(&element.ty);
                                quote::quote!{
                                    #variant_ident_upper_camel_case_token_stream {
                                        #[eo_error_occurence]
                                        error: #element_type_option_to_update_try_generate_error_named_upper_camel_case,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    }
                                }
                            });
                            quote::quote!{#(#variants_token_stream),*}
                        };
                        let ident_json_array_change_try_generate_postgresql_json_type_error_named_token_stream = quote::quote!{
                            #ident_json_array_change_try_generate_error_named_upper_camel_case {
                                #[eo_error_occurence]
                                error: #ident_json_array_change_try_generate_error_named_upper_camel_case, 
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        };
                        match &postgresql_json_type {
                            PostgresqlJsonType::Object => generate_postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream(&object_and_std_option_option_object_variants_token_stream),
                            PostgresqlJsonType::StdOptionOptionObject => generate_postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream(&quote::quote!{
                                #checked_add_variant_declaration_token_stream,
                                #object_and_std_option_option_object_variants_token_stream
                            }),
                            PostgresqlJsonType::StdVecVecObjectWithId => generate_postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream(
                                &ident_json_array_change_try_generate_postgresql_json_type_error_named_token_stream
                            ),
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => generate_postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream(&quote::quote!{
                                #checked_add_variant_declaration_token_stream,
                                #ident_json_array_change_try_generate_postgresql_json_type_error_named_token_stream
                            })
                        }
                    };
                    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_option_to_update_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &generate_tokens_update_upper_camel_case_token_stream(postgresql_json_type),
                        &proc_macro2::TokenStream::new(),
                        &{
                            let value = match &postgresql_json_type {
                                PostgresqlJsonType::Object |
                                PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
                                PostgresqlJsonType::StdOptionOptionObject |
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))},
                            };
                            quote::quote!{Self #value}
                        }
                    );
                    quote::quote!{
                        #std_vec_vec_object_with_id_or_std_option_option_std_vec_vec_object_with_id_ident_token_stream
                        #postgresql_json_type_tokens_option_to_update_token_stream
                        #postgresql_json_type_tokens_option_to_update_try_generate_postgresql_json_type_error_named_token_stream
                        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_tokens_option_to_update_token_stream
                    }
                };
                let value_snake_case = naming::ValueSnakeCase;
                let field_ident_snake_case = naming::FieldIdentSnakeCase;
                let postgresql_json_type_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
                    &postgresql_crud_path_token_stream,
                    &tokens_upper_camel_case,
                    &tokens_create_upper_camel_case,
                    &match &postgresql_json_type {
                        PostgresqlJsonType::Object => quote::quote!{#value_snake_case.0.#create_query_part_snake_case(#increment_snake_case)},
                        PostgresqlJsonType::StdOptionOptionObject => quote::quote!{
                            match &#value_snake_case.0 {
                                Some(#value_snake_case) => match #value_snake_case.#create_query_part_snake_case(#increment_snake_case) {
                                    Ok(#value_snake_case) => Ok(#value_snake_case),
                                    Err(error) => Err(error)
                                },
                                //maybe not use null here and use increment logic
                                None => Ok(std::string::String::from("null"))
                            }
                        },
                        PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{
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
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{
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
                    &match &postgresql_json_type {
                        PostgresqlJsonType::Object => quote::quote!{#value_snake_case.0.#create_query_bind_snake_case(#query_snake_case)},
                        PostgresqlJsonType::StdOptionOptionObject => quote::quote!{
                            if let Some(#value_snake_case) = #value_snake_case.0 {
                                #query_snake_case = #value_snake_case.#create_query_bind_snake_case(#query_snake_case);
                            }
                            #query_snake_case
                        },
                        PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{
                            for element in #value_snake_case.0 {
                                #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                            }
                            #query_snake_case
                        },
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{
                            if let Some(#value_snake_case) = #value_snake_case.0 {
                                for element in #value_snake_case {
                                    #query_snake_case = element.#create_query_bind_snake_case(#query_snake_case);
                                }
                            }
                            #query_snake_case
                        },
                    },
                    &tokens_field_reader_upper_camel_case,
                    &tokens_options_to_read_upper_camel_case,
                    &{
                        let column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageFieldIdentSnakeCase;
                        let generate_acc_push_str_variant_logic_token_stream = |
                            variant_name_token_stream: &dyn quote::ToTokens,
                            field_ident_double_quotes_token_stream: &dyn quote::ToTokens,
                            column_name_and_maybe_field_getter_token_stream: &dyn quote::ToTokens,
                            element_type: &dyn quote::ToTokens,
                        |{
                            let tokens_field_to_read_with_or_without_id_upper_camel_case_token_stream: &dyn quote::ToTokens = match &postgresql_json_type {
                                PostgresqlJsonType::Object |
                                PostgresqlJsonType::StdOptionOptionObject => &ident_field_to_read_without_id_upper_camel_case,
                                PostgresqlJsonType::StdVecVecObjectWithId |
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &ident_field_to_read_with_id_upper_camel_case
                            };
                            let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_to_tokens_token_stream(&element_type);
                            quote::quote!{
                                #tokens_field_to_read_with_or_without_id_upper_camel_case_token_stream::#variant_name_token_stream(value) => #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #field_reader_query_part_snake_case(
                                    value,
                                    #field_ident_double_quotes_token_stream,
                                    #column_name_and_maybe_field_getter_token_stream,
                                    &#column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case,
                                    false,
                                )
                            }
                        };
                        let column_name_and_maybe_field_getter_field_ident_snake_case = naming::ColumnNameAndMaybeFieldGetterFieldIdentSnakeCase;
                        let value_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&value_snake_case);
                        let (
                            maybe_id_variant_token_stream,
                            variants_token_stream
                        ) = {
                            let maybe_id_variant_token_stream = match &postgresql_json_type {
                                PostgresqlJsonType::Object |
                                PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                                PostgresqlJsonType::StdVecVecObjectWithId |
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                                    let id_upper_camel_case = naming::IdUpperCamelCase;
                                    let id_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&naming::IdSnakeCase);
                                    let value = generate_acc_push_str_variant_logic_token_stream(
                                        &quote::quote!{#id_upper_camel_case},
                                        &id_snake_case_double_quotes_token_stream,
                                        &value_snake_case_double_quotes_token_stream,
                                        &quote::quote!{#postgresql_crud_path_token_stream postgresql_json_type::UuidUuid},
                                    );
                                    quote::quote!{#value,}
                                }
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
                                    &match &postgresql_json_type {
                                        PostgresqlJsonType::Object |
                                        PostgresqlJsonType::StdOptionOptionObject => quote::quote!{&#column_name_and_maybe_field_getter_field_ident_snake_case},
                                        PostgresqlJsonType::StdVecVecObjectWithId |
                                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{#value_snake_case_double_quotes_token_stream}
                                    },
                                    &{
                                        let element_type = &element.ty;
                                        quote::quote!{#element_type}
                                    }
                                )
                            });
                            (
                                maybe_id_variant_token_stream,
                                variants_token_stream
                            )
                        };
                        let self_field_vec_token_stream = match &postgresql_json_type {
                            PostgresqlJsonType::Object |
                            PostgresqlJsonType::StdOptionOptionObject => quote::quote!{0},
                            PostgresqlJsonType::StdVecVecObjectWithId |
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{field_vec}
                        };
                        let maybe_pagination_start_end_initialization_token_stream = match &postgresql_json_type {
                            PostgresqlJsonType::Object |
                            PostgresqlJsonType::StdOptionOptionObject => proc_macro2::TokenStream::new(),
                            PostgresqlJsonType::StdVecVecObjectWithId |
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&naming::FieldReaderSnakeCase)
                        };
                        let column_name_and_maybe_field_getter_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("{{{column_name_and_maybe_field_getter_snake_case}}}->'{{{field_ident_snake_case}}}'")
                        );
                        let column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("{{{column_name_and_maybe_field_getter_for_error_message_snake_case}}}.{{{field_ident_snake_case}}}")
                        );
                        let (
                            if_postgresql_type_is_true_format_handle_double_quotes_token_stream,
                            if_postgresql_type_is_false_format_handle_double_quotes_token_stream
                        ) = {
                            let wrap_into_jsonb_build_object_field_ident = |value: &dyn std::fmt::Display| {
                                format!("jsonb_build_object('{{{field_ident_snake_case}}}', {value})")
                            };
                            let wrap_into_jsonb_build_object_value = |value: &dyn std::fmt::Display| {
                                format!("jsonb_build_object('{value_snake_case}',{value})")
                            };
                            let acc_format_handle = {
                                let acc_snake_case = naming::AccSnakeCase;
                                format!("{{{acc_snake_case}}}")
                            };
                            let jsonb_build_object_value_acc_format_handle = wrap_into_jsonb_build_object_value(&acc_format_handle);
                            let null_snake_case = naming::NullSnakeCase;
                            match postgresql_json_type {
                                PostgresqlJsonType::Object => {
                                    (
                                        acc_format_handle.clone(),
                                        wrap_into_jsonb_build_object_field_ident(&jsonb_build_object_value_acc_format_handle)
                                    )
                                },
                                PostgresqlJsonType::StdOptionOptionObject => {
                                    (
                                        format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else {acc_format_handle} end"),
                                        wrap_into_jsonb_build_object_field_ident(&{
                                            let jsonb_build_object_value_null = wrap_into_jsonb_build_object_value(&null_snake_case);
                                            format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {jsonb_build_object_value_null} else {jsonb_build_object_value_acc_format_handle} end")
                                        })
                                    )
                                },
                                PostgresqlJsonType::StdVecVecObjectWithId => {
                                    let format_handle = format!("(select jsonb_agg({{acc}}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}})");
                                    (
                                        format_handle.clone(),
                                        wrap_into_jsonb_build_object_field_ident(&wrap_into_jsonb_build_object_value(&format_handle))
                                    )
                                },
                                PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                                    let format_handle = format!("case when jsonb_typeof({{{column_name_and_maybe_field_getter_field_ident_snake_case}}}) = 'null' then {null_snake_case} else (select jsonb_agg({acc_format_handle}) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_field_ident_snake_case}}})) with ordinality where ordinality between {{start}} and {{end}}) end");
                                    (
                                        format_handle.clone(),
                                        wrap_into_jsonb_build_object_field_ident(&
                                            wrap_into_jsonb_build_object_value(&format_handle)
                                        )
                                    )
                                },
                            }
                        };
                        quote::quote!{
                            let mut acc = std::string::String::default();
                            let #column_name_and_maybe_field_getter_field_ident_snake_case = if is_postgresql_type {
                                column_name_and_maybe_field_getter.to_string()
                            } else {
                                format!(#column_name_and_maybe_field_getter_format_handle_token_stream)
                            };
                            let #column_name_and_maybe_field_getter_for_error_message_field_ident_snake_case = format!(#column_name_and_maybe_field_getter_for_error_message_format_handle_token_stream);
                            for element in &#value_snake_case.#self_field_vec_token_stream {
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
                    },
                    &tokens_where_element_upper_camel_case,
                    &tokens_where_upper_camel_case,
                    &generate_tokens_update_upper_camel_case_token_stream(postgresql_json_type),
                    &tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream,
                    &match &postgresql_json_type {
                        PostgresqlJsonType::Object => {
                            let object_acc_snake_case = naming::StdOptionOptionObjectAccSnakeCase;
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',{{{object_acc_snake_case}}})")
                            );
                            let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
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
                                                return Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#variant_ident_upper_camel_case_token_stream {
                                                    error,
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                        }
                                    }
                                }
                            });
                            let some_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end")
                            );
                            // let none_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            //     &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})")
                            // );
                            quote::quote!{
                                let mut #object_acc_snake_case = format!(#some_format_handle_token_stream);
                                #generate_jsonb_set_target_token_stream
                                for element in &#value_snake_case.0.0 {
                                    match element {
                                        #(#try_generate_bind_increments_variants_token_stream),*
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
                        PostgresqlJsonType::StdOptionOptionObject => {
                            let std_option_option_object_acc_snake_case = naming::StdOptionOptionObjectAccSnakeCase;
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',{{{std_option_option_object_acc_snake_case}}})")
                            );
                            let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(element);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        match #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_part_snake_case(
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
                                                return Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#variant_ident_upper_camel_case_token_stream {
                                                    error,
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                        }
                                    }
                                }
                            });
                            let some_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("case when jsonb_typeof({{{jsonb_set_target_snake_case}}}) = 'object' then ({{{jsonb_set_target_snake_case}}})::jsonb else '{{{{}}}}'::jsonb end")
                            );
                            let none_format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})")
                            );
                            quote::quote!{
                                match &#value_snake_case.0 {
                                    Some(value) => {
                                        let mut #std_option_option_object_acc_snake_case = format!(#some_format_handle_token_stream);
                                        #generate_jsonb_set_target_token_stream
                                        for element in &value.0 {
                                            match element {
                                                #(#try_generate_bind_increments_variants_token_stream),*
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
                                        None => Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#checked_add_variant_initialization_token_stream)
                                    }
                                }
                            }
                        },
                        PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{
                            match #value_snake_case.0.#update_query_part_snake_case(
                                #jsonb_set_accumulator_snake_case,
                                #jsonb_set_target_snake_case,
                                #jsonb_set_path_snake_case,
                                #increment_snake_case,
                            ) {
                                Ok(value) => Ok(value),
                                Err(error) => Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#ident_json_array_change_try_generate_error_named_upper_camel_case {
                                    error,
                                    code_occurence: error_occurence_lib::code_occurence!()
                                })
                            }
                        },
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{{jsonb_set_path_snake_case}}}}}}}',${{{increment_snake_case}}})"));
                            quote::quote!{
                                match &#value_snake_case.0 {
                                    Some(value) => {
                                        match value.#update_query_part_snake_case(
                                            #jsonb_set_accumulator_snake_case,
                                            #jsonb_set_target_snake_case,
                                            #jsonb_set_path_snake_case,
                                            #increment_snake_case,
                                        ) {
                                            Ok(value) => Ok(value),
                                            Err(error) => {
                                                return Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#ident_json_array_change_try_generate_error_named_upper_camel_case {
                                                    error,
                                                    code_occurence: error_occurence_lib::code_occurence!()
                                                });
                                            }
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
                                        None => {
                                            return Err(#tokens_option_to_update_try_generate_error_named_upper_camel_case_token_stream::#checked_add_variant_initialization_token_stream);
                                        }
                                    },
                                }
                            }
                        },
                    },
                    &match &postgresql_json_type {
                        PostgresqlJsonType::Object => {
                            let bind_value_to_postgresql_query_part_to_update_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                                    }
                                }
                            });
                            quote::quote!{
                                for element in #value_snake_case.0.0 {
                                    match element {
                                        #(#bind_value_to_postgresql_query_part_to_update_variants_token_stream),*
                                    }
                                }
                                #query_snake_case
                            }
                        },
                        PostgresqlJsonType::StdOptionOptionObject => {
                            let bind_value_to_postgresql_query_part_to_update_variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let variant_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream = generate_field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream(element);
                                quote::quote!{
                                    #ident_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                        #query_snake_case = #field_type_as_postgresql_crud_postgresql_json_type_from_field_token_stream #update_query_bind_snake_case(value.value, #query_snake_case);
                                    }
                                }
                            });
                            quote::quote!{
                                match #value_snake_case.0 {
                                    Some(value) => {
                                        for element in value.0 {
                                            match element {
                                                #(#bind_value_to_postgresql_query_part_to_update_variants_token_stream),*
                                            }
                                        }
                                    },
                                    None => {
                                        #query_snake_case = #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<#ident_update_origin_upper_camel_case>>>));
                                    }
                                }
                                #query_snake_case
                            }
                        },
                        PostgresqlJsonType::StdVecVecObjectWithId => quote::quote!{
                            #query_snake_case = #value_snake_case.0.#update_query_bind_snake_case(#query_snake_case);
                            #query_snake_case
                        },
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => quote::quote!{
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
                    }
                );
                quote::quote!{
                    #tokens_token_stream

                    #create_token_stream
                    #read_token_stream
                    #update_token_stream

                    #postgresql_json_type_token_stream
                }
            };
            let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream = {
                enum PostgresqlType {
                    JsonbNullable,
                    JsonbNotNull,
                }
                impl PostgresqlType {
                    fn add_postfix(&self, value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
                        match &self {
                            //todo maybe refactor it somehow? fails with "creates a temporary value which is freed while still in use"
                            PostgresqlType::JsonbNullable => {
                                let value = naming::parameter::SelfAsPostgresqlJsonbNullableUpperCamelCase::from_tokens(&value);
                                quote::quote!{#value}
                            },
                            PostgresqlType::JsonbNotNull => {
                                let value = naming::parameter::SelfAsPostgresqlJsonbNotNullUpperCamelCase::from_tokens(&value);
                                quote::quote!{#value}
                            },
                        }
                    }
                }

                let generate_postgresql_json_types_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream = |postgresql_type: PostgresqlType|{
                    let tokens_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                        PostgresqlJsonType::Object => &naming::parameter::ObjectSelfUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfUpperCamelCase::from_tokens(&ident),
                        PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfUpperCamelCase::from_tokens(&ident),
                    };
                    let tokens_as_postgresql_json_type_token_stream = quote::quote!{<#tokens_upper_camel_case as postgresql_crud::PostgresqlJsonType>::};
                    let tokens_as_type_upper_camel_case = postgresql_type.add_postfix(tokens_upper_camel_case);
                    let tokens_as_type_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&tokens_as_type_upper_camel_case);
                    let tokens_tokens_stream = {
                        let tokens_token_stream = {
                            quote::quote!{
                                #[derive(
                                    Debug,
                                    Clone,
                                    PartialEq,
                                    //- serde::Serialize,
                                    //- serde::Deserialize,
                                    schemars::JsonSchema,
                                )]
                                pub struct #tokens_as_type_upper_camel_case(#tokens_as_type_create_upper_camel_case);
                            }
                        };
                        let impl_std_fmt_display_for_tokens_token_stream = {
                            quote::quote!{
                                impl std::fmt::Display for #tokens_as_type_upper_camel_case {
                                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                        write!(formatter, "{:?}", self)
                                    }
                                }
                            }
                        };
                        let impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream = {
                            quote::quote!{
                                impl error_occurence_lib::ToStdStringString for #tokens_as_type_upper_camel_case {
                                    fn to_std_string_string(&self) -> std::string::String {
                                        format!("{self}")
                                    }
                                }
                            }
                        };
                        let impl_postgresql_crud_create_table_column_query_part_for_tokens_token_stream = {
                            let jsonb = "jsonb";
                            let create_table_query_part_for_tokens_stringified: &dyn std::fmt::Display = match &postgresql_type {
                                PostgresqlType::JsonbNullable => &jsonb,
                                PostgresqlType::JsonbNotNull => &format!("{jsonb} not null"),
                            };
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("{{column}} {create_table_query_part_for_tokens_stringified} check (jsonb_matches_schema('{{}}', {{column}}))")
                            );
                            quote::quote!{
                                impl #tokens_as_type_upper_camel_case {
                                    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
                                        format!(#format_handle_token_stream, serde_json::to_string(&schemars::schema_for!(#tokens_as_type_upper_camel_case)).unwrap())
                                    }
                                }
                            }
                        };
                        quote::quote!{
                            #tokens_token_stream
                            #impl_std_fmt_display_for_tokens_token_stream
                            #impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream
                            #impl_postgresql_crud_create_table_column_query_part_for_tokens_token_stream
                        }
                    };
                    let tokens_create_token_stream = {
                        let tokens_create_token_stream = {
                            //todo maybe std_option_option_object use nullable_object
                            //todo maybe remove options for postgresql nullable and nullable json
                            let tokens_create_declaration_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{std::option::Option<#tokens_create_upper_camel_case>},

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{#tokens_create_upper_camel_case},
                            };
                            quote::quote!{
                                #[derive(
                                    Debug,
                                    Clone,
                                    PartialEq,
                                    serde::Serialize,
                                    serde::Deserialize,
                                    schemars::JsonSchema,
                                )]
                                pub struct #tokens_as_type_create_upper_camel_case (#tokens_create_declaration_token_stream);
                            }
                        };
                        let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_create_token_stream = {
                            let try_generate_bind_increments_content_for_postgresql_type_tokens_to_create_token_stream = {
                                let generate_ok_try_generate_postgresql_json_type_to_create_token_stream = |is_self_zero: std::primitive::bool|{
                                    let first_argument_token_stream: &dyn quote::ToTokens = if is_self_zero {
                                        &quote::quote!{&self.0}
                                    }
                                    else {
                                        &value_snake_case
                                    };
                                    //todo remove .unwrap()
                                    quote::quote!{Ok(#tokens_as_postgresql_json_type_token_stream create_query_part(#first_argument_token_stream, increment).unwrap())}
                                };
                                match &postgresql_type {
                                    PostgresqlType::JsonbNullable
                                    => {
                                        // let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
                                        // let postgresql_crud_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream = quote::quote!{
                                        //     postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed
                                        // };
                                        let ok_as_postgresql_json_type_token_stream = generate_ok_try_generate_postgresql_json_type_to_create_token_stream(false);
                                        quote::quote!{
                                            match &self.0 {
                                                Some(#value_snake_case) => #ok_as_postgresql_json_type_token_stream,
                                                None => match increment.checked_add(1) {
                                                    Some(#value_snake_case) => {
                                                        *increment = #value_snake_case;
                                                        Ok(format!("${increment}"))
                                                    }
                                                    None => 
                                                    //- Err(#postgresql_crud_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream::#checked_add_upper_camel_case {
                                                    //-     code_occurence: error_occurence_lib::code_occurence!()
                                                    //- })
                                                    todo!() //todo make generic error type instead of PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed
                                                    ,
                                                }
                                            }
                                        }
                                    },
                                    PostgresqlType::JsonbNotNull
                                    => generate_ok_try_generate_postgresql_json_type_to_create_token_stream(true)
                                }
                            };
                            let bind_value_to_query_content_for_postgresql_type_tokens_to_create_token_stream = {
                                let query_snake_case = naming::QuerySnakeCase;
                                let generate_bind_value_to_postgresql_query_part_to_create_token_stream = |is_self_zero: std::primitive::bool|{
                                    let first_argument_token_stream: &dyn quote::ToTokens = if is_self_zero {
                                        &quote::quote!{self.0}
                                    }
                                    else {
                                        &value_snake_case
                                    };
                                    //todo reuse naming
                                    quote::quote!{#tokens_as_postgresql_json_type_token_stream create_query_bind(#first_argument_token_stream, #query_snake_case)}
                                };
                                match &postgresql_type {
                                    PostgresqlType::JsonbNullable
                                    => {
                                        let bind_value_to_postgresql_query_part_to_create_token_stream = generate_bind_value_to_postgresql_query_part_to_create_token_stream(false);
                                        quote::quote!{
                                            #query_snake_case = match self.0 {
                                                Some(#value_snake_case) => #bind_value_to_postgresql_query_part_to_create_token_stream,
                                                None => #query_snake_case.bind(std::option::Option::<sqlx::types::Json<#tokens_create_upper_camel_case>>::None)
                                            };
                                            #query_snake_case
                                        }
                                    },
                                    PostgresqlType::JsonbNotNull
                                    => generate_bind_value_to_postgresql_query_part_to_create_token_stream(true)
                                }
                            };
                            quote::quote!{
                                impl postgresql_crud::BindQuery for #tokens_as_type_create_upper_camel_case {
                                    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
                                        #try_generate_bind_increments_content_for_postgresql_type_tokens_to_create_token_stream
                                    }
                                    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                        #bind_value_to_query_content_for_postgresql_type_tokens_to_create_token_stream
                                    }
                                }
                            }
                        };
                        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_create_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &tokens_as_type_create_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &{
                                let value = match &postgresql_type {
                                    PostgresqlType::JsonbNullable => quote::quote!{(Some(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    ))},
                                    PostgresqlType::JsonbNotNull => quote::quote!{(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    )},
                                };
                                quote::quote!{Self #value}
                            },
                        );
                        quote::quote!{
                            #tokens_create_token_stream
                            #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_create_token_stream
                            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_create_token_stream
                        }
                    };
                    let tokens_as_type_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&tokens_as_type_upper_camel_case);
                    let tokens_select_token_stream = {
                        let tokens_field_reader_upper_camel_case = naming::parameter::SelfFieldReaderUpperCamelCase::from_tokens(&tokens_upper_camel_case);
                        let tokens_select_token_stream = {
                            let pub_struct_tokens_select_declaration_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{std::option::Option<#tokens_field_reader_upper_camel_case>},

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{#tokens_field_reader_upper_camel_case},
                            };
                            quote::quote!{
                                #[derive(
                                    Debug,
                                    Clone,
                                    PartialEq,
                                    serde::Serialize,
                                    serde::Deserialize,
                                    schemars::JsonSchema,
                                )]
                                pub struct #tokens_as_type_select_upper_camel_case(#pub_struct_tokens_select_declaration_token_stream);
                            }
                        };
                        let impl_sqlx_type_sqlx_postgres_for_tokens_select_token_stream = {
                            let type_info_content_for_tokens_select_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{<std::option::Option<sqlx::types::Json<#tokens_field_reader_upper_camel_case>> as sqlx::Type<sqlx::Postgres>>::type_info()},

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{<sqlx::types::Json<#tokens_field_reader_upper_camel_case> as sqlx::Type<sqlx::Postgres>>::type_info()},
                            };
                            quote::quote!{
                                impl sqlx::Type<sqlx::Postgres> for #tokens_as_type_select_upper_camel_case {
                                    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                                        #type_info_content_for_tokens_select_token_stream
                                    }
                                }
                            }
                        };
                        let impl_sqlx_decode_sqlx_postgres_for_tokens_select_token_stream = {
                            let decode_content_for_tokens_select_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable => quote::quote!{
                                    match <std::option::Option<sqlx::types::Json<#tokens_field_reader_upper_camel_case>> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                                        Ok(value) => match value {
                                            Some(value) => Ok(Self(Some(value.0))),
                                            None => Ok(Self(None)),
                                        },
                                        Err(error) => Err(error),
                                    }
                                },
                                PostgresqlType::JsonbNotNull => quote::quote!{
                                    match <sqlx::types::Json<#tokens_field_reader_upper_camel_case> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                                        Ok(value) => Ok(Self(value.0)),
                                        Err(error) => Err(error),
                                    }
                                },
                            };
                            quote::quote!{
                                impl sqlx::Decode<'_, sqlx::Postgres> for #tokens_as_type_select_upper_camel_case {
                                    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                                        #decode_content_for_tokens_select_token_stream
                                    }
                                }
                            }
                        };
                       let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &tokens_as_type_select_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &{
                                let value = match &postgresql_type {
                                    PostgresqlType::JsonbNullable => quote::quote!{(Some(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    ))},
                                    PostgresqlType::JsonbNotNull => quote::quote!{(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    )},
                                };
                                quote::quote!{Self #value}
                            },
                        );
                        quote::quote!{
                            #tokens_select_token_stream
                            #impl_sqlx_type_sqlx_postgres_for_tokens_select_token_stream
                            #impl_sqlx_decode_sqlx_postgres_for_tokens_select_token_stream
                            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_select_token_stream
                        }
                    };
                    let tokens_as_type_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&tokens_as_type_upper_camel_case);
                    let tokens_where_element_token_stream = {
                        let tokens_where_element_token_stream = {
                            let variants_token_stream = vec_syn_field.iter().map(|element| {
                                let field_ident_stringified = element
                                    .ident
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                    })
                                    .to_string();
                                let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                let field_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_type_last_segment(&element.ty);
                                quote::quote!{
                                    #field_ident_upper_camel_case_token_stream(#field_ident_where_upper_camel_case)
                                }
                            });
                            quote::quote!{
                                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                                pub enum #tokens_as_type_where_element_upper_camel_case {
                                    #(#variants_token_stream),*
                                }
                            }
                        };
                        let impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_where_element_token_stream = postgresql_crud_macros_common::impl_postgresql_type_self_where_filter_for_ident_token_stream(
                            &tokens_as_type_where_element_upper_camel_case,
                            &{
                                let where_query_part_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{column}}->'{field_ident_stringified}'"));
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(
                                            value,
                                            increment,
                                            &format!(#format_handle_token_stream),
                                            is_need_to_add_logical_operator,
                                        )
                                    }
                                });
                                quote::quote!{
                                    match &self {
                                        #(#where_query_part_variants_token_stream),*
                                    }
                                }
                            },
                            &{
                                let where_query_bind_variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(value) => postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query)
                                    }
                                });
                                quote::quote!{
                                    match self {
                                        #(#where_query_bind_variants_token_stream),*
                                    }
                                }
                            },
                            &postgresql_crud_macros_common::PostgresqlTypeSelfWhereFilterPath::PostgresqlCrud,
                        );
                        let impl_error_occurence_lib_to_std_string_string_for_tokens_where_element_token_stream = {
                            quote::quote!{
                                impl error_occurence_lib::ToStdStringString for #tokens_as_type_where_element_upper_camel_case {
                                    fn to_std_string_string(&self) -> std::string::String {
                                        format!("{self:#?}")
                                    }
                                }
                            }
                        };
                        let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_where_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &tokens_as_type_where_element_upper_camel_case,
                            &{
                                let variants_token_stream = vec_syn_field.iter().map(|element| {
                                    let field_ident_stringified = element
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{}", naming::FIELD_IDENT_IS_NONE);
                                        })
                                        .to_string();
                                    let field_ident_upper_camel_case_token_stream = naming::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&field_ident_stringified);
                                    quote::quote!{
                                        Self::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                                    }
                                });
                                quote::quote!{vec![#(#variants_token_stream),*]}
                            },
                        );
                        quote::quote!{
                            #tokens_where_element_token_stream
                            #impl_postgresql_crud_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_where_element_token_stream
                            #impl_error_occurence_lib_to_std_string_string_for_tokens_where_element_token_stream
                            #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_where_element_token_stream
                        }
                    };
                    let postgresql_type_tokens_as_type_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&tokens_as_type_upper_camel_case);
                    let postgresql_type_tokens_to_read_token_stream = {
                        let tokens_options_to_read_upper_camel_case: &dyn quote::ToTokens = match &postgresql_json_type {
                            PostgresqlJsonType::Object => &naming::parameter::ObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdOptionOptionObject => &naming::parameter::StdOptionOptionObjectSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdVecVecObjectWithId => &naming::parameter::StdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                            PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId => &naming::parameter::StdOptionOptionStdVecVecObjectWithIdSelfOptionsToReadUpperCamelCase::from_tokens(&ident),
                        };
                        let postgresql_type_tokens_to_read_token_stream = {
                            let pub_struct_postgresql_type_tokens_to_read_declaration_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{std::option::Option<#tokens_options_to_read_upper_camel_case>},

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{#tokens_options_to_read_upper_camel_case},
                            };
                            quote::quote!{
                                #[derive(
                                    Debug,
                                    Clone,
                                    PartialEq,
                                    serde::Serialize,
                                    serde::Deserialize,
                                    schemars::JsonSchema,
                                )]
                                pub struct #postgresql_type_tokens_as_type_read_upper_camel_case(#pub_struct_postgresql_type_tokens_to_read_declaration_token_stream);
                            }
                        };
                        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_tokens_to_read_token_stream = {
                            let type_info_content_for_postgresql_type_tokens_to_read_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{<std::option::Option<sqlx::types::Json<#tokens_options_to_read_upper_camel_case>> as sqlx::Type<sqlx::Postgres>>::type_info()},

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{<sqlx::types::Json<#tokens_options_to_read_upper_camel_case> as sqlx::Type<sqlx::Postgres>>::type_info()},
                            };
                            quote::quote!{
                                impl sqlx::Type<sqlx::Postgres> for #postgresql_type_tokens_as_type_read_upper_camel_case {
                                    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                                        #type_info_content_for_postgresql_type_tokens_to_read_token_stream
                                    }
                                }
                            }
                        };
                        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_tokens_to_read_token_stream = {
                            let decode_content_for_postgresql_type_tokens_to_read_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{
                                    match <std::option::Option<sqlx::types::Json<#tokens_options_to_read_upper_camel_case>> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                                        Ok(value) => match value {
                                            Some(value) => Ok(Self(Some(value.0))),
                                            None => Ok(Self(None)),
                                        },
                                        Err(error) => Err(error),
                                    }
                                },

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{
                                    match <sqlx::types::Json<#tokens_options_to_read_upper_camel_case> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                                        Ok(value) => Ok(Self(value.0)),
                                        Err(error) => Err(error),
                                    }
                                },
                            };
                            quote::quote!{
                                impl sqlx::Decode<'_, sqlx::Postgres> for #postgresql_type_tokens_as_type_read_upper_camel_case {
                                    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                                        #decode_content_for_postgresql_type_tokens_to_read_token_stream
                                    }
                                }
                            }
                        };
                        quote::quote!{
                            #postgresql_type_tokens_to_read_token_stream
                            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_tokens_to_read_token_stream
                            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_tokens_to_read_token_stream
                        }
                    };
                    let postgresql_type_tokens_as_type_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&tokens_as_type_upper_camel_case);
                    let postgresql_json_type_tokens_option_to_update_upper_camel_case_token_stream = generate_tokens_update_upper_camel_case_token_stream(postgresql_json_type);
                    let postgresql_type_tokens_to_update_token_stream = {
                        let postgresql_type_tokens_to_update_token_stream = {
                            let pub_struct_postgresql_type_tokens_to_update_declaration_token_stream = {
                                match &postgresql_type {
                                    PostgresqlType::JsonbNullable => quote::quote!{(std::option::Option<#postgresql_json_type_tokens_option_to_update_upper_camel_case_token_stream>);},
                                    PostgresqlType::JsonbNotNull => quote::quote!{(#postgresql_json_type_tokens_option_to_update_upper_camel_case_token_stream);},
                                }
                            };
                            quote::quote!{
                                #[derive(
                                    Debug,
                                    Clone,
                                    PartialEq,
                                    serde::Serialize,
                                    serde::Deserialize,
                                    schemars::JsonSchema,
                                )]
                                pub struct #postgresql_type_tokens_as_type_update_upper_camel_case #pub_struct_postgresql_type_tokens_to_update_declaration_token_stream
                            }
                        };
                        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_to_update_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &postgresql_type_tokens_as_type_update_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &{
                                let value = match &postgresql_type {
                                    PostgresqlType::JsonbNullable => quote::quote!{(Some(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    ))},
                                    PostgresqlType::JsonbNotNull => quote::quote!{(
                                        #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                                    )},
                                };
                                quote::quote!{Self #value}
                            },
                        );
                        quote::quote!{
                            #postgresql_type_tokens_to_update_token_stream
                            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_to_update_token_stream
                        }
                    };
                    let tokens_update_query_part_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type {
                        PostgresqlType::JsonbNullable => &naming::parameter::SelfAsPostgresqlJsonbUpdateQueryPartErrorNamedUpperCamelCase::from_tokens(&tokens_upper_camel_case),
                        PostgresqlType::JsonbNotNull => &naming::parameter::SelfAsPostgresqlJsonbNotNullUpdateQueryPartErrorNamedUpperCamelCase::from_tokens(&tokens_upper_camel_case),
                    };
                    let postgresql_type_tokens_to_update_query_part_error_named_token_stream = {
                        quote::quote!{
                            #[derive(
                                Debug,
                                Clone,
                                // PartialEq,
                                serde::Serialize,
                                serde::Deserialize,
                            )]
                            pub enum #tokens_update_query_part_error_named_upper_camel_case {
                                #checked_add_variant_declaration_token_stream,
                                Todo//todo
                            }
                        }
                    };
                    let impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream = {
                        let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
                        let postgresql_type_self_upper_camel_case = naming::PostgresqlTypeSelfUpperCamelCase;
                        let create_upper_camel_case = naming::CreateUpperCamelCase;
                        let create_query_part_token_stream = {
                            quote::quote! {
                                fn create_query_part(
                                    value: &Self::#create_upper_camel_case,
                                    increment: &mut std::primitive::u64
                                ) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
                                    //todo
                                    postgresql_crud::BindQuery::try_generate_bind_increments(value, increment)
                                }
                            }
                        };
                        let create_query_bind_token_stream = {
                            quote::quote! {
                                fn create_query_bind(
                                    value: Self::Create,
                                    query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                                ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                    //todo
                                    postgresql_crud::BindQuery::bind_value_to_query(value, query)
                                }
                            }
                        };
                        let select_upper_camel_case = naming::SelectUpperCamelCase;
                        let select_query_part_token_stream = {
                            let format_value_token_stream = match &postgresql_type {
                                PostgresqlType::JsonbNullable
                                => quote::quote!{
                                    match &#value_snake_case.0 {
                                        Some(#value_snake_case) => #tokens_as_postgresql_json_type_token_stream field_reader_query_part(value, &column, &column, &column, true),
                                        None => "null".to_string()
                                    }
                                },

                                PostgresqlType::JsonbNotNull
                                => quote::quote!{
                                    #tokens_as_postgresql_json_type_token_stream field_reader_query_part(&#value_snake_case.0, &column, &column, &column, true)
                                },
                            };
                            quote::quote!{
                                fn select_query_part(
                                    #value_snake_case: &Self::#select_upper_camel_case,
                                    column: &std::primitive::str,
                                ) -> std::string::String {
                                    format!(
                                        "{} as {column}",
                                        #format_value_token_stream
                                    )
                                }
                            }
                        };
                        let where_element_upper_camel_case = naming::WhereElementUpperCamelCase;
                        let read_upper_camel_case = naming::ReadUpperCamelCase;
                        let update_upper_camel_case = naming::UpdateUpperCamelCase;
                        let update_query_part_error_named_upper_camel_case = naming::UpdateQueryPartErrorNamedUpperCamelCase;
                        let update_query_part_token_stream = {
                            let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                            //todo remove jsonb_ prefix (coz it can be json, jsonb, json not null, jsonb not null)
                            let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                            let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
                            let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
                            let increment_snake_case = naming::IncrementSnakeCase;
                            let postgresql_type_self_to_update_query_part_content_token_stream = {
                                let generate_ok_try_generate_postgresql_json_type_to_update_token_stream = |is_postgresql_type_self_to_update_zero: std::primitive::bool|{
                                    let first_argument_token_stream: &dyn quote::ToTokens = if is_postgresql_type_self_to_update_zero {
                                        &quote::quote!{&#value_snake_case.0}
                                    }
                                    else {
                                        &value_snake_case
                                    };
                                    //todo proper error handling - remove .unwrap()
                                    quote::quote!{
                                        Ok(#tokens_as_postgresql_json_type_token_stream update_query_part(
                                            #first_argument_token_stream,
                                            #jsonb_set_accumulator_snake_case,
                                            #jsonb_set_target_snake_case,
                                            #jsonb_set_path_snake_case,
                                            #increment_snake_case,
                                        ).unwrap())
                                    }
                                };
                                match &postgresql_type {
                                    PostgresqlType::JsonbNullable => {
                                        let ok_try_generate_postgresql_json_type_to_update_token_stream = generate_ok_try_generate_postgresql_json_type_to_update_token_stream(false);
                                        quote::quote!{
                                            match &#value_snake_case.0 {
                                                Some(#value_snake_case) => #ok_try_generate_postgresql_json_type_to_update_token_stream,
                                                None => match increment.checked_add(1) {
                                                    Some(#value_snake_case) => {
                                                        *increment = #value_snake_case;
                                                        Ok(format!("${increment}"))
                                                    },
                                                    //todo make generic error type instead of PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed
                                                    None => Err(#tokens_update_query_part_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                                        code_occurence: error_occurence_lib::code_occurence!()
                                                    })
                                                }
                                            }
                                        }
                                    },
                                    PostgresqlType::JsonbNotNull => generate_ok_try_generate_postgresql_json_type_to_update_token_stream(true),
                                }
                            };
                            quote::quote!{
                                fn #update_query_part_snake_case(
                                    //few parameters usefull only with json types. maybe refactor it later
                                    #value_snake_case: &Self::#update_upper_camel_case,
                                    #jsonb_set_accumulator_snake_case: &std::primitive::str,
                                    #jsonb_set_target_snake_case: &std::primitive::str,
                                    #jsonb_set_path_snake_case: &std::primitive::str,
                                    #increment_snake_case: &mut std::primitive::u64
                                ) -> Result<std::string::String, Self::#update_query_part_error_named_upper_camel_case> {
                                    #postgresql_type_self_to_update_query_part_content_token_stream
                                }
                            }
                        };
                        let update_query_bind_token_stream = {
                            let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
                            let self_to_update_bind_query_part_content_token_stream = {
                                let generate_bind_value_to_postgresql_query_part_to_update_token_stream = |is_postgresql_type_self_to_update_zero: std::primitive::bool|{
                                    let first_argument_token_stream: &dyn quote::ToTokens = if is_postgresql_type_self_to_update_zero {
                                        &quote::quote!{#value_snake_case.0}
                                    }
                                    else {
                                        &value_snake_case
                                    };
                                    quote::quote!{
                                        #tokens_as_postgresql_json_type_token_stream update_query_bind(
                                            #first_argument_token_stream,
                                            query
                                        )
                                    }
                                };
                                match &postgresql_type {
                                    PostgresqlType::JsonbNullable => {
                                        let bind_value_to_postgresql_query_part_to_update_token_stream = generate_bind_value_to_postgresql_query_part_to_update_token_stream(false);
                                        quote::quote!{
                                            query = match #value_snake_case.0 {
                                                Some(#value_snake_case) => #bind_value_to_postgresql_query_part_to_update_token_stream,
                                                None => {
                                                    #query_snake_case.bind(sqlx::types::Json(None::<std::option::Option<#postgresql_json_type_tokens_option_to_update_upper_camel_case_token_stream>>))
                                                }
                                            };
                                            query
                                        }
                                    },
                                    PostgresqlType::JsonbNotNull => generate_bind_value_to_postgresql_query_part_to_update_token_stream(true),
                                }
                            };
                            quote::quote!{
                                fn #update_query_bind_snake_case(
                                    #value_snake_case: Self::#update_upper_camel_case,
                                    mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                                ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                    #self_to_update_bind_query_part_content_token_stream
                                }
                            }
                        };
                        quote::quote!{
                            impl postgresql_crud::postgresql_type::postgresql_type_trait::#postgresql_type_upper_camel_case for #tokens_as_type_upper_camel_case {
                                type #postgresql_type_self_upper_camel_case = #tokens_as_type_upper_camel_case;
                                type #create_upper_camel_case = #tokens_as_type_create_upper_camel_case;
                                #create_query_part_token_stream
                                #create_query_bind_token_stream
                                type #select_upper_camel_case = #tokens_as_type_select_upper_camel_case;
                                #select_query_part_token_stream
                                type #where_element_upper_camel_case = #tokens_as_type_where_element_upper_camel_case;
                                type #read_upper_camel_case = #postgresql_type_tokens_as_type_read_upper_camel_case;
                                type #update_upper_camel_case = #postgresql_type_tokens_as_type_update_upper_camel_case;
                                type #update_query_part_error_named_upper_camel_case = #tokens_update_query_part_error_named_upper_camel_case;
                                #update_query_part_token_stream
                                #update_query_bind_token_stream
                            }
                        }
                    };
                    quote::quote!{
                        #tokens_tokens_stream
                        #tokens_select_token_stream
                        #tokens_create_token_stream
                        #postgresql_type_tokens_to_read_token_stream
                        #postgresql_type_tokens_to_update_token_stream
                        #postgresql_type_tokens_to_update_query_part_error_named_token_stream
                        #tokens_where_element_token_stream
                        #impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream
                    }
                };
                // let postgresql_json_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_as_postgresql_jsonb_nullable_token_stream = generate_postgresql_json_types_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream(PostgresqlType::JsonbNullable);
                let postgresql_json_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_as_postgresql_jsonb_not_null_token_stream = generate_postgresql_json_types_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream(PostgresqlType::JsonbNotNull);
                quote::quote!{
                    //todo return it
                    // #postgresql_json_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_as_postgresql_jsonb_nullable_token_stream
                    #postgresql_json_impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_as_postgresql_jsonb_not_null_token_stream
                }
            };
            let generate_postgresql_crud_temporary_handle = if need_to_generate_postgresql_crud_logic {
                impl_postgresql_crud_postgresql_types_postgresql_type_postgresql_type_for_tokens_token_stream
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote!{
                #impl_postgresql_crud_postgresql_json_type_for_tokens_ident_token_stream
                #generate_postgresql_crud_temporary_handle
            }
        };
        let postgresql_type_traits_object_ident_token_stream = generate_postgresql_type_traits_token_stream(&PostgresqlJsonType::Object, true);
        // let postgresql_type_traits_std_option_option_object_ident_token_stream = generate_postgresql_type_traits_token_stream(&PostgresqlJsonType::StdOptionOptionObject, false);
        // let postgresql_type_traits_std_vec_vec_object_with_id_ident_token_stream = generate_postgresql_type_traits_token_stream(&PostgresqlJsonType::StdVecVecObjectWithId, false);
        // let postgresql_type_traits_std_option_option_std_vec_vec_object_with_id_ident_token_stream = generate_postgresql_type_traits_token_stream(&PostgresqlJsonType::StdOptionOptionStdVecVecObjectWithId, false);
        quote::quote!{
            #postgresql_type_traits_object_ident_token_stream

            //todo return it
            // #postgresql_type_traits_std_option_option_object_ident_token_stream
            // #postgresql_type_traits_std_vec_vec_object_with_id_ident_token_stream
            // #postgresql_type_traits_std_option_option_std_vec_vec_object_with_id_ident_token_stream
        }
    };
    let generated = quote::quote! {
        #common_token_stream

        #ident_token_stream
        //todo return it later
        // #object_with_id_ident_token_stream

        #postgresql_type_traits_token_stream
    };
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlJsonType",
        //     &generated,
        // );
    //     // quote::quote!{}.into()
    // }
    // // else {
    // //     generated.into()
    // // }
    generated.into()
}
//todo impl custom serde::Deserialize for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhere