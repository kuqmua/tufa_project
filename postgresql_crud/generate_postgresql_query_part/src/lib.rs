//todo maybe generate example of valid json to create - maybe with serde_json::to_string() adn #[derive(Default)} then println or write into file
//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
#[proc_macro_derive(GeneratePostgresqlQueryPart)]
pub fn generate_postgresql_query_part(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlQueryPart";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().map(|element| element).collect::<std::vec::Vec<&syn::Field>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let ident_options_to_read_upper_camel_case = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_field_to_update_upper_camel_case = naming_conventions::SelfFieldToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let add_postfix_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_stringified = |value: &std::primitive::str| format!("{value}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecErrorNamedUpperCamelCase);
    let ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = {
        let value = add_postfix_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_stringified(&ident.to_string());
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generic_ident_upper_camel_case = naming_conventions::GenericSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    fn generate_impl_std_fmt_display_for_tokens_token_stream(value_token_stream: &impl quote::ToTokens) -> proc_macro2::TokenStream {
        quote::quote!{
            impl std::fmt::Display for #value_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", &self)
                }
            }
        }
    }
    
    let generate_template_field_to_read_struct_token_stream = |
        tokens_field_to_read_token_stream: &dyn quote::ToTokens,
        additional_content_token_stream: &dyn quote::ToTokens,
    |{
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident_stringified = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                })
                .to_string();
            let serialize_deserialize_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let type_path_field_reader_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::FieldReaderUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
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

    let ident_field_to_read_upper_camel_case = naming_conventions::SelfFieldToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_with_id_field_to_read_upper_camel_case = naming_conventions::SelfWithIdFieldToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);

    let generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream = |value: &dyn quote::ToTokens|{
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #value {
                fn to_std_string_string(&self) -> std::string::String {
                    format!("{self:?}")
                }
            }
        }
    };

    let postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()};

    let (
        fields_some_value_self_options_to_read_initialization_content_token_stream,
        fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
    ) = {
        let generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = |
            field_ident: &syn::Ident,
        |{
            quote::quote!{
                #field_ident: Some(postgresql_crud::Value { 
                    value: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                })
            }
        };
        let fields_some_value_self_options_to_read_initialization_content_token_stream = {
            let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                &element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                })
            ));
            quote::quote!{{#(#fields_token_stream),*}}
        };
        let fields_with_id_some_value_self_options_to_read_initialization_content_token_stream = {
            let fields_token_stream = {
                let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                    &element
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                ));
                let id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                    &syn::Ident::new(&naming_conventions::IdSnakeCase.to_string(), ident.span()),
                );
                quote::quote!{
                    #id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    #(#fields_token_stream),*
                }
            };
            quote::quote!{{ #fields_token_stream }}
        };
        (
            fields_some_value_self_options_to_read_initialization_content_token_stream,
            fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
        )
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream = |
        struct_ident_token_stream: &dyn quote::ToTokens,
        self_initialization_content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #struct_ident_token_stream {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self #self_initialization_content_token_stream
                }
            }
        }
    };

    let ident_field_to_read_token_stream = generate_template_field_to_read_struct_token_stream(
        &ident_field_to_read_upper_camel_case,
        &proc_macro2::TokenStream::new(),
    );
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_field_to_read_upper_camel_case);
    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream = {
        let elements_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident = &element.ident.as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let field_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
            quote::quote!{
                #ident_field_to_read_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
            }
        });
        quote::quote!{
            impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_field_to_read_upper_camel_case {
                fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                    vec![
                        #(#elements_token_stream),*
                    ]
                }
            }
        }
    };

    let ident_with_id_field_to_read_token_stream = generate_template_field_to_read_struct_token_stream(
        &ident_with_id_field_to_read_upper_camel_case,
        &quote::quote!{
            #[serde(rename(serialize = "id", deserialize = "id"))]
             Id(postgresql_crud::JsonUuidFieldReader),
        },
    );
    let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_with_id_field_to_read_upper_camel_case);
    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream = {
        let elements_token_stream = {
            let elements_token_stream = vec_syn_field.iter().map(|element|{
                let field_ident = &element.ident.as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    });
                let field_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
                quote::quote!{
                    #ident_with_id_field_to_read_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                }
            });
            quote::quote!{
                #ident_with_id_field_to_read_upper_camel_case::Id(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream),
                #(#elements_token_stream),*
            }
        };
        quote::quote!{
            impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_with_id_field_to_read_upper_camel_case {
                fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                    vec![#elements_token_stream]
                }
            }
        }
    };

    let ident_field_to_update_token_stream = {
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
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
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
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

    fn generate_supported_generics_template_struct_token_stream(struct_ident_token_stream: &(impl quote::ToTokens + ?Sized), content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #struct_ident_token_stream #content_token_stream
        }
    }

    let fields_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_type = &element.ty;
            quote::quote!{pub #element_ident: #element_type}
        });
        quote::quote!{#(#fields_token_stream),*}
    };

    let (
        generate_tuple_struct_tokens_double_quotes_token_stream,
        generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream
    ) = {
        const TUPLE_STRUCT_SPACE_STRINGIFIED: &std::primitive::str = "tuple struct ";
        let generate_tuple_struct_tokens_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        };
        let generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{TUPLE_STRUCT_SPACE_STRINGIFIED}{value} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        };
        (
            generate_tuple_struct_tokens_double_quotes_token_stream,
            generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream
        )
    };
    //
    let postgersql_crud_pagination_token_stream = quote::quote!{postgresql_crud::Pagination};

    enum FieldReaderContent {
        GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent,
        GenericIdentAndStdOptionOptionGenericIdent,
        StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
    }
    
    let generate_tokens_field_reader_token_stream = |
        struct_prefix_stringified: &dyn std::fmt::Display, 
        field_reader_content: &FieldReaderContent
    |{
        let struct_ident = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_std_fmt_display(&struct_prefix_stringified);
        let std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream = quote::quote!{std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>};
        let std_vec_vec_ident_field_to_read_upper_camel_case_token_stream = quote::quote!{std::vec::Vec<#ident_field_to_read_upper_camel_case>};
        let field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream = quote::quote!{field_vec: std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>, pagination: #postgersql_crud_pagination_token_stream};
        let content_token_stream = match &field_reader_content {
            FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent => quote::quote!{(#std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream);},
            FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent => quote::quote!{(#std_vec_vec_ident_field_to_read_upper_camel_case_token_stream);},
            FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent => quote::quote!{
                {
                    #field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream
                }
            },
        };
        let struct_prefix_try_new_error_named_upper_camel_case = naming_conventions::SelfTryNewErrorNamedUpperCamelCase::from_dyn_std_fmt_display(&struct_prefix_stringified);
        let fields_filter_is_empty_upper_camel_case = naming_conventions::FieldsFilterIsEmptyUpperCamelCase;
        let not_unique_field_filter_upper_camel_case = naming_conventions::NotUniqueFieldFilterUpperCamelCase;
        let value_snake_case = naming_conventions::ValueSnakeCase;
        let generate_impl_pub_fn_try_new_token_stream = |
            contains_id: std::primitive::bool,
            input_parameters_token_stream: &dyn quote::ToTokens,
            is_vec: std::primitive::bool,
        |{
            let field_type_token_stream = if contains_id {
                quote::quote!{#ident_with_id_field_to_read_upper_camel_case}
            }
            else {
                quote::quote!{#ident_field_to_read_upper_camel_case}
            };
            
            let field_vec_snake_case = naming_conventions::FieldVecSnakeCase;
            let pagination_snake_case = naming_conventions::PaginationSnakeCase;
            let check_handle_token_stream = if is_vec {
                quote::quote!{#field_vec_snake_case}
            }
            else {
                quote::quote!{#value_snake_case}
            };
            let unique_snake_case = naming_conventions::UniqueSnakeCase;
            let self_initialization_token_stream = if is_vec {
                quote::quote!{{ #field_vec_snake_case: #unique_snake_case, #pagination_snake_case }}
            }
            else {
                quote::quote!{(#unique_snake_case)}
            };
            quote::quote!{
                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #struct_prefix_try_new_error_named_upper_camel_case {
                    #fields_filter_is_empty_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #not_unique_field_filter_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #field_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
                impl #struct_ident {
                    pub fn try_new(#input_parameters_token_stream) -> Result<Self, #struct_prefix_try_new_error_named_upper_camel_case> {
                        if #check_handle_token_stream.is_empty() {
                            return Err(#struct_prefix_try_new_error_named_upper_camel_case::#fields_filter_is_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                        let mut #unique_snake_case = vec![];
                        for element in #check_handle_token_stream {
                            if #unique_snake_case.contains(&element) {
                                return Err(#struct_prefix_try_new_error_named_upper_camel_case::#not_unique_field_filter_upper_camel_case {
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
            match &field_reader_content {
                FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent => {
                    generate_impl_pub_fn_try_new_token_stream(
                        true,
                        &generate_value_input_parameter_type_token_stream(&std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream),
                        false,
                    )
                },
                FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent => {
                    generate_impl_pub_fn_try_new_token_stream(
                        false,
                        &generate_value_input_parameter_type_token_stream(&std_vec_vec_ident_field_to_read_upper_camel_case_token_stream),
                        false,
                    )
                },
                FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent => {
                    generate_impl_pub_fn_try_new_token_stream(
                        true,
                        &field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream,
                        true,
                    )
                },
            }
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #struct_ident #content_token_stream
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
    //
    let generate_tokens_options_to_read_token_stream = |
        token_options_to_read_token_stream: &dyn quote::ToTokens,
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
            #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_impl_serde_deserialize_token_stream utoipa::ToSchema)]
            pub struct #token_options_to_read_token_stream #content_token_stream
        }
    };
    //
    let generate_struct_tokens_options_to_read_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool|{
        generate_tokens_options_to_read_token_stream(
            &struct_ident_token_stream,
            false,
            &{
                let maybe_id_token_stream = if contains_id {
                    quote::quote!{
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonUuidOptionsToRead>>,
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
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        });
                    let type_path_options_to_read_token_stream = {
                        let value = format!(
                            "{}{}",
                            {
                                let type_path = &element.ty;
                                quote::quote!{#type_path}.to_string()
                            },
                            naming_conventions::OptionsToReadUpperCamelCase
                        );
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{
                        #[serde(skip_serializing_if = "Option::is_none")]
                        #field_ident: std::option::Option<postgresql_crud::Value<#type_path_options_to_read_token_stream>>
                    }
                });
                quote::quote!{{
                    #maybe_id_token_stream
                    #(#fields_token_stream),*
                }}
            },
        )
    };

    let generate_impl_serde_deserialize_for_options_to_read_origin_token_stream = |
        struct_ident_stringified: &dyn std::fmt::Display,
        struct_ident_token_stream: &proc_macro2::TokenStream,
        contains_id: std::primitive::bool,
    |{
        let range_end = {
            let vec_syn_field_len = vec_syn_field.len();
            if contains_id {
                vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} vec_syn_field_len + 1 is None(int overflow)"))
            }
            else {
                vec_syn_field_len
            }
        };
        let field_enum_variants_token_stream = {
            let mut vec = vec![];
            for element in 0..range_end {
                let value = format!("__{}{element}", naming_conventions::FieldSnakeCase);
                vec.push(
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                );
            }
            vec
        };
        let generate_field_index_token_stream = |index: std::primitive::usize| {
            let value = format!("__field{index}");
            value
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let visit_u64_value_enum_variants_token_stream = {
            let mut acc = vec![];
            for index in 0..range_end {
                let index_u64_token_stream = {
                    let value = format!("{index}u64");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
                index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} vec_syn_field_len + 1 is None(int overflow)"))
            }
            else {
                index
            }
        };
        let visit_str_value_enum_variants_token_stream = {
            let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_name_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &{
                        element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string()
                    },
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &field_name_double_quotes_token_stream,
                    index,
                )
            });
            let maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &quote::quote!{"id"},
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
                    let element_ident_double_quotes_stringified = proc_macro_common::generate_quotes::double_quotes_stringified(
                        &element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string(),
                    );
                    let value = format!("b{element_ident_double_quotes_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &b_field_name_double_quotes_token_stream,
                    index,
                )
            });
            let maybe_b_field_ident_double_quotes_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &quote::quote!{b"id"},
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
        let struct_ident_options_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case}"), &proc_macro_name_upper_camel_case_ident_stringified);
        let struct_ident_options_with_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("struct {ident_options_to_read_upper_camel_case} with {range_end} elements"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let visit_seq_fields_initialization_token_stream = {
            let generate_serde_de_seq_access_next_element_token_stream = |
                index: std::primitive::usize,
                type_options_to_read_token_stream: &dyn quote::ToTokens,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    let #field_index_token_stream = match serde::de::SeqAccess::next_element::<
                        std::option::Option<postgresql_crud::Value<#type_options_to_read_token_stream>>,
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
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_serde_de_seq_access_next_element_token_stream(
                    index,
                    &type_options_to_read_token_stream,
                )
            });
            let maybe_id_serde_de_seq_access_next_element_token_stream = if contains_id {
                generate_serde_de_seq_access_next_element_token_stream(
                    0,
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
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
        let if_let_nones_fields_serde_custom_error_token_stream = {
            let nones_token_stream = {
                let mut acc = vec![];
                for _ in 0..range_end {
                    acc.push(quote::quote!{None});
                }
                acc
            };
            let fields_token_stream = {
                let mut acc = vec![];
                for element in 0..range_end {
                    let field_index_token_stream = generate_field_index_token_stream(element);
                    acc.push(quote::quote!{&#field_index_token_stream});
                }
                acc
            };
            let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("custom serde error deserializing {struct_ident_token_stream}: all fields are None"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            quote::quote!{
                if let (#(#nones_token_stream),*) = (#(#fields_token_stream),*) {
                    return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                }
            }
        };
        let visit_seq_fields_assignment_token_stream = {
            let generate_field_ident_field_index_token_stream = |
                field_ident: &dyn quote::ToTokens,
                index: std::primitive::usize,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    #field_ident: #field_index_token_stream
                }
            };
            let visit_seq_fields_assignment_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
                generate_field_ident_field_index_token_stream(
                    &quote::quote!{#field_ident},
                    index,
                )
            });
            let maybe_id_field_ident_field_index_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_field_index_token_stream(
                    &quote::quote!{id},
                    0,
                );
                quote::quote!{#value_token_stream,}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_field_ident_field_index_token_stream
                #(#visit_seq_fields_assignment_token_stream),*
            }
        };
        let visit_map_fields_initialization_token_stream = {
            let generate_mut_field_index_serde_private_option_token_stream = |
                index: std::primitive::usize,
                type_token_stream: &dyn quote::ToTokens,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    let mut #field_index_token_stream: serde::__private::Option<
                        std::option::Option<postgresql_crud::Value<#type_token_stream>>,
                    > = serde::__private::None;
                }
            };
            let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_mut_field_index_serde_private_option_token_stream(
                    index,
                    &type_options_to_read_token_stream,
                )
            });
            let maybe_id_mut_field_index_serde_private_option_token_stream = if contains_id {
                generate_mut_field_index_serde_private_option_token_stream(
                    0,
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
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
        let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &value
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                    .to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            )
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
                                std::option::Option<postgresql_crud::Value<#type_token_stream>>,
                            >(&mut __map)?,
                        );
                    }
                }
            };
            let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_field_initialization_token_stream(
                    index,
                    &field_ident_double_quotes_token_stream,
                    &type_options_to_read_token_stream,
                )
            });
            let id_field_initialization_token_stream = if contains_id {
                generate_field_initialization_token_stream(
                    0,
                    &quote::quote!{"id"},
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
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
                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
                generate_missing_field_token_stream(
                    index,
                    &field_ident_double_quotes_token_stream
                )
            });
            let maybe_id_missing_field_token_stream = if contains_id {
                generate_missing_field_token_stream(
                    0,
                    &quote::quote!{"id"}
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
            let fields_array_elements_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(&element));
            let maybe_id_double_quotes_comma_token_stream = if contains_id {
                quote::quote! {"id",}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_double_quotes_comma_token_stream
                #(#fields_array_elements_token_stream),*
            }
        };
        let std_vec_vec_generic_with_id_ident_options_to_read_origin_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &struct_ident_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
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
                            #if_let_nones_fields_serde_custom_error_token_stream
                            serde::__private::Ok(#struct_ident_token_stream {
                                #visit_seq_fields_assignment_token_stream
                            })
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
                            #if_let_nones_fields_serde_custom_error_token_stream
                            serde::__private::Ok(#struct_ident_token_stream {
                                #visit_seq_fields_assignment_token_stream
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[#fields_array_elements_token_stream];
                    serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #std_vec_vec_generic_with_id_ident_options_to_read_origin_double_quotes_token_stream,
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
    
    let ident_options_to_read_without_id_upper_camel_case = naming_conventions::SelfOptionsToReadWithoutIdUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_options_to_read_with_id_upper_camel_case = naming_conventions::SelfOptionsToReadWithIdUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_options_to_read_without_id_token_stream = generate_struct_tokens_options_to_read_token_stream(&ident_options_to_read_without_id_upper_camel_case, false);
    let ident_options_to_read_with_id_token_stream = generate_struct_tokens_options_to_read_token_stream(&ident_options_to_read_with_id_upper_camel_case, true);
    //
    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_without_id_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
        &ident_options_to_read_without_id_upper_camel_case,
        &fields_some_value_self_options_to_read_initialization_content_token_stream,
    );
    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_with_id_token_stream =     generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
        &ident_options_to_read_with_id_upper_camel_case,
        &fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
    );
    ///////
    //todo remove quote
    let impl_serde_deserialize_for_ident_options_to_read_without_id_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
        &ident_options_to_read_without_id_upper_camel_case,
        &quote::quote!{#ident_options_to_read_without_id_upper_camel_case},
        false,
    );
    let impl_serde_deserialize_for_ident_options_to_read_with_id_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
        &ident_options_to_read_with_id_upper_camel_case,
        &quote::quote!{#ident_options_to_read_with_id_upper_camel_case},
        true,
    );
    let generate_options_to_read_alias_token_stream = |tokens_options_to_read_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool|{
        let options_to_read_with_or_without_id_token_stream = if contains_id {
            quote::quote!{#ident_options_to_read_with_id_upper_camel_case}
        }
        else {
            quote::quote!{#ident_options_to_read_without_id_upper_camel_case}
        };
        quote::quote!{pub type #tokens_options_to_read_token_stream = #options_to_read_with_or_without_id_token_stream;}
    };
    ///////
    //
    let generate_tokens_to_create_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens|{
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let type_path_to_create_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::ToCreateUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #field_ident: #type_path_to_create_token_stream
            }
        });
        generate_supported_generics_template_struct_token_stream(
            struct_ident_token_stream,
            &quote::quote!{{ #(#fields_token_stream),*}}
        )
    };

    let postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream = |tokens_field_reader_token_stream: &dyn quote::ToTokens|{
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &tokens_field_reader_token_stream,
            &quote::quote!{(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
        )
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream = |tokens_field_reader_upper_camel_case: &dyn quote::ToTokens|{
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &tokens_field_reader_upper_camel_case,
            &quote::quote!{
                {
                    field_vec: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    pagination: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                }
            },
        )
    };

    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens|{
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            quote::quote!{
                #field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            }
        });
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &struct_ident_token_stream,
            &quote::quote!{{#(#fields_token_stream),*}}
        )
    };
    let generate_impl_postgresql_crud_json_create_bind_query_for_tokens_token_stream = |
        struct_ident_token_stream: &dyn quote::ToTokens,
        json_create_try_generate_bind_increments_content_token_stream: &dyn quote::ToTokens,
        json_create_bind_value_to_query_content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl<'a> postgresql_crud::JsonCreateBindQuery<'a> for #struct_ident_token_stream {
                fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed> {
                    #json_create_try_generate_bind_increments_content_token_stream
                }
                fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #json_create_bind_value_to_query_content_token_stream
                }
            }
        }
    };
    enum CreateBindQueryVariant {
        Generic,
        StdOptionOptionGeneric,
        GenericVecOrigin,
    }
    let generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, contains_id: std::primitive::bool, create_bind_query_variant: CreateBindQueryVariant|{
        generate_impl_postgresql_crud_json_create_bind_query_for_tokens_token_stream(
            &struct_ident_token_stream,
            &{
                let increment_initialization_string_content_token_stream = if contains_id {
                    match &create_bind_query_variant {
                        CreateBindQueryVariant::Generic => quote::quote!{"'id', to_jsonb(gen_random_uuid()),"},
                        CreateBindQueryVariant::StdOptionOptionGeneric => quote::quote!{"'id', to_jsonb(gen_random_uuid()),"},
                        CreateBindQueryVariant::GenericVecOrigin => quote::quote!{"jsonb_build_object('id', to_jsonb(gen_random_uuid()))||"},
                    }
                }
                else {
                    quote::quote!{""}
                };
                let json_create_try_generate_bind_increments_fields_token_stream = vec_syn_field.iter().map(|element| {
                    let element_field_ident = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        });
                    let element_field_ident_value_comma_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &match &create_bind_query_variant {
                            CreateBindQueryVariant::Generic => format!("jsonb_build_object('{element_field_ident}',{{value}})||"),
                            CreateBindQueryVariant::StdOptionOptionGeneric => format!("jsonb_build_object('{element_field_ident}',{{value}})||"),
                            CreateBindQueryVariant::GenericVecOrigin => format!("jsonb_build_object('{element_field_ident}',{{value}})||"),
                        },
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    //todo maybe wrap into own generic error type
                    quote::quote!{
                        match self.#element_field_ident.json_create_try_generate_bind_increments(increment) {
                            Ok(value) => {
                                increments.push_str(&format!(#element_field_ident_value_comma_double_quotes_token_stream));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    }
                });
                let maybe_additional_pop_token_stream = match &create_bind_query_variant {
                    CreateBindQueryVariant::Generic => quote::quote!{let _ = increments.pop();},
                    CreateBindQueryVariant::StdOptionOptionGeneric => quote::quote!{let _ = increments.pop();},
                    CreateBindQueryVariant::GenericVecOrigin => quote::quote!{let _ = increments.pop();},
                };
                let format_handle_token_stream = match &create_bind_query_variant {
                    CreateBindQueryVariant::Generic => quote::quote!{"{increments}"},
                    CreateBindQueryVariant::StdOptionOptionGeneric => quote::quote!{"{increments}"},
                    CreateBindQueryVariant::GenericVecOrigin => quote::quote!{"{increments}"},
                };
                quote::quote!{
                    let mut increments = std::string::String::from(#increment_initialization_string_content_token_stream);
                    #(#json_create_try_generate_bind_increments_fields_token_stream)*
                    let _ = increments.pop();
                    #maybe_additional_pop_token_stream
                    Ok(format!(#format_handle_token_stream))
                }
            },
            &{
                let json_create_bind_value_to_query_fields_token_stream = vec_syn_field.iter().map(|element| {
                    let element_field_ident = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        });
                    quote::quote!{
                        query = self.#element_field_ident.json_create_bind_value_to_query(query);
                    }
                });
                quote::quote!{
                    #(#json_create_bind_value_to_query_fields_token_stream)*
                    query
                }
            },
        )
    };
    let generate_tokens_reader_token_stream = |struct_ident_token_stream: &dyn quote::ToTokens, struct_options_to_read_token_stream: &dyn quote::ToTokens|{
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #struct_ident_token_stream(pub #struct_options_to_read_token_stream);
        }
    };


    let generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream = |
        tokens_field_reader_token_stream: &dyn quote::ToTokens,
        contains_id: std::primitive::bool,
        format_handle_double_quotes_token_stream: &dyn quote::ToTokens,
    |{
        let tokens_field_to_read_upper_camel_case_token_stream = if contains_id {
            quote::quote!{#ident_with_id_field_to_read_upper_camel_case}
        }
        else {
            quote::quote!{#ident_field_to_read_upper_camel_case}
        };
        let generate_acc_push_str_variant_logic_token_stream = |
            variant_name_token_stream: &dyn quote::ToTokens,
            field_ident_double_quotes_token_stream: &dyn quote::ToTokens,
            column_name_and_maybe_field_getter_token_stream: &dyn quote::ToTokens,
        |{
            quote::quote!{
                #tokens_field_to_read_upper_camel_case_token_stream::#variant_name_token_stream(value) => {
                    acc.push_str(&format!(
                        "{}||",
                        postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
                            value,
                            #field_ident_double_quotes_token_stream,
                            #column_name_and_maybe_field_getter_token_stream,
                            &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
                        )
                    ));
                }
            }
        };
        let value_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&naming_conventions::ValueSnakeCase.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident_stringified = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                })
                .to_string();
            generate_acc_push_str_variant_logic_token_stream(
                &proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified),
                &proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified),
                &if contains_id {
                    value_snake_case_double_quotes_token_stream.clone()
                }
                else {
                    quote::quote!{&format!("{column_name_and_maybe_field_getter}->'{field_ident}'")}
                },
            )
        });
        let self_field_vec_token_stream = if contains_id {
            quote::quote!{field_vec}
        }
        else {
            quote::quote!{0}
        };
        let maybe_id_variant_token_stream = if contains_id {
            let id_upper_camel_case = naming_conventions::IdUpperCamelCase;
            let id_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&naming_conventions::IdSnakeCase.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            generate_acc_push_str_variant_logic_token_stream(
                &quote::quote!{#id_upper_camel_case},
                &id_snake_case_double_quotes_token_stream,
                &value_snake_case_double_quotes_token_stream,
            )
        }
        else {
            proc_macro2::TokenStream::new()
        };
        let maybe_pagination_start_end_initialization_token_stream = if contains_id {
            proc_macro_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream()
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for #tokens_field_reader_token_stream {
                fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
                    let mut acc = std::string::String::default();
                    for element in &self.#self_field_vec_token_stream {
                        match element {
                            #maybe_id_variant_token_stream
                            #(#variants_token_stream),*
                        }
                    }
                    let _ = acc.pop();
                    let _ = acc.pop();
                    #maybe_pagination_start_end_initialization_token_stream
                    format!(#format_handle_double_quotes_token_stream)
                }
            }
        }
    };

    let ident_option_to_update_origin_upper_camel_case = naming_conventions::SelfOptionToUpdateOriginUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let (
        generate_struct_tokens_double_quotes_token_stream,
        generate_struct_tokens_with_2_elements_double_quotes_token_stream
    ) = {
        const STRUCT_SPACE_STRINGIFIED: &str = "struct ";
        let generate_struct_tokens_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{STRUCT_SPACE_STRINGIFIED}{value}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        };
        let generate_struct_tokens_with_2_elements_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{STRUCT_SPACE_STRINGIFIED}{value} with 2 elements"),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        };
        (
            generate_struct_tokens_double_quotes_token_stream,
            generate_struct_tokens_with_2_elements_double_quotes_token_stream
        )
    };

    let generate_tokens_option_to_update_token_stream = |
        struct_ident_token_stream: &dyn quote::ToTokens,
        self_type_content_token_stream: &dyn quote::ToTokens,
        impl_deserialize: bool
    |{
        let maybe_impl_deserialize_token_stream = if impl_deserialize {
            quote::quote!{serde::Deserialize,}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_impl_deserialize_token_stream utoipa::ToSchema)]
            pub struct #struct_ident_token_stream(pub #self_type_content_token_stream);
        }
    };

    let generate_custom_serde_error_deserializing_option_to_update_field_is_not_unique_to_update_token_stream = |
        tokens_option_to_update_upper_camel_case: &dyn std::fmt::Display,
        variant_ident_upper_camel_case: &dyn std::fmt::Display,
    |{
        proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("custom serde error deserializing {tokens_option_to_update_upper_camel_case}: {ident_field_to_update_upper_camel_case} variant {variant_ident_upper_camel_case} is not unique to update"),
            &proc_macro_name_upper_camel_case_ident_stringified
        )
    };

    let generate_tokens_try_generate_bind_increments_error_named_token_stream = |
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

    let generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream = |
        struct_token_stream: &dyn quote::ToTokens,
        tokens_try_generate_bind_increments_error_named_upper_camel_case_token_stream: &dyn quote::ToTokens,
        try_generate_bind_increments_content_token_stream: &dyn quote::ToTokens,
        bind_value_to_query_content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<#tokens_try_generate_bind_increments_error_named_upper_camel_case_token_stream> for #struct_token_stream {
                fn try_generate_bind_increments(
                    &self,
                    jsonb_set_accumulator: &std::primitive::str,
                    jsonb_set_target: &std::primitive::str,
                    jsonb_set_path: &std::primitive::str,
                    increment: &mut std::primitive::u64,
                ) -> Result<std::string::String, #tokens_try_generate_bind_increments_error_named_upper_camel_case_token_stream> {
                    #try_generate_bind_increments_content_token_stream
                }
                fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #bind_value_to_query_content_token_stream
                }
            }
        }
    };
    
    let ident_to_create_origin_with_generated_id_upper_camel_case = naming_conventions::SelfToCreateOriginWithGeneratedIdUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    
    
    let ident_to_create_origin_with_generated_id_token_stream = generate_tokens_to_create_token_stream(&ident_to_create_origin_with_generated_id_upper_camel_case);
    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_origin_with_generated_id_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&ident_to_create_origin_with_generated_id_upper_camel_case);
    let impl_postgresql_crud_json_create_bind_query_for_ident_to_create_origin_with_generated_id_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(
        &ident_to_create_origin_with_generated_id_upper_camel_case,
        true,
        CreateBindQueryVariant::GenericVecOrigin
    );
    
    
    let ident_options_to_update_upper_camel_case = naming_conventions::SelfOptionsToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_options_to_update_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
            pub struct #ident_options_to_update_upper_camel_case {
                id: postgresql_crud::JsonUuidOptionToUpdate,
                fields: std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
            }
        }
    };
    let impl_pub_fn_try_new_for_ident_options_to_update_token_stream = {
        let ident_options_to_update_try_new_error_named_upper_camel_case = naming_conventions::SelfOptionsToUpdateTryNewErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
        let fields_are_empty_upper_camel_case = naming_conventions::FieldsAreEmptyUpperCamelCase;
        let pub_enum_tokens_options_to_update_try_new_error_named_token_stream = {
            let variants_token_stream = vec_syn_field.iter().map(|element| {
                let field_ident = element
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    });
                let not_unique_field_self_upper_camel_case_token_stream = naming_conventions::NotUniqueFieldSelfUpperCamelCase::from_dyn_quote_to_tokens(&field_ident);
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
                pub enum #ident_options_to_update_try_new_error_named_upper_camel_case {
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
                        if fields.is_empty() {
                            return Err(#ident_options_to_update_try_new_error_named_upper_camel_case::#fields_are_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                };
                let check_unique_fields_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            });
                        let field_ident_stringified = field_ident.to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("not unique {field_ident_stringified} field"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let not_unique_field_self_upper_camel_case_token_stream = naming_conventions::NotUniqueFieldSelfUpperCamelCase::from_dyn_quote_to_tokens(&field_ident);
                        quote::quote!{
                            #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(_) => {
                                let value = #ident_field_to_update_upper_camel_case::#variant_ident_upper_camel_case_token_stream;
                                if acc.contains(&value) {
                                    return Err(#ident_options_to_update_try_new_error_named_upper_camel_case::#not_unique_field_self_upper_camel_case_token_stream {
                                        error: format!(#format_handle_double_quotes_token_stream),
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
                            for element in &fields {
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
                impl #ident_options_to_update_upper_camel_case {
                    pub fn try_new(
                        id: postgresql_crud::JsonUuidOptionToUpdate,
                        fields: std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                    ) -> Result<Self, #ident_options_to_update_try_new_error_named_upper_camel_case> {
                        #custom_checks_token_stream
                        Ok(Self {
                            id,
                            fields
                        })
                    }
                }
            }
        };
        quote::quote!{
            #pub_enum_tokens_options_to_update_try_new_error_named_token_stream
            #impl_pub_fn_try_new_token_stream
        }
    };
    let impl_serde_deserialize_for_ident_options_to_update_token_stream = {
        let struct_std_vec_vec_generic_with_id_ident_options_to_update_double_quotes_token_stream = generate_struct_tokens_double_quotes_token_stream(&ident_options_to_update_upper_camel_case);
        let struct_std_vec_vec_generic_with_id_ident_options_to_update_with_2_elements_double_quotes_token_stream = generate_struct_tokens_with_2_elements_double_quotes_token_stream(&ident_options_to_update_upper_camel_case);
        let std_vec_vec_generic_with_id_ident_options_to_update_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &ident_options_to_update_upper_camel_case,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let try_new_token_stream = quote::quote!{
            match #ident_options_to_update_upper_camel_case::try_new(__field0, __field1) {
                Ok(value) => serde::__private::Ok(value),
                Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
            }
        };
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_options_to_update_upper_camel_case {
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
                                "id" => serde::__private::Ok(__Field::__field0),
                                "fields" => serde::__private::Ok(__Field::__field1),
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
                                b"id" => serde::__private::Ok(__Field::__field0),
                                b"fields" => serde::__private::Ok(__Field::__field1),
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
                            #ident_options_to_update_upper_camel_case,
                        >,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_options_to_update_upper_camel_case;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_std_vec_vec_generic_with_id_ident_options_to_update_double_quotes_token_stream,
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
                                postgresql_crud::JsonUuidOptionToUpdate,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &#struct_std_vec_vec_generic_with_id_ident_options_to_update_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            let __field1 = match serde::de::SeqAccess::next_element::<
                                std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            1usize,
                                            &#struct_std_vec_vec_generic_with_id_ident_options_to_update_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            #try_new_token_stream
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
                                postgresql_crud::JsonUuidOptionToUpdate,
                            > = serde::__private::None;
                            let mut __field1: serde::__private::Option<
                                std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                            > = serde::__private::None;
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if serde::__private::Option::is_some(&__field0) {
                                            return serde::__private::Err(
                                                <__A::Error as serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = serde::__private::Some(
                                            serde::de::MapAccess::next_value::<
                                                postgresql_crud::JsonUuidOptionToUpdate,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if serde::__private::Option::is_some(&__field1) {
                                            return serde::__private::Err(
                                                <__A::Error as serde::de::Error>::duplicate_field("fields"),
                                            );
                                        }
                                        __field1 = serde::__private::Some(
                                            serde::de::MapAccess::next_value::<
                                                std::vec::Vec<
                                                    #ident_option_to_update_origin_upper_camel_case,
                                                >,
                                            >(&mut __map)?,
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
                                    serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                serde::__private::Some(__field1) => __field1,
                                serde::__private::None => {
                                    serde::__private::de::missing_field("fields")?
                                }
                            };
                            #try_new_token_stream
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "fields"];
                    serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #std_vec_vec_generic_with_id_ident_options_to_update_upper_camel_case_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: serde::__private::PhantomData::<
                                #ident_options_to_update_upper_camel_case,
                            >,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
        &ident_options_to_update_upper_camel_case,
        &quote::quote!{{
            id: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
            fields: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
        }}
    );

    let (
        checked_add_variant_declaration_token_stream,
        checked_add_variant_initialization_token_stream
    ) = {
        let checked_add_upper_camel_case = naming_conventions::CheckedAddUpperCamelCase;
        let code_occurence_snake_case = naming_conventions::CodeOccurenceSnakeCase;
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


    let ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::SelfJsonArrayChangeTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_json_array_change_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
        &ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case,
        &{
            let variants_token_stream = vec_syn_field.iter().map(|element| {
                let field_ident_stringified = element
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                    .to_string();
                let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                let element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
                    let value = format!(
                        "{}{}",
                        {
                            let element_type = &element.ty;
                            quote::quote!{#element_type}
                        },
                        naming_conventions::OptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase
                    );
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    #variant_ident_upper_camel_case_token_stream {
                        #[eo_error_occurence]
                        error: #element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            });
            quote::quote!{
                #checked_add_variant_declaration_token_stream,
                Create {
                    #[eo_error_occurence]
                    error: postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #(#variants_token_stream),*
            }
        }
    );
    let generate_ident_json_array_change_token_stream = |
        struct_ident_token_stream: &proc_macro2::TokenStream,
        struct_ident_try_new_error_named: &dyn quote::ToTokens,
        is_nullable: std::primitive::bool,
    |{
        let ident_json_array_change_token_stream = quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema)]
            pub struct #struct_ident_token_stream {
                #[serde(skip_serializing_if = "Vec::is_empty")]
                create: std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>,
                #[serde(skip_serializing_if = "Vec::is_empty")]
                update: std::vec::Vec<#ident_options_to_update_upper_camel_case>,
                #[serde(skip_serializing_if = "Vec::is_empty")]
                delete: std::vec::Vec<postgresql_crud::JsonUuidOptionToUpdate>,
            }
        };
        let custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified = format!("custom serde error deserializing {struct_ident_token_stream}");
        let impl_try_new_for_ident_json_array_change_token_stream = {
            let create_update_delete_check_fields_are_empty_upper_camel_case = naming_conventions::CreateUpdateDeleteCheckFieldsAreEmptyUpperCamelCase;
            let not_unique_id_in_json_update_array_upper_camel_case = naming_conventions::NotUniqueIdInJsonUpdateArrayUpperCamelCase;
            let not_unique_id_in_json_delete_array_upper_camel_case = naming_conventions::NotUniqueIdInJsonDeleteArrayUpperCamelCase;
            let not_unique_id_in_json_update_and_delete_arrays_upper_camel_case = naming_conventions::NotUniqueIdInJsonUpdateAndDeleteArraysUpperCamelCase;
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
                        if create.is_empty() && update.is_empty() && delete.is_empty() {
                            return Err(#struct_ident_try_new_error_named::#create_update_delete_check_fields_are_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                    }
                };
                let check_not_unique_id_token_stream = {
                    let check_not_unique_id_in_update_array_token_stream = {
                        let not_unique_id_in_json_update_array_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique id in json update array: {{}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            let update_acc = {
                                let mut update_acc = vec![];
                                for element in &update {
                                    let id = &element.id;
                                    if update_acc.contains(&id) {
                                        return Err(#struct_ident_try_new_error_named::#not_unique_id_in_json_update_array_upper_camel_case {
                                            error: format!(#not_unique_id_in_json_update_array_double_quotes_token_stream, id.0),
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    } else {
                                        update_acc.push(id);
                                    }
                                }
                                update_acc
                            };
                        }
                    };
                    let check_not_unique_id_in_delete_aray_token_stream = {
                        let not_unique_id_in_json_delete_array_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique id in json delete array: {{}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
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
                        let not_unique_id_in_json_update_and_delete_arrays_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{custom_serde_error_deserializing_tokens_json_array_change_upper_camel_case_token_stream_stringified}: not unique id in json update and delete arrays: {{}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
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
                            create: std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>,
                            update: std::vec::Vec<#ident_options_to_update_upper_camel_case>,
                            delete: std::vec::Vec<postgresql_crud::JsonUuidOptionToUpdate>,
                        ) -> Result<Self, #struct_ident_try_new_error_named> {
                            #maybe_check_create_update_delete_check_fields_are_empty_token_stream
                            #check_not_unique_id_token_stream
                            Ok(Self {
                                create,
                                update,
                                delete
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
            let struct_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &struct_ident_token_stream.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let try_new_token_stream = quote::quote!{
                match #struct_ident_token_stream::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                }
            };
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
                                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>>(&mut __seq)? {
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
                                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<postgresql_crud::JsonUuidOptionToUpdate>>(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        vec![]
                                    }
                                };
                                #try_new_token_stream
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::MapAccess<'de>,
                            {
                                let mut __field0: serde::__private::Option<std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>> = serde::__private::None;
                                let mut __field1: serde::__private::Option<std::vec::Vec<#ident_options_to_update_upper_camel_case>> = serde::__private::None;
                                let mut __field2: serde::__private::Option<std::vec::Vec<postgresql_crud::JsonUuidOptionToUpdate>> = serde::__private::None;
                                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if serde::__private::Option::is_some(&__field0) {
                                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                                            }
                                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>>(&mut __map)?);
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
                                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<postgresql_crud::JsonUuidOptionToUpdate>>(&mut __map)?);
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
                                #try_new_token_stream
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
        let impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_json_array_change_try_generate_bind_increments_error_named_for_ident_json_array_change_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
            &struct_ident_token_stream,
            &ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case,
            &{
                let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                    let jsonb_set_target_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("{{jsonb_set_target}}->'{field_ident_stringified}'"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &field_ident_stringified,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                            match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                                &value.value,
                                &element_acc,
                                &format!(#jsonb_set_target_field_ident_double_quotes_token_stream),
                                #field_ident_double_quotes_token_stream,
                                increment,
                            ) {
                                Ok(value) => {
                                    element_acc = value;
                                }
                                Err(error) => {
                                    return Err(#ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                    }
                });
                let ok_format_handle_token_stream = if is_nullable {
                    quote::quote!{
                        Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}', case when jsonb_typeof({jsonb_set_target}) = 'array' then case when jsonb_array_length({jsonb_set_target}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}), '[]'::jsonb)) end else '[]'::jsonb end {maybe_jsonb_build_array})"))
                    }
                }
                else {
                    quote::quote!{
                        Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}', case when jsonb_array_length({jsonb_set_target}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}), '[]'::jsonb)) end {maybe_jsonb_build_array})"))
                    }
                };
                quote::quote!{
                    let update_query_part_acc = {
                        let mut element_acc = std::string::String::from("elem");
                        if self.update.is_empty() {
                            element_acc
                        }
                        else {
                            let mut update_query_part_acc = std::string::String::default();
                            for element_handle in &self.update {
                                let id_increment = match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        increment.to_string()
                                    }
                                    None => {
                                        return Err(#ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
                                    }
                                };
                                for element in &element_handle.fields {
                                    match element {
                                        #(#try_generate_bind_increments_variants_token_stream),*
                                    }
                                }
                                update_query_part_acc.push_str(&format!("when elem ->> 'id' = ${id_increment} then {element_acc} "));
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
                                    return Err(#ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
                                }
                            }
                        }
                        delete_query_part_acc
                    };
                    let create_query_part_acc = {
                        let mut create_query_part_acc = std::string::String::default();
                        for element in &self.create {
                            match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(element, increment) {
                                Ok(value) => {
                                    create_query_part_acc.push_str(&format!("{value},"));
                                }
                                Err(error) => {
                                    return Err(#ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case::Create {
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
                let bind_value_to_query_variants_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                    quote::quote!{
                        #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                            query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value, query);
                        }
                    }
                });
                quote::quote!{
                    for element_handle in self.update {
                        query = query.bind(element_handle.id.0.to_string());//postgresql: error returned from database: operator does not exist: text = jsonb
                        for element in element_handle.fields {
                            match element {
                                #(#bind_value_to_query_variants_token_stream),*
                            }
                        }
                    }
                    for element in self.delete {
                        query = query.bind(element.0.to_string());//postgresql: error returned from database: operator does not exist: text <> jsonb
                    }
                    for element in self.create {
                        query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(element, query);
                    }
                    query
                }
            },
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &struct_ident_token_stream,
            &quote::quote!{{
                create: vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream],
                update: vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream],
                delete: vec![::core::default::Default::default()],
            }},
        );
        quote::quote!{
            #ident_json_array_change_token_stream
            #impl_try_new_for_ident_json_array_change_token_stream
            #impl_serde_deserialize_for_ident_json_array_change_token_stream
            #impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_json_array_change_try_generate_bind_increments_error_named_for_ident_json_array_change_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_json_array_change_with_content_token_stream
        }
    };


    let field0_token_stream = quote::quote!{__field0};
    let field0_field1_token_stream = quote::quote!{__field0, __field1};

    let ident_option_to_update_origin_token_stream = {
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let type_path_option_to_update_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::OptionToUpdateUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
            let field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            quote::quote!{
                #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                #variant_ident_upper_camel_case_token_stream(postgresql_crud::Value<#type_path_option_to_update_token_stream>)
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub enum #ident_option_to_update_origin_upper_camel_case {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_origin_token_stream = {
        let elements_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element.ident.as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
            quote::quote!{
                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(postgresql_crud::Value {
                    value: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                })
            }
        });
        quote::quote!{
            impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_option_to_update_origin_upper_camel_case {
                fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<#ident_option_to_update_origin_upper_camel_case> {
                    vec![#(#elements_token_stream),*]
                }
            }
        }
    };

    //its for GeneratePostgresqlCrud
    let ident_token_stream = {
        let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&ident);

        let create_token_stream = {
            let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let ident_to_create_token_stream = generate_tokens_to_create_token_stream(&ident_to_create_upper_camel_case);
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&ident_to_create_upper_camel_case);
            let impl_postgresql_crud_bind_query_for_ident_to_create_token_stream = {
                let try_generate_bind_increments_token_stream = vec_syn_field.iter().map(|element| {
                    let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    });
                    let element_ident_value_comma_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_build_object('{element_ident}',{{value}})||"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(&self.#element_ident, increment) {
                            Ok(value) => {
                                increments.push_str(&format!(#element_ident_value_comma_double_quotes_token_stream));
                            }
                            Err(error) => {
                                return Err(error.into());
                            }
                        }
                    }
                });
                let bind_value_to_query_token_stream = vec_syn_field.iter().map(|element| {
                    let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    });
                    quote::quote!{
                        query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(self.#element_ident, query);
                    }
                });
                quote::quote!{
                    impl<'a> postgresql_crud::BindQuery<'a> for #ident_to_create_upper_camel_case {
                        fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                            todo!()
                        }
                        fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                            let mut increments = std::string::String::from("");
                            #(#try_generate_bind_increments_token_stream)*
                            let _ = increments.pop();
                            let _ = increments.pop();
                            Ok(format!("{increments}"))
                        }
                        fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                            #(#bind_value_to_query_token_stream)*
                            query
                        }
                    }
                }
            };
            quote::quote!{
                #ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
                #impl_postgresql_crud_bind_query_for_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let ident_options_to_read_upper_camel_case = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let ident_options_to_read_alias_token_stream = generate_options_to_read_alias_token_stream(&ident_options_to_read_upper_camel_case, false);

            let ident_reader_token_stream = generate_tokens_reader_token_stream(
                &naming_conventions::SelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &ident_options_to_read_upper_camel_case
            );

            let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &ident.to_string(),
                &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
            );
            let impl_serde_deserialize_for_ident_field_reader_token_stream = {
                let tuple_struct_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&ident_field_reader_upper_camel_case);
                let tuple_struct_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&ident_field_reader_upper_camel_case);
                let ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
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
                                    let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case> = <std::vec::Vec<
                                        #ident_field_to_read_upper_camel_case,
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
                                        std::vec::Vec<#ident_field_to_read_upper_camel_case>,
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

            //this is temporary, todo remove and refactor later
            let impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
                // let ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = {
                //     let value = format!("{ident}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecErrorNamedUpperCamelCase);
                //     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                // };
                let pub_enum_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
                    quote::quote!{
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
                            Todo {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }
                    }
                };
                let impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
                    quote::quote!{
                        impl error_occurence_lib::ToStdStringString for #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
                            fn to_std_string_string(&self) -> std::string::String {
                                format!("{self:?}")
                            }
                        }
                    }
                };
                let impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
                        quote::quote!{
                            #ident_field_to_read_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                acc.push_str(&format!(
                                    "{}||",
                                    postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
                                        value,
                                        #field_ident_double_quotes_token_stream,
                                        column_name_and_maybe_field_getter,
                                        column_name_and_maybe_field_getter_for_error_message
                                    )
                                ));
                            }
                        }
                    });
                    quote::quote!{
                        impl postgresql_crud::GeneratePostgresqlQueryPartToRead<#ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream, ()> for #ident_field_to_read_upper_camel_case {
                            fn generate_postgresql_query_part_to_read_from_self_vec(
                                value: &std::vec::Vec<Self>,
                                column_name_and_maybe_field_getter: &std::primitive::str,
                                column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
                                is_optional: std::primitive::bool
                            ) -> Result<std::string::String, #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream> {
                                let mut acc = std::string::String::default();
                                for element in value {
                                    match element {
                                        #(#variants_token_stream),*
                                    }
                                }
                                let _ = acc.pop();
                                let _ = acc.pop();
                                Ok(format!("{acc}"))
                            }
                            fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, ()> {
                                todo!()
                            }
                        }
                    }
                };
                quote::quote!{
                    #pub_enum_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
                    #impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream
                }
            };
            quote::quote!{
                #ident_options_to_read_alias_token_stream

                #ident_field_reader_token_stream
                #impl_serde_deserialize_for_ident_field_reader_token_stream

                #ident_reader_token_stream

                #impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream
            }
        };
        let update_token_stream = {
            let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            //todo custom deserialize to check unique fields
            let ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &ident_option_to_update_upper_camel_case,
                &quote::quote!{std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>},
                false,
            );
            let impl_serde_deserialize_for_ident_option_to_update_token_stream = {
                let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_ident_option_to_update_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&ident_option_to_update_upper_camel_case);
                let tuple_struct_ident_option_to_update_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&ident_option_to_update_upper_camel_case);
                let ident_option_to_update_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &ident_option_to_update_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let custom_checks_token_stream = {
                    let check_fields_are_empty_token_stream = {
                        let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("custom serde error deserializing {ident_option_to_update_upper_camel_case}: fields are empty"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            if __field0.is_empty() {
                                return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                            }
                        }
                    };
                    let check_unique_fields_token_stream = {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            let format_handle_double_quotes_token_stream = generate_custom_serde_error_deserializing_option_to_update_field_is_not_unique_to_update_token_stream(
                                &ident_option_to_update_upper_camel_case,
                                &variant_ident_upper_camel_case_token_stream,
                            );
                            quote::quote!{
                                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(_) => {
                                    let value = #ident_field_to_update_upper_camel_case::#variant_ident_upper_camel_case_token_stream;
                                    if acc.contains(&value) {
                                        return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
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
                                for element in &__field0 {
                                    match &element {
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
                    impl<'de> serde::Deserialize<'de> for #ident_option_to_update_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#ident_option_to_update_upper_camel_case>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #ident_option_to_update_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_ident_option_to_update_double_quotes_token_stream,
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
                                    let __field0: std::vec::Vec<#ident_option_to_update_origin_upper_camel_case> = <std::vec::Vec<
                                        #ident_option_to_update_origin_upper_camel_case,
                                    > as serde::Deserialize>::deserialize(__e)?;
                                    #custom_checks_token_stream
                                    serde::__private::Ok(#ident_option_to_update_upper_camel_case(__field0))
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
                                        std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_ident_option_to_update_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #custom_checks_token_stream
                                    serde::__private::Ok(#ident_option_to_update_upper_camel_case(__field0))
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #ident_option_to_update_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #ident_option_to_update_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &ident_option_to_update_upper_camel_case,
                &quote::quote!{(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            );

            let ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::SelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let ident_option_to_update_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
                &ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                &{
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
                            let value = format!(
                                "{}{}",
                                {
                                    let element_type = &element.ty;
                                    quote::quote!{#element_type}
                                },
                                naming_conventions::OptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase
                            );
                            value.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            #variant_ident_upper_camel_case_token_stream {
                                #[eo_error_occurence]
                                error: #element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream,
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
            let impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_option_to_update_try_generate_bind_increments_error_named_for_ident_option_to_update_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
                &ident_option_to_update_upper_camel_case,
                &ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                &{
                    let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let jsonb_set_target_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{{jsonb_set_target}}->'{field_ident_stringified}'"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let previous_jsonb_set_path_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{{previous_jsonb_set_path}}{field_ident_stringified}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                                &value.value,
                                &acc,
                                &format!(#jsonb_set_target_field_ident_double_quotes_token_stream),
                                &format!(#previous_jsonb_set_path_field_ident_double_quotes_token_stream),
                                increment,
                            ) {
                                Ok(value) => {
                                    acc = value;
                                }
                                Err(error) => {
                                    return Err(#ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!()
                                    });
                                }
                            }
                        }
                    });
                    quote::quote!{
                        let mut acc = std::string::String::from(jsonb_set_accumulator);
                        let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
                            true => std::string::String::default(),
                            false => format!("{jsonb_set_path},"),
                        };
                        for element in &self.0 {
                            match &element {
                                #(#try_generate_bind_increments_variants_token_stream),*
                            }
                        }
                        Ok(acc)
                    }
                },
                &{
                    let bind_value_to_query_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        quote::quote!{
                            #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value, query);
                            }
                        }
                    });
                    quote::quote!{
                        for element in self.0 {
                            match element {
                                #(#bind_value_to_query_variants_token_stream),*
                            }
                        }
                        query
                    }
                },
            );
            quote::quote!{
                #ident_option_to_update_token_stream
                #impl_serde_deserialize_for_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream
                #ident_option_to_update_try_generate_bind_increments_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_option_to_update_try_generate_bind_increments_error_named_for_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #impl_std_fmt_display_for_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };

    let generic_with_id_ident_upper_camel_case = naming_conventions::GenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    
    //its for GeneratePostgresqlQueryPart (json logic)
    let generic_with_id_ident_token_stream = {
        let generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &generic_with_id_ident_upper_camel_case,
            &{
                quote::quote!{{
                    pub id: postgresql_crud::JsonUuidOptionToUpdate,
                    #fields_token_stream
                }}
            }
        );
        let impl_std_fmt_display_for_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&generic_with_id_ident_upper_camel_case);

        let create_token_stream = {
            let generic_with_id_ident_to_create_upper_camel_case = naming_conventions::GenericWithIdSelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let generic_with_id_ident_to_create_token_stream = generate_tokens_to_create_token_stream(&generic_with_id_ident_to_create_upper_camel_case);
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&generic_with_id_ident_to_create_upper_camel_case);
            quote::quote!{
                #generic_with_id_ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let generic_with_id_ident_options_to_read_upper_camel_case = naming_conventions::GenericWithIdSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let generic_with_id_ident_options_to_read_alias_token_stream = generate_options_to_read_alias_token_stream(&generic_with_id_ident_options_to_read_upper_camel_case, true);

            let generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &naming_conventions::GenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent
            );
            let impl_serde_deserialize_for_generic_with_id_ident_field_reader_token_stream = {
                let generic_with_id_ident_field_reader_upper_camel_case = naming_conventions::GenericWithIdSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_generic_with_id_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&generic_with_id_ident_field_reader_upper_camel_case);
                let tuple_struct_generic_with_id_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&generic_with_id_ident_field_reader_upper_camel_case);
                let generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &generic_with_id_ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &generic_with_id_ident_field_reader_upper_camel_case,
                    &field0_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #generic_with_id_ident_field_reader_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#generic_with_id_ident_field_reader_upper_camel_case>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #generic_with_id_ident_field_reader_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_generic_with_id_ident_field_reader_double_quotes_token_stream,
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
                                    let __field0: std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case> = <std::vec::Vec<
                                        #ident_with_id_field_to_read_upper_camel_case,
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
                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_generic_with_id_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #generic_with_id_ident_field_reader_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let generic_with_id_ident_reader_upper_camel_case = naming_conventions::GenericWithIdSelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(&generic_with_id_ident_reader_upper_camel_case, &generic_with_id_ident_options_to_read_upper_camel_case);
            quote::quote!{
                #generic_with_id_ident_options_to_read_alias_token_stream

                #generic_with_id_ident_field_reader_token_stream
                // #impl_try_new_token_stream
                #impl_serde_deserialize_for_generic_with_id_ident_field_reader_token_stream

                #generic_with_id_ident_reader_token_stream
            }
        };
        quote::quote!{
            #generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_generic_with_id_ident_token_stream

            #create_token_stream
            #read_token_stream
        }
    };

    //its for GeneratePostgresqlQueryPart (json logic)
    let generic_ident_token_stream = {
        let generic_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &generic_ident_upper_camel_case,
            &quote::quote!{{#fields_token_stream}}
        );
        let impl_std_fmt_display_for_generic_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&generic_ident_upper_camel_case);


        let create_token_stream = {
            let generic_ident_to_create_upper_camel_case = naming_conventions::GenericSelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let generic_ident_to_create_token_stream = generate_tokens_to_create_token_stream(&generic_ident_to_create_upper_camel_case);
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&generic_ident_to_create_upper_camel_case);
            let impl_postgresql_crud_json_create_bind_query_for_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(
                &generic_ident_to_create_upper_camel_case,
                false,
                CreateBindQueryVariant::Generic
            );
            quote::quote!{
                #generic_ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_to_create_token_stream
                #impl_postgresql_crud_json_create_bind_query_for_generic_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let generic_ident_options_to_read_upper_camel_case = naming_conventions::GenericSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let generic_ident_options_to_read_alias_token_stream = generate_options_to_read_alias_token_stream(&generic_ident_options_to_read_upper_camel_case, false);

            let generic_ident_field_reader_upper_camel_case_token_stream = naming_conventions::GenericSelfFieldReaderUpperCamelCase::from_dyn_std_fmt_display(&ident);
            let generic_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &naming_conventions::GenericSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
            );
            let impl_serde_deserialize_for_generic_ident_field_reader_token_stream = {
                let generic_ident_field_reader_upper_camel_case = naming_conventions::GenericSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_generic_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&generic_ident_field_reader_upper_camel_case);
                let tuple_struct_generic_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&generic_ident_field_reader_upper_camel_case);
                let generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &generic_ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &generic_ident_field_reader_upper_camel_case_token_stream,
                    &field0_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #generic_ident_field_reader_upper_camel_case_token_stream {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#generic_ident_field_reader_upper_camel_case_token_stream>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #generic_ident_field_reader_upper_camel_case_token_stream;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_generic_ident_field_reader_double_quotes_token_stream,
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
                                    let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case> = <std::vec::Vec<
                                        #ident_field_to_read_upper_camel_case,
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
                                        std::vec::Vec<#ident_field_to_read_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_generic_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<#generic_ident_field_reader_upper_camel_case_token_stream>,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream(&generic_ident_field_reader_upper_camel_case_token_stream);
            /////////////
            //todo
            let generic_ident_reader_token_stream = generate_tokens_reader_token_stream(
                &naming_conventions::GenericSelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &generic_ident_options_to_read_upper_camel_case
            );
        
            let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
                &generic_ident_field_reader_upper_camel_case_token_stream,
                false,
                &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))"}
            );
            quote::quote!{
                #generic_ident_options_to_read_alias_token_stream

                #generic_ident_field_reader_token_stream
                #impl_serde_deserialize_for_generic_ident_field_reader_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_field_reader_token_stream

                #generic_ident_reader_token_stream

                #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_token_stream
            }
        };
        let update_token_stream = {
            let generic_ident_option_to_update_upper_camel_case = naming_conventions::GenericSelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            //todo custom deserialize to check unique fields
            let generic_ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &generic_ident_option_to_update_upper_camel_case,
                &quote::quote!{std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>},
                false,
            );
            let impl_serde_deserialize_for_generic_ident_option_to_update_token_stream = {
                let generic_ident_option_to_update_upper_camel_case = naming_conventions::GenericSelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_generic_ident_option_to_update_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&generic_ident_option_to_update_upper_camel_case);
                let tuple_struct_generic_ident_option_to_update_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&generic_ident_option_to_update_upper_camel_case);
                let generic_ident_option_to_update_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &generic_ident_option_to_update_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let custom_checks_token_stream = {
                    let check_fields_are_empty_token_stream = {
                        let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("custom serde error deserializing {generic_ident_option_to_update_upper_camel_case}: fields are empty"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            if __field0.is_empty() {
                                return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                            }
                        }
                    };
                    let check_unique_fields_token_stream = {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            let format_handle_double_quotes_token_stream = generate_custom_serde_error_deserializing_option_to_update_field_is_not_unique_to_update_token_stream(
                                &generic_ident_option_to_update_upper_camel_case,
                                &variant_ident_upper_camel_case_token_stream,
                            );
                            quote::quote!{
                                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(_) => {
                                    let value = #ident_field_to_update_upper_camel_case::#variant_ident_upper_camel_case_token_stream;
                                    if acc.contains(&value) {
                                        return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
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
                                for element in &__field0 {
                                    match &element {
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
                    impl<'de> serde::Deserialize<'de> for #generic_ident_option_to_update_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<#generic_ident_option_to_update_upper_camel_case>,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #generic_ident_option_to_update_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_generic_ident_option_to_update_double_quotes_token_stream,
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
                                    let __field0: std::vec::Vec<#ident_option_to_update_origin_upper_camel_case> = <std::vec::Vec<
                                        #ident_option_to_update_origin_upper_camel_case,
                                    > as serde::Deserialize>::deserialize(__e)?;
                                    #custom_checks_token_stream
                                    serde::__private::Ok(#generic_ident_option_to_update_upper_camel_case(__field0))
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
                                        std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_generic_ident_option_to_update_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #custom_checks_token_stream
                                    serde::__private::Ok(#generic_ident_option_to_update_upper_camel_case(__field0))
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #generic_ident_option_to_update_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #generic_ident_option_to_update_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_option_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &generic_ident_option_to_update_upper_camel_case,
                &quote::quote!{(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            );
            let (
                generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                impl_postgresql_crud_generate_postgresql_query_part_to_update_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_generic_ident_option_to_update_token_stream
            ) = {
                let generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::GenericSelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
                    &generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &{
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            let element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
                                let value = format!(
                                    "{}{}",
                                    {
                                        let element_type = &element.ty;
                                        quote::quote!{#element_type}
                                    },
                                    naming_conventions::OptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase
                                );
                                value.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
                                #variant_ident_upper_camel_case_token_stream {
                                    #[eo_error_occurence]
                                    error: #element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream,
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
                let impl_postgresql_crud_generate_postgresql_query_part_to_update_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_generic_ident_option_to_update_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
                    &generic_ident_option_to_update_upper_camel_case,
                    &generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &{
                        let generic_acc_snake_case = naming_conventions::GenericAccSnakeCase;
                        let generic_acc_jsonb_set_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_set({{jsonb_set_accumulator}},'{{{{{{jsonb_set_path}}}}}}',{{{generic_acc_snake_case}}})"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            let jsonb_set_target_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                                &format!("{{jsonb_set_target}}->'{field_ident_stringified}'"),
                                &proc_macro_name_upper_camel_case_ident_stringified
                            );
                            let jsonb_set_path_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                                &field_ident_stringified,
                                &proc_macro_name_upper_camel_case_ident_stringified
                            );
                            quote::quote!{
                                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                                    //todo fix parameters
                                    &value.value,
                                    &#generic_acc_snake_case,
                                    &format!(#jsonb_set_target_field_ident_double_quotes_token_stream),
                                    #jsonb_set_path_field_ident_double_quotes_token_stream,
                                    increment,
                                ) {
                                    Ok(value) => {
                                        #generic_acc_snake_case = value;
                                    }
                                    Err(error) => {
                                        return Err(#generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            }
                        });
                        quote::quote!{
                            let mut acc = std::string::String::from(jsonb_set_accumulator);
                            let mut #generic_acc_snake_case = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
                            for element in &self.0 {
                                match &element {
                                    #(#try_generate_bind_increments_variants_token_stream),*
                                }
                            }
                            Ok(format!(#generic_acc_jsonb_set_double_quotes_token_stream))
                        }
                    },
                    &{
                        let bind_value_to_query_variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            quote::quote!{
                                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                    query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value, query);
                                }
                            }
                        });
                        quote::quote!{
                            for element in self.0 {
                                match element {
                                    #(#bind_value_to_query_variants_token_stream),*
                                }
                            }
                            query
                        }
                    },
                );
                (
                    generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                    impl_postgresql_crud_generate_postgresql_query_part_to_update_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_generic_ident_option_to_update_token_stream
                )
            };
            quote::quote!{
                #generic_ident_option_to_update_token_stream
                #impl_serde_deserialize_for_generic_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_option_to_update_token_stream
                #generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_update_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_generic_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #generic_ident_token_stream
            #impl_std_fmt_display_for_generic_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_option_option_generic_ident_token_stream = {
        let (
            std_option_option_generic_ident_token_stream,
            impl_std_fmt_display_for_std_option_option_generic_ident_token_stream
        ) = {
            let std_option_option_generic_ident_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_option_option_generic_ident_upper_camel_case,
                &quote::quote!{(pub std::option::Option<#generic_ident_upper_camel_case>);}
            );
            let impl_std_fmt_display_for_std_option_option_generic_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_option_option_generic_ident_upper_camel_case);
            (
                std_option_option_generic_ident_token_stream,
                impl_std_fmt_display_for_std_option_option_generic_ident_token_stream
            )
        };

        let create_token_stream = {
            //todo maybe later reuse it as generic_ident_to_create
            let std_option_option_generic_ident_to_create_origin_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfToCreateOriginUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_to_create_origin_token_stream = generate_tokens_to_create_token_stream(&std_option_option_generic_ident_to_create_origin_upper_camel_case);
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&std_option_option_generic_ident_to_create_origin_upper_camel_case);
            let impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(
                &std_option_option_generic_ident_to_create_origin_upper_camel_case,
                false,
                CreateBindQueryVariant::StdOptionOptionGeneric
            );
            let std_option_option_generic_ident_to_create_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_option_option_generic_ident_to_create_upper_camel_case,
                &quote::quote!{(pub std::option::Option<#std_option_option_generic_ident_to_create_origin_upper_camel_case>);}
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_option_option_generic_ident_to_create_upper_camel_case,
                &quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))}
            );
            let impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_token_stream(
                &std_option_option_generic_ident_to_create_upper_camel_case,
                &quote::quote!{
                    match &self.0 {
                        Some(value) => match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(value, increment) {
                            Ok(value) => Ok(value),
                            //todo additional error variant
                            Err(error) => Err(error)
                        },
                        //maybe not use null here and use increment logic
                        None => Ok(std::string::String::from("null"))
                    }
                },
                &quote::quote!{
                    if let Some(value) = self.0 {
                        query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(value, query);
                    }
                    query
                },
            );
            quote::quote!{
                #std_option_option_generic_ident_to_create_origin_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_origin_token_stream
                #impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_origin_token_stream

                #std_option_option_generic_ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_token_stream
                #impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let std_option_option_generic_ident_options_to_read_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_options_to_read_alias_token_stream = generate_options_to_read_alias_token_stream(&std_option_option_generic_ident_options_to_read_upper_camel_case, false);

            let std_option_option_generic_ident_field_reader_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &naming_conventions::StdOptionOptionGenericSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
            );
            let impl_serde_deserialize_for_std_option_option_generic_ident_field_reader_token_stream = {
                let std_option_option_generic_ident_field_reader_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_std_option_option_generic_ident_field_reader_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&std_option_option_generic_ident_field_reader_upper_camel_case);
                let tuple_struct_std_option_option_generic_ident_field_reader_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&std_option_option_generic_ident_field_reader_upper_camel_case);
                let std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_option_option_generic_ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &std_option_option_generic_ident_field_reader_upper_camel_case,
                    &field0_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #std_option_option_generic_ident_field_reader_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<
                                    #std_option_option_generic_ident_field_reader_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_option_option_generic_ident_field_reader_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_std_option_option_generic_ident_field_reader_double_quotes_token_stream,
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
                                    let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case> = <std::vec::Vec<
                                        #ident_field_to_read_upper_camel_case,
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
                                        std::vec::Vec<#ident_field_to_read_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_std_option_option_generic_ident_field_reader_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #match_try_new_in_deserialize_token_stream
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_option_option_generic_ident_field_reader_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream(&std_option_option_generic_ident_field_reader_upper_camel_case);
            //

            let std_option_option_generic_ident_reader_token_stream = generate_tokens_reader_token_stream(
                &naming_conventions::StdOptionOptionGenericSelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &ident_options_to_read_without_id_upper_camel_case
            );

            let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
                &std_option_option_generic_ident_field_reader_upper_camel_case,
                false,
                &quote::quote!{"jsonb_build_object('{field_ident}', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('value', null) else jsonb_build_object('value',{acc}) end)"}
            );
            quote::quote!{
                #std_option_option_generic_ident_options_to_read_alias_token_stream

                #std_option_option_generic_ident_field_reader_token_stream
                #impl_serde_deserialize_for_std_option_option_generic_ident_field_reader_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_field_reader_token_stream
                // #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_upper_camel_case_token_stream
                #std_option_option_generic_ident_reader_token_stream

                #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_token_stream
            }
        };
        let update_token_stream = {
            let std_option_option_generic_ident_option_to_update_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &std_option_option_generic_ident_option_to_update_upper_camel_case,
                &quote::quote!{std::option::Option<std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>>},
                false,
            );
            let impl_serde_deserialize_for_std_option_option_generic_ident_option_to_update_token_stream = {
                let tuple_struct_std_option_option_generic_ident_option_to_update_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&std_option_option_generic_ident_option_to_update_upper_camel_case);
                let tuple_struct_std_option_option_generic_ident_option_to_update_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&std_option_option_generic_ident_option_to_update_upper_camel_case);
                let std_option_option_generic_ident_option_to_update_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_option_option_generic_ident_option_to_update_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let custom_checks_token_stream = {
                    let check_fields_are_empty_token_stream = {
                        let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("custom serde error deserializing {std_option_option_generic_ident_option_to_update_upper_camel_case}: fields are empty"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            if let Some(value) = &__field0 {
                                if value.is_empty() {
                                    return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                                }
                            }
                        }
                    };
                    let check_unique_fields_token_stream = {
                        let variants_token_stream = vec_syn_field.iter().map(|element| {
                            let field_ident_stringified = element
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                                })
                                .to_string();
                            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                            let format_handle_double_quotes_token_stream = generate_custom_serde_error_deserializing_option_to_update_field_is_not_unique_to_update_token_stream(
                                &std_option_option_generic_ident_option_to_update_upper_camel_case,
                                &variant_ident_upper_camel_case_token_stream,
                            );
                            quote::quote!{
                                #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(_) => {
                                    let value = #ident_field_to_update_upper_camel_case::#variant_ident_upper_camel_case_token_stream;
                                    if acc.contains(&value) {
                                        return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                                    }
                                    else {
                                        acc.push(value);
                                    }
                                }
                            }
                        });
                        quote::quote!{
                            if let Some(value) = &__field0 {
                                let mut acc = vec![];
                                for element in value {
                                    match &element {
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
                    impl<'de> serde::Deserialize<'de> for #std_option_option_generic_ident_option_to_update_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<
                                    #std_option_option_generic_ident_option_to_update_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_option_option_generic_ident_option_to_update_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_std_option_option_generic_ident_option_to_update_double_quotes_token_stream,
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
                                        std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                                    > = <std::option::Option<
                                        std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>,
                                    > as serde::Deserialize>::deserialize(__e)?;
                                    #custom_checks_token_stream
                                    serde::__private::Ok(
                                        #std_option_option_generic_ident_option_to_update_upper_camel_case(__field0),
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
                                        std::option::Option<
                                            std::vec::Vec<
                                                #ident_option_to_update_origin_upper_camel_case
                                            >,
                                        >,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_std_option_option_generic_ident_option_to_update_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #custom_checks_token_stream
                                    serde::__private::Ok(
                                        #std_option_option_generic_ident_option_to_update_upper_camel_case(__field0),
                                    )
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #std_option_option_generic_ident_option_to_update_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_option_option_generic_ident_option_to_update_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_option_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_option_option_generic_ident_option_to_update_upper_camel_case,
                &quote::quote!{(Some(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))}
            );

            let std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::StdOptionOptionGenericSelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
                &std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                &{
                    let variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
                            let value = format!(
                                "{}{}",
                                {
                                    let element_type = &element.ty;
                                    quote::quote!{#element_type}
                                },
                                naming_conventions::OptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase
                            );
                            value.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            #variant_ident_upper_camel_case_token_stream {
                                #[eo_error_occurence]
                                error: #element_type_option_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream,
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
            let impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_generic_ident_option_to_update_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
                &std_option_option_generic_ident_option_to_update_upper_camel_case,
                &std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                &{
                    let std_option_option_generic_acc_snake_case = naming_conventions::StdOptionOptionGenericAccSnakeCase;
                    let std_option_option_generic_acc_jsonb_set_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_set({{jsonb_set_accumulator}},'{{{{{{jsonb_set_path}}}}}}',{{{std_option_option_generic_acc_snake_case}}})"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let try_generate_bind_increments_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        let jsonb_set_target_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{{jsonb_set_target}}->'{field_ident_stringified}'"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let jsonb_set_path_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &field_ident_stringified,
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        quote::quote!{
                            #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                                    &value.value,
                                    &#std_option_option_generic_acc_snake_case,
                                    &format!(#jsonb_set_target_field_ident_double_quotes_token_stream),
                                    #jsonb_set_path_field_ident_double_quotes_token_stream,
                                    increment,
                                ) {
                                    Ok(value) => {
                                        #std_option_option_generic_acc_snake_case = value;
                                    }
                                    Err(error) => {
                                        return Err(#std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::#variant_ident_upper_camel_case_token_stream {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                        }
                    });
                    quote::quote!{
                        Ok(match &self.0 {
                            Some(value) => {
                                let mut #std_option_option_generic_acc_snake_case = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
                                for element in value {
                                    match element {
                                        #(#try_generate_bind_increments_variants_token_stream),*
                                    }
                                }
                                format!(#std_option_option_generic_acc_jsonb_set_double_quotes_token_stream)
                            },
                            None => match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")
                                },
                                None => {
                                    return Err(#std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
                                }
                            }
                        })
                    }
                },
                &{
                    let bind_value_to_query_variants_token_stream = vec_syn_field.iter().map(|element| {
                        let field_ident_stringified = element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string();
                        let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                        quote::quote!{
                            #ident_option_to_update_origin_upper_camel_case::#variant_ident_upper_camel_case_token_stream(value) => {
                                query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value, query);
                            }
                        }
                    });
                    quote::quote!{
                        match self.0 {
                            Some(value) => {
                                for element in value {
                                    match element {
                                        #(#bind_value_to_query_variants_token_stream),*
                                    }
                                }
                            },
                            None => {
                                query = query.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<#ident_option_to_update_origin_upper_camel_case>>>));
                            }
                        }
                        query
                    }
                },
            );
            quote::quote!{
                #std_option_option_generic_ident_option_to_update_token_stream
                #impl_serde_deserialize_for_std_option_option_generic_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_option_to_update_token_stream
                #std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_generic_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_generic_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #std_option_option_generic_ident_token_stream
            #impl_std_fmt_display_for_std_option_option_generic_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_vec_vec_generic_with_id_ident_token_stream = {
        let (
            std_vec_vec_generic_with_id_ident_token_stream,
            impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream
        ) = {
            let std_vec_vec_generic_with_id_ident_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_vec_vec_generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_vec_vec_generic_with_id_ident_upper_camel_case,
                &quote::quote!{(pub std::vec::Vec<#generic_with_id_ident_upper_camel_case>);}
            );
            let impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_vec_vec_generic_with_id_ident_upper_camel_case);
            (
                std_vec_vec_generic_with_id_ident_token_stream,
                impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream
            )
        };


        let create_token_stream = {
            let std_vec_vec_generic_with_id_ident_to_create_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{(pub std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>);}
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])}
            );
            let impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_token_stream(
                &std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{
                    let mut acc = std::string::String::default();
                    for element in &self.0 {
                        match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(element, increment) {
                            Ok(value) => {
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
                &quote::quote!{
                    for element in self.0 {
                        query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(element, query);
                    }
                    query
                },
            );
            quote::quote!{
                #std_vec_vec_generic_with_id_ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_token_stream
                #impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_vec_vec_generic_with_id_ident_options_to_read_token_stream = generate_tokens_options_to_read_token_stream(
                &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                false,
                &quote::quote!{(pub std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>);},
            );

            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream =             generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                &quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
            );

            let impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = {
                let std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case);
                let tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case);
                let std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let check_not_unique_id_token_stream = {
                    quote::quote!{
                        {
                            let mut acc = vec![];
                            for element in &__field0 {
                                if let Some(value) = &element.id {
                                    if acc.contains(&&value.value) {
                                        return Err(serde::de::Error::custom(format!("not unique id {}", value.value.0)));
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
                    impl<'de> serde::Deserialize<'de> for #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<
                                    #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream,
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
                                    #check_not_unique_id_token_stream
                                    serde::__private::Ok(
                                        #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case(__field0),
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
                                        std::vec::Vec<
                                            #ident_options_to_read_with_id_upper_camel_case,
                                        >,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #check_not_unique_id_token_stream
                                    serde::__private::Ok(
                                        #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case(__field0),
                                    )
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };

            let std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &naming_conventions::StdVecVecGenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
            );
            let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = {
                let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let struct_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream = generate_struct_tokens_double_quotes_token_stream(&std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);
                let struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = generate_struct_tokens_with_2_elements_double_quotes_token_stream(&std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);
                let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                    &field0_field1_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case {
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
                                    #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream,
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
                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
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
                                                    &#struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
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
                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
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
                                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
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
                                #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream(&std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);

            let std_vec_vec_generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(
                &naming_conventions::StdVecVecGenericWithIdSelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case
            );
            let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
                &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                true,
                &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value',(select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))"}
            );
            quote::quote!{
                #std_vec_vec_generic_with_id_ident_options_to_read_token_stream
                #impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream

                #std_vec_vec_generic_with_id_ident_field_reader_token_stream
                #impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
                // #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_ident_field_reader_upper_camel_case_token_stream_token_stream
                #std_vec_vec_generic_with_id_ident_reader_token_stream

                #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            }
        };
        let update_token_stream = {
            let std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfJsonArrayChangeUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_vec_vec_generic_with_id_ident_json_array_change_token_stream = generate_ident_json_array_change_token_stream(
                &quote::quote!{#std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case},
                &naming_conventions::StdVecVecGenericWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                false,
            );

            let std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                &std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case,
                true,
            );
            //todo maybe should impl trait for convetion  tokens_option_to_update into field to update. t
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                &quote::quote!{(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            );
            // println!("{impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_json_array_change_token_stream}");
            let (
                std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                impl_postgresql_crud_generate_postgresql_query_part_to_update_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
            ) = {
                let std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::StdVecVecGenericWithIdSelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
                    &std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &{
                        quote::quote!{
                            JsonArrayChange {
                                #[eo_error_occurence]
                                error: #ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case, 
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }
                    }
                );
                let impl_postgresql_crud_generate_postgresql_query_part_to_update_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
                    &std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                    &std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &quote::quote!{
                        match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                            &self.0,
                            jsonb_set_accumulator,
                            jsonb_set_target,
                            jsonb_set_path,
                            increment,
                        ) {
                            Ok(value) => Ok(value),
                            Err(error) => {
                                return Err(#std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::JsonArrayChange {
                                    error,
                                    code_occurence: error_occurence_lib::code_occurence!()
                                });
                            }
                        }
                    },
                    &quote::quote!{
                        query = self.0.bind_value_to_query(query);
                        query
                    },
                );
                (
                    std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                    impl_postgresql_crud_generate_postgresql_query_part_to_update_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
                )
            };
            quote::quote!{
                #std_vec_vec_generic_with_id_ident_json_array_change_token_stream
                #std_vec_vec_generic_with_id_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream

                #std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_update_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #std_vec_vec_generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_option_option_std_vec_vec_generic_with_id_ident_token_stream = {
        let (
            std_option_option_std_vec_vec_generic_with_id_ident_token_stream,
            impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream
        ) = {
            let std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_std_vec_vec_generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case,
                &quote::quote!{(std::option::Option<std::vec::Vec<#generic_with_id_ident_upper_camel_case>>);}
            );
            let impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case);
            (
                std_option_option_std_vec_vec_generic_with_id_ident_token_stream,
                impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream
            )
        };

        let create_token_stream = {
            let std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{(pub std::option::Option<std::vec::Vec<#ident_to_create_origin_with_generated_id_upper_camel_case>>);}
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream =  generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))}
            );
            let impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case,
                &quote::quote!{
                    match &self.0 {
                        Some(value) => {
                            let mut acc = std::string::String::default();
                            for element in value {
                                match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(element, increment) {
                                    Ok(value) => {
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
                &quote::quote!{
                    if let Some(value) = self.0 {
                        for element in value {
                            query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(element, query);
                        }
                    }
                    query
                },
            );
            quote::quote!{
                #std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream
                #impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream
            }
        };
        let read_token_stream = {
            let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
        
            let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = generate_tokens_options_to_read_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                false,
                &quote::quote!{(pub std::option::Option<std::vec::Vec<#ident_options_to_read_with_id_upper_camel_case>>);},
            );

            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                &quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))},
            );

            let impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = {
                let tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream = generate_tuple_struct_tokens_double_quotes_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case);
                let tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = generate_tuple_struct_tokens_with_1_element_double_quotes_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case);
                let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let check_not_unique_id_token_stream = {
                    quote::quote!{
                        if let Some(value) = &__field0 {
                            let mut acc = vec![];
                            for element in value {
                                if let Some(value) = &element.id {
                                    if acc.contains(&&value.value) {
                                        return Err(serde::de::Error::custom(format!("not unique id {}", value.value.0)));
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
                    impl<'de> serde::Deserialize<'de> for #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: serde::__private::PhantomData<
                                    #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream,
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
                                    #check_not_unique_id_token_stream
                                    serde::__private::Ok(
                                        #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case(
                                            __field0,
                                        ),
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
                                                    &#tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    #check_not_unique_id_token_stream
                                    serde::__private::Ok(
                                        #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case(
                                            __field0,
                                        ),
                                    )
                                }
                            }
                            serde::Deserializer::deserialize_newtype_struct(
                                __deserializer,
                                #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };

            let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            
            let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
                &naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
            );
            let impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = {
                let struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream = generate_struct_tokens_double_quotes_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);
                let struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = generate_struct_tokens_with_2_elements_double_quotes_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);
                let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                    &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                    &field0_field1_token_stream,
                );
                quote::quote!{
                    impl<'de> serde::Deserialize<'de> for #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case {
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
                                    #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                                >,
                                lifetime: serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut serde::__private::Formatter<'_>,
                                ) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream,
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
                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
                                    >(&mut __seq)? {
                                        serde::__private::Some(__value) => __value,
                                        serde::__private::None => {
                                            return serde::__private::Err(
                                                serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
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
                                                    &#struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
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
                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
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
                                                        std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case>,
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
                                #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: serde::__private::PhantomData::<
                                        #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                                    >,
                                    lifetime: serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            };
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream =   generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case);

            let std_option_option_std_vec_vec_generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(
                &naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case
            );
            let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case,
                true,
                &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then null else (select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end}) end))"}
            );
            quote::quote!{
                #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
                #impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream

                #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
                #impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
                #std_option_option_std_vec_vec_generic_with_id_ident_reader_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            }
        };
        let update_token_stream = {
            let std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
            let std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfJsonArrayChangeUpperCamelCase::from_dyn_quote_to_tokens(&ident);

            let std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_token_stream = generate_ident_json_array_change_token_stream(
                &quote::quote!{#std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case},
                &naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfJsonArrayChangeTryNewErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident),
                true,
            );
            let std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_tokens_option_to_update_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                &quote::quote!{std::option::Option<#std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case>},
                true,
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                &quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))}
            );

            let (
                std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
            ) = {
                let std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::StdOptionOptionStdVecVecGenericWithIdSelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
                let std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = generate_tokens_try_generate_bind_increments_error_named_token_stream(
                    &std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &{
                        quote::quote!{
                            #checked_add_variant_declaration_token_stream,
                            JsonArrayChange {
                                #[eo_error_occurence]
                                error: #ident_json_array_change_try_generate_bind_increments_error_named_upper_camel_case,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        }
                    }
                );
                let impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_to_update_token_stream(
                    &std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_upper_camel_case,
                    &std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case,
                    &quote::quote!{
                        match &self.0 {
                            Some(value) => {
                                match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                                    value,
                                    jsonb_set_accumulator,
                                    jsonb_set_target,
                                    jsonb_set_path,
                                    increment,
                                ) {
                                    Ok(value) => Ok(value),
                                    Err(error) => {
                                        return Err(#std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::JsonArrayChange {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!()
                                        });
                                    }
                                }
                            }
                            None => match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                                }
                                None => {
                                    return Err(#std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::#checked_add_variant_initialization_token_stream);
                                }
                            },
                        }
                    },
                    &quote::quote!{
                        //todo write some Into Destructor for structs or enums what implements bind_value_to_query. need this to remove .clone() calls
                        match self.0 {
                            Some(value) => {
                                query = value.bind_value_to_query(query);
                            }
                            None => {
                                query = query.bind(sqlx::types::Json(None::<std::option::Option<#std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_upper_camel_case>>));
                            }
                        }
                        query
                    },
                );
                (
                    std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream,
                    impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
                )
            };
            quote::quote!{
                #std_option_option_std_vec_vec_generic_with_id_ident_json_array_change_token_stream
                #std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
                #std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_update_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_try_generate_bind_increments_error_named_for_std_option_option_std_vec_vec_generic_with_id_ident_option_to_update_token_stream
            }
        };
        quote::quote!{
            #std_option_option_std_vec_vec_generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream

            #create_token_stream
            #read_token_stream
            #update_token_stream
        }
    };
    let generated = quote::quote! {
        #ident_to_create_origin_with_generated_id_token_stream
        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_origin_with_generated_id_token_stream
        #impl_postgresql_crud_json_create_bind_query_for_ident_to_create_origin_with_generated_id_token_stream


        #ident_field_to_read_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream
        #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream
        #ident_with_id_field_to_read_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream
        #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream

        #ident_field_to_update_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream

        #ident_option_to_update_origin_token_stream
        #ident_json_array_change_try_generate_bind_increments_error_named_token_stream
        #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_origin_token_stream


        #ident_options_to_read_without_id_token_stream
        #ident_options_to_read_with_id_token_stream
        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_without_id_token_stream
        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_with_id_token_stream
        #impl_serde_deserialize_for_ident_options_to_read_without_id_token_stream
        #impl_serde_deserialize_for_ident_options_to_read_with_id_token_stream


        #ident_options_to_update_token_stream
        #impl_pub_fn_try_new_for_ident_options_to_update_token_stream
        #impl_serde_deserialize_for_ident_options_to_update_token_stream
        #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream
        //

        #ident_token_stream

        #generic_with_id_ident_token_stream

        #generic_ident_token_stream
        #std_option_option_generic_ident_token_stream
        #std_vec_vec_generic_with_id_ident_token_stream
        #std_option_option_std_vec_vec_generic_with_id_ident_token_stream
    };
    // if ident == "Something" {
    //     // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     //     "GeneratePostgresqlQueryPart",
    //     //     &generated,
    //     //     "GeneratePostgresqlQueryPart",
    //     // );
    //     quote::quote!{}.into()
    // }
    // else {
    //     generated.into()
    // }
    generated.into()
}
