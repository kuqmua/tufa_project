#[proc_macro]
pub fn generate_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let dimensions_snake_case = naming::DimensionsSnakeCase;
    let t_token_stream = quote::quote! {T};
    let t_annotation_generic_token_stream = quote::quote! {<#t_token_stream>};
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    //todo reuse ?
    fn generate_core_default_default_default_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{::core::default::Default::default()}
    }
    let core_default_default_default_token_stream = generate_core_default_default_default_token_stream();
    fn generate_path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream() -> proc_macro2::TokenStream {
        quote::quote! {
            crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
        }
    }
    let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
    // let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = quote::quote!{
    //     crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    // };
    let pub_value_t_token_stream = quote::quote! {pub value: T};
    //todo rewrite it as UniqueVec
    fn generate_unsigned_part_of_std_primitive_i32_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{crate::UnsignedPartOfStdPrimitiveI32}
    }
    let unsigned_part_of_std_primitive_i32_token_stream = generate_unsigned_part_of_std_primitive_i32_token_stream();
    fn generate_not_zero_unsigned_part_of_std_primitive_i32_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{crate::NotZeroUnsignedPartOfStdPrimitiveI32}
    }
    let not_zero_unsigned_part_of_std_primitive_i32_token_stream = generate_not_zero_unsigned_part_of_std_primitive_i32_token_stream();
    let value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream = quote::quote! {value: #not_zero_unsigned_part_of_std_primitive_i32_token_stream};
    let pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream = quote::quote! {pub #value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream};
    #[derive(Clone)]
    enum ShouldAddDeclarationOfStructIdentGeneric {
        True {
            maybe_additional_traits_token_stream: std::option::Option<proc_macro2::TokenStream>
        },
        False,
    }
    struct Field<'a> {
        field_name: &'a dyn std::fmt::Display,
        field_type: &'a dyn quote::ToTokens,
    }
    let value_not_zero_unsigned_part_of_std_primitive_i32_field = Field {
        field_name: &naming::ValueSnakeCase,
        field_type: &not_zero_unsigned_part_of_std_primitive_i32_token_stream, //todo i32 or i64 or something between? or more? or less?
    };
    fn generate_value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream() -> proc_macro2::TokenStream {
        let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
        quote::quote! {
            value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
        }
    }
    let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
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
            ShouldAddDeclarationOfStructIdentGeneric::True {
                maybe_additional_traits_token_stream
            } => &match maybe_additional_traits_token_stream {
                Some(value) => quote::quote! {<#t_token_stream: #value>},
                None => quote::quote! {<#t_token_stream>},
            },
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
                ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: _
                } => &quote::quote! {<T #generic_requirements_token_stream>},
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            };
            let ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: _
                } => &t_annotation_generic_token_stream,
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
            let maybe_struct_visitor_where_generic_trait_annotation_token_stream = match &option_additional_traits_annotations_token_stream {
                Some(value) => quote::quote! {
                    where
                        T: _serde::Deserialize<'de> #value,
                },
                None => proc_macro2::TokenStream::new()
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
    let generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = |
        should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
        ident: &dyn quote::ToTokens,
        impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream: &dyn quote::ToTokens
    | {
        postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream
                } => match &maybe_additional_traits_token_stream {
                    Some(value) => quote::quote! {<T: #value + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
                    None => quote::quote! {<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>}
                },
                ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new(),
            },
            &postgresql_crud_macros_common::ImportPath::Crate,
            &ident,
            match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: _
                } => &t_annotation_generic_token_stream,
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
                    ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream } => {
                        let send_and_lifetime_token_stream = quote::quote!{std::marker::Send + 'a};
                        let serde_serialize_token_stream = quote::quote!{serde::Serialize};
                        let content_token_stream = match (&filter_type, &maybe_additional_traits_token_stream) {
                            (FilterType::PostgresqlType, Some(value)) => &quote::quote! {#value + #send_and_lifetime_token_stream},
                            (FilterType::PostgresqlType, None) => &send_and_lifetime_token_stream,
                            (FilterType::PostgresqlJsonType, Some(value)) => &quote::quote! {#value + #serde_serialize_token_stream + #send_and_lifetime_token_stream},
                            (FilterType::PostgresqlJsonType, None) => &quote::quote! {#serde_serialize_token_stream + #send_and_lifetime_token_stream},
                        };
                        &quote::quote! {, T: #content_token_stream}
                    },
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {<'a #maybe_t_additional_traits_for_postgresql_type_where_filter_token_stream>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream: _ } => &t_annotation_generic_token_stream,
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
        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
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
    let equal_sign = "=";
    let greater_than_sign = ">";
    let query_bind_one_value_token_stream = quote::quote! {
        query = query.bind(self.value);
        query
    };
    let should_add_declaration_of_struct_ident_generic_false = ShouldAddDeclarationOfStructIdentGeneric::False;
    let should_add_declaration_of_struct_ident_generic_true_none = ShouldAddDeclarationOfStructIdentGeneric::True {
        maybe_additional_traits_token_stream: None
    };
    fn generate_match_increment_checked_add_one_initialization_token_stream(ident_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let crate_query_part_error_named_checked_add_initialization_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_checked_add_initialization_token_stream();
        quote::quote!{
            let #ident_token_stream = match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    value
                }
                None => {
                    return Err(#crate_query_part_error_named_checked_add_initialization_token_stream);
                },
            };
        }
    }
    enum IsZeroCanBeInDimensionPosition {
        True,
        False
    }
    fn generate_struct_additional_fields_token_stream<T>(value: T, is_zero_can_be_in_dimension_position: &IsZeroCanBeInDimensionPosition) -> proc_macro2::TokenStream
    where
        T: IntoIterator<Item = std::primitive::u8>,
    {
        let type_token_stream = match &is_zero_can_be_in_dimension_position {
            IsZeroCanBeInDimensionPosition::True => generate_unsigned_part_of_std_primitive_i32_token_stream(),
            IsZeroCanBeInDimensionPosition::False => generate_not_zero_unsigned_part_of_std_primitive_i32_token_stream()
        };
        let content_token_stream = value.into_iter().map(|element|{
            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
            quote::quote! {
                pub #dimension_number_position_token_stream: #type_token_stream,
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    }
    fn generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream<T>(value: T) -> proc_macro2::TokenStream
    where
        T: IntoIterator<Item = std::primitive::u8>,
    {
        let content_token_stream = value.into_iter().map(|element|{
            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
            let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
            quote::quote! {
                #dimension_number_position_token_stream: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    }
    fn generate_increments_initialization_token_stream<T>(value: T) -> proc_macro2::TokenStream
    where
        T: IntoIterator<Item = std::primitive::u8>,
    {
        let content_token_stream = value.into_iter().map(|element|{
            generate_match_increment_checked_add_one_initialization_token_stream(&format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap())
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    }
    fn generate_format_increments_token_stream<T>(value: T) -> proc_macro2::TokenStream
    where
        T: IntoIterator<Item = std::primitive::u8>,
    {
        let content_token_stream = value.into_iter().map(|element|{
            let value = format!("increment{element}").parse::<proc_macro2::TokenStream>().unwrap();
            quote::quote!{#value,}
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    }
    fn generate_query_bind_dimension_position_token_stream<T>(value: T) -> proc_macro2::TokenStream
    where
        T: IntoIterator<Item = std::primitive::u8>,
    {
        let content_token_stream = value.into_iter().map(|element|{
            let dimension_number_position_token_stream = format!("dimension{element}_position").parse::<proc_macro2::TokenStream>().unwrap();
            quote::quote! {
                query = query.bind(self.#dimension_number_position_token_stream);
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    }
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone = ShouldAddDeclarationOfStructIdentGeneric::True {
        maybe_additional_traits_token_stream: Some(quote::quote!{std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone})
    };
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
        maybe_additional_traits_token_stream: Some(quote::quote!{
            std::fmt::Debug
            + std::cmp::PartialEq
            + PartialOrd
            + std::clone::Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
        })
    };
    let value_between_t_token_stream = quote::quote!{value: crate::Between<T>};
    let pub_value_between_t_token_stream = quote::quote!{pub #value_between_t_token_stream};
    fn generate_query_self_value_query_bind_token_stream() -> proc_macro2::TokenStream {
        quote::quote! {
            query = self.value.query_bind(query);
            query
        }
    }
    let query_self_value_query_bind_token_stream = generate_query_self_value_query_bind_token_stream();
    let query_self_value_query_bind_one_by_one_token_stream = quote::quote! {
        query = self.value.query_bind_one_by_one(query);
        query
    };
    #[derive(Clone)]
    enum DimensionNumber {
        One,
        Two,
        Three,
        Four
    }
    impl DimensionNumber {
        fn dimension_std_primitive_u8(&self) -> std::primitive::u8 {
            match &self {
                Self::One => 1,
                Self::Two => 2,
                Self::Three => 3,
                Self::Four => 4
            }
        }
        fn dimension_token_stream(&self) -> proc_macro2::TokenStream {
            self.dimension_std_primitive_u8().to_string().parse::<proc_macro2::TokenStream>().unwrap()
        }
        fn dimension_minus_one_token_stream(&self) -> proc_macro2::TokenStream {
            self.dimension_std_primitive_u8().saturating_sub(1).to_string().parse::<proc_macro2::TokenStream>().unwrap()
        }
        // fn dimension_plus_one_token_stream(&self) -> proc_macro2::TokenStream {
        //     self.dimension_std_primitive_u8().checked_add(1).unwrap().to_string().parse::<proc_macro2::TokenStream>().unwrap()
        // }
    }
    enum KindOfUnsignedPartOfStdPrimitiveI32 {
        CanBeZero,
        CanNotBeZero
    }
    impl quote::ToTokens for KindOfUnsignedPartOfStdPrimitiveI32 {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                KindOfUnsignedPartOfStdPrimitiveI32::CanBeZero => quote::quote!{UnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
                KindOfUnsignedPartOfStdPrimitiveI32::CanNotBeZero => quote::quote!{NotZeroUnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
            }
        }
    }
    let generate_pub_dimensions_bounded_vec_token_stream = |
        vec_length_token_stream: &dyn quote::ToTokens,
        kind_of_unsigned_part_of_std_primitive_i32: &KindOfUnsignedPartOfStdPrimitiveI32,
    |{
        quote::quote! {pub #dimensions_snake_case: crate::BoundedStdVecVec<crate::#kind_of_unsigned_part_of_std_primitive_i32, #vec_length_token_stream>}
    };
    let value_match_increment_checked_add_one_initialization_token_stream = generate_match_increment_checked_add_one_initialization_token_stream(&quote::quote!{value});
    let generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream = |
        ident_token_stream: &dyn quote::ToTokens,
        field_token_stream: &dyn quote::ToTokens,
        function_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            let #ident_token_stream = match self.#field_token_stream.#function_token_stream(increment, column, is_need_to_add_logical_operator) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            };
        }
    };
    let value_match_self_value_query_part_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
        &value_snake_case,
        &value_snake_case,
        &quote::quote!{query_part}
    );
    let generate_ok_format_token_stream = |format_handle_token_stream: &dyn quote::ToTokens, other_parameters: &dyn quote::ToTokens|{
        quote::quote!{
            Ok(format!(
                #format_handle_token_stream,
                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                column,
                dimensions_indexes,
                #other_parameters
            ))
        }
    };
    let generate_ok_format_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
        generate_ok_format_token_stream(format_handle_token_stream, &quote::quote!{value})
    };
    let dimensions_default_initialization_token_stream = quote::quote!{
        #dimensions_snake_case: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
    };
    let query_self_dimensions_query_bind_query_token_stream = quote::quote!{query = self.#dimensions_snake_case.query_bind(query);};
    let postgresql_type_token_stream = {
        let is_zero_can_be_in_dimension_position_false = IsZeroCanBeInDimensionPosition::False;
        #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum PostgresqlTypeFilterInitializedWithTryNew {
            RangeLength,
            DimensionOneRangeLength,
        }
        impl std::convert::TryFrom<&postgresql_crud_macros_common::PostgresqlTypeFilter> for PostgresqlTypeFilterInitializedWithTryNew {
            type Error = ();
            fn try_from(value: &postgresql_crud_macros_common::PostgresqlTypeFilter) -> Result<Self, Self::Error> {
                match &value {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthMoreThan => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ContainedWithinRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneContainedWithinRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ContainsAnotherRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneContainsAnotherRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanLowerBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanLowerBound {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange {
                        ident: _
                    } => Err(()),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => Ok(Self::RangeLength),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength => Ok(Self::DimensionOneRangeLength),
                }
            }
        }
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlTypeFilter| {
            let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let ident_try_new_error_named = naming::parameter::PostgresqlTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                query_part_content_token_stream,
                query_bind_content_token_stream
            ) = {
                let range_1_1 = 1..=1;
                fn generate_postgresql_array_indexes_stringified<T>(value: T) -> std::string::String
                where
                    T: IntoIterator<Item = std::primitive::u8>,
                {
                    value.into_iter().fold(std::string::String::new(), |mut acc, _| {
                        acc.push_str(&"[${}]");
                        acc
                    })
                }
                let value_t_range_1_1_declaration_token_stream = {
                    let struct_additional_fields_token_stream = generate_struct_additional_fields_token_stream(range_1_1.clone(), &is_zero_can_be_in_dimension_position_false);
                    quote::quote! {
                        #struct_additional_fields_token_stream
                        #pub_value_t_token_stream
                    }
                };
                let value_t_range_1_1_default_initialization_token_stream = {
                    let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream = generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream(
                        range_1_1.clone()
                    );
                    quote::quote! {
                        #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    }
                };
                let generate_value_t_range_1_1_query_part_token_stream = |operator: &dyn std::fmt::Display|{
                    let increments_initialization_token_stream = generate_increments_initialization_token_stream(range_1_1.clone());
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                        "{{}}({{}}{} {operator} ${{}})",
                        generate_postgresql_array_indexes_stringified(range_1_1.clone())
                    ));
                    let format_increments_token_stream = generate_format_increments_token_stream(range_1_1.clone());
                    quote::quote! {
                        #increments_initialization_token_stream
                        #value_match_increment_checked_add_one_initialization_token_stream
                        Ok(format!(
                            #format_handle_token_stream,
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            column,
                            #format_increments_token_stream
                            value
                        ))
                    }
                };
                let value_t_range_query_bind_content_token_stream = {
                    let query_bind_dimension_position_token_stream = generate_query_bind_dimension_position_token_stream(range_1_1.clone());
                    quote::quote! {
                        #query_bind_dimension_position_token_stream
                        #query_bind_one_value_token_stream
                    }
                };
                let should_add_declaration_of_struct_ident_generic_true_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: Some(quote::quote!{sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>})
                };
                let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: Some(quote::quote!{std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>})
                };
                let dimensions_indexes_postgresql_type_query_part_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                    &quote::quote!{dimensions_indexes},
                    &dimensions_snake_case,
                    &quote::quote!{postgresql_type_query_part}
                );
                let dimension_number_one = DimensionNumber::One;
                let dimension_number_one_token_stream = dimension_number_one.dimension_token_stream();
                let generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream = |vec_length_token_stream: &dyn quote::ToTokens|{
                    generate_pub_dimensions_bounded_vec_token_stream(&vec_length_token_stream, &KindOfUnsignedPartOfStdPrimitiveI32::CanNotBeZero)
                };
                let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream = generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream(&dimension_number_one_token_stream);
                let generate_dimension_6bad7b4b_e612_42bd_8464_915d8e717255_token_stream = |
                    dimension_number: &DimensionNumber,
                    operator: &dyn std::fmt::Display
                | -> (
                    ShouldAddDeclarationOfStructIdentGeneric,
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                ) {
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        {
                            let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream(&dimension_number.dimension_token_stream());
                            quote::quote! {
                                #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream,
                                #pub_value_t_token_stream
                            }
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_value_token_stream(&generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{{}} {operator} ${{}})")));
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            #query_bind_one_value_token_stream
                        }
                    )
                };
                let pub_value_postgresql_type_not_empty_unique_vec_t_token_stream = quote::quote!{pub value: crate::PostgresqlTypeNotEmptyUniqueVec<T>};
                match &filter {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&equal_sign)),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual { ident: _ } => generate_dimension_6bad7b4b_e612_42bd_8464_915d8e717255_token_stream(
                        &dimension_number_one,
                        &equal_sign
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&greater_than_sign)),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan { ident: _ } => generate_dimension_6bad7b4b_e612_42bd_8464_915d8e717255_token_stream(
                        &dimension_number_one,
                        &greater_than_sign
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        pub_value_between_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        {
                            quote::quote! {
                                #value_match_self_value_query_part_initialization_token_stream
                                Ok(format!(
                                    "{}({} {})",
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    value
                                ))
                            }
                        },
                        query_self_value_query_bind_token_stream.clone()
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            #pub_value_between_t_token_stream
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_value_token_stream(&quote::quote!{"{}({}{} {})"});
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_self_value_query_part_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            #query_self_value_query_bind_token_stream
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone_type_encode.clone(),
                        pub_value_postgresql_type_not_empty_unique_vec_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        quote::quote! {
                            let mut acc = std::string::String::default();
                            for element in self.value.to_vec() {
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
                            Ok(format!(
                                "{}({} in ({}))",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                acc
                            ))
                        },
                        quote::quote! {
                            for element in self.value.into_vec() {
                                query = query.bind(element);
                            }
                            query
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone_type_encode.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            #pub_value_postgresql_type_not_empty_unique_vec_t_token_stream
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_value_token_stream(&quote::quote!{"{}({}{} in ({}))"});
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                let value = {
                                    let mut acc = std::string::String::default();
                                    for element in self.value.to_vec() {
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
                                    acc
                                };
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            for element in self.value.into_vec() {
                                query = query.bind(element);
                            }
                            query
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        regular_expression_case_and_value_declaration_token_stream.clone(),
                        regular_expression_case_and_value_default_initialization_token_stream.clone(),
                        generate_query_part_regular_expression_token_stream(&quote::quote!{"{}({} {} ${})"}),
                        query_equals_query_self_value_to_string_token_stream.clone()
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            #regular_expression_case_and_value_declaration_token_stream
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #regular_expression_case_and_value_default_initialization_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(
                                &quote::quote!{"{}({}{} {} ${})"},
                                &quote::quote!{
                                    self.regular_expression_case.postgreql_syntax(),
                                    value
                                }
                            );
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            #query_equals_query_self_value_to_string_token_stream
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("<")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            #pub_value_t_token_stream
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_value_token_stream(&quote::quote!{"{}({}{} < ${})"});
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            #query_bind_one_value_token_stream
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_date)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} = current_date)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_date)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} > current_date)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_timestamp)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} = current_timestamp)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_timestamp)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} > current_timestamp)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} = current_time)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} = current_time)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        proc_macro2_token_stream_new.clone(),
                        proc_macro2_token_stream_new.clone(),
                        generate_query_part_zero_value_token_stream(&quote::quote! {"{}({} > current_time)"}),
                        quote::quote! {#query_snake_case},
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream.clone(),
                        dimensions_default_initialization_token_stream.clone(),
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(&quote::quote!{"{}({}{} > current_time)"}, &proc_macro2::TokenStream::new());
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote!{
                            #query_self_dimensions_query_bind_query_token_stream
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(array_length({}, 1) = ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthMoreThan => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(array_length({}, 1) > ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            pub encode_format: crate::postgresql_type::EncodeFormat,
                            pub encoded_string_representation: std::string::String,
                        },
                        quote::quote! {
                            encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            encoded_string_representation: #core_default_default_default_token_stream
                        },
                        quote::quote! {
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                "{}(encode({}, '{}') = ${})",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                &self.encode_format,
                                value
                            ))
                        },
                        quote::quote! {
                            query = query.bind(self.encoded_string_representation);
                            query
                        }
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            pub encode_format: crate::postgresql_type::EncodeFormat,
                            pub encoded_string_representation: std::string::String,
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            encoded_string_representation: #core_default_default_default_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_token_stream(
                                &quote::quote!{"{}(encode({}{}, '{}') = ${})"},
                                &quote::quote!{
                                    &self.encode_format,
                                    value
                                }
                            );
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            query = query.bind(self.encoded_string_representation);
                            query
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ContainedWithinRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneContainedWithinRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        quote::quote! {
                            #pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_dimension_number_one_token_stream,
                            #pub_value_t_token_stream
                        },
                        quote::quote! {
                            #dimensions_default_initialization_token_stream,
                            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                        },
                        {
                            let ok_format_token_stream = generate_ok_format_value_token_stream(&quote::quote!{"{}({}{} @> ${})"});
                            quote::quote! {
                                #dimensions_indexes_postgresql_type_query_part_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                #ok_format_token_stream
                            }
                        },
                        quote::quote! {
                            #query_self_dimensions_query_bind_query_token_stream
                            #query_bind_one_value_token_stream
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ContainsAnotherRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneContainsAnotherRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&<")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&<")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&>")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&>")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(lower({}) = ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(lower({}) = ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(upper({}) = ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&quote::quote! {"{}(upper({}) = ${})"}),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanLowerBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&greater_than_sign)),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanLowerBound { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&greater_than_sign)),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&&")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&&")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("-|-")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange { ident: _ } => (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        pub_value_t_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("-|-")),
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => (
                        ShouldAddDeclarationOfStructIdentGeneric::False,
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        quote::quote! {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
                                }
                                None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                            }
                        },
                        query_bind_one_value_token_stream.clone(),
                    ),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength => (
                        ShouldAddDeclarationOfStructIdentGeneric::False,
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        quote::quote! {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
                                }
                                None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                            }
                        },
                        query_bind_one_value_token_stream.clone(),
                    ),
                }
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
                        PostgresqlTypeFilterInitializedWithTryNew::RangeLength => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote! {
                                LengthIsNegativeOrZero {//todo rename
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #not_zero_unsigned_part_of_std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream,
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
                            &vec![&value_not_zero_unsigned_part_of_std_primitive_i32_field],
                        ),
                        PostgresqlTypeFilterInitializedWithTryNew::DimensionOneRangeLength => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote! {
                                LengthIsNegativeOrZero {//todo rename
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #not_zero_unsigned_part_of_std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream,
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
                            &vec![&value_not_zero_unsigned_part_of_std_primitive_i32_field],
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
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
            );
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
            // match &filter {
            //     postgresql_crud_macros_common::PostgresqlTypeFilter::In {ident: _} => {
            //         macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //             "GeneratePostgresqlTypeWhereElementFilter",
            //             &generated,
            //         );
            //         proc_macro2::TokenStream::new()
            //     },
            //     _ => generated
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
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlJsonTypeFilter| {
            let ident = naming::parameter::PostgresqlJsonTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream = quote::quote!{pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>};
            let query_bind_sqlx_types_json_self_value_token_stream = quote::quote! {
                query = query.bind(sqlx::types::Json(self.value));
                query
            };
            let generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = |vec_length_token_stream: &dyn quote::ToTokens|{
                generate_pub_dimensions_bounded_vec_token_stream(&vec_length_token_stream, &KindOfUnsignedPartOfStdPrimitiveI32::CanBeZero)
            };
            let generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream = |dimension_number: &DimensionNumber|{
                generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream(&dimension_number.dimension_token_stream())
            };
            let generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = |dimension_number: &DimensionNumber|{
                generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream(&dimension_number.dimension_minus_one_token_stream())
            };
            let dimensions_default_value_default_initialization_token_stream = {
                let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
                quote::quote!{
                    #dimensions_default_initialization_token_stream,
                    #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                }
            };
            let dimensions_default_regular_expression_default_initialization_token_stream = quote::quote! {
                #dimensions_default_initialization_token_stream,
                #regular_expression_case_and_value_default_initialization_token_stream
            };
            let query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream = quote::quote!{
                #query_self_dimensions_query_bind_query_token_stream
                #query_bind_sqlx_types_json_self_value_token_stream
            };
            let query_dimensions_bind_query_equals_query_self_value_to_string_token_stream = quote::quote!{
                #query_self_dimensions_query_bind_query_token_stream
                #query_equals_query_self_value_to_string_token_stream
            };
            let dimensions_indexes_postgresql_json_type_query_part_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                &quote::quote!{dimensions_indexes},
                &dimensions_snake_case,
                &quote::quote!{postgresql_json_type_query_part}
            );
            let generate_dimension_array_number_operation_token_stream = |
                dimension_number: &DimensionNumber,
                operator: &dyn std::fmt::Display,
            | -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,
                            #pub_value_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(&generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{{}} {operator} ${{}})")));
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
                )
            };
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
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,
                            pub value: #unsigned_part_of_std_primitive_i32_token_stream
                        }
                    },
                    quote::quote! {
                        #dimensions_default_initialization_token_stream,
                        value: #core_default_default_default_token_stream
                    },
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&format!("{{}}(jsonb_array_length({{}}{{}}) {operator} ${{}})"))
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    quote::quote! {
                        #query_self_dimensions_query_bind_query_token_stream
                        #query_bind_one_value_token_stream
                    }
                )
            };
            let generate_dimension_equal_token_stream = |dimension_number: &DimensionNumber|generate_dimension_array_number_operation_token_stream(&dimension_number, &equal_sign);
            let generate_dimension_all_elements_equal_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #pub_value_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{{}}) as el where (el) <> ${{}}))"))
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
                )
            };
            let generate_dimension_length_equal_token_stream = |dimension_number: &DimensionNumber|generate_dimension_length_operation_token_stream(&dimension_number, &equal_sign);
            let generate_dimension_greater_than_token_stream = |dimension_number: &DimensionNumber|generate_dimension_array_number_operation_token_stream(&dimension_number, &greater_than_sign);
            let generate_dimension_contains_element_greater_than_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #pub_value_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&format!("{{}}(exists(select 1 from jsonb_array_elements({{}}{{}}) as el where (el) > ${{}}))"))
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
                )
            };
            let generate_dimension_all_elements_greater_than_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #pub_value_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <= ${}))")
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
                )
            };
            let query_self_value_query_bind_token_stream = generate_query_self_value_query_bind_token_stream();
            let generate_dimension_between_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,
                            #pub_value_between_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}({}{} {})")
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    quote::quote! {
                        #query_self_dimensions_query_bind_query_token_stream
                        #query_self_value_query_bind_token_stream
                    },
                )
            };
            let generate_dimension_in_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,
                            #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}({}{} in ({}))")
                        );
                        let value_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                            &value_snake_case,
                            &value_snake_case,
                            &quote::quote!{query_part_one_by_one}
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    quote::quote! {
                        #query_self_dimensions_query_bind_query_token_stream
                        #query_self_value_query_bind_one_by_one_token_stream
                    },
                )
            };
            let generate_dimension_regular_expression_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,
                            #regular_expression_case_and_value_declaration_token_stream
                        }
                    },
                    dimensions_default_regular_expression_default_initialization_token_stream.clone(),
                    {
                        let dimensions_indexes_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                            &quote::quote!{dimensions_indexes},
                            &dimensions_snake_case,
                            &quote::quote!{postgresql_json_type_query_part_minus_one}
                        );
                        let last_dimensions_index_intialization_token_stream = generate_match_increment_checked_add_one_initialization_token_stream(&quote::quote!{last_dimensions_index});
                        let ok_format_token_stream = generate_ok_format_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}((trim(both '\\\"' from ({}{}->>${})::text) {} ${}))"),
                            &quote::quote!{
                                last_dimensions_index,
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            }
                        );
                        quote::quote! {
                            #dimensions_indexes_initialization_token_stream
                            #last_dimensions_index_intialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_equals_query_self_value_to_string_token_stream.clone(),
                )
            };
            let generate_dimension_contains_element_regular_expression_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #regular_expression_case_and_value_declaration_token_stream
                        }
                    },
                    dimensions_default_regular_expression_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}(exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))"),
                            &quote::quote!{
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            }
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_equals_query_self_value_to_string_token_stream.clone()
                )
            };
            let generate_dimension_all_elements_regular_expression_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #regular_expression_case_and_value_declaration_token_stream
                        }
                    },
                    dimensions_default_regular_expression_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}(not exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))"),
                            &quote::quote!{
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            }
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_equals_query_self_value_to_string_token_stream.clone()
                )
            };
            let generate_dimension_length_more_than_token_stream = |dimension_number: &DimensionNumber|generate_dimension_length_operation_token_stream(&dimension_number, &greater_than_sign);
            let generate_dimension_contains_all_elements_of_array_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}({}{} @> {})")
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
                )
            };
            let generate_dimension_overlaps_with_array_token_stream = |dimension_number: &DimensionNumber| -> (
                ShouldAddDeclarationOfStructIdentGeneric,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
                proc_macro2::TokenStream,
            ) {
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    {
                        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream = generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream(&dimension_number);
                        quote::quote! {
                            #pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_dimension_minus_one_token_stream,
                            #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                        }
                    },
                    dimensions_default_value_default_initialization_token_stream.clone(),
                    {
                        let ok_format_token_stream = generate_ok_format_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&"{}(exists (select 1 from jsonb_array_elements_text({}{}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))")
                        );
                        quote::quote! {
                            #dimensions_indexes_postgresql_json_type_query_part_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            #ok_format_token_stream
                        }
                    },
                    query_dimensions_bind_query_bind_sqlx_types_json_self_value_token_stream.clone()
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
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    pub_value_t_token_stream.clone(),
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    generate_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&equal_sign)),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                    ident: _
                } => generate_dimension_equal_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                    ident: _
                } => generate_dimension_equal_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                    ident: _
                } => generate_dimension_equal_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                    ident: _
                } => generate_dimension_equal_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                    ident: _
                } => generate_dimension_all_elements_equal_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                    ident: _
                } => generate_dimension_all_elements_equal_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                    ident: _
                } => generate_dimension_all_elements_equal_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsEqual {
                    ident: _
                } => generate_dimension_all_elements_equal_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => generate_dimension_length_equal_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                    ident: _
                } => (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    pub_value_t_token_stream.clone(),
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    generate_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(&greater_than_sign)),
                    query_bind_sqlx_types_json_self_value_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                    ident: _
                } => generate_dimension_greater_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                    ident: _
                } => generate_dimension_greater_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                    ident: _
                } => generate_dimension_greater_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                    ident: _
                } => generate_dimension_greater_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                    ident: _
                } => generate_dimension_contains_element_greater_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                    ident: _
                } => generate_dimension_contains_element_greater_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                    ident: _
                } => generate_dimension_contains_element_greater_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElementGreaterThan {
                    ident: _
                } => generate_dimension_contains_element_greater_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                    ident: _
                } => generate_dimension_all_elements_greater_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                    ident: _
                } => generate_dimension_all_elements_greater_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                    ident: _
                } => generate_dimension_all_elements_greater_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsGreaterThan {
                    ident: _
                } => generate_dimension_all_elements_greater_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between {
                    ident: _
                } => (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    pub_value_between_t_token_stream.clone(),
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    quote::quote! {
                        #value_match_self_value_query_part_initialization_token_stream
                        Ok(format!(
                            "{}({column} {value})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator)
                        ))
                    },
                    query_self_value_query_bind_token_stream,
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                    ident: _
                } => generate_dimension_between_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                    ident: _
                } => generate_dimension_between_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                    ident: _
                } => generate_dimension_between_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                    ident: _
                } => generate_dimension_between_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                    ident: _
                } => (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>},
                    value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                    {
                        let value_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                            &value_snake_case,
                            &value_snake_case,
                            &quote::quote!{query_part_one_by_one}
                        );
                        quote::quote! {
                            #value_initialization_token_stream
                            Ok(format!(
                                "{}({} in ({}))",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                value
                            ))
                        }
                    },
                    query_self_value_query_bind_one_by_one_token_stream.clone(),
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                    ident: _
                } => generate_dimension_in_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                    ident: _
                } => generate_dimension_in_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                    ident: _
                } => generate_dimension_in_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                    ident: _
                } => generate_dimension_in_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression => (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    regular_expression_case_and_value_declaration_token_stream.clone(),
                    regular_expression_case_and_value_default_initialization_token_stream.clone(),
                    generate_query_part_regular_expression_token_stream(&quote::quote!{"{}(trim(both '\"' from ({})::text) {} ${})"}),
                    query_equals_query_self_value_to_string_token_stream.clone()
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression => generate_dimension_regular_expression_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression => generate_dimension_regular_expression_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression => generate_dimension_regular_expression_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression => generate_dimension_regular_expression_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression => generate_dimension_contains_element_regular_expression_token_stream(
                    &DimensionNumber::One
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression => generate_dimension_contains_element_regular_expression_token_stream(
                    &DimensionNumber::Two
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression => generate_dimension_contains_element_regular_expression_token_stream(
                    &DimensionNumber::Three
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElementRegularExpression => generate_dimension_contains_element_regular_expression_token_stream(
                    &DimensionNumber::Four
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression => generate_dimension_all_elements_regular_expression_token_stream(
                    &DimensionNumber::One
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression => generate_dimension_all_elements_regular_expression_token_stream(
                    &DimensionNumber::Two
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression => generate_dimension_all_elements_regular_expression_token_stream(
                    &DimensionNumber::Three
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsRegularExpression => generate_dimension_all_elements_regular_expression_token_stream(
                    &DimensionNumber::Four
                ),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => generate_dimension_length_more_than_token_stream(&DimensionNumber::Four),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                    ident: _
                } => generate_dimension_contains_all_elements_of_array_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                    ident: _
                } => generate_dimension_contains_all_elements_of_array_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                    ident: _
                } => generate_dimension_contains_all_elements_of_array_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                    ident: _
                } => generate_dimension_contains_all_elements_of_array_token_stream(&DimensionNumber::Four),
                // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray { 
                    ident: _
                } => generate_dimension_overlaps_with_array_token_stream(&DimensionNumber::One),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                    ident: _
                } => generate_dimension_overlaps_with_array_token_stream(&DimensionNumber::Two),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                    ident: _
                } => generate_dimension_overlaps_with_array_token_stream(&DimensionNumber::Three),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                    ident: _
                } => generate_dimension_overlaps_with_array_token_stream(&DimensionNumber::Four),
            };
            let struct_token_stream = generate_struct_token_stream(
                false,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &struct_additional_fields_token_stream,
            );
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
            let generated = quote::quote! {
                #struct_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                #impl_postgresql_type_where_filter_token_stream
            };
            // match &filter {
            //     postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {ident: _} => {
            //         // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //         //     "GeneratePostgresqlTypeWhereElementFilter",
            //         //     &generated,
            //         // );
            //         proc_macro2::TokenStream::new()
            //     },
            //     _ => generated
            // }
            generated
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
