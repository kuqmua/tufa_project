#[proc_macro]
pub fn generate_where_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Clone)]
    enum ShouldAddDeclarationOfStructIdentGeneric {
        True { maybe_additional_traits_token_stream: Option<proc_macro2::TokenStream> },
        False,
    }
    enum FilterType {
        PostgresqlType,
        PostgresqlJsonType,
    }
    #[derive(Clone)]
    enum PostgresqlTypePatternHandle {
        Standart,
        ArrayDimension1,
        ArrayDimension2,
        ArrayDimension3,
        ArrayDimension4,
    }
    impl TryFrom<&PostgresqlTypePatternHandle> for DimensionNumber {
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
    #[derive(Clone)]
    enum DimensionNumber {
        One,
        Two,
        Three,
        Four,
    }
    impl DimensionNumber {
        const fn dimension_std_primitive_u8(&self) -> u8 {
            match &self {
                Self::One => 1,
                Self::Two => 2,
                Self::Three => 3,
                Self::Four => 4,
            }
        }
        fn dimension_token_stream(&self) -> proc_macro2::TokenStream {
            self.dimension_std_primitive_u8().to_string().parse::<proc_macro2::TokenStream>().expect("error 18c32bc0-2e55-4b4d-a6a8-b8680e5fe463")
        }
    }
    enum KindOfUnsignedPartOfStdPrimitiveI32 {
        CanBeZero,
        CanNotBeZero,
    }
    impl quote::ToTokens for KindOfUnsignedPartOfStdPrimitiveI32 {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::CanBeZero => quote::quote! {UnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
                Self::CanNotBeZero => quote::quote! {NotZeroUnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
            }
        }
    }
    enum PostgresqlTypeKind {
        Standart,
        ArrayDimension,
    }
    impl PostgresqlTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Standart => "",
                Self::ArrayDimension => "{}",
            }
        }
    }
    panic_location::panic_location();
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let element_snake_case = naming::ElementSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let dimensions_snake_case = naming::DimensionsSnakeCase;
    let dimensions_indexes_snake_case = naming::DimensionsIndexesSnakeCase;
    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
    let t_token_stream = quote::quote! {T};
    let t_annotation_generic_token_stream = quote::quote! {<#t_token_stream>};
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
    let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let pub_value_t_token_stream = quote::quote! {pub #value_snake_case: T};
    let unsigned_part_of_std_primitive_i32_token_stream = quote::quote! {postgresql_crud_common::UnsignedPartOfStdPrimitiveI32};
    let not_zero_unsigned_part_of_std_primitive_i32_token_stream = quote::quote! {postgresql_crud_common::NotZeroUnsignedPartOfStdPrimitiveI32};
    let value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream = quote::quote! {#value_snake_case: #not_zero_unsigned_part_of_std_primitive_i32_token_stream};
    let pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream = quote::quote! {pub #value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream};
    let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote! {
        #value_snake_case: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    let generate_query_part_one_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                Ok(#value_snake_case) => Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case, #increment_snake_case)),
                Err(#error_snake_case) => Err(#error_snake_case),
            }
        }
    };
    let generate_struct_token_stream = |filter_initialized_with_try_new_result_is_ok: bool, should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn quote::ToTokens, struct_additional_fields_token_stream: &dyn quote::ToTokens| {
        let maybe_pub_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_token_stream_new } else { &naming::PubSnakeCase };
        let maybe_derive_serde_deserialize_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_token_stream_new } else { &quote::quote! {serde::Deserialize,} };
        let maybe_declaration_of_struct_ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
            ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream } => {
                &maybe_additional_traits_token_stream.as_ref().map_or_else(|| quote::quote! {<#t_token_stream>}, |value| quote::quote! {<#t_token_stream: #value>})
            }
            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
        };
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_derive_serde_deserialize_token_stream schemars::JsonSchema)]
            pub struct #ident #maybe_declaration_of_struct_ident_generic_token_stream {
                #maybe_pub_token_stream logical_operator: #import_path::LogicalOperator,
                #struct_additional_fields_token_stream
            }
        }
    };
    let generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = |should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn quote::ToTokens, impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream: &dyn quote::ToTokens| {
        postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream } => {
                    maybe_additional_traits_token_stream.as_ref().map_or_else(
                        || quote::quote! {<T: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream>},
                        |value| quote::quote! {<T: #value + #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream>}
                    )
                }
                ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new(),
            },
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon,
            &ident,
            match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_token_stream,
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            },
            &quote::quote! {
                Self {
                    logical_operator: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                }
            },
        )
    };
    let generate_impl_postgresql_type_where_filter_token_stream = |filter_type: &FilterType,
                                                                   should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
                                                                   ident: &dyn quote::ToTokens,
                                                                   increment_parameter_underscore: &postgresql_crud_macros_common::IncrementParameterUnderscore,
                                                                   is_need_to_add_logical_operator_underscore: &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore,
                                                                   query_part_content_token_stream: &dyn quote::ToTokens,
                                                                   is_query_bind_mutable: &postgresql_crud_macros_common::IsQueryBindMutable,
                                                                   query_bind_content_token_stream: &dyn quote::ToTokens| {
        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
            &{
                let maybe_t_additional_traits_for_postgresql_type_where_filter_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream } => {
                        let send_and_lifetime_token_stream = quote::quote! {Send + 'lifetime};
                        let serde_serialize_token_stream = quote::quote! {serde::Serialize};
                        let content_token_stream = match (&filter_type, &maybe_additional_traits_token_stream) {
                            (FilterType::PostgresqlType, Some(value)) => &quote::quote! {#value + #send_and_lifetime_token_stream},
                            (FilterType::PostgresqlType, None) => &send_and_lifetime_token_stream,
                            (FilterType::PostgresqlJsonType, Some(value)) => &quote::quote! {#value + #serde_serialize_token_stream + #send_and_lifetime_token_stream},
                            (FilterType::PostgresqlJsonType, None) => &quote::quote! {#serde_serialize_token_stream + #send_and_lifetime_token_stream},
                        };
                        &quote::quote! {, T: #content_token_stream}
                    }
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {<'lifetime #maybe_t_additional_traits_for_postgresql_type_where_filter_token_stream>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_token_stream,
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
            },
            increment_parameter_underscore,
            &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
            is_need_to_add_logical_operator_underscore,
            &query_part_content_token_stream,
            is_query_bind_mutable,
            &query_bind_content_token_stream,
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon,
        )
    };
    let regular_expression_case_and_value_declaration_token_stream = quote::quote! {
        pub regular_expression_case: RegularExpressionCase,
        pub value: RegexRegex
    };
    let regular_expression_case_and_value_default_initialization_token_stream = quote::quote! {
        regular_expression_case: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
    };
    let if_let_err_query_try_bind_self_value_to_string_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.try_bind(#self_snake_case.#value_snake_case.to_string()) {
            return Err(#error_snake_case.to_string());
        }
        Ok(#query_snake_case)
    };
    let if_let_err_query_try_bind_self_value_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.try_bind(#self_snake_case.#value_snake_case) {
            return Err(#error_snake_case.to_string());
        }
    };
    let query_bind_one_value_token_stream = quote::quote! {
        #if_let_err_query_try_bind_self_value_token_stream
        Ok(#query_snake_case)
    };
    let should_add_declaration_of_struct_ident_generic_false = ShouldAddDeclarationOfStructIdentGeneric::False;
    let should_add_declaration_of_struct_ident_generic_true_none = ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_token_stream: None };
    let generate_match_increment_checked_add_one_initialization_token_stream = |ident_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            let #ident_token_stream = match postgresql_crud_common::increment_checked_add_one_returning_increment(#increment_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    return Err(#error_snake_case);
                },
            };
        }
    };
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone = ShouldAddDeclarationOfStructIdentGeneric::True {
        maybe_additional_traits_token_stream: Some(quote::quote! {std::fmt::Debug + PartialEq + Clone}),
    };
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
        maybe_additional_traits_token_stream: Some(quote::quote! {
            std::fmt::Debug
            + PartialEq
            + PartialOrd
            + Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
        }),
    };
    let value_between_t_token_stream = quote::quote! {#value_snake_case: Between<T>};
    let pub_value_between_t_token_stream = quote::quote! {pub #value_between_t_token_stream};
    let query_self_value_query_bind_token_stream = quote::quote! {
        match #self_snake_case.#value_snake_case.query_bind(#query_snake_case) {
            Ok(#value_snake_case) => {
                #query_snake_case = #value_snake_case;
            },
            Err(#error_snake_case) => {
                return Err(#error_snake_case);
            }
        }
        Ok(#query_snake_case)
    };
    let postgresql_type_pattern_handle_standart = PostgresqlTypePatternHandle::Standart;
    let postgresql_type_pattern_handle_array_dimension1 = PostgresqlTypePatternHandle::ArrayDimension1;
    let postgresql_type_pattern_handle_array_dimension2 = PostgresqlTypePatternHandle::ArrayDimension2;
    let postgresql_type_pattern_handle_array_dimension3 = PostgresqlTypePatternHandle::ArrayDimension3;
    let postgresql_type_pattern_handle_array_dimension4 = PostgresqlTypePatternHandle::ArrayDimension4;
    let generate_pub_dimensions_bounded_vec_token_stream = |vec_length_token_stream: &dyn quote::ToTokens, kind_of_unsigned_part_of_std_primitive_i32: &KindOfUnsignedPartOfStdPrimitiveI32| {
        quote::quote! {pub #dimensions_snake_case: BoundedStdVecVec<postgresql_crud_common::#kind_of_unsigned_part_of_std_primitive_i32, #vec_length_token_stream>}
    };
    let value_match_increment_checked_add_one_initialization_token_stream = generate_match_increment_checked_add_one_initialization_token_stream(&value_snake_case);
    let generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream = |ident_token_stream: &dyn quote::ToTokens, field_token_stream: &dyn quote::ToTokens, function_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            let #ident_token_stream = match self.#field_token_stream.#function_token_stream(#increment_snake_case, #column_snake_case, is_need_to_add_logical_operator) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    return Err(#error_snake_case);
                }
            };
        }
    };
    let value_match_self_value_query_part_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(&value_snake_case, &value_snake_case, &quote::quote! {query_part});
    let dimensions_default_initialization_token_stream = quote::quote! {
        #dimensions_snake_case: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    let dimensions_default_initialization_comma_token_stream = quote::quote! {#dimensions_default_initialization_token_stream,};
    let query_self_dimensions_query_bind_query_token_stream = quote::quote! {
        match #self_snake_case.#dimensions_snake_case.query_bind(#query_snake_case) {
            Ok(#value_snake_case) => {
                #query_snake_case = #value_snake_case;
            }
            Err(#error_snake_case) => {
                return Err(#error_snake_case);
            }
        }
    };
    let dimensions_indexes_comma_token_stream = quote::quote! {#dimensions_indexes_snake_case,};
    let generate_maybe_dimensions_declaration_pub_value_t_token_stream = |maybe_dimensions_declaration_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            #maybe_dimensions_declaration_token_stream
            #pub_value_t_token_stream
        }
    };
    let generate_maybe_dimensions_default_initialization_value_default_token_stream = |maybe_dimensions_default_initialization_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            #maybe_dimensions_default_initialization_token_stream
            #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
        }
    };
    let is_query_bind_mutable_true = postgresql_crud_macros_common::IsQueryBindMutable::True;
    let is_query_bind_mutable_false = postgresql_crud_macros_common::IsQueryBindMutable::False;
    let generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream = |dimension_number: &DimensionNumber| {
        let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_token_stream(&dimension_number.dimension_token_stream(), &KindOfUnsignedPartOfStdPrimitiveI32::CanNotBeZero);
        quote::quote! {#pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_token_stream,}
    };
    let generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream = |dimension_number: &DimensionNumber| {
        let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream = generate_pub_dimensions_bounded_vec_token_stream(&dimension_number.dimension_token_stream(), &KindOfUnsignedPartOfStdPrimitiveI32::CanBeZero);
        quote::quote! {#pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_token_stream,}
    };
    let generate_postgresql_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType| {
        DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
            |()| (proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
            |dimension_number| (
                match &postgresql_type_or_postgresql_json_type {
                    postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                    postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                },
                dimensions_default_initialization_comma_token_stream.clone(),
                generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(
                    &dimensions_indexes_snake_case,
                    &dimensions_snake_case,
                    &match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => quote::quote! {postgresql_type_query_part},
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => quote::quote! {postgresql_json_type_query_part},
                    },
                ),
                PostgresqlTypeKind::ArrayDimension,
                dimensions_indexes_comma_token_stream.clone(),
                query_self_dimensions_query_bind_query_token_stream.clone(),
            )
        )
    };
    let postgresql_type_token_stream = {
        let generate_filters_token_stream = |filter: &postgresql_crud_macros_common::PostgresqlTypeFilter| {
            let ident = naming::parameter::PostgresqlTypeWhereSelfUpperCamelCase::from_display(&filter);
            let (should_add_declaration_of_struct_ident_generic, struct_additional_fields_token_stream, impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream, increment_parameter_underscore, query_part_content_token_stream, is_query_bind_mutable, query_bind_content_token_stream) = {
                let sqlx_type_postgresq_encode_token_stream = quote::quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let should_add_declaration_of_struct_ident_generic_true_type_encode = ShouldAddDeclarationOfStructIdentGeneric::True {
                    maybe_additional_traits_token_stream: Some(sqlx_type_postgresq_encode_token_stream.clone()),
                };
                let pub_value_postgresql_type_not_empty_unique_vec_t_token_stream = quote::quote! {pub #value_snake_case: PostgresqlTypeNotEmptyUniqueVec<T>};
                let generate_postgresql_type_dimensions_helpers_postgresql_type = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_postgresql_type_dimensions_helpers(
                    postgresql_type_pattern_handle,
                    &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType
                );
                let generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, generate_format_handle_stringified: &dyn Fn(&PostgresqlTypeKind) -> String| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&generate_format_handle_stringified(&postgresql_type_kind));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        },
                    )
                };
                let generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operator: &dyn std::fmt::Display| generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}({{}}{} {operator} ${{}})", postgresql_type_kind.format_argument()));
                let generate_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &">");
                let generate_between_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_between_t_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_self_value_query_part_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_self_value_query_bind_token_stream
                        },
                    )
                };
                let generate_in_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::True {
                            maybe_additional_traits_token_stream: Some(quote::quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_postgresq_encode_token_stream}),
                        },
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_postgresql_type_not_empty_unique_vec_t_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} in ({{}}))", postgresql_type_kind.format_argument()));
                            let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "${value},"}, &quote::quote! {panic!("error 87f47f75-b2db-4d88-a0f0-e254ac7d14a3");});
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                let #value_snake_case = {
                                    let mut #acc_snake_case = String::default();
                                    for _ in #self_snake_case.#value_snake_case.to_vec() {
                                        match postgresql_crud_common::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                            Ok(#value_snake_case) => {
                                                #if_write_is_err_token_stream
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            },
                                        }
                                    }
                                    let _ = #acc_snake_case.pop();
                                    #acc_snake_case
                                };
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            for #element_snake_case in #self_snake_case.#value_snake_case.into_vec() {
                                if let Err(#error_snake_case) = #query_snake_case.try_bind(#element_snake_case) {
                                    return Err(#error_snake_case.to_string());
                                }
                            }
                            Ok(#query_snake_case)
                        },
                    )
                };
                let generate_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #regular_expression_case_and_value_declaration_token_stream
                        },
                        quote::quote! {
                            #maybe_dimensions_default_initialization_token_stream
                            #regular_expression_case_and_value_default_initialization_token_stream
                        },
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {{}} ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #self_snake_case.regular_expression_case.postgreql_syntax(),
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            #if_let_err_query_try_bind_self_value_to_string_token_stream
                        },
                    )
                };
                let generate_before_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} < ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        },
                    )
                };
                let generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, postgresql_syntax: &dyn std::fmt::Display| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        maybe_dimensions_declaration_token_stream,
                        maybe_dimensions_default_initialization_token_stream,
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => postgresql_crud_macros_common::IncrementParameterUnderscore::True,
                            PostgresqlTypePatternHandle::ArrayDimension1 | PostgresqlTypePatternHandle::ArrayDimension2 | PostgresqlTypePatternHandle::ArrayDimension3 | PostgresqlTypePatternHandle::ArrayDimension4 => postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        },
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {postgresql_syntax})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                ))
                            }
                        },
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => is_query_bind_mutable_false,
                            PostgresqlTypePatternHandle::ArrayDimension1 | PostgresqlTypePatternHandle::ArrayDimension2 | PostgresqlTypePatternHandle::ArrayDimension3 | PostgresqlTypePatternHandle::ArrayDimension4 => is_query_bind_mutable_true,
                        },
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            Ok(#query_snake_case)
                        },
                    )
                };
                let generate_current_date_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"= current_date");
                let generate_greater_than_current_date_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"> current_date");
                let generate_current_timestamp_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"= current_timestamp");
                let generate_greater_than_current_timestamp_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"> current_timestamp");
                let generate_current_time_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"= current_time");
                let generate_greater_than_current_time_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_token_stream(postgresql_type_pattern_handle, &"> current_time");
                let generate_equal_to_encoded_string_representation_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            pub encode_format: EncodeFormat,
                            pub encoded_string_representation: String,
                        },
                        quote::quote! {
                            #maybe_dimensions_default_initialization_token_stream
                            encode_format: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            encoded_string_representation: #core_default_default_default_token_stream
                        },
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(encode({{}}{}, '{{}}') = ${{}})", postgresql_type_kind.format_argument()));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    &#self_snake_case.encode_format,
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            if let Err(#error_snake_case) = #query_snake_case.try_bind(self.encoded_string_representation) {
                                return Err(#error_snake_case.to_string());
                            }
                            Ok(#query_snake_case)
                        },
                    )
                };
                let generate_find_ranges_within_given_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"<@");
                let generate_find_ranges_that_fully_contain_the_given_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"@>");
                let generate_strictly_to_left_of_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"&<");
                let generate_strictly_to_right_of_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"&>");
                let generate_overlap_with_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"&&");
                let generate_adjacent_with_range_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_token_stream(postgresql_type_pattern_handle, &"-|-");
                let generate_length_filter_pattern_token_stream = |operator: &dyn std::fmt::Display| {
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream.clone(),
                        value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream.clone(),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        generate_query_part_one_value_token_stream(&generate_quotes::double_quotes_token_stream(&format!("{{}}(array_length({{}}, 1) {operator} ${{}})"))),
                        is_query_bind_mutable_true,
                        query_bind_one_value_token_stream.clone(),
                    )
                };
                let generate_included_lower_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(lower({{}}{}) = ${{}})", postgresql_type_kind.format_argument()));
                let generate_excluded_upper_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(upper({{}}{}) = ${{}})", postgresql_type_kind.format_argument()));
                let generate_greater_than_included_lower_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(lower({{}}{}) > ${{}})", postgresql_type_kind.format_argument()));
                let generate_greater_than_excluded_upper_bound_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_32abfefc_c087_480b_b502_cb78533dafb0_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(upper({{}}{}) > ${{}})", postgresql_type_kind.format_argument()));
                let generate_range_length_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream,
                        maybe_dimensions_indexes_initialization_token_stream,
                        postgresql_type_kind,
                        maybe_additional_parameters_token_stream,
                        maybe_dimensions_query_bind_content_token_stream
                    ) = DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
                        |()| (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, quote::quote! {#column_snake_case,}, proc_macro2::TokenStream::new()),
                        |dimension_number| (
                            generate_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                            dimensions_default_initialization_comma_token_stream.clone(),
                            {
                                let dimensions_indexes1_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(&quote::quote! {dimensions_indexes1}, &dimensions_snake_case, &quote::quote! {postgresql_type_query_part});
                                let dimensions_indexes2_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(&quote::quote! {dimensions_indexes2}, &dimensions_snake_case, &quote::quote! {postgresql_type_query_part});
                                quote::quote! {
                                    #dimensions_indexes1_token_stream
                                    #dimensions_indexes2_token_stream
                                }
                            },
                            PostgresqlTypeKind::ArrayDimension,
                            quote::quote! {
                                dimensions_indexes1,
                                column,
                                dimensions_indexes2,
                            },
                            quote::quote! {
                                match #self_snake_case.#dimensions_snake_case.clone().query_bind(#query_snake_case) {
                                    Ok(#value_snake_case) => {
                                        #query_snake_case = #value_snake_case;
                                    },
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                }
                                #query_self_dimensions_query_bind_query_token_stream
                            },
                        )
                    );
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::False,
                        quote::quote! {
                            #maybe_dimensions_declaration_token_stream
                            #pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_token_stream
                        },
                        generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})", postgresql_type_kind.format_argument(), postgresql_type_kind.format_argument(),));
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                #value_match_increment_checked_add_one_initialization_token_stream
                                Ok(format!(
                                    #format_handle_token_stream,
                                    &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #maybe_additional_parameters_token_stream
                                    #value_snake_case
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote::quote! {
                            #maybe_dimensions_query_bind_content_token_stream
                            #query_bind_one_value_token_stream
                        },
                    )
                };
                match &filter {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { .. } => {
                        let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, _, _, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(&postgresql_type_pattern_handle_standart);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_token_stream: Some(quote::quote! {#sqlx_type_postgresq_encode_token_stream + postgresql_crud_common::PostgresqlTypeEqualOperator}),
                            },
                            generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                            generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                let operator = <T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#self_snake_case.#value_snake_case);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgresql_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_token_stream
                                        format!("{operator_query_str} ${value}")
                                    },
                                    postgresql_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({} {content})", &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case))
                            },
                            is_query_bind_mutable_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_content_token_stream
                                if let postgresql_crud_common::EqualOperator::Equal = &<T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#self_snake_case.#value_snake_case) {
                                    #if_let_err_query_try_bind_self_value_token_stream
                                }
                                Ok(#query_snake_case)
                            },
                        )
                    }
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual { .. } => {
                        let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, _, _, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_type_dimensions_helpers_postgresql_type(&postgresql_type_pattern_handle_array_dimension1);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_token_stream: Some(quote::quote! {#sqlx_type_postgresq_encode_token_stream + postgresql_crud_common::PostgresqlTypeEqualOperator}),
                            },
                            generate_maybe_dimensions_declaration_pub_value_t_token_stream(&maybe_dimensions_declaration_token_stream),
                            generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            quote::quote! {
                                #maybe_dimensions_indexes_initialization_token_stream
                                let operator = <T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#self_snake_case.#value_snake_case);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgresql_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_token_stream
                                        format!("{operator_query_str} ${value}")
                                    }
                                    postgresql_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({}{dimensions_indexes} {content})", &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case))
                            },
                            is_query_bind_mutable_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_content_token_stream
                                if let postgresql_crud_common::EqualOperator::Equal = &<T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(
                                    &#self_snake_case.#value_snake_case
                                ) {
                                    #if_let_err_query_try_bind_self_value_token_stream
                                }
                                Ok(#query_snake_case)
                            },
                        )
                    }
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression => generate_regular_expression_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before { .. } => generate_before_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore { .. } => generate_before_token_stream(&postgresql_type_pattern_handle_array_dimension1),
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
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthGreaterThan => generate_length_filter_pattern_token_stream(&">"),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => generate_equal_to_encoded_string_representation_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => generate_equal_to_encoded_string_representation_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesWithinGivenRange { .. } => generate_find_ranges_within_given_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesWithinGivenRange { .. } => generate_find_ranges_within_given_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => generate_find_ranges_that_fully_contain_the_given_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => generate_find_ranges_that_fully_contain_the_given_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange { .. } => generate_strictly_to_left_of_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange { .. } => generate_strictly_to_left_of_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange { .. } => generate_strictly_to_right_of_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange { .. } => generate_strictly_to_right_of_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound { .. } => generate_included_lower_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound { .. } => generate_included_lower_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound { .. } => generate_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound { .. } => generate_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanIncludedLowerBound { .. } => generate_greater_than_included_lower_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanIncludedLowerBound { .. } => generate_greater_than_included_lower_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanExcludedUpperBound { .. } => generate_greater_than_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanExcludedUpperBound { .. } => generate_greater_than_excluded_upper_bound_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange { .. } => generate_overlap_with_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange { .. } => generate_overlap_with_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange { .. } => generate_adjacent_with_range_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange { .. } => generate_adjacent_with_range_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => generate_range_length_token_stream(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength => generate_range_length_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                }
            };
            let struct_token_stream = generate_struct_token_stream(false, &should_add_declaration_of_struct_ident_generic, &ident, &struct_additional_fields_token_stream);
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&should_add_declaration_of_struct_ident_generic, &ident, &impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream);
            let impl_postgresql_type_where_filter_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                &FilterType::PostgresqlType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &increment_parameter_underscore,
                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
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
            //         macros_helpers::write_token_stream_into_file(
            //             "GeneratePostgresqlTypeWhereFilter",
            //             &generated,
            //             &macros_helpers::FormatWithRustfmt::True
            //         );
            //         proc_macro2::TokenStream::new(),
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
            let ident = naming::parameter::PostgresqlJsonTypeWhereSelfUpperCamelCase::from_display(&filter);
            let pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream = quote::quote! {
                pub #value_snake_case: PostgresqlJsonTypeNotEmptyUniqueVec<T>
            };
            let query_bind_sqlx_types_json_self_value_token_stream = quote::quote! {
                if let Err(#error_snake_case) = #query_snake_case.try_bind(sqlx::types::Json(#self_snake_case.#value_snake_case)) {
                    return Err(#error_snake_case.to_string());
                }
                Ok(#query_snake_case)
            };
            let generate_postgresql_json_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_postgresql_type_dimensions_helpers(postgresql_type_pattern_handle, &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType);
            let generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, generate_format_handle_stringified: &dyn Fn(&PostgresqlTypeKind) -> String| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_none.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_t_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&generate_format_handle_stringified(&postgresql_type_kind));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    },
                )
            };
            let generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operator: &dyn std::fmt::Display| generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}({{}}{} {operator} ${{}})", postgresql_type_kind.format_argument()));
            let generate_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream(postgresql_type_pattern_handle, &"=");
            let generate_all_elements_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))", postgresql_type_kind.format_argument()));
            let generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, operation: &dyn std::fmt::Display| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        pub #value_snake_case: #unsigned_part_of_std_primitive_i32_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #value_snake_case: #core_default_default_default_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(jsonb_array_length({{}}{}) {operation} ${{}})", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_one_value_token_stream
                    },
                )
            };
            let generate_length_equal_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream(postgresql_type_pattern_handle, &"=");
            let generate_length_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_token_stream(postgresql_type_pattern_handle, &">");
            let generate_greater_than_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_7cc8e29b_53e1_4bee_9947_71987439148c_token_stream(postgresql_type_pattern_handle, &">");
            let generate_contains_element_greater_than_token_stream =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))", postgresql_type_kind.format_argument()));
            let generate_all_elements_greater_than_token_stream =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| generate_1763ccf3_10be_4527_912b_363d8ea05f4b_token_stream(postgresql_type_pattern_handle, &|postgresql_type_kind: &PostgresqlTypeKind| format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))", postgresql_type_kind.format_argument()));
            let generate_between_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_between_t_token_stream
                    },
                    generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} {{}})", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    },
                )
            };
            let generate_in_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    generate_maybe_dimensions_default_initialization_value_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}}{} in ({{}}))", postgresql_type_kind.format_argument()));
                        let value_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(&value_snake_case, &value_snake_case, &quote::quote! {query_part_one_by_one});
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        match #self_snake_case.#value_snake_case.query_bind_one_by_one(#query_snake_case) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            }
                            Err(#error_snake_case) => {
                                return Err(#error_snake_case);
                            }
                        }
                        Ok(#query_snake_case)
                    },
                )
            };
            let generate_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_token_stream,
                    maybe_dimensions_default_initialization_token_stream,
                    maybe_dimensions_indexes_initialization_token_stream,
                    postgresql_type_kind, maybe_additional_parameters_token_stream,
                    maybe_dimensions_query_bind_content_token_stream
                ) = DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
                    |()| (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
                    |dimension_number| (
                        generate_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_token_stream(&dimension_number),
                        dimensions_default_initialization_comma_token_stream.clone(),
                        {
                            let dimensions_indexes_initialization_token_stream = generate_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_token_stream(&dimensions_indexes_snake_case, &dimensions_snake_case, &quote::quote! {postgresql_json_type_query_part_minus_one});
                            let last_dimensions_index_intialization_token_stream = generate_match_increment_checked_add_one_initialization_token_stream(&quote::quote! {last_dimensions_index});
                            quote::quote! {
                                #dimensions_indexes_initialization_token_stream
                                #last_dimensions_index_intialization_token_stream
                            }
                        },
                        PostgresqlTypeKind::ArrayDimension,
                        quote::quote! {
                            last_dimensions_index,
                            #dimensions_indexes_snake_case,
                        },
                        query_self_dimensions_query_bind_query_token_stream.clone(),
                    )
                );
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                            "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                            match &postgresql_type_kind {
                                PostgresqlTypeKind::Standart => "",
                                PostgresqlTypeKind::ArrayDimension => "{}->>${}",
                            }
                        ));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #self_snake_case.regular_expression_case.postgreql_syntax(),
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #if_let_err_query_try_bind_self_value_to_string_token_stream
                    },
                )
            };
            let generate_contains_element_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) {{}} ${{}}))", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #self_snake_case.regular_expression_case.postgreql_syntax(),
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #if_let_err_query_try_bind_self_value_to_string_token_stream
                    },
                )
            };
            let generate_all_elements_regular_expression_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #regular_expression_case_and_value_declaration_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #regular_expression_case_and_value_default_initialization_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{{}} ${{}}))", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_increment_checked_add_one_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #self_snake_case.regular_expression_case.postgreql_syntax(),
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #if_let_err_query_try_bind_self_value_to_string_token_stream
                    },
                )
            };
            let generate_contains_all_elements_of_array_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    quote::quote! {
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
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    },
                )
            };
            let generate_overlaps_with_array_token_stream = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (maybe_dimensions_declaration_token_stream, maybe_dimensions_default_initialization_token_stream, maybe_dimensions_indexes_initialization_token_stream, postgresql_type_kind, maybe_additional_parameters_token_stream, maybe_dimensions_query_bind_content_token_stream) = generate_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone.clone(),
                    quote::quote! {
                        #maybe_dimensions_declaration_token_stream
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_token_stream
                    },
                    quote::quote! {
                        #maybe_dimensions_default_initialization_token_stream
                        #value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                    },
                    {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))", postgresql_type_kind.format_argument()));
                        quote::quote! {
                            #maybe_dimensions_indexes_initialization_token_stream
                            #value_match_self_value_query_part_initialization_token_stream
                            Ok(format!(
                                #format_handle_token_stream,
                                &#self_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                #maybe_additional_parameters_token_stream
                                #value_snake_case
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote::quote! {
                        #maybe_dimensions_query_bind_content_token_stream
                        #query_bind_sqlx_types_json_self_value_token_stream
                    },
                )
            };
            let (should_add_declaration_of_struct_ident_generic, struct_additional_fields_token_stream, impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream, query_part_content_token_stream, is_query_bind_mutable, query_bind_content_token_stream) = match &filter {
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { .. } => generate_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual { .. } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual { .. } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual { .. } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual { .. } => generate_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual { .. } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual { .. } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual { .. } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual { .. } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsEqual { .. } => generate_all_elements_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => generate_length_equal_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan => generate_length_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan => generate_length_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan => generate_length_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan => generate_length_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthGreaterThan => generate_length_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan { .. } => generate_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan { .. } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan { .. } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan { .. } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan { .. } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElementGreaterThan { .. } => generate_contains_element_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan { .. } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan { .. } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan { .. } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan { .. } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsGreaterThan { .. } => generate_all_elements_greater_than_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween { .. } => generate_between_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn { .. } => generate_in_token_stream(&postgresql_type_pattern_handle_array_dimension4),
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
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsAllElementsOfArray { .. } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray { .. } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray { .. } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray { .. } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray { .. } => generate_contains_all_elements_of_array_token_stream(&postgresql_type_pattern_handle_array_dimension4),
                // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray { .. } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray { .. } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray { .. } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray { .. } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray { .. } => generate_overlaps_with_array_token_stream(&postgresql_type_pattern_handle_array_dimension4),
            };
            let struct_token_stream = generate_struct_token_stream(false, &should_add_declaration_of_struct_ident_generic, &ident, &struct_additional_fields_token_stream);
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&should_add_declaration_of_struct_ident_generic, &ident, &impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream);
            let impl_postgresql_type_where_filter_token_stream = generate_impl_postgresql_type_where_filter_token_stream(
                &FilterType::PostgresqlJsonType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
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
            //         // macros_helpers::write_token_stream_into_file(
            //         //     "GeneratePostgresqlTypeWhereFilter",
            //         //     &generated,
            //         //     &macros_helpers::FormatWithRustfmt::True
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
        //     macros_helpers::write_token_stream_into_file(
        //         "GeneratePostgresqlTypeWhereFilter",
        //         &generated,
        //         &macros_helpers::FormatWithRustfmt::True
        //     );
        // }
        generated
    };
    let generated = quote::quote! {
        #postgresql_type_token_stream
        #postgresql_json_type_token_stream
    };
    // macros_helpers::write_token_stream_into_file(
    //     "GeneratePostgresqlTypeWhereFilters",
    //     &generated,
    //     &macros_helpers::FormatWithRustfmt::True
    // );
    generated.into()
}
