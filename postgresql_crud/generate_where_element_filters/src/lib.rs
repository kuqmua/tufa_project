#[proc_macro]
pub fn generate_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let dimensions_snake_case = naming::DimensionsSnakeCase;
    let dimensions_indexes_snake_case = naming::DimensionsIndexesSnakeCase;
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
    fn generate_value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream() -> proc_macro2::TokenStream {
        let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
        quote::quote! {
            value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
        }
    }
    let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream();
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
        is_increment_parameter_underscore: &postgresql_crud_macros_common::IsIncrementParameterUnderscore,
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
            &is_increment_parameter_underscore,
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
    let query_equals_query_self_value_to_string_token_stream = quote::quote! {
        query = query.bind(self.value.to_string());
        query
    };
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
    #[derive(Clone)]
    enum PostgresqlTypePatternHandle {
        Standart,
        ArrayDimension1,
        ArrayDimension2,
        ArrayDimension3,
        ArrayDimension4,
    }
    impl std::convert::TryFrom<&PostgresqlTypePatternHandle> for DimensionNumber {
        type Error = ();
        fn try_from(value: &PostgresqlTypePatternHandle) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlTypePatternHandle::Standart => Err(()),
                PostgresqlTypePatternHandle::ArrayDimension1 => Ok(Self::One),
                PostgresqlTypePatternHandle::ArrayDimension2 => Ok(Self::Two),
                PostgresqlTypePatternHandle::ArrayDimension3 => Ok(Self::Three),
                PostgresqlTypePatternHandle::ArrayDimension4 => Ok(Self::Four),
            }
        }
    }
    let postgresql_type_pattern_handle_standart = PostgresqlTypePatternHandle::Standart;
    let postgresql_type_pattern_handle_array_dimension1 = PostgresqlTypePatternHandle::ArrayDimension1;
    let postgresql_type_pattern_handle_array_dimension2 = PostgresqlTypePatternHandle::ArrayDimension2;
    let postgresql_type_pattern_handle_array_dimension3 = PostgresqlTypePatternHandle::ArrayDimension3;
    let postgresql_type_pattern_handle_array_dimension4 = PostgresqlTypePatternHandle::ArrayDimension4;
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
    let dimensions_default_initialization_token_stream = quote::quote!{
        #dimensions_snake_case: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
    };
    let dimensions_default_initialization_comma_token_stream = quote::quote!{#dimensions_default_initialization_token_stream,};
    let query_self_dimensions_query_bind_query_token_stream = quote::quote!{query = self.#dimensions_snake_case.query_bind(query);};
    enum PostgresqlTypeKind {
        Standart,
        ArrayDimension
    }
    impl PostgresqlTypeKind {
        fn format_argument(&self) -> &'static std::primitive::str {
            match &self {
                PostgresqlTypeKind::Standart => "",
                PostgresqlTypeKind::ArrayDimension => "{}",
            }
        }
    }
    let dimensions_indexes_comma_token_stream = quote::quote!{#dimensions_indexes_snake_case,};
    let generate_maybe_dimensions_declaration_pub_value_t_token_stream = |maybe_dimensions_declaration_token_stream: &dyn quote::ToTokens|{
        quote::quote! {
            #maybe_dimensions_declaration_token_stream
            #pub_value_t_token_stream
        }
    };
    let generate_maybe_dimensions_default_initialization_value_default_token_stream = |maybe_dimensions_default_initialization_token_stream: &dyn quote::ToTokens|{
        quote::quote!{
            #maybe_dimensions_default_initialization_token_stream
            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
        }
    };
    let is_query_bind_mutable_true = postgresql_crud_macros_common::IsQueryBindMutable::True;
    let is_query_bind_mutable_false = postgresql_crud_macros_common::IsQueryBindMutable::False;
    let generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream = |dimension_number: &DimensionNumber|{
        let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_token_stream(&dimension_number.dimension_token_stream(), &KindOfUnsignedPartOfStdPrimitiveI32::CanNotBeZero);
        quote::quote! {#pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream,}
    };
    let generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream = |dimension_number: &DimensionNumber| {
        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_token_stream(&dimension_number.dimension_token_stream(), &KindOfUnsignedPartOfStdPrimitiveI32::CanBeZero);
        quote::quote! {#pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,}
    };
    enum PostgresqlTypeOrPostgresqlJsonType {
        PostgresqlType,
        PostgresqlJsonType
    }
    let generate_postgresql_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, postgresql_type_or_postgresql_json_type: &PostgresqlTypeOrPostgresqlJsonType|{
        if let Ok(dimension_number) = DimensionNumber::try_from(postgresql_type_pattern_handle) {
            (
                match &postgresql_type_or_postgresql_json_type {
                    PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                    PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                },
                dimensions_default_initialization_comma_token_stream.clone(),
                generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                    &dimensions_indexes_snake_case,
                    &dimensions_snake_case,
                    &match &postgresql_type_or_postgresql_json_type {
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => quote::quote!{postgresql_type_query_part},
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => quote::quote!{postgresql_json_type_query_part}
                    },
                ),
                PostgresqlTypeKind::ArrayDimension,
                dimensions_indexes_comma_token_stream.clone(),
                query_self_dimensions_query_bind_query_token_stream.clone()
            )
        }
        else {
            (
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
                PostgresqlTypeKind::Standart,
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new()
            )
        }
    };
    let postgresql_type_token_stream = {
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlTypeFilter| {
            let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                is_increment_parameter_underscore,
                query_part_content_token_stream,
                is_query_bind_mutable,
                query_bind_content_token_stream
            ) = {
                let should_add_declaration_of_struct_ident_generic_true_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: Some(quote::quote!{sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>})
                };
                let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: Some(quote::quote!{std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>})
                };
                let pub_value_postgresql_type_not_empty_unique_vec_t_token_stream = quote::quote!{pub value: crate::PostgresqlTypeNotEmptyUniqueVec<T>};
                let generate_postgresql_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle|{
                    generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlType)
                };
                let generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream = |
                    postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                    generate_format_handle_stringified: &dyn Fn(&PostgresqlTypeKind) -> std::string::String
                | {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&generate_format_handle_stringified(&postgresql_type_kind));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        }
                    )
                };
                let generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operator: &dyn std::fmt::Display| {
                    generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(
                        &postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}({{}}{} {operator} ${{}})", postgresql_type_kind.format_argument())
                    )
                };
                let generate_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"=")
                };
                let generate_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &">")
                };
                let generate_between_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_between_t_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_self_value_query_part_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_self_value_query_bind_token_stream
                        }
                    )
                };
                let generate_in_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone_type_encode.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_postgresql_type_not_empty_unique_vec_t_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} in ({{}}))", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                let value = {
                                    let mut acc = std::string::String::default();
                                    for _ in self.value.to_vec() {
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
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            for element in self.value.into_vec() {
                                query = query.bind(element);
                            }
                            query
                        }
                    )
                };
                let generate_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #regular_expression_case_and_value_declaration_token_stream
                        },
                        quote::quote!{
                            #maybe_dimensions_default_initialization_token_stream
                            #regular_expression_case_and_value_default_initialization_token_stream
                        },
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {{}} ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    self.regular_expression_case.postgreql_syntax(),
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_equals_query_self_value_to_string_token_stream
                        }
                    )
                };
                let generate_before_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} < ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        }
                    )
                };
                let generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, postgresql_syntax: &dyn std::fmt::Display| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => postgresql_crud_macros_common::IsIncrementParameterUnderscore::True,
                            PostgresqlTypePatternHandle::ArrayDimension1 |
                            PostgresqlTypePatternHandle::ArrayDimension2 |
                            PostgresqlTypePatternHandle::ArrayDimension3 |
                            PostgresqlTypePatternHandle::ArrayDimension4 => postgresql_crud_macros_common::IsIncrementParameterUnderscore::False
                        },
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {postgresql_syntax})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                ))
                            }
                        },
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => is_query_bind_mutable_false.clone(),
                            PostgresqlTypePatternHandle::ArrayDimension1 |
                            PostgresqlTypePatternHandle::ArrayDimension2 |
                            PostgresqlTypePatternHandle::ArrayDimension3 |
                            PostgresqlTypePatternHandle::ArrayDimension4 => is_query_bind_mutable_true.clone()
                        },
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_snake_case
                        }
                    )
                };
                let generate_current_date_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"= current_date")
                };
                let generate_greater_than_current_date_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"> current_date")
                };
                let generate_current_timestamp_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"= current_timestamp")
                };
                let generate_greater_than_current_timestamp_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"> current_timestamp")
                };
                let generate_current_time_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"= current_time")
                };
                let generate_greater_than_current_time_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(&postgresql_type_pattern_handle, &"> current_time")
                };
                let generate_equal_to_encoded_string_representation_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            pub encode_format: crate::postgresql_type::EncodeFormat,
                            pub encoded_string_representation: std::string::String,
                        },
                        quote::quote!{
                            #maybe_dimensions_default_initialization_token_stream
                            encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            encoded_string_representation: #core_default_default_default_token_stream
                        },
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(encode({{}}{}, '{{}}') = ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    &self.encode_format,
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            query = query.bind(self.encoded_string_representation);
                            query
                        }
                    )
                };
                let generate_find_ranges_within_given_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"<@")
                };
                let generate_find_ranges_that_fully_contain_the_given_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"@>")
                };
                let generate_strictly_to_left_of_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"&<")
                };
                let generate_strictly_to_right_of_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"&>")
                };
                let generate_overlap_with_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"&&")
                };
                let generate_adjacent_with_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(&postgresql_type_pattern_handle, &"-|-")
                };
                let generate_length_filter_pattern_token_stream = |operator: &dyn std::fmt::Display|{
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        generate_query_part_one_value_token_stream(
                            &generate_quotes::double_quotes_token_stream(&format!("{{}}(array_length({{}}, 1) {operator} ${{}})"))
                        ),
                        is_query_bind_mutable_true.clone(),
                        query_bind_one_value_token_stream.clone(),
                    )
                };
                let generate_included_lower_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(
                        &postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(lower({{}}{}) = ${{}})", postgresql_type_kind.format_argument())
                    )
                };
                let generate_excluded_upper_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(
                        &postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(upper({{}}{}) = ${{}})", postgresql_type_kind.format_argument())
                    )
                };
                let generate_greater_than_included_lower_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(
                        &postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(lower({{}}{}) > ${{}})", postgresql_type_kind.format_argument())
                    )
                };
                let generate_greater_than_excluded_upper_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(
                        &postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(upper({{}}{}) > ${{}})", postgresql_type_kind.format_argument())
                    )
                };
                let generate_range_length_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = if let Ok(dimension_number) = DimensionNumber::try_from(postgresql_type_pattern_handle) {
                        (
                            generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                            dimensions_default_initialization_comma_token_stream.clone(),
                            {
                                let dimensions_indexes1_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                                    &quote::quote!{dimensions_indexes1},
                                    &dimensions_snake_case,
                                    &quote::quote!{postgresql_type_query_part}
                                );
                                let dimensions_indexes2_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                                    &quote::quote!{dimensions_indexes2},
                                    &dimensions_snake_case,
                                    &quote::quote!{postgresql_type_query_part}
                                );
                                quote::quote!{
                                    #dimensions_indexes1_token_stream
                                    #dimensions_indexes2_token_stream
                                }
                            },
                            PostgresqlTypeKind::ArrayDimension,
                            quote::quote!{
                                dimensions_indexes1,
                                column,
                                dimensions_indexes2,
                            },
                            quote::quote!{
                                query = self.#dimensions_snake_case.clone().query_bind(query);
                                #query_self_dimensions_query_bind_query_token_stream
                            }
                        )
                    }
                    else {
                        (
                            proc_macro2::TokenStream::new(),
                            proc_macro2::TokenStream::new(),
                            proc_macro2::TokenStream::new(),
                            PostgresqlTypeKind::Standart,
                            quote::quote!{column,},
                            proc_macro2::TokenStream::new()
                        )
                    };
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::False,
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                                &format!("{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                postgresql_type_kind.format_argument(),
                                postgresql_type_kind.format_argument(),
                            ));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    column,
                                    #maybe_additional_parameters_token_stream
                                    value
                                ))
                            }
                        },
                        is_query_bind_mutable_true.clone(),
                        quote::quote!{
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        }
                    )
                };
                match &filter {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { ident: _ } => generate_equal_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual { ident: _ } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan { ident: _ } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan { ident: _ } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between { ident: _ } => generate_between_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween { ident: _ } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In { ident: _ } => generate_in_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn { ident: _ } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before { ident: _ } => generate_before_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore { ident: _ } => generate_before_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => generate_current_date_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate => generate_current_date_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => generate_greater_than_current_date_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate => generate_greater_than_current_date_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => generate_current_timestamp_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp => generate_current_timestamp_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => generate_greater_than_current_timestamp_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp => generate_greater_than_current_timestamp_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => generate_current_time_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime => generate_current_time_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => generate_greater_than_current_time_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime => generate_greater_than_current_time_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual => generate_length_filter_pattern_token_stream(&"="),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthMoreThan => generate_length_filter_pattern_token_stream(&">"),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => generate_equal_to_encoded_string_representation_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => generate_equal_to_encoded_string_representation_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesWithinGivenRange { ident: _ } => generate_find_ranges_within_given_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesWithinGivenRange { ident: _ } => generate_find_ranges_within_given_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesThatFullyContainTheGivenRange { ident: _ } => generate_find_ranges_that_fully_contain_the_given_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange { ident: _ } => generate_find_ranges_that_fully_contain_the_given_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange { ident: _ } => generate_strictly_to_left_of_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange { ident: _ } => generate_strictly_to_left_of_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange { ident: _ } => generate_strictly_to_right_of_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange { ident: _ } => generate_strictly_to_right_of_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound { ident: _ } => generate_included_lower_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound { ident: _ } => generate_included_lower_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound { ident: _ } => generate_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound { ident: _ } => generate_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanIncludedLowerBound { ident: _ } => generate_greater_than_included_lower_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanIncludedLowerBound { ident: _ } => generate_greater_than_included_lower_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanExcludedUpperBound { ident: _ } => generate_greater_than_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanExcludedUpperBound { ident: _ } => generate_greater_than_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange { ident: _ } => generate_overlap_with_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange { ident: _ } => generate_overlap_with_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange { ident: _ } => generate_adjacent_with_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange { ident: _ } => generate_adjacent_with_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => generate_range_length_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength => generate_range_length_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                }
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
                &FilterType::PostgresqlType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &is_increment_parameter_underscore,
                &query_part_content_token_stream,
                &is_query_bind_mutable,
                &query_bind_content_token_stream,
            );
            let generated = quote::quote! {
                #struct_token_stream
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
            let generate_postgresql_json_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle|{
                generate_postgresql_type_dimensions_helpers(&postgresql_type_pattern_handle, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType)
            };
            let generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream = |
                postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                generate_format_handle_stringified: &dyn Fn(&PostgresqlTypeKind) -> std::string::String
            | {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_t_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            &generate_format_handle_stringified(&postgresql_type_kind)
                        );
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    }
                )
            };
            let generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operator: &dyn std::fmt::Display| {
                generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(
                    &postgresql_type_pattern_handle,
                    &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}({{}}{} {operator} ${{}})", postgresql_type_kind.format_argument())
                )
            };
            let generate_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream(&postgresql_type_pattern_handle, &"=")
            };
            let generate_all_elements_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(
                    &postgresql_type_pattern_handle,
                    &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))", postgresql_type_kind.format_argument())
                )
            };
            let generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operation: &dyn std::fmt::Display| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        pub value: #unsigned_part_of_std_primitive_i32_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        value: #core_default_default_default_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(jsonb_array_length({{}}{}) {operation} ${{}})", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_one_value_token_stream
                    }
                )
            };
            let generate_length_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream(&postgresql_type_pattern_handle, &"=")
            };
            let generate_length_more_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle|{
                generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream(&postgresql_type_pattern_handle, &">")
            };
            let generate_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream(&postgresql_type_pattern_handle, &">")
            };
            let generate_contains_element_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(
                    &postgresql_type_pattern_handle,
                    &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))", postgresql_type_kind.format_argument())
                )
            };
            let generate_all_elements_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(
                    &postgresql_type_pattern_handle,
                    &|postgresql_type_kind: &PostgresqlTypeKind|format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))", postgresql_type_kind.format_argument())
                )
            };
            let generate_between_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_between_t_token_stream
                    },
                    generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}({{}}{} {{}})",
                            postgresql_type_kind.format_argument()
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    }
                )
            };
            let generate_in_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}({{}}{} in ({{}}))",
                            postgresql_type_kind.format_argument()
                        ));
                        let value_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                            &value_snake_case,
                            &value_snake_case,
                            &quote::quote!{query_part_one_by_one}
                        );
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        query = self.value.query_bind_one_by_one(query);
                        query
                    }
                )
            };
            let generate_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = if let Ok(dimension_number) = DimensionNumber::try_from(postgresql_type_pattern_handle) {
                    (
                        generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                        dimensions_default_initialization_comma_token_stream.clone(),
                        {
                            let dimensions_indexes_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                                &dimensions_indexes_snake_case,
                                &dimensions_snake_case,
                                &quote::quote!{postgresql_json_type_query_part_minus_one}
                            );
                            let last_dimensions_index_intialization_token_stream = generate_match_increment_checked_add_one_initialization_token_stream(&quote::quote!{last_dimensions_index});
                            quote::quote!{
                                #dimensions_indexes_initialization_token_stream
                                #last_dimensions_index_intialization_token_stream
                            }
                        },
                        PostgresqlTypeKind::ArrayDimension,
                        quote::quote!{
                            last_dimensions_index,
                            #dimensions_indexes_snake_case,
                        },
                        query_self_dimensions_query_bind_query_token_stream.clone()
                    )
                }
                else {
                    (
                        proc_macro2::TokenStream::new(),
                        proc_macro2::TokenStream::new(),
                        proc_macro2::TokenStream::new(),
                        PostgresqlTypeKind::Standart,
                        proc_macro2::TokenStream::new(),
                        proc_macro2::TokenStream::new()
                    )
                };
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                            match &postgresql_type_kind {
                                PostgresqlTypeKind::Standart => "",
                                PostgresqlTypeKind::ArrayDimension => "{}->>${}"
                            }
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_equals_query_self_value_to_string_token_stream
                    }
                )
            };
            let generate_contains_element_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) {{}} ${{}}))",
                            postgresql_type_kind.format_argument()
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_equals_query_self_value_to_string_token_stream
                    }
                )
            };
            let generate_all_elements_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{{}} ${{}}))",
                            postgresql_type_kind.format_argument()
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                self.regular_expression_case.postgreql_syntax(),
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_equals_query_self_value_to_string_token_stream
                    }
                )
            };
            let generate_contains_all_elements_of_array_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} @> {{}})", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    }
                )
            };
            let generate_overlaps_with_array_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind,
                    maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = generate_postgresql_json_type_dimensions_helpers(&postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    quote::quote!{
                        #maybe_dimensions_default_initialization_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))",
                            postgresql_type_kind.format_argument()
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                column,
                                #maybe_additional_parameters_token_stream
                                value
                            ))
                        }
                    },
                    is_query_bind_mutable_true.clone(),
                    quote::quote!{
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    }
                )
            };
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                query_part_content_token_stream,
                is_query_bind_mutable,
                query_bind_content_token_stream
            ) = match &filter {
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal {
                    ident: _
                } => generate_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                    ident: _
                } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                    ident: _
                } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                    ident: _
                } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                    ident: _
                } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                    ident: _
                } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                    ident: _
                } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                    ident: _
                } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                    ident: _
                } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsEqual {
                    ident: _
                } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthMoreThan => generate_length_more_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => generate_length_more_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => generate_length_more_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => generate_length_more_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => generate_length_more_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                    ident: _
                } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                    ident: _
                } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                    ident: _
                } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                    ident: _
                } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                    ident: _
                } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                    ident: _
                } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                    ident: _
                } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                    ident: _
                } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                    ident: _
                } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElementGreaterThan {
                    ident: _
                } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                    ident: _
                } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                    ident: _
                } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                    ident: _
                } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                    ident: _
                } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsGreaterThan {
                    ident: _
                } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between {
                    ident: _
                } => generate_between_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                    ident: _
                } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                    ident: _
                } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                    ident: _
                } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                    ident: _
                } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                    ident: _
                } => generate_in_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                    ident: _
                } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                    ident: _
                } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                    ident: _
                } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                    ident: _
                } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression => generate_contains_element_regular_expression_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression => generate_contains_element_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression => generate_contains_element_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression => generate_contains_element_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElementRegularExpression => generate_contains_element_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression => generate_all_elements_regular_expression_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression => generate_all_elements_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression => generate_all_elements_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression => generate_all_elements_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsRegularExpression => generate_all_elements_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsAllElementsOfArray {
                    ident: _
                } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                    ident: _
                } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                    ident: _
                } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                    ident: _
                } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                    ident: _
                } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray { 
                    ident: _
                } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray { 
                    ident: _
                } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                    ident: _
                } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                    ident: _
                } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                    ident: _
                } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension4),
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
                &postgresql_crud_macros_common::IsIncrementParameterUnderscore::False,
                &query_part_content_token_stream,
                &is_query_bind_mutable,
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
