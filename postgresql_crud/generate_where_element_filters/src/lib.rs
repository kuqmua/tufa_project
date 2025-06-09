#[proc_macro]
pub fn generate_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let query_snake_case = naming::QuerySnakeCase;
    let t_token_stream = quote::quote! {T};
    let t_annotation_generic_token_stream = quote::quote! {<#t_token_stream>};
    let std_vec_vec_t_token_stream = &quote::quote! {std::vec::Vec<T>};
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    //todo reuse ?
    let core_default_default_default_token_stream = quote::quote! {
        ::core::default::Default::default()
    };
    let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote! {
        crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
    };
    // let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = quote::quote!{
    //     crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    // };
    let value_t_token_stream = quote::quote! {value: T};
    let pub_value_t_token_stream = quote::quote! {pub value: #t_token_stream};
    let value_std_vec_vec_t_token_stream = quote::quote! {value: #std_vec_vec_t_token_stream};
    let unsigned_part_of_std_primitive_i32_token_stream = quote::quote!{crate::UnsignedPartOfStdPrimitiveI32};
    let value_declaration_token_stream = quote::quote! {value: #unsigned_part_of_std_primitive_i32_token_stream};
    enum ShouldAddDeclarationOfStructIdentGeneric {
        True,
        False,
    }
    struct Field<'a> {
        field_name: &'a dyn std::fmt::Display,
        field_type: &'a dyn quote::ToTokens,
    }
    let value_unsigned_part_of_std_primitive_i32_field = Field {
        field_name: &naming::ValueSnakeCase,
        field_type: &unsigned_part_of_std_primitive_i32_token_stream, //todo i32 or i64 or something between? or more? or less?
    };
    let start_t_field = Field {
        field_name: &naming::StartSnakeCase,
        field_type: &t_token_stream,
    };
    let end_t_field = Field {
        field_name: &naming::EndSnakeCase,
        field_type: &t_token_stream,
    };
    let value_std_vec_vec_t_field = Field {
        field_name: &naming::ValueSnakeCase,
        field_type: &std_vec_vec_t_token_stream,
    };
    let value_code_default_token_stream = quote::quote! {
        value: #core_default_default_default_token_stream
    };
    let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote! {
        value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
    };
    let generate_query_part_zero_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column))
        }
    };
    let crate_query_part_error_named_checked_add_initialization_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_checked_add_initialization_token_stream();
    let generate_query_part_one_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                }
                None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
            }
        }
    };
    let generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream = |value: &std::primitive::str| generate_quotes::double_quotes_token_stream(&format!("{{}}({{}} {value} ${{}})"));
    let generate_struct_token_stream = |
        filter_initialized_with_try_new_result_is_ok: std::primitive::bool,
        should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
        ident: &dyn quote::ToTokens,
        struct_additional_fields_token_stream: &dyn quote::ToTokens,
    | {
        let maybe_pub_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_token_stream_new } else { &naming::PubSnakeCase };
        let maybe_derive_serde_deserialize_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_token_stream_new } else { &quote::quote! {serde::Deserialize,} };
        let maybe_declaration_of_struct_ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
            ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
        };
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_derive_serde_deserialize_token_stream schemars::JsonSchema)]
            pub struct #ident #maybe_declaration_of_struct_ident_generic_token_stream {
                #maybe_pub_token_stream logical_operator: crate::LogicalOperator,
                #struct_additional_fields_token_stream
            }
        }
    };
    enum ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed {
        True,
        False,
    }
    let generate_try_new_logic_token_stream = |
        ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
        ident_try_new_error_named: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
        should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named: &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed,
        enum_ident_try_new_error_named_content_token_stream: &dyn quote::ToTokens,
        generic_requirements_token_stream: &dyn quote::ToTokens,
        additional_input_parameters_token_stream: &dyn quote::ToTokens,
        impl_try_new_for_ident_content_token_stream: &dyn quote::ToTokens,
        option_additional_traits_annotations_token_stream: std::option::Option<proc_macro2::TokenStream>,
        additional_fields: &std::vec::Vec<&Field>
    | {
        let generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = |should_add_declaration_of_generic_parameter_to_ident_try_new_error_named: &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed| -> &dyn quote::ToTokens {
            match &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named {
                ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True => &t_annotation_generic_token_stream,
                ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False => &proc_macro2_token_stream_new,
            }
        };
        let enum_ident_try_new_error_named_token_stream = {
            let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(should_add_declaration_of_generic_parameter_to_ident_try_new_error_named);
            quote::quote! {
                #[derive(
                    Debug,
                    Clone,
                    serde::Serialize,
                    serde::Deserialize,
                    thiserror::Error,
                    error_occurence_lib::ErrorOccurence,
                )]
                pub enum #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream {
                    #enum_ident_try_new_error_named_content_token_stream
                }
            }
        };
        let impl_try_new_for_ident_token_stream = {
            let impl_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True => &quote::quote! {<T #generic_requirements_token_stream>},
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            };
            let ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            };
            let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(should_add_declaration_of_generic_parameter_to_ident_try_new_error_named);
            quote::quote! {
                impl #impl_generic_token_stream #ident #ident_generic_token_stream {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        #additional_input_parameters_token_stream
                    ) -> Result<Self, #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream> {
                        #impl_try_new_for_ident_content_token_stream
                    }
                }
            }
        };
        let impl_serde_deserialize_for_ident_token_stream = {
            let maybe_impl_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() { &quote::quote! {, T} } else { &proc_macro2_token_stream_new };
            let maybe_ident_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() { &t_annotation_generic_token_stream } else { &proc_macro2_token_stream_new };
            let maybe_where_generic_trait_annotation_token_stream: &dyn quote::ToTokens = match &option_additional_traits_annotations_token_stream {
                Some(value) => &quote::quote! {
                    where
                        T: std::fmt::Debug + _serde::Deserialize<'de> #value,
                },
                None => &proc_macro2_token_stream_new,
            };
            let maybe_struct_visitor_where_generic_trait_annotation_token_stream = if option_additional_traits_annotations_token_stream.is_some() {
                quote::quote! {
                    where
                        T: _serde::Deserialize<'de>,
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let logical_operator_field = Field {
                field_name: &naming::LogicalOperatorSnakeCase,
                field_type: &quote::quote! {crate::LogicalOperator},
            };
            let fields = {
                let mut acc = vec![&logical_operator_field];
                for element in additional_fields {
                    acc.push(element);
                }
                acc
            };
            let (struct_ident_double_quotes_token_stream, struct_ident_with_number_of_elements_double_quotes_token_stream, ident_double_quotes_token_stream) = postgresql_crud_macros_common::generate_serde_deserialize_double_quotes_token_stream(&ident, fields.len());
            let enum_field_fields_token_stream = {
                let value = fields.iter().enumerate().map(|(index, _)| format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap());
                quote::quote! {#(#value),*}
            };
            let visit_u64_match_variants_token_stream = fields.iter().enumerate().map(|(index, _)| format!("{index}u64 => _serde::__private::Ok(__Field::__field{index})").parse::<proc_macro2::TokenStream>().unwrap());
            let visit_str_match_variants_token_stream = fields
                .iter()
                .enumerate()
                .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
            let visit_bytes_match_variants_token_stream = fields
                .iter()
                .enumerate()
                .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::binary_double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
            let visit_seq_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                let element_field_type_token_stream = &element.field_type;
                let index_usize_token_stream = format!("{index}usize").parse::<proc_macro2::TokenStream>().unwrap();
                quote::quote! {
                    let #field_index_token_stream = match _serde::de::SeqAccess::next_element::<
                        #element_field_type_token_stream,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    #index_usize_token_stream,
                                    &#struct_ident_with_number_of_elements_double_quotes_token_stream,
                                ),
                            );
                        }
                    };
                }
            });
            let visit_map_declaration_token_stream = fields.iter().enumerate().map(|(index, element)| {
                let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                let element_field_type_token_stream = &element.field_type;
                quote::quote! {
                    let mut #field_index_token_stream: _serde::__private::Option<
                        #element_field_type_token_stream,
                    > = _serde::__private::None;
                }
            });
            let visit_map_match_variants_token_stream = fields.iter().enumerate().map(|(index, element)| {
                let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                let element_field_type_token_stream = &element.field_type;
                quote::quote! {
                    __Field::#field_index_token_stream => {
                        if _serde::__private::Option::is_some(&#field_index_token_stream) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    #element_field_name_double_quotes_token_stream,
                                ),
                            );
                        }
                        #field_index_token_stream = _serde::__private::Some(
                            _serde::de::MapAccess::next_value::<
                                #element_field_type_token_stream,
                            >(&mut __map)?,
                        );
                    }
                }
            });
            let visit_map_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                quote::quote! {
                    let #field_index_token_stream = match #field_index_token_stream {
                        _serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field(#element_field_name_double_quotes_token_stream)?
                        }
                    };
                }
            });
            let field_names_double_quotes_token_stream = fields.iter().map(|element| generate_quotes::double_quotes_token_stream(&element.field_name));
            let try_new_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(&ident, &enum_field_fields_token_stream);
            quote::quote! {
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de #maybe_impl_generic_token_stream> _serde::Deserialize<'de> for #ident #maybe_ident_generic_token_stream
                    #maybe_where_generic_trait_annotation_token_stream
                    {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                #enum_field_fields_token_stream,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __f: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __f,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        #(#visit_u64_match_variants_token_stream),*,
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        #(#visit_str_match_variants_token_stream),*,
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        #(#visit_bytes_match_variants_token_stream),*,
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de #maybe_impl_generic_token_stream>
                            #maybe_struct_visitor_where_generic_trait_annotation_token_stream
                            {
                                marker: _serde::__private::PhantomData<
                                    #ident #maybe_ident_generic_token_stream,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de #maybe_impl_generic_token_stream> _serde::de::Visitor<'de> for __Visitor<'de #maybe_impl_generic_token_stream>
                            #maybe_where_generic_trait_annotation_token_stream
                            {
                                type Value = #ident #maybe_ident_generic_token_stream;
                                fn expecting(
                                    &self,
                                    __f: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __f,
                                        #struct_ident_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {

                                    #(#visit_seq_initialization_token_stream)*
                                    #try_new_token_stream
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    #(#visit_map_declaration_token_stream)*

                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            #(#visit_map_match_variants_token_stream)*
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    #(#visit_map_initialization_token_stream)*
                                    #try_new_token_stream
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[#(#field_names_double_quotes_token_stream),*];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #ident_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #ident #maybe_ident_generic_token_stream,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        quote::quote! {
            #enum_ident_try_new_error_named_token_stream
            #impl_try_new_for_ident_token_stream
            #impl_serde_deserialize_for_ident_token_stream
        }
    };
    let generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream =
        |should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn quote::ToTokens, impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream: &dyn quote::ToTokens| {
            postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => quote::quote! {<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
                    ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new(),
                },
                &postgresql_crud_macros_common::ImportPath::Crate,
                &ident,
                match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                },
                &{
                    quote::quote! {
                        Self {
                            logical_operator: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                        }
                    }
                },
            )
        };
    enum FilterType {
        PostgresqlType,
        PostgresqlJsonType,
    }
    let generate_impl_postgresql_type_where_filter_token_stream = |
        filter_type: &FilterType,
        should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
        ident: &dyn quote::ToTokens,
        query_part_content_token_stream: &dyn quote::ToTokens,
        is_query_bind_mutable: &postgresql_crud_macros_common::IsQueryBindMutable,
        query_bind_content_token_stream: &dyn quote::ToTokens
    | {
        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
            &{
                let maybe_t_additional_traits_for_postgresql_type_where_filter_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => match &filter_type {
                        FilterType::PostgresqlType => &quote::quote! {, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send},
                        FilterType::PostgresqlJsonType => &quote::quote! {, T: std::marker::Send + serde::Serialize + 'a},
                    },
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {<'a #maybe_t_additional_traits_for_postgresql_type_where_filter_token_stream>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            },
            &query_part_content_token_stream,
            is_query_bind_mutable,
            &query_bind_content_token_stream,
            &postgresql_crud_macros_common::ImportPath::Crate,
        )
    };

    let regular_expression_case_and_value_declaration_token_stream = quote::quote! {
        pub regular_expression_case: crate::RegularExpressionCase,
        pub value: crate::RegexRegex
    };
    let regular_expression_case_and_value_default_initialization_token_stream = quote::quote! {
        regular_expression_case: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
        value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
    };
    let generate_query_part_regular_expression_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
        quote::quote! {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!(
                        #format_handle_token_stream,
                        &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                        column,
                        self.regular_expression_case.postgreql_syntax(),
                        increment
                    ))
                }
                None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
            }
        }
    };
    let query_equals_query_self_value_to_string_token_stream = quote::quote! {
        query = query.bind(self.value.to_string());
        query
    };
    let postgresql_type_token_stream = {
        #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum PostgresqlTypeFilterInitializedWithTryNew {
            Between,
            In,
            RangeLength,
        }
        impl std::convert::TryFrom<&postgresql_crud_macros_common::PostgresqlTypeFilter> for PostgresqlTypeFilterInitializedWithTryNew {
            type Error = ();
            fn try_from(value: &postgresql_crud_macros_common::PostgresqlTypeFilter) -> Result<Self, Self::Error> {
                match &value {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between => Ok(Self::Between),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In => Ok(Self::In),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthDimensionOne => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthMoreThanDimensionOne => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ValueIsContainedWithinRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ContainsAnotherRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanLowerBound => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => Ok(Self::RangeLength),
                }
            }
        }
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlTypeFilter| {
            let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let ident_try_new_error_named = naming::parameter::PostgresqlTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
            let query_bind_one_value_token_stream = quote::quote! {
                query = query.bind(self.value);
                query
            };
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                query_part_content_token_stream,
                query_bind_content_token_stream
            ) = match &filter {
                postgresql_crud_macros_common::PostgresqlTypeFilter::Equal => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("=")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::Between => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote! {
                        start: T,
                        end: T,
                    },
                    &quote::quote! {
                        start: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        end: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    },
                    &quote::quote! {
                        match increment.checked_add(1) {
                            Some(first_value) => {
                                *increment = first_value;
                                match increment.checked_add(1) {
                                    Some(second_value) => {
                                        *increment = second_value;
                                        let between_snake_case = naming::BetweenSnakeCase;
                                        let and_snake_case = naming::AndSnakeCase;
                                        Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                                    }
                                    None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                                }
                            }
                            None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                        }
                    },
                    &quote::quote! {
                        query = query.bind(self.start);//here change
                        query = query.bind(self.end);//here change
                        query
                    },
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::In => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_std_vec_vec_t_token_stream,
                    &quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream]
                    },
                    &quote::quote! {
                        let mut acc = std::string::String::default();
                        for element in &self.value {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc.push_str(&format!("${},", value));
                                }
                                None => {
                                    return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                }
                            }
                        }
                        let _ = acc.pop();
                        let in_snake_case = naming::InSnakeCase;
                        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
                    },
                    &quote::quote! {
                        for element in self.value {
                            query = query.bind(element);
                        }
                        query
                    },
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &regular_expression_case_and_value_declaration_token_stream,
                    &regular_expression_case_and_value_default_initialization_token_stream,
                    &generate_query_part_regular_expression_token_stream(&quote::quote!{"{}({} {} ${})"}),
                    &query_equals_query_self_value_to_string_token_stream
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::Before => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("<")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_date)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_date)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_timestamp)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_timestamp)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_time)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_time)"}),
                    &quote::quote! {#query_snake_case},
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthDimensionOne => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_declaration_token_stream,
                    &value_code_default_token_stream,
                    &generate_query_part_one_value_token_stream(&quote::quote! {"{}(array_length({}, 1) = ${})"}),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthMoreThanDimensionOne => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_declaration_token_stream,
                    &value_code_default_token_stream,
                    &generate_query_part_one_value_token_stream(&quote::quote! {"{}(array_length({}, 1) > ${})"}),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote! {
                        pub encode_format: crate::postgresql_type::EncodeFormat,
                        pub encoded_string_representation: T,
                    },
                    &quote::quote! {
                        encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        encoded_string_representation: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    },
                    &quote::quote! {
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, &self.encode_format, increment))
                            }
                            None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                        }
                    },
                    &quote::quote! {
                        query = query.bind(self.encoded_string_representation);
                        query
                    },
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::ValueIsContainedWithinRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::ContainsAnotherRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&<")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&>")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&quote::quote! {"{}(lower({}) = ${})"}),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&quote::quote! {"{}(upper({}) = ${})"}),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&&")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("-|-")),
                    &query_bind_one_value_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_declaration_token_stream,
                    &value_code_default_token_stream,
                    &quote::quote! {
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
                            }
                            None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                        }
                    },
                    &query_bind_one_value_token_stream,
                ),
            };
            let filter_initialized_with_try_new_result = PostgresqlTypeFilterInitializedWithTryNew::try_from(filter);
            let struct_token_stream = generate_struct_token_stream(
                filter_initialized_with_try_new_result.is_ok(),
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &struct_additional_fields_token_stream,
            );
            let maybe_try_new_logic_token_stream = match filter_initialized_with_try_new_result {
                Ok(value) => {
                    let (
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        should_add_declaration_of_struct_ident_generic,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields,
                    ) = match &value {
                        PostgresqlTypeFilterInitializedWithTryNew::Between => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote! {
                                StartMoreOrEqualToEnd {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    start: T,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    end: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote! {: std::cmp::PartialOrd},
                            &quote::quote! {
                                start: T,
                                end: T,
                            },
                            &quote::quote! {
                                if start < end {//removed .0
                                    Ok(Self {
                                        logical_operator,
                                        start,
                                        end
                                    })
                                } else {
                                    Err(#ident_try_new_error_named::StartMoreOrEqualToEnd {
                                        start,
                                        end,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            Some(quote::quote! {+ std::cmp::PartialOrd}),
                            &vec![&start_t_field, &end_t_field],
                        ),
                        PostgresqlTypeFilterInitializedWithTryNew::In => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote! {
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                NotUnique {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote! {: PartialEq + Clone},
                            &value_std_vec_vec_t_token_stream,
                            &quote::quote! {
                                if value.is_empty() {
                                    return Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                                {
                                    let mut acc = vec![];
                                    for element in &value {
                                        if !acc.contains(&element) {
                                            acc.push(element);
                                        } else {
                                            return Err(#ident_try_new_error_named::NotUnique {
                                                value: element.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                Ok(Self { logical_operator, value })
                            },
                            Some(quote::quote! {+ PartialEq + Clone}),
                            &vec![&value_std_vec_vec_t_field],
                        ),
                        PostgresqlTypeFilterInitializedWithTryNew::RangeLength => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote! {
                                LengthIsNegativeOrZero {//todo rename
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #unsigned_part_of_std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_declaration_token_stream,
                            &quote::quote! {
                                if value.get() > 0 {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::LengthIsNegativeOrZero {
                                        value,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            None,
                            &vec![&value_unsigned_part_of_std_primitive_i32_field],
                        ),
                    };
                    generate_try_new_logic_token_stream(
                        &ident,
                        &ident_try_new_error_named,
                        should_add_declaration_of_struct_ident_generic,
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields,
                    )
                }
                Err(_) => proc_macro2::TokenStream::new(),
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream =
                generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&should_add_declaration_of_struct_ident_generic, &ident, &impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream);
            let impl_postgresql_type_where_filter_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                &FilterType::PostgresqlType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &query_part_content_token_stream,
                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                &query_bind_content_token_stream,
            );
            let generated = quote::quote! {
                #struct_token_stream
                #maybe_try_new_logic_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                #impl_postgresql_type_where_filter_token_stream
            };
            // if let postgresql_crud_macros_common::PostgresqlTypeFilter:: = &filter {
            //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //         "GeneratePostgresqlTypeWhereElementFilter",
            //         &generated,
            //     );
            // }
            generated
        };
        let filter_array_token_stream = postgresql_crud_macros_common::PostgresqlTypeFilter::into_array().map(|element| generate_filters_token_stream(&element));
        // let _token_stream = generate_filters_token_stream(&postgresql_crud_macros_common::PostgresqlTypeFilter::);
        quote::quote! {
            #(#filter_array_token_stream)*
            //#_token_stream
        }
    };
    let postgresql_json_type_token_stream = {
        #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum PostgresqlJsonTypeFilterInitializedWithTryNew {
            Between,
            In,
            OverlapsWithArray,
        }
        impl std::convert::TryFrom<&postgresql_crud_macros_common::PostgresqlJsonTypeFilter> for PostgresqlJsonTypeFilterInitializedWithTryNew {
            type Error = ();
            fn try_from(value: &postgresql_crud_macros_common::PostgresqlJsonTypeFilter) -> Result<Self, Self::Error> {
                match &value {
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between {
                        ident: _
                    } => Ok(Self::Between),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                        ident: _
                    } => Ok(Self::In),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                        ident: _
                    } => Err(()),
                    // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray {
                        ident: _
                    } => Ok(Self::OverlapsWithArray),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression => Err(()),
                }
            }
        }
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlJsonTypeFilter| {
            let ident = naming::parameter::PostgresqlJsonTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let ident_try_new_error_named = naming::parameter::PostgresqlJsonTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
            let query_bind_sqlx_types_json_self_value_token_stream = quote::quote! {
                query = query.bind(sqlx::types::Json(self.value));
                query
            };
            #[derive(Clone)]
            enum DimensionNumber {
                One,
                Two,
                Three,
                Four
            }
            impl std::convert::Into<std::primitive::u8> for DimensionNumber {
                fn into(self) -> std::primitive::u8 {
                    match &self {
                        Self::One => 1,
                        Self::Two => 2,
                        Self::Three => 3,
                        Self::Four => 4
                    }
                }
            }
            let generate_dimension_position_number_operation_token_stream = |
                dimension_number: &DimensionNumber,
                operator: &dyn std::fmt::Display,
            | -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                let dimension_number_std_primitive_u8 = std::convert::Into::<std::primitive::u8>::into(dimension_number.clone());
                let dimension_number_std_primitive_u8_plus_one = dimension_number_std_primitive_u8.checked_add(1).unwrap();
                (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    {
                        let struct_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #unsigned_part_of_std_primitive_i32_token_stream
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#struct_additional_fields_token_stream),*,
                            value: T,
                        }
                    },
                    {
                        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #core_default_default_default_token_stream,
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote!{
                            #(#impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream)*
                            value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        }
                    },
                    {
                        let increments_initialization_token_stream = (1..=dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            let increment_number_token_stream = format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #increment_number_token_stream = match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        value
                                    },
                                    None => {
                                        return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                    },
                                };
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        let format_handle_token_stream = {
                            let indexes = (1..=dimension_number_std_primitive_u8).into_iter().fold(std::string::String::new(), |mut acc, _| {
                                acc.push_str(
                                    &"->${}"
                                );
                                acc
                            });
                            generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{indexes} {operator} ${{}})"))
                        };
                        let format_increments_token_stream = (1..=dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap()
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#increments_initialization_token_stream)*
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #(#format_increments_token_stream),*
                            ))
                        }
                    },
                    {
                        let query_bind_dimension_position_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                query = query.bind(self.#dimension_number_position_token_stream);
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#query_bind_dimension_position_token_stream)*
                            query = query.bind(sqlx::types::Json(self.value));
                            query
                        }
                    },
                )
            };
            let generate_dimension_position_equal_token_stream = |dimension_number: &DimensionNumber|generate_dimension_position_number_operation_token_stream(&dimension_number, &"=");
            let generate_dimension_position_greater_than_token_stream = |dimension_number: &DimensionNumber|generate_dimension_position_number_operation_token_stream(&dimension_number, &">");
            let generate_dimension_length_operation_token_stream = |
                dimension_number: &DimensionNumber,
                operator: &dyn std::fmt::Display,
            | -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                let dimension_number_std_primitive_u8 = std::convert::Into::<std::primitive::u8>::into(dimension_number.clone());
                let dimension_number_std_primitive_u8_plus_one = dimension_number_std_primitive_u8.checked_add(1).unwrap();
                (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    {
                        let struct_additional_fields_token_stream = (1..dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #unsigned_part_of_std_primitive_i32_token_stream,
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#struct_additional_fields_token_stream)*
                            value: #unsigned_part_of_std_primitive_i32_token_stream
                        }
                    },
                    {
                        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream = (1..dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #core_default_default_default_token_stream,
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream)*
                            value: #core_default_default_default_token_stream
                        }
                    },
                    {
                        let increments_initialization_token_stream = (1..dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            let increment_number_token_stream = format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #increment_number_token_stream = match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        value
                                    },
                                    None => {
                                        return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                    },
                                };
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        let format_handle_token_stream = {
                            let indexes = (1..dimension_number_std_primitive_u8).into_iter().fold(std::string::String::new(), |mut acc, _| {
                                acc.push_str(
                                    &"->${}"
                                );
                                acc
                            });
                            generate_quotes::double_quotes_token_stream(&format!("{{}}(jsonb_array_length({{}}{indexes}) {operator} ${{}})"))
                        };
                        let format_increments_token_stream = (1..dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap()
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#increments_initialization_token_stream)*
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #(#format_increments_token_stream),*
                            ))
                        }
                    },
                    {
                        let query_bind_dimension_position_token_stream = (1..dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                query = query.bind(self.#dimension_number_position_token_stream);
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#query_bind_dimension_position_token_stream)*
                            query = query.bind(self.value);
                            query
                        }
                    }
                )
            };
            let generate_dimension_length_equal_token_stream = |dimension_number: &DimensionNumber|generate_dimension_length_operation_token_stream(&dimension_number, &"=");
            let generate_dimension_length_more_than_token_stream = |dimension_number: &DimensionNumber|generate_dimension_length_operation_token_stream(&dimension_number, &">");
            let generate_dimension_position_regular_expression_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                let dimension_number_std_primitive_u8 = std::convert::Into::<std::primitive::u8>::into(dimension_number.clone());
                (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    {
                        let struct_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #unsigned_part_of_std_primitive_i32_token_stream,
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#struct_additional_fields_token_stream)*
                            pub regular_expression_case: crate::RegularExpressionCase,
                            pub value: crate::RegexRegex
                        }
                    },
                    {
                        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #core_default_default_default_token_stream,
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream)*
                            regular_expression_case: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        }
                    },
                    {
                        let increments_initialization_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let increment_number_token_stream = format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #increment_number_token_stream = match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        value
                                    },
                                    None => {
                                        return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                    },
                                };
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        let format_handle_token_stream = {
                            let indexes = (2..=dimension_number_std_primitive_u8).into_iter().fold(std::string::String::new(), |mut acc, _| {
                                acc.push_str(
                                    &"->${}"
                                );
                                acc
                            });
                            generate_quotes::double_quotes_token_stream(&format!(
                                "{{}}((trim(both '\\\"' from ({{}}{indexes}->>${{}})::text) {{}} ${{}}))"
                            ))
                        };
                        let format_increments_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap()
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#increments_initialization_token_stream)*
                            let last_increment = match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    value
                                },
                                None => {
                                    return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                },
                            };
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #(#format_increments_token_stream),*,
                                self.regular_expression_case.postgreql_syntax(),
                                last_increment
                            ))
                        }
                    },
                    {
                        let query_bind_dimension_position_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                query = query.bind(self.#dimension_number_position_token_stream);
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#query_bind_dimension_position_token_stream)*
                            query = query.bind(self.value.to_string());
                            query
                        }
                    },
                )
            };
            let generate_dimension_position_contains_all_elements_of_array_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                let dimension_number_std_primitive_u8 = std::convert::Into::<std::primitive::u8>::into(dimension_number.clone());
                let dimension_number_std_primitive_u8_plus_one = dimension_number_std_primitive_u8.checked_add(1).unwrap();
                (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    {
                        let struct_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #unsigned_part_of_std_primitive_i32_token_stream
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#struct_additional_fields_token_stream),*,
                            value: crate::NotEmptyUniqueVec<T>
                        }
                    },
                    {
                        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                #dimension_number_position_token_stream: #core_default_default_default_token_stream
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream),*,
                            value: //here
                        }
                    },
                    {
                        let increments_initialization_token_stream = (1..=dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            let increment_number_token_stream = format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #increment_number_token_stream = match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        value
                                    },
                                    None => {
                                        return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                    },
                                };
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        let format_handle_token_stream = {
                            let indexes = (1..=dimension_number_std_primitive_u8).into_iter().fold(std::string::String::new(), |mut acc, _| {
                                acc.push_str(
                                    &"->${}"
                                );
                                acc
                            });
                            generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{indexes} @> ${{}})"))
                        };
                        let format_increments_token_stream = (1..=dimension_number_std_primitive_u8_plus_one).into_iter().map(|element|{
                            format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap()
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#increments_initialization_token_stream)*
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #(#format_increments_token_stream),*
                            ))
                        }
                    },
                    {
                        let query_bind_dimension_position_token_stream = (1..=dimension_number_std_primitive_u8).into_iter().map(|element|{
                            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                query = query.bind(self.#dimension_number_position_token_stream);
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                        quote::quote! {
                            #(#query_bind_dimension_position_token_stream)*
                            query = query.bind(sqlx::types::Json(self.value));
                            query
                        }
                    },
                )
            };
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                query_part_content_token_stream,
                query_bind_content_token_stream
            ) = match &filter {
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    value_t_token_stream.clone(),
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("=")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    pub_value_t_token_stream.clone(),
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    generate_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {
                        start: T,
                        end: T,
                    },
                    quote::quote! {
                        start: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        end: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    },
                    quote::quote! {
                        match increment.checked_add(1) {
                            Some(first_value) => {
                                *increment = first_value;
                                match increment.checked_add(1) {
                                    Some(second_value) => {
                                        *increment = second_value;
                                        let between_snake_case = naming::BetweenSnakeCase;
                                        let and_snake_case = naming::AndSnakeCase;
                                        Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                                    }
                                    None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                                }
                            }
                            None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                        }
                    },
                    quote::quote! {
                        query = query.bind(sqlx::types::Json(self.start));//here change
                        query = query.bind(sqlx::types::Json(self.end));//here change
                        query
                    },
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    value_std_vec_vec_t_token_stream.clone(),
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream]
                    },
                    quote::quote! {
                        let mut acc = std::string::String::default();
                        for element in &self.value {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc.push_str(&format!("${},", value));
                                }
                                None => {
                                    return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                                }
                            }
                        }
                        let _ = acc.pop();
                        let in_snake_case = naming::InSnakeCase;
                        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
                    },
                    quote::quote! {
                        for element in self.value {
                            query = query.bind(sqlx::types::Json(element));
                        }
                        query
                    },
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    regular_expression_case_and_value_declaration_token_stream.clone(),
                    regular_expression_case_and_value_default_initialization_token_stream.clone(),
                    generate_query_part_regular_expression_token_stream(&quote::quote!{"{}(trim(both '\"' from ({})::text) {} ${})"}),
                    query_equals_query_self_value_to_string_token_stream.clone()
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                    ident: _
                } => generate_dimension_position_equal_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                    ident: _
                } => generate_dimension_position_equal_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionEqual {
                    ident: _
                } => generate_dimension_position_equal_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionEqual {
                    ident: _
                } => generate_dimension_position_equal_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionGreaterThan { 
                    ident: _
                } => generate_dimension_position_greater_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionGreaterThan { 
                    ident: _
                } => generate_dimension_position_greater_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionGreaterThan { 
                    ident: _
                } => generate_dimension_position_greater_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionGreaterThan { 
                    ident: _
                } => generate_dimension_position_greater_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionRegularExpression => generate_dimension_position_regular_expression_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionRegularExpression => generate_dimension_position_regular_expression_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionRegularExpression => generate_dimension_position_regular_expression_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionRegularExpression => generate_dimension_position_regular_expression_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                    ident: _
                } => 
                // generate_dimension_position_contains_all_elements_of_array_token_stream(&DimensionNumber::One),
                (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&quote::quote! {"{}(exists (select 1 from jsonb_array_elements_text({}) as e1 join jsonb_array_elements_text(${}) as e2 on e1.value = e2.value))"}),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {pub value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&quote::quote! {"{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <> ${}))"}),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {pub value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&quote::quote! {"{}(exists(select 1 from jsonb_array_elements({}) as el where (el) > ${}))"}),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan { 
                    ident: _
                } => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    quote::quote! {pub value: std::vec::Vec<T>},
                    quote::quote! {
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream],
                    },
                    generate_query_part_one_value_token_stream(&quote::quote! {"{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <= ${}))"}),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    regular_expression_case_and_value_declaration_token_stream.clone(),
                    regular_expression_case_and_value_default_initialization_token_stream.clone(),
                    generate_query_part_regular_expression_token_stream(&quote::quote!{"{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))"}),
                    query_equals_query_self_value_to_string_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    regular_expression_case_and_value_declaration_token_stream.clone(),
                    regular_expression_case_and_value_default_initialization_token_stream.clone(),
                    generate_query_part_regular_expression_token_stream(&quote::quote!{"{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))"}),
                    query_equals_query_self_value_to_string_token_stream.clone(),
                ),
            };
            let filter_initialized_with_try_new_result = PostgresqlJsonTypeFilterInitializedWithTryNew::try_from(filter);
            let struct_token_stream = generate_struct_token_stream(
                filter_initialized_with_try_new_result.is_ok(),
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &struct_additional_fields_token_stream,
            );
            let maybe_try_new_logic_token_stream = match filter_initialized_with_try_new_result {
                Ok(value) => {
                    let (
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields,
                    ) = match &value {
                        PostgresqlJsonTypeFilterInitializedWithTryNew::Between => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote! {
                                StartMoreOrEqualToEnd {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    start: T,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    end: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &quote::quote! {: std::cmp::PartialOrd},
                            &quote::quote! {
                                start: T,
                                end: T,
                            },
                            &quote::quote! {
                                if start < end {//removed .0
                                    Ok(Self {
                                        logical_operator,
                                        start,
                                        end
                                    })
                                } else {
                                    Err(#ident_try_new_error_named::StartMoreOrEqualToEnd {
                                        start,
                                        end,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            Some(quote::quote! {+ std::cmp::PartialOrd}),
                            &vec![&start_t_field, &end_t_field],
                        ),
                        PostgresqlJsonTypeFilterInitializedWithTryNew::In => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote! {
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                NotUnique {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &quote::quote! {: PartialEq + Clone},
                            &value_std_vec_vec_t_token_stream,
                            &quote::quote! {
                                if value.is_empty() {
                                    return Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                                {
                                    let mut acc = vec![];
                                    for element in &value {
                                        if !acc.contains(&element) {
                                            acc.push(element);
                                        } else {
                                            return Err(#ident_try_new_error_named::NotUnique {
                                                value: element.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                Ok(Self { logical_operator, value })
                            },
                            Some(quote::quote! {+ std::cmp::PartialOrd + Clone}),
                            &vec![&value_std_vec_vec_t_field],
                        ),
                        PostgresqlJsonTypeFilterInitializedWithTryNew::OverlapsWithArray => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote! {
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                NotUnique {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &quote::quote! {: std::cmp::PartialEq + Clone},
                            &quote::quote! {value: std::vec::Vec<T>},
                            &quote::quote! {
                                if value.is_empty() {
                                    return Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                                {
                                    let mut acc = vec![];
                                    for element in &value {
                                        if !acc.contains(&element) {
                                            acc.push(element);
                                        } else {
                                            return Err(#ident_try_new_error_named::NotUnique {
                                                value: element.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                Ok(Self { logical_operator, value })
                            },
                            Some(quote::quote! {+ std::cmp::PartialEq + Clone}),
                            &vec![&value_std_vec_vec_t_field],
                        ),
                    };
                    generate_try_new_logic_token_stream(
                        &ident,
                        &ident_try_new_error_named,
                        &should_add_declaration_of_struct_ident_generic,
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields,
                    )
                }
                Err(_) => proc_macro2::TokenStream::new(),
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
            );
            let impl_postgresql_type_where_filter_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                &FilterType::PostgresqlJsonType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &query_part_content_token_stream,
                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                &query_bind_content_token_stream,
            );
            //todo pub fields or not
            let f = quote::quote! {
                #struct_token_stream
                #maybe_try_new_logic_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                #impl_postgresql_type_where_filter_token_stream
            };
            match &filter {
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {ident: _} => {
                    proc_macro2::TokenStream::new()
                }
                _ => f
            }
            // f
        };
        let filter_array_token_stream = postgresql_crud_macros_common::PostgresqlJsonTypeFilter::into_array().map(|element| generate_filters_token_stream(&element));
        // let _token_stream = generate_filters_token_stream(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::);
        let generated = quote::quote! {
            #(#filter_array_token_stream)*
            // #_token_stream
        };
        // if let postgresql_crud_macros_common::PostgresqlTypeFilter:: = &filter {
        //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //         "GeneratePostgresqlTypeWhereElementFilter",
        //         &generated,
        //     );
        // }
        generated
    };
    let generated = quote::quote! {
        #postgresql_type_token_stream
        #postgresql_json_type_token_stream
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypeWhereElementFilters",
    //     &generated,
    // );
    generated.into()
}
