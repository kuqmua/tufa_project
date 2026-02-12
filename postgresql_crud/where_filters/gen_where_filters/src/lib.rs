use naming::{
    ColumnSc, DimensionsIndexesSc, DimensionsSc, ErrorSc, IncrementSc, PubSc, QuerySc, SelfSc,
    ValueSc,
    parameter::{PostgresqlJsonTypeWhereSelfUcc, PostgresqlTypeWhereSelfUcc},
};
use quote::quote;
use std::fmt::Display;

#[proc_macro]
pub fn gen_where_filters(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Clone)]
    enum ShouldAddDeclarationOfStructIdentGeneric {
        False,
        True {
            maybe_additional_traits_ts: Option<proc_macro2::TokenStream>,
        },
    }
    enum FilterType {
        PostgresqlJsonType,
        PostgresqlType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
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
    #[allow(clippy::arbitrary_source_item_ordering)]
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
        fn dimension_ts(&self) -> proc_macro2::TokenStream {
            self.dimension_std_primitive_u8()
                .to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("18c32bc0-2e55-4b4d-a6a8-b8680e5fe463")
        }
    }
    enum KindOfUnsignedPartOfStdPrimitiveI32 {
        CanBeZero,
        CanNotBeZero,
    }
    impl quote::ToTokens for KindOfUnsignedPartOfStdPrimitiveI32 {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::CanBeZero => quote! {UnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
                Self::CanNotBeZero => {
                    quote! {NotZeroUnsignedPartOfStdPrimitiveI32}.to_tokens(tokens);
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
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
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    struct GenWhereFiltersConfig {
        postgresql_types_content_write_into_gen_where_filters_postgresql_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        postgresql_json_types_content_write_into_gen_where_filters_postgresql_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_where_filters: macros_helpers::ShouldWriteTokenStreamIntoFile,
    }
    panic_location::panic_location();
    let gen_where_filters_config =
        serde_json::from_str::<GenWhereFiltersConfig>(&input_ts.to_string())
            .expect("1217b73b-456c-4a3e-8a5a-5d5b8b8c3452");
    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
    let t_ts = quote! {T};
    let t_annotation_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = proc_macro2::TokenStream::new();
    let core_default_default_default_ts = token_patterns::CoreDefaultDefaultDefault;
    let postgresql_crud_common_default_option_some_vec_one_el_ts =
        token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneEl;
    let postgresql_crud_common_default_option_some_vec_one_el_call_ts =
        token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneElCall;
    let pub_value_t_ts = quote! {pub #ValueSc: T};
    let unsigned_part_of_std_primitive_i32_ts =
        quote! {postgresql_crud_common::UnsignedPartOfStdPrimitiveI32};
    let not_zero_unsigned_part_of_std_primitive_i32_ts =
        quote! {postgresql_crud_common::NotZeroUnsignedPartOfStdPrimitiveI32};
    let value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts =
        quote! {#ValueSc: #not_zero_unsigned_part_of_std_primitive_i32_ts};
    let pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts =
        quote! {pub #value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts};
    let value_default_option_some_vec_one_el_ts = quote! {
        #ValueSc: #postgresql_crud_common_default_option_some_vec_one_el_call_ts
    };
    let gen_struct_ts = |filter_initialized_with_try_new_result_is_ok: bool, should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn quote::ToTokens, struct_additional_fields_ts: &dyn quote::ToTokens| {
        let maybe_pub_ts: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_ts_new } else { &PubSc };
        let maybe_derive_serde_deserialize_ts: &dyn quote::ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_ts_new } else { &quote! {serde::Deserialize,} };
        let maybe_declaration_of_struct_ident_generic_ts: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
            ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                &maybe_additional_traits_ts.as_ref().map_or_else(|| quote! {<#t_ts>}, |value_d05f3d4f| quote! {<#t_ts: #value_d05f3d4f>})
            }
        };
        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_derive_serde_deserialize_ts schemars::JsonSchema)]
            pub struct #ident #maybe_declaration_of_struct_ident_generic_ts {
                #maybe_pub_ts logical_operator: #import_path::LogicalOperator,
                #struct_additional_fields_ts
            }
        }
    };
    let gen_impl_default_option_some_vec_one_el_ts = |should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn quote::ToTokens, impl_default_option_some_vec_one_el_additional_fields_ts: &dyn quote::ToTokens| {
        postgresql_crud_macros_common::gen_impl_default_option_some_vec_one_el_ts(
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new(),
                ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                    maybe_additional_traits_ts.as_ref().map_or_else(
                        || quote! {<T: #postgresql_crud_common_default_option_some_vec_one_el_ts>},
                        |value_29913af7| quote! {<T: #value_29913af7 + #postgresql_crud_common_default_option_some_vec_one_el_ts>}
                    )
                }
            },
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon,
            &ident,
            match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            &quote! {
                Self {
                    logical_operator: #postgresql_crud_common_default_option_some_vec_one_el_call_ts,
                    #impl_default_option_some_vec_one_el_additional_fields_ts
                }
            },
        )
    };
    let gen_impl_postgresql_type_where_filter_ts = |filter_type: &FilterType,
                                                                   should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
                                                                   ident: &dyn quote::ToTokens,
                                                                   increment_parameter_underscore: &postgresql_crud_macros_common::IncrementParameterUnderscore,
                                                                   is_need_to_add_logical_operator_underscore: &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore,
                                                                   query_part_content_ts: &dyn quote::ToTokens,
                                                                   is_query_bind_mutable: &postgresql_crud_macros_common::IsQueryBindMutable,
                                                                   query_bind_content_ts: &dyn quote::ToTokens| {
        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_ts(
            &{
                let maybe_t_additional_traits_for_postgresql_type_where_filter_ts: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                    ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                        let send_and_lifetime_ts = quote! {Send + 'lifetime};
                        let serde_serialize_ts = quote! {serde::Serialize};
                        let content_ts = match (&filter_type, &maybe_additional_traits_ts) {
                            (FilterType::PostgresqlType, Some(value)) => &quote! {#value + #send_and_lifetime_ts},
                            (FilterType::PostgresqlType, None) => &send_and_lifetime_ts,
                            (FilterType::PostgresqlJsonType, Some(value)) => &quote! {#value + #serde_serialize_ts + #send_and_lifetime_ts},
                            (FilterType::PostgresqlJsonType, None) => &quote! {#serde_serialize_ts + #send_and_lifetime_ts},
                        };
                        &quote! {, T: #content_ts}
                    }
                };
                quote! {<'lifetime #maybe_t_additional_traits_for_postgresql_type_where_filter_ts>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            increment_parameter_underscore,
            &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
            is_need_to_add_logical_operator_underscore,
            &query_part_content_ts,
            is_query_bind_mutable,
            &query_bind_content_ts,
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon,
        )
    };
    let regular_expression_case_and_value_declaration_ts = quote! {
        pub regular_expression_case: RegularExpressionCase,
        pub value: RegexRegex
    };
    let regular_expression_case_and_value_default_initialization_ts = quote! {
        regular_expression_case: #postgresql_crud_common_default_option_some_vec_one_el_call_ts,
        #value_default_option_some_vec_one_el_ts
    };
    let if_let_err_query_try_bind_self_value_to_string_ts = quote! {
        if let Err(#ErrorSc) = #QuerySc.try_bind(#SelfSc.#ValueSc.to_string()) {
            return Err(#ErrorSc.to_string());
        }
        Ok(#QuerySc)
    };
    let if_let_err_query_try_bind_self_value_ts = quote! {
        if let Err(#ErrorSc) = #QuerySc.try_bind(#SelfSc.#ValueSc) {
            return Err(#ErrorSc.to_string());
        }
    };
    let query_bind_one_value_ts = quote! {
        #if_let_err_query_try_bind_self_value_ts
        Ok(#QuerySc)
    };
    let should_add_declaration_of_struct_ident_generic_false =
        ShouldAddDeclarationOfStructIdentGeneric::False;
    let should_add_declaration_of_struct_ident_generic_true_none =
        ShouldAddDeclarationOfStructIdentGeneric::True {
            maybe_additional_traits_ts: None,
        };
    let gen_match_increment_checked_add_one_initialization_ts = |ident_ts: &dyn quote::ToTokens| {
        quote! {
            let #ident_ts = match postgresql_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
                Ok(value_25d59e01) => value_25d59e01,
                Err(#ErrorSc) => {
                    return Err(#ErrorSc);
                },
            };
        }
    };
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone =
        ShouldAddDeclarationOfStructIdentGeneric::True {
            maybe_additional_traits_ts: Some(quote! {std::fmt::Debug + PartialEq + Clone}),
        };
    let should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode =
        ShouldAddDeclarationOfStructIdentGeneric::True {
            maybe_additional_traits_ts: Some(quote! {
                std::fmt::Debug
                + PartialEq
                + PartialOrd
                + Clone
                + sqlx::Type<sqlx::Postgres>
                + for<'__> sqlx::Encode<'__, sqlx::Postgres>
            }),
        };
    let value_between_t_ts = quote! {#ValueSc: Between<T>};
    let pub_value_between_t_ts = quote! {pub #value_between_t_ts};
    let query_self_value_query_bind_ts = quote! {
        match #SelfSc.#ValueSc.query_bind(#QuerySc) {
            Ok(value_f6d31bdd) => {
                #QuerySc = value_f6d31bdd;
            },
            Err(#ErrorSc) => {
                return Err(#ErrorSc);
            }
        }
        Ok(#QuerySc)
    };
    let postgresql_type_pattern_handle_standart = PostgresqlTypePatternHandle::Standart;
    let postgresql_type_pattern_handle_array_dimension1 =
        PostgresqlTypePatternHandle::ArrayDimension1;
    let postgresql_type_pattern_handle_array_dimension2 =
        PostgresqlTypePatternHandle::ArrayDimension2;
    let postgresql_type_pattern_handle_array_dimension3 =
        PostgresqlTypePatternHandle::ArrayDimension3;
    let postgresql_type_pattern_handle_array_dimension4 =
        PostgresqlTypePatternHandle::ArrayDimension4;
    let gen_pub_dimensions_bounded_vec_ts =
        |vec_length_ts: &dyn quote::ToTokens,
         kind_of_unsigned_part_of_std_primitive_i32: &KindOfUnsignedPartOfStdPrimitiveI32| {
            quote! {pub #DimensionsSc: BoundedStdVecVec<postgresql_crud_common::#kind_of_unsigned_part_of_std_primitive_i32, #vec_length_ts>}
        };
    let value_match_increment_checked_add_one_initialization_ts =
        gen_match_increment_checked_add_one_initialization_ts(&ValueSc);
    let gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts =
        |ident_ts: &dyn quote::ToTokens,
         field_ts: &dyn quote::ToTokens,
         function_ts: &dyn quote::ToTokens| {
            quote! {
                let #ident_ts = match self.#field_ts.#function_ts(#IncrementSc, #ColumnSc, is_need_to_add_logical_operator) {
                    Ok(value_0a22ee9a) => value_0a22ee9a,
                    Err(#ErrorSc) => {
                        return Err(#ErrorSc);
                    }
                };
            }
        };
    let value_match_self_value_query_part_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&ValueSc, &ValueSc, &quote! {query_part});
    let dimensions_default_initialization_ts = quote! {
        #DimensionsSc: #postgresql_crud_common_default_option_some_vec_one_el_call_ts
    };
    let dimensions_default_initialization_comma_ts =
        quote! {#dimensions_default_initialization_ts,};
    let query_self_dimensions_query_bind_query_ts = quote! {
        match #SelfSc.#DimensionsSc.query_bind(#QuerySc) {
            Ok(value_ed6f1157) => {
                #QuerySc = value_ed6f1157;
            }
            Err(#ErrorSc) => {
                return Err(#ErrorSc);
            }
        }
    };
    let dimensions_indexes_comma_ts = quote! {#DimensionsIndexesSc,};
    let gen_maybe_dimensions_declaration_pub_value_t_ts =
        |maybe_dimensions_declaration_ts: &dyn quote::ToTokens| {
            quote! {
                #maybe_dimensions_declaration_ts
                #pub_value_t_ts
            }
        };
    let gen_maybe_dimensions_default_initialization_value_default_ts =
        |maybe_dimensions_default_initialization_ts: &dyn quote::ToTokens| {
            quote! {
                #maybe_dimensions_default_initialization_ts
                #value_default_option_some_vec_one_el_ts
            }
        };
    let is_query_bind_mutable_true = postgresql_crud_macros_common::IsQueryBindMutable::True;
    let is_query_bind_mutable_false = postgresql_crud_macros_common::IsQueryBindMutable::False;
    let gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_ts =
        |dimension_number: &DimensionNumber| {
            let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_ts =
                gen_pub_dimensions_bounded_vec_ts(
                    &dimension_number.dimension_ts(),
                    &KindOfUnsignedPartOfStdPrimitiveI32::CanNotBeZero,
                );
            quote! {#pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_ts,}
        };
    let gen_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_ts =
        |dimension_number: &DimensionNumber| {
            let pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_ts =
                gen_pub_dimensions_bounded_vec_ts(
                    &dimension_number.dimension_ts(),
                    &KindOfUnsignedPartOfStdPrimitiveI32::CanBeZero,
                );
            quote! {#pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_ts,}
        };
    let gen_postgresql_type_dimensions_helpers = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle, postgresql_type_or_postgresql_json_type: &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType| {
        DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
            |()| (proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
            |dimension_number| (
                match &postgresql_type_or_postgresql_json_type {
                    postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                    postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => gen_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                },
                dimensions_default_initialization_comma_ts.clone(),
                gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(
                    &DimensionsIndexesSc,
                    &DimensionsSc,
                    &match &postgresql_type_or_postgresql_json_type {
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => quote! {postgresql_type_query_part},
                        postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => quote! {postgresql_json_type_query_part},
                    },
                ),
                PostgresqlTypeKind::ArrayDimension,
                dimensions_indexes_comma_ts.clone(),
                query_self_dimensions_query_bind_query_ts.clone(),
            )
        )
    };
    let postgresql_type_ts = {
        let gen_filters_ts = |filter: &postgresql_crud_macros_common::PostgresqlTypeFilter| {
            let ident = PostgresqlTypeWhereSelfUcc::from_display(&filter);
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_ts,
                impl_default_option_some_vec_one_el_additional_fields_ts,
                increment_parameter_underscore,
                query_part_content_ts,
                is_query_bind_mutable,
                query_bind_content_ts,
            ) = {
                let sqlx_type_postgresq_encode_ts = quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let should_add_declaration_of_struct_ident_generic_true_type_encode =
                    ShouldAddDeclarationOfStructIdentGeneric::True {
                        maybe_additional_traits_ts: Some(sqlx_type_postgresq_encode_ts.clone()),
                    };
                let pub_value_postgresql_type_not_empty_unique_vec_t_ts =
                    quote! {pub #ValueSc: PostgresqlTypeNotEmptyUniqueVec<T>};
                let gen_postgresql_type_dimensions_helpers_postgresql_type =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_postgresql_type_dimensions_helpers(
                    postgresql_type_pattern_handle,
                    &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlType
                )
                    };
                let gen_32abfefc_c087_480b_b502_cb78533dafb0_ts =
                        |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                         gen_format_handle_str: &dyn Fn(
                            &PostgresqlTypeKind,
                        )
                            -> String| {
                            let (
                                maybe_dimensions_declaration_ts,
                                maybe_dimensions_default_initialization_ts,
                                maybe_dimensions_indexes_initialization_ts,
                                postgresql_type_kind,
                                maybe_additional_parameters_ts,
                                maybe_dimensions_query_bind_content_ts,
                            ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                                postgresql_type_pattern_handle,
                            );
                            (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                        gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&gen_format_handle_str(&postgresql_type_kind));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_bind_one_value_ts
                        },
                    )
                        };
                let gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                     operator: &dyn Display| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgresql_type_pattern_handle,
                            &|postgresql_type_kind: &PostgresqlTypeKind| {
                                format!(
                                    "{{}}({{}}{} {operator} ${{}})",
                                    postgresql_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &">",
                        )
                    };
                let gen_between_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgresql_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                            postgresql_type_pattern_handle,
                        );
                        (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_between_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", postgresql_type_kind.format_argument()));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_self_value_query_part_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_self_value_query_bind_ts
                        },
                    )
                    };
                let gen_in_ts = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                        postgresql_type_pattern_handle,
                    );
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::True {
                            maybe_additional_traits_ts: Some(
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_postgresq_encode_ts},
                            ),
                        },
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgresql_type_not_empty_unique_vec_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                postgresql_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                                &quote! {acc_14596a52, "${value_daedba9c},"},
                                &quote! {panic!("87f47f75-b2db-4d88-a0f0-e254ac7d14a3");},
                            );
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let #ValueSc = {
                                    let mut acc_14596a52 = String::default();
                                    for _ in #SelfSc.#ValueSc.to_vec() {
                                        match postgresql_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
                                            Ok(value_daedba9c) => {
                                                #if_write_is_err_ts
                                            },
                                            Err(#ErrorSc) => {
                                                return Err(#ErrorSc);
                                            },
                                        }
                                    }
                                    let _ = acc_14596a52.pop();
                                    acc_14596a52
                                };
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            for el_ea865d8c in #SelfSc.#ValueSc.into_vec() {
                                if let Err(#ErrorSc) = #QuerySc.try_bind(el_ea865d8c) {
                                    return Err(#ErrorSc.to_string());
                                }
                            }
                            Ok(#QuerySc)
                        },
                    )
                };
                let gen_regular_expression_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgresql_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                            postgresql_type_pattern_handle,
                        );
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            quote! {
                                #maybe_dimensions_declaration_ts
                                #regular_expression_case_and_value_declaration_ts
                            },
                            quote! {
                                #maybe_dimensions_default_initialization_ts
                                #regular_expression_case_and_value_default_initialization_ts
                            },
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}({{}}{} {{}} ${{}})",
                                    postgresql_type_kind.format_argument()
                                ));
                                quote! {
                                    #maybe_dimensions_indexes_initialization_ts
                                    #value_match_increment_checked_add_one_initialization_ts
                                    Ok(format!(
                                        #format_handle_ts,
                                        &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                        #ColumnSc,
                                        #maybe_additional_parameters_ts
                                        #SelfSc.regular_expression_case.postgreql_syntax(),
                                        #ValueSc
                                    ))
                                }
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                #if_let_err_query_try_bind_self_value_to_string_ts
                            },
                        )
                    };
                let gen_before_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgresql_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                            postgresql_type_pattern_handle,
                        );
                        (
                            should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                            gen_maybe_dimensions_declaration_pub_value_t_ts(
                                &maybe_dimensions_declaration_ts,
                            ),
                            gen_maybe_dimensions_default_initialization_value_default_ts(
                                &maybe_dimensions_default_initialization_ts,
                            ),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}({{}}{} < ${{}})",
                                    postgresql_type_kind.format_argument()
                                ));
                                quote! {
                                    #maybe_dimensions_indexes_initialization_ts
                                    #value_match_increment_checked_add_one_initialization_ts
                                    Ok(format!(
                                        #format_handle_ts,
                                        &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                        #ColumnSc,
                                        #maybe_additional_parameters_ts
                                        #ValueSc
                                    ))
                                }
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                #query_bind_one_value_ts
                            },
                        )
                    };
                let gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                     postgresql_syntax: &dyn Display| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgresql_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                            postgresql_type_pattern_handle,
                        );
                        (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => postgresql_crud_macros_common::IncrementParameterUnderscore::True,
                            PostgresqlTypePatternHandle::ArrayDimension1 | PostgresqlTypePatternHandle::ArrayDimension2 | PostgresqlTypePatternHandle::ArrayDimension3 | PostgresqlTypePatternHandle::ArrayDimension4 => postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {postgresql_syntax})", postgresql_type_kind.format_argument()));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                ))
                            }
                        },
                        match &postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => is_query_bind_mutable_false,
                            PostgresqlTypePatternHandle::ArrayDimension1 | PostgresqlTypePatternHandle::ArrayDimension2 | PostgresqlTypePatternHandle::ArrayDimension3 | PostgresqlTypePatternHandle::ArrayDimension4 => is_query_bind_mutable_true,
                        },
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            Ok(#QuerySc)
                        },
                    )
                    };
                let gen_current_date_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"= current_date",
                        )
                    };
                let gen_greater_than_current_date_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"> current_date",
                        )
                    };
                let gen_current_timestamp_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"= current_timestamp",
                        )
                    };
                let gen_greater_than_current_timestamp_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"> current_timestamp",
                        )
                    };
                let gen_current_time_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"= current_time",
                        )
                    };
                let gen_greater_than_current_time_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgresql_type_pattern_handle,
                            &"> current_time",
                        )
                    };
                let gen_equal_to_encoded_string_representation_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgresql_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgresql_type_dimensions_helpers_postgresql_type(
                            postgresql_type_pattern_handle,
                        );
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            quote! {
                                #maybe_dimensions_declaration_ts
                                pub encode_format: EncodeFormat,
                                pub encoded_string_representation: String,
                            },
                            quote! {
                                #maybe_dimensions_default_initialization_ts
                                encode_format: #postgresql_crud_common_default_option_some_vec_one_el_call_ts,
                                encoded_string_representation: #core_default_default_default_ts
                            },
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                    postgresql_type_kind.format_argument()
                                ));
                                quote! {
                                    #maybe_dimensions_indexes_initialization_ts
                                    #value_match_increment_checked_add_one_initialization_ts
                                    Ok(format!(
                                        #format_handle_ts,
                                        &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                        #ColumnSc,
                                        #maybe_additional_parameters_ts
                                        &#SelfSc.encode_format,
                                        #ValueSc
                                    ))
                                }
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let Err(#ErrorSc) = #QuerySc.try_bind(self.encoded_string_representation) {
                                    return Err(#ErrorSc.to_string());
                                }
                                Ok(#QuerySc)
                            },
                        )
                    };
                let gen_find_ranges_within_given_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"<@",
                        )
                    };
                let gen_find_ranges_that_fully_contain_the_given_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"@>",
                        )
                    };
                let gen_strictly_to_left_of_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"&<",
                        )
                    };
                let gen_strictly_to_right_of_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"&>",
                        )
                    };
                let gen_overlap_with_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"&&",
                        )
                    };
                let gen_adjacent_with_range_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgresql_type_pattern_handle,
                            &"-|-",
                        )
                    };
                let gen_length_filter_pattern_ts = |operator: &dyn Display| {
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts
                            .clone(),
                        value_default_option_some_vec_one_el_ts.clone(),
                        postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(array_length({{}}, 1) {operator} ${{}})"
                            ));
                            quote! {
                                match #import_path::increment_checked_add_one_returning_increment(#IncrementSc) {
                                    Ok(value_f7988de8) => Ok(format!(#format_handle_ts, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc, value_f7988de8)),
                                    Err(#ErrorSc) => Err(#ErrorSc),
                                }
                            }
                        },
                        is_query_bind_mutable_true,
                        query_bind_one_value_ts.clone(),
                    )
                };
                let gen_included_lower_bound_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgresql_type_pattern_handle,
                            &|postgresql_type_kind: &PostgresqlTypeKind| {
                                format!(
                                    "{{}}(lower({{}}{}) = ${{}})",
                                    postgresql_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_excluded_upper_bound_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgresql_type_pattern_handle,
                            &|postgresql_type_kind: &PostgresqlTypeKind| {
                                format!(
                                    "{{}}(upper({{}}{}) = ${{}})",
                                    postgresql_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_included_lower_bound_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgresql_type_pattern_handle,
                            &|postgresql_type_kind: &PostgresqlTypeKind| {
                                format!(
                                    "{{}}(lower({{}}{}) > ${{}})",
                                    postgresql_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_excluded_upper_bound_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgresql_type_pattern_handle,
                            &|postgresql_type_kind: &PostgresqlTypeKind| {
                                format!(
                                    "{{}}(upper({{}}{}) > ${{}})",
                                    postgresql_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_range_length_ts =
                    |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                        let (
                        maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts
                    ) = DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
                        |()| (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, quote! {#ColumnSc,}, proc_macro2::TokenStream::new()),
                        |dimension_number| (
                            gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                            dimensions_default_initialization_comma_ts.clone(),
                            {
                                let dimensions_indexes1_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes1}, &DimensionsSc, &quote! {postgresql_type_query_part});
                                let dimensions_indexes2_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes2}, &DimensionsSc, &quote! {postgresql_type_query_part});
                                quote! {
                                    #dimensions_indexes1_ts
                                    #dimensions_indexes2_ts
                                }
                            },
                            PostgresqlTypeKind::ArrayDimension,
                            quote! {
                                dimensions_indexes1,
                                column,
                                dimensions_indexes2,
                            },
                            quote! {
                                match #SelfSc.#DimensionsSc.clone().query_bind(#QuerySc) {
                                    Ok(value_6cb14cdc) => {
                                        #QuerySc = value_6cb14cdc;
                                    },
                                    Err(#ErrorSc) => {
                                        return Err(#ErrorSc);
                                    }
                                }
                                #query_self_dimensions_query_bind_query_ts
                            },
                        )
                    );
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::False,
                            quote! {
                                #maybe_dimensions_declaration_ts
                                #pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts
                            },
                            gen_maybe_dimensions_default_initialization_value_default_ts(
                                &maybe_dimensions_default_initialization_ts,
                            ),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                    postgresql_type_kind.format_argument(),
                                    postgresql_type_kind.format_argument(),
                                ));
                                quote! {
                                    #maybe_dimensions_indexes_initialization_ts
                                    #value_match_increment_checked_add_one_initialization_ts
                                    Ok(format!(
                                        #format_handle_ts,
                                        &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                        #ColumnSc,
                                        #maybe_additional_parameters_ts
                                        #ValueSc
                                    ))
                                }
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                #query_bind_one_value_ts
                            },
                        )
                    };
                match &filter {
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { .. } => {
                        let (maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts, maybe_dimensions_indexes_initialization_ts, _, _, maybe_dimensions_query_bind_content_ts) = gen_postgresql_type_dimensions_helpers_postgresql_type(&postgresql_type_pattern_handle_standart);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(quote! {#sqlx_type_postgresq_encode_ts + postgresql_crud_common::PostgresqlTypeEqualOperator}),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                            gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgresql_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    },
                                    postgresql_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let postgresql_crud_common::EqualOperator::Equal = &<T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#SelfSc.#ValueSc) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual { .. } => {
                        let (maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts, maybe_dimensions_indexes_initialization_ts, _, _, maybe_dimensions_query_bind_content_ts) = gen_postgresql_type_dimensions_helpers_postgresql_type(&postgresql_type_pattern_handle_array_dimension1);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(quote! {#sqlx_type_postgresq_encode_ts + postgresql_crud_common::PostgresqlTypeEqualOperator}),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                            gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                            postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgresql_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    }
                                    postgresql_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({}{dimensions_indexes} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let postgresql_crud_common::EqualOperator::Equal = &<T as postgresql_crud_common::PostgresqlTypeEqualOperator>::operator(
                                    &#SelfSc.#ValueSc
                                ) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Between { .. } => gen_between_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween { .. } => gen_between_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::In { .. } => gen_in_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn { .. } => gen_in_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::Before { .. } => gen_before_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore { .. } => gen_before_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate => gen_current_date_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate => gen_current_date_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate => gen_greater_than_current_date_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate => gen_greater_than_current_date_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp => gen_current_timestamp_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp => gen_current_timestamp_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp => gen_greater_than_current_timestamp_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp => gen_greater_than_current_timestamp_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime => gen_current_time_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime => gen_current_time_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime => gen_greater_than_current_time_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime => gen_greater_than_current_time_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual => gen_length_filter_pattern_ts(&"="),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthGreaterThan => gen_length_filter_pattern_ts(&">"),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation => gen_equal_to_encoded_string_representation_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => gen_equal_to_encoded_string_representation_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesWithinGivenRange { .. } => gen_find_ranges_within_given_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesWithinGivenRange { .. } => gen_find_ranges_within_given_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => gen_find_ranges_that_fully_contain_the_given_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => gen_find_ranges_that_fully_contain_the_given_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange { .. } => gen_strictly_to_left_of_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange { .. } => gen_strictly_to_left_of_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange { .. } => gen_strictly_to_right_of_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange { .. } => gen_strictly_to_right_of_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound { .. } => gen_included_lower_bound_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound { .. } => gen_included_lower_bound_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound { .. } => gen_excluded_upper_bound_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound { .. } => gen_excluded_upper_bound_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanIncludedLowerBound { .. } => gen_greater_than_included_lower_bound_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanIncludedLowerBound { .. } => gen_greater_than_included_lower_bound_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanExcludedUpperBound { .. } => gen_greater_than_excluded_upper_bound_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanExcludedUpperBound { .. } => gen_greater_than_excluded_upper_bound_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange { .. } => gen_overlap_with_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange { .. } => gen_overlap_with_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange { .. } => gen_adjacent_with_range_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange { .. } => gen_adjacent_with_range_ts(&postgresql_type_pattern_handle_array_dimension1),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength => gen_range_length_ts(&postgresql_type_pattern_handle_standart),
                    postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength => gen_range_length_ts(&postgresql_type_pattern_handle_array_dimension1),
                }
            };
            let struct_ts = gen_struct_ts(
                false,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &struct_additional_fields_ts,
            );
            let impl_default_option_some_vec_one_el_ts = gen_impl_default_option_some_vec_one_el_ts(
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &impl_default_option_some_vec_one_el_additional_fields_ts,
            );
            let impl_postgresql_type_where_filter_ts = gen_impl_postgresql_type_where_filter_ts(
                &FilterType::PostgresqlType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &increment_parameter_underscore,
                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_postgresql_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts = postgresql_crud_macros_common::PostgresqlTypeFilter::into_array()
            .map(|el_7cfb1929| gen_filters_ts(&el_7cfb1929));
        let gend = quote! {#(#filter_array_ts)*};
        macros_helpers::maybe_write_ts_into_file(
            gen_where_filters_config
                .postgresql_types_content_write_into_gen_where_filters_postgresql_types,
            "gen_where_filters_postgresql_types",
            &gend,
            &macros_helpers::FormatWithCargofmt::True,
        );
        gend
    };
    let postgresql_json_type_ts = {
        let gen_filters_ts = |filter: &postgresql_crud_macros_common::PostgresqlJsonTypeFilter| {
            let ident = PostgresqlJsonTypeWhereSelfUcc::from_display(&filter);
            let pub_value_postgresql_json_type_not_empty_unique_vec_t_ts = quote! {
                pub #ValueSc: PostgresqlJsonTypeNotEmptyUniqueVec<T>
            };
            let query_bind_sqlx_types_json_self_value_ts = quote! {
                if let Err(#ErrorSc) = #QuerySc.try_bind(sqlx::types::Json(#SelfSc.#ValueSc)) {
                    return Err(#ErrorSc.to_string());
                }
                Ok(#QuerySc)
            };
            let gen_postgresql_json_type_dimensions_helpers =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_postgresql_type_dimensions_helpers(postgresql_type_pattern_handle, &postgresql_crud_macros_common::PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType)
                };
            let gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                 gen_format_handle_str: &dyn Fn(&PostgresqlTypeKind) -> String| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_none.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(
                                &gen_format_handle_str(&postgresql_type_kind),
                            );
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_bind_sqlx_types_json_self_value_ts
                        },
                    )
                };
            let gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                 operator: &dyn Display| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind| {
                            format!(
                                "{{}}({{}}{} {operator} ${{}})",
                                postgresql_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_equal_ts = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(postgresql_type_pattern_handle, &"=")
            };
            let gen_all_elements_equal_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind| {
                            format!(
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))",
                                postgresql_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle,
                 operation: &dyn Display| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            pub #ValueSc: #unsigned_part_of_std_primitive_i32_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #ValueSc: #core_default_default_default_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(jsonb_array_length({{}}{}) {operation} ${{}})",
                                postgresql_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_bind_one_value_ts
                        },
                    )
                };
            let gen_length_equal_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(
                        postgresql_type_pattern_handle,
                        &"=",
                    )
                };
            let gen_length_greater_than_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(
                        postgresql_type_pattern_handle,
                        &">",
                    )
                };
            let gen_greater_than_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(
                        postgresql_type_pattern_handle,
                        &">",
                    )
                };
            let gen_contains_el_greater_than_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind| {
                            format!(
                                "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))",
                                postgresql_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_all_elements_greater_than_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgresql_type_pattern_handle,
                        &|postgresql_type_kind: &PostgresqlTypeKind| {
                            format!(
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))",
                                postgresql_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_between_ts = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgresql_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_between_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                    {
                        let content_ts: &dyn quote::ToTokens = match postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => &quote!{
                                let value = match self.value.query_part(
                                    increment,
                                    column,
                                    is_need_to_add_logical_operator
                                ) {
                                    Ok(value_cc8dda2f) => value_cc8dda2f,
                                    Err(error) => {
                                        return Err(error);
                                    }
                                };
                            },
                            PostgresqlTypePatternHandle::ArrayDimension1 |
                            PostgresqlTypePatternHandle::ArrayDimension2 |
                            PostgresqlTypePatternHandle::ArrayDimension3 |
                            PostgresqlTypePatternHandle::ArrayDimension4 => &value_match_increment_checked_add_one_initialization_ts
                        };
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", postgresql_type_kind.format_argument()));
                        quote! {
                            #maybe_dimensions_indexes_initialization_ts
                            #content_ts
                            Ok(format!(
                                #format_handle_ts,
                                &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #ColumnSc,
                                #maybe_additional_parameters_ts
                                #ValueSc
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    {
                        let content_ts: &dyn quote::ToTokens = match postgresql_type_pattern_handle {
                            PostgresqlTypePatternHandle::Standart => &quote!{
                                match self.value.query_bind(query) {
                                    Ok(value) => {
                                        query = value;
                                    },
                                    Err(error) => {
                                        return Err(error.to_string());
                                    }
                                }
                                Ok(query)
                            },
                            PostgresqlTypePatternHandle::ArrayDimension1 |
                            PostgresqlTypePatternHandle::ArrayDimension2 |
                            PostgresqlTypePatternHandle::ArrayDimension3 |
                            PostgresqlTypePatternHandle::ArrayDimension4 => &query_bind_sqlx_types_json_self_value_ts
                        };
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #content_ts
                        }
                    },
                )
            };
            let gen_in_ts = |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgresql_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_postgresql_json_type_not_empty_unique_vec_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(
                        &maybe_dimensions_default_initialization_ts,
                    ),
                    {
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                            "{{}}({{}}{} in ({{}}))",
                            postgresql_type_kind.format_argument()
                        ));
                        let value_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&ValueSc, &ValueSc, &quote! {query_part_one_by_one});
                        quote! {
                            #maybe_dimensions_indexes_initialization_ts
                            #value_initialization_ts
                            Ok(format!(
                                #format_handle_ts,
                                &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #ColumnSc,
                                #maybe_additional_parameters_ts
                                #ValueSc
                            ))
                        }
                    },
                    is_query_bind_mutable_true,
                    quote! {
                        #maybe_dimensions_query_bind_content_ts
                        match #SelfSc.#ValueSc.query_bind_one_by_one(#QuerySc) {
                            Ok(value_c79b2256) => {
                                #QuerySc = value_c79b2256;
                            }
                            Err(#ErrorSc) => {
                                return Err(#ErrorSc);
                            }
                        }
                        Ok(#QuerySc)
                    },
                )
            };
            let gen_regular_expression_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgresql_type_kind, maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts
                ) = DimensionNumber::try_from(postgresql_type_pattern_handle).map_or_else(
                    |()| (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), PostgresqlTypeKind::Standart, proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
                    |dimension_number| (
                        gen_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                        dimensions_default_initialization_comma_ts.clone(),
                        {
                            let dimensions_indexes_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&DimensionsIndexesSc, &DimensionsSc, &quote! {postgresql_json_type_query_part_minus_one});
                            let last_dimensions_index_intialization_ts = gen_match_increment_checked_add_one_initialization_ts(&quote! {last_dimensions_index});
                            quote! {
                                #dimensions_indexes_initialization_ts
                                #last_dimensions_index_intialization_ts
                            }
                        },
                        PostgresqlTypeKind::ArrayDimension,
                        quote! {
                            last_dimensions_index,
                            #DimensionsIndexesSc,
                        },
                        query_self_dimensions_query_bind_query_ts.clone(),
                    )
                );
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                                match &postgresql_type_kind {
                                    PostgresqlTypeKind::Standart => "",
                                    PostgresqlTypeKind::ArrayDimension => "{}->>${}",
                                }
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #SelfSc.regular_expression_case.postgreql_syntax(),
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
            let gen_contains_el_regular_expression_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                //todo test it properly using all strange string variants
                                "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') {{}} ${{}}))",
                                // "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) {{}} ${{}}))",
                                postgresql_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #SelfSc.regular_expression_case.postgreql_syntax(),
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
            let gen_all_elements_regular_expression_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                //todo test it properly using all strange string variants
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') !{{}} ${{}}))",
                                // "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{{}} ${{}}))",
                                postgresql_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_increment_checked_add_one_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #SelfSc.regular_expression_case.postgreql_syntax(),
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
            let gen_contains_all_elements_of_array_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgresql_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} @> {{}})",
                                postgresql_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_self_value_query_part_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_bind_sqlx_types_json_self_value_ts
                        },
                    )
                };
            let gen_overlaps_with_array_ts =
                |postgresql_type_pattern_handle: &PostgresqlTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgresql_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgresql_json_type_dimensions_helpers(postgresql_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgresql_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))",
                                postgresql_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                #value_match_self_value_query_part_initialization_ts
                                Ok(format!(
                                    #format_handle_ts,
                                    &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #ColumnSc,
                                    #maybe_additional_parameters_ts
                                    #ValueSc
                                ))
                            }
                        },
                        is_query_bind_mutable_true,
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #query_bind_sqlx_types_json_self_value_ts
                        },
                    )
                };
            let (
                    should_add_declaration_of_struct_ident_generic,
                    struct_additional_fields_ts,
                    impl_default_option_some_vec_one_el_additional_fields_ts,
                    query_part_content_ts,
                    is_query_bind_mutable,
                    query_bind_content_ts
                ) = match &filter {
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { .. } => gen_equal_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual { .. } => gen_equal_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual { .. } => gen_equal_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual { .. } => gen_equal_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual { .. } => gen_equal_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual { .. } => gen_all_elements_equal_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual => gen_length_equal_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual => gen_length_equal_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => gen_length_equal_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => gen_length_equal_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual => gen_length_equal_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan => gen_length_greater_than_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan => gen_length_greater_than_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan => gen_length_greater_than_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan => gen_length_greater_than_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthGreaterThan => gen_length_greater_than_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan { .. } => gen_greater_than_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { .. } => gen_between_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween { .. } => gen_between_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween { .. } => gen_between_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween { .. } => gen_between_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween { .. } => gen_between_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In { .. } => gen_in_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn { .. } => gen_in_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn { .. } => gen_in_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn { .. } => gen_in_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn { .. } => gen_in_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression => gen_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgresql_type_pattern_handle_array_dimension4),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgresql_type_pattern_handle_array_dimension4),
                // postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainedInArray => todo!(),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgresql_type_pattern_handle_standart),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgresql_type_pattern_handle_array_dimension1),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgresql_type_pattern_handle_array_dimension2),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgresql_type_pattern_handle_array_dimension3),
                postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgresql_type_pattern_handle_array_dimension4),
            };
            let struct_ts = gen_struct_ts(
                false,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &struct_additional_fields_ts,
            );
            let impl_default_option_some_vec_one_el_ts = gen_impl_default_option_some_vec_one_el_ts(
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &impl_default_option_some_vec_one_el_additional_fields_ts,
            );
            let impl_postgresql_type_where_filter_ts = gen_impl_postgresql_type_where_filter_ts(
                &FilterType::PostgresqlJsonType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_postgresql_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts = postgresql_crud_macros_common::PostgresqlJsonTypeFilter::into_array()
            .map(|el_6a4ac539| gen_filters_ts(&el_6a4ac539));
        let gend = quote! {#(#filter_array_ts)*};
        macros_helpers::maybe_write_ts_into_file(
            gen_where_filters_config
                .postgresql_json_types_content_write_into_gen_where_filters_postgresql_json_types,
            "gen_where_filters_postgresql_json_types",
            &gend,
            &macros_helpers::FormatWithCargofmt::True,
        );
        gend
    };
    let gend = quote! {
        #postgresql_type_ts
        #postgresql_json_type_ts
    };
    macros_helpers::maybe_write_ts_into_file(
        gen_where_filters_config.whole_content_write_into_gen_where_filters,
        "gen_where_filters",
        &gend,
        &macros_helpers::FormatWithCargofmt::True,
    );
    gend.into()
}
