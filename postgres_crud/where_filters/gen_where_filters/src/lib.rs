use naming::{
    ColumnSc, DimensionsIndexesSc, DimensionsSc, ErrorSc, IncrementSc, PubSc, QuerySc, SelfSc,
    ValueSc,
    parameter::{PostgresJsonTypeWhereSelfUcc, PostgresTypeWhereSelfUcc},
};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::fmt::Display;

#[proc_macro]
pub fn gen_where_filters(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Clone)]
    enum ShouldAddDeclarationOfStructIdentGeneric {
        False,
        True {
            maybe_additional_traits_ts: Option<Ts2>,
        },
    }
    enum FilterType {
        PostgresJsonType,
        PostgresType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone)]
    enum PostgresTypePatternHandle {
        Standart,
        ArrayDimension1,
        ArrayDimension2,
        ArrayDimension3,
        ArrayDimension4,
    }
    impl TryFrom<&PostgresTypePatternHandle> for DimensionNumber {
        type Error = ();
        fn try_from(value: &PostgresTypePatternHandle) -> Result<Self, Self::Error> {
            match &value {
                PostgresTypePatternHandle::Standart => Err(()),
                PostgresTypePatternHandle::ArrayDimension1 => Ok(Self::One),
                PostgresTypePatternHandle::ArrayDimension2 => Ok(Self::Two),
                PostgresTypePatternHandle::ArrayDimension3 => Ok(Self::Three),
                PostgresTypePatternHandle::ArrayDimension4 => Ok(Self::Four),
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
        fn dimension_ts(&self) -> Ts2 {
            self.dimension_std_primitive_u8()
                .to_string()
                .parse::<Ts2>()
                .expect("18c32bc0-2e55-4b4d-a6a8-b8680e5fe463")
        }
    }
    enum KindOfUnsignedPartOfStdPrimitiveI32 {
        CanBeZero,
        CanNotBeZero,
    }
    impl quote::ToTokens for KindOfUnsignedPartOfStdPrimitiveI32 {
        fn to_tokens(&self, tokens: &mut Ts2) {
            match &self {
                Self::CanBeZero => quote! {UnsignedPartOfStdPrimitiveI32}.to_tokens(tokens),
                Self::CanNotBeZero => {
                    quote! {NotZeroUnsignedPartOfStdPrimitiveI32}.to_tokens(tokens);
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PostgresTypeKind {
        Standart,
        ArrayDimension,
    }
    impl PostgresTypeKind {
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
        postgres_types_content_write_into_gen_where_filters_postgres_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        postgres_json_types_content_write_into_gen_where_filters_postgres_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_where_filters: macros_helpers::ShouldWriteTokenStreamIntoFile,
    }
    panic_location::panic_location();
    let gen_where_filters_config =
        serde_json::from_str::<GenWhereFiltersConfig>(&input_ts.to_string())
            .expect("1217b73b-456c-4a3e-8a5a-5d5b8b8c3452");
    let import_path = postgres_crud_macros_common::ImportPath::PostgresCrudCommon;
    let t_ts = quote! {T};
    let t_annotation_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let core_default_default_default_ts = token_patterns::CoreDefaultDefaultDefault;
    let postgres_crud_common_default_option_some_vec_one_el_ts =
        token_patterns::PostgresCrudCommonDefaultOptionSomeVecOneEl;
    let postgres_crud_common_default_option_some_vec_one_el_call_ts =
        token_patterns::PostgresCrudCommonDefaultOptionSomeVecOneElCall;
    let pub_value_t_ts = quote! {pub #ValueSc: T};
    let unsigned_part_of_std_primitive_i32_ts =
        quote! {postgres_crud_common::UnsignedPartOfStdPrimitiveI32};
    let not_zero_unsigned_part_of_std_primitive_i32_ts =
        quote! {postgres_crud_common::NotZeroUnsignedPartOfStdPrimitiveI32};
    let value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts =
        quote! {#ValueSc: #not_zero_unsigned_part_of_std_primitive_i32_ts};
    let pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts =
        quote! {pub #value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts};
    let value_default_option_some_vec_one_el_ts = quote! {
        #ValueSc: #postgres_crud_common_default_option_some_vec_one_el_call_ts
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
        postgres_crud_macros_common::gen_impl_default_option_some_vec_one_el_ts(
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => Ts2::new(),
                ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                    maybe_additional_traits_ts.as_ref().map_or_else(
                        || quote! {<T: #postgres_crud_common_default_option_some_vec_one_el_ts>},
                        |value_29913af7| quote! {<T: #value_29913af7 + #postgres_crud_common_default_option_some_vec_one_el_ts>}
                    )
                }
            },
            &postgres_crud_macros_common::ImportPath::PostgresCrudCommon,
            &ident,
            match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            &quote! {
                Self {
                    logical_operator: #postgres_crud_common_default_option_some_vec_one_el_call_ts,
                    #impl_default_option_some_vec_one_el_additional_fields_ts
                }
            },
        )
    };
    let gen_impl_postgres_type_where_filter_ts = |filter_type: &FilterType,
                                                                   should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
                                                                   ident: &dyn quote::ToTokens,
                                                                   increment_parameter_underscore: &postgres_crud_macros_common::IncrementParameterUnderscore,
                                                                   is_need_to_add_logical_operator_underscore: &postgres_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore,
                                                                   query_part_content_ts: &dyn quote::ToTokens,
                                                                   is_query_bind_mutable: &postgres_crud_macros_common::IsQueryBindMutable,
                                                                   query_bind_content_ts: &dyn quote::ToTokens| {
        postgres_crud_macros_common::impl_postgres_type_where_filter_for_ident_ts(
            &{
                let maybe_t_additional_traits_for_postgres_type_where_filter_ts: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                    ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                        let send_and_lifetime_ts = quote! {Send + 'lifetime};
                        let serde_serialize_ts = quote! {serde::Serialize};
                        let content_ts = match (&filter_type, &maybe_additional_traits_ts) {
                            (FilterType::PostgresType, Some(value)) => &quote! {#value + #send_and_lifetime_ts},
                            (FilterType::PostgresType, None) => &send_and_lifetime_ts,
                            (FilterType::PostgresJsonType, Some(value)) => &quote! {#value + #serde_serialize_ts + #send_and_lifetime_ts},
                            (FilterType::PostgresJsonType, None) => &quote! {#serde_serialize_ts + #send_and_lifetime_ts},
                        };
                        &quote! {, T: #content_ts}
                    }
                };
                quote! {<'lifetime #maybe_t_additional_traits_for_postgres_type_where_filter_ts>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            increment_parameter_underscore,
            &postgres_crud_macros_common::ColumnParameterUnderscore::False,
            is_need_to_add_logical_operator_underscore,
            &query_part_content_ts,
            is_query_bind_mutable,
            &query_bind_content_ts,
            &postgres_crud_macros_common::ImportPath::PostgresCrudCommon,
        )
    };
    let regular_expression_case_and_value_declaration_ts = quote! {
        pub regular_expression_case: RegularExpressionCase,
        pub value: RegexRegex
    };
    let regular_expression_case_and_value_default_initialization_ts = quote! {
        regular_expression_case: #postgres_crud_common_default_option_some_vec_one_el_call_ts,
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
            let #ident_ts = match postgres_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
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
    let postgres_type_pattern_handle_standart = PostgresTypePatternHandle::Standart;
    let postgres_type_pattern_handle_array_dimension1 = PostgresTypePatternHandle::ArrayDimension1;
    let postgres_type_pattern_handle_array_dimension2 = PostgresTypePatternHandle::ArrayDimension2;
    let postgres_type_pattern_handle_array_dimension3 = PostgresTypePatternHandle::ArrayDimension3;
    let postgres_type_pattern_handle_array_dimension4 = PostgresTypePatternHandle::ArrayDimension4;
    let gen_pub_dimensions_bounded_vec_ts =
        |vec_length_ts: &dyn quote::ToTokens,
         kind_of_unsigned_part_of_std_primitive_i32: &KindOfUnsignedPartOfStdPrimitiveI32| {
            quote! {pub #DimensionsSc: BoundedStdVecVec<postgres_crud_common::#kind_of_unsigned_part_of_std_primitive_i32, #vec_length_ts>}
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
        #DimensionsSc: #postgres_crud_common_default_option_some_vec_one_el_call_ts
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
    let is_query_bind_mutable_true = postgres_crud_macros_common::IsQueryBindMutable::True;
    let is_query_bind_mutable_false = postgres_crud_macros_common::IsQueryBindMutable::False;
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
    let gen_postgres_type_dimensions_helpers = |postgres_type_pattern_handle: &PostgresTypePatternHandle, postgres_type_or_postgres_json_type: &postgres_crud_macros_common::PostgresTypeOrPostgresJsonType| {
        DimensionNumber::try_from(postgres_type_pattern_handle).map_or_else(
            |()| (Ts2::new(),Ts2::new(), Ts2::new(), PostgresTypeKind::Standart, Ts2::new(), Ts2::new()),
            |dimension_number| (
                match &postgres_type_or_postgres_json_type {
                    postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType => gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                    postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType => gen_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                },
                dimensions_default_initialization_comma_ts.clone(),
                gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(
                    &DimensionsIndexesSc,
                    &DimensionsSc,
                    &match &postgres_type_or_postgres_json_type {
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType => quote! {postgres_type_query_part},
                        postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType => quote! {postgres_json_type_query_part},
                    },
                ),
                PostgresTypeKind::ArrayDimension,
                dimensions_indexes_comma_ts.clone(),
                query_self_dimensions_query_bind_query_ts.clone(),
            )
        )
    };
    let postgres_type_ts = {
        let gen_filters_ts = |filter: &postgres_crud_macros_common::PostgresTypeFilter| {
            let ident = PostgresTypeWhereSelfUcc::from_display(&filter);
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
                let pub_value_postgres_type_not_empty_unique_vec_t_ts =
                    quote! {pub #ValueSc: PostgresTypeNotEmptyUniqueVec<T>};
                let gen_postgres_type_dimensions_helpers_postgres_type =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_postgres_type_dimensions_helpers(
                    postgres_type_pattern_handle,
                    &postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresType
                )
                    };
                let gen_32abfefc_c087_480b_b502_cb78533dafb0_ts =
                        |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                         gen_format_handle_str: &dyn Fn(
                            &PostgresTypeKind,
                        )
                            -> String| {
                            let (
                                maybe_dimensions_declaration_ts,
                                maybe_dimensions_default_initialization_ts,
                                maybe_dimensions_indexes_initialization_ts,
                                postgres_type_kind,
                                maybe_additional_parameters_ts,
                                maybe_dimensions_query_bind_content_ts,
                            ) = gen_postgres_type_dimensions_helpers_postgres_type(
                                postgres_type_pattern_handle,
                            );
                            (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                        gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                        postgres_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&gen_format_handle_str(&postgres_type_kind));
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
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                     operator: &dyn Display| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgres_type_pattern_handle,
                            &|postgres_type_kind: &PostgresTypeKind| {
                                format!(
                                    "{{}}({{}}{} {operator} ${{}})",
                                    postgres_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &">",
                        )
                    };
                let gen_between_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_type_dimensions_helpers_postgres_type(
                        postgres_type_pattern_handle,
                    );
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_between_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                        postgres_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", postgres_type_kind.format_argument()));
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
                let gen_in_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_type_dimensions_helpers_postgres_type(
                        postgres_type_pattern_handle,
                    );
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::True {
                            maybe_additional_traits_ts: Some(
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_postgresq_encode_ts},
                            ),
                        },
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgres_type_not_empty_unique_vec_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        postgres_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                postgres_type_kind.format_argument()
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
                                        match postgres_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
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
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgres_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgres_type_dimensions_helpers_postgres_type(
                            postgres_type_pattern_handle,
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
                            postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}({{}}{} {{}} ${{}})",
                                    postgres_type_kind.format_argument()
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
                let gen_before_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_type_dimensions_helpers_postgres_type(
                        postgres_type_pattern_handle,
                    );
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        gen_maybe_dimensions_declaration_pub_value_t_ts(
                            &maybe_dimensions_declaration_ts,
                        ),
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        postgres_crud_macros_common::IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} < ${{}})",
                                postgres_type_kind.format_argument()
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
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                     postgres_syntax: &dyn Display| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgres_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgres_type_dimensions_helpers_postgres_type(
                            postgres_type_pattern_handle,
                        );
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            match &postgres_type_pattern_handle {
                                PostgresTypePatternHandle::Standart => {
                                    postgres_crud_macros_common::IncrementParameterUnderscore::True
                                }
                                PostgresTypePatternHandle::ArrayDimension1
                                | PostgresTypePatternHandle::ArrayDimension2
                                | PostgresTypePatternHandle::ArrayDimension3
                                | PostgresTypePatternHandle::ArrayDimension4 => {
                                    postgres_crud_macros_common::IncrementParameterUnderscore::False
                                }
                            },
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}({{}}{} {postgres_syntax})",
                                    postgres_type_kind.format_argument()
                                ));
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
                            match &postgres_type_pattern_handle {
                                PostgresTypePatternHandle::Standart => is_query_bind_mutable_false,
                                PostgresTypePatternHandle::ArrayDimension1
                                | PostgresTypePatternHandle::ArrayDimension2
                                | PostgresTypePatternHandle::ArrayDimension3
                                | PostgresTypePatternHandle::ArrayDimension4 => {
                                    is_query_bind_mutable_true
                                }
                            },
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                Ok(#QuerySc)
                            },
                        )
                    };
                let gen_current_date_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"= current_date",
                        )
                    };
                let gen_greater_than_current_date_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"> current_date",
                        )
                    };
                let gen_current_timestamp_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"= current_timestamp",
                        )
                    };
                let gen_greater_than_current_timestamp_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"> current_timestamp",
                        )
                    };
                let gen_current_time_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"= current_time",
                        )
                    };
                let gen_greater_than_current_time_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            postgres_type_pattern_handle,
                            &"> current_time",
                        )
                    };
                let gen_equal_to_encoded_string_representation_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            postgres_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_postgres_type_dimensions_helpers_postgres_type(
                            postgres_type_pattern_handle,
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
                                encode_format: #postgres_crud_common_default_option_some_vec_one_el_call_ts,
                                encoded_string_representation: #core_default_default_default_ts
                            },
                            postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                    postgres_type_kind.format_argument()
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
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"<@",
                        )
                    };
                let gen_find_ranges_that_fully_contain_the_given_range_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"@>",
                        )
                    };
                let gen_strictly_to_left_of_range_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"&<",
                        )
                    };
                let gen_strictly_to_right_of_range_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"&>",
                        )
                    };
                let gen_overlap_with_range_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"&&",
                        )
                    };
                let gen_adjacent_with_range_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(
                            postgres_type_pattern_handle,
                            &"-|-",
                        )
                    };
                let gen_length_filter_pattern_ts = |operator: &dyn Display| {
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_std_primitive_i32_declaration_ts
                            .clone(),
                        value_default_option_some_vec_one_el_ts.clone(),
                        postgres_crud_macros_common::IncrementParameterUnderscore::False,
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
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgres_type_pattern_handle,
                            &|postgres_type_kind: &PostgresTypeKind| {
                                format!(
                                    "{{}}(lower({{}}{}) = ${{}})",
                                    postgres_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_excluded_upper_bound_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgres_type_pattern_handle,
                            &|postgres_type_kind: &PostgresTypeKind| {
                                format!(
                                    "{{}}(upper({{}}{}) = ${{}})",
                                    postgres_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_included_lower_bound_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgres_type_pattern_handle,
                            &|postgres_type_kind: &PostgresTypeKind| {
                                format!(
                                    "{{}}(lower({{}}{}) > ${{}})",
                                    postgres_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_excluded_upper_bound_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            postgres_type_pattern_handle,
                            &|postgres_type_kind: &PostgresTypeKind| {
                                format!(
                                    "{{}}(upper({{}}{}) > ${{}})",
                                    postgres_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_range_length_ts =
                    |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                        let (
                        maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts
                    ) = DimensionNumber::try_from(postgres_type_pattern_handle).map_or_else(
                        |()| (Ts2::new(), Ts2::new(), Ts2::new(), PostgresTypeKind::Standart, quote! {#ColumnSc,}, Ts2::new()),
                        |dimension_number| (
                            gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                            dimensions_default_initialization_comma_ts.clone(),
                            {
                                let dimensions_indexes1_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes1}, &DimensionsSc, &quote! {postgres_type_query_part});
                                let dimensions_indexes2_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes2}, &DimensionsSc, &quote! {postgres_type_query_part});
                                quote! {
                                    #dimensions_indexes1_ts
                                    #dimensions_indexes2_ts
                                }
                            },
                            PostgresTypeKind::ArrayDimension,
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
                            postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                    postgres_type_kind.format_argument(),
                                    postgres_type_kind.format_argument(),
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
                    postgres_crud_macros_common::PostgresTypeFilter::Equal { .. } => {
                        let (maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts, maybe_dimensions_indexes_initialization_ts, _, _, maybe_dimensions_query_bind_content_ts) = gen_postgres_type_dimensions_helpers_postgres_type(&postgres_type_pattern_handle_standart);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(quote! {#sqlx_type_postgresq_encode_ts + postgres_crud_common::PostgresTypeEqualOperator}),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                            gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                            postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as postgres_crud_common::PostgresTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgres_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    },
                                    postgres_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let postgres_crud_common::EqualOperator::Equal = &<T as postgres_crud_common::PostgresTypeEqualOperator>::operator(&#SelfSc.#ValueSc) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneEqual { .. } => {
                        let (maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts, maybe_dimensions_indexes_initialization_ts, _, _, maybe_dimensions_query_bind_content_ts) = gen_postgres_type_dimensions_helpers_postgres_type(&postgres_type_pattern_handle_array_dimension1);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(quote! {#sqlx_type_postgresq_encode_ts + postgres_crud_common::PostgresTypeEqualOperator}),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(&maybe_dimensions_declaration_ts),
                            gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                            postgres_crud_macros_common::IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as postgres_crud_common::PostgresTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    postgres_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    }
                                    postgres_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({}{dimensions_indexes} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let postgres_crud_common::EqualOperator::Equal = &<T as postgres_crud_common::PostgresTypeEqualOperator>::operator(
                                    &#SelfSc.#ValueSc
                                ) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::Between { .. } => gen_between_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneBetween { .. } => gen_between_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::In { .. } => gen_in_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneIn { .. } => gen_in_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::RegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneRegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::Before { .. } => gen_before_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneBefore { .. } => gen_before_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::CurrentDate => gen_current_date_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentDate => gen_current_date_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentDate => gen_greater_than_current_date_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentDate => gen_greater_than_current_date_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::CurrentTimestamp => gen_current_timestamp_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentTimestamp => gen_current_timestamp_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentTimestamp => gen_greater_than_current_timestamp_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentTimestamp => gen_greater_than_current_timestamp_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::CurrentTime => gen_current_time_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentTime => gen_current_time_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentTime => gen_greater_than_current_time_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentTime => gen_greater_than_current_time_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneLengthEqual => gen_length_filter_pattern_ts(&"="),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneLengthGreaterThan => gen_length_filter_pattern_ts(&">"),
                    postgres_crud_macros_common::PostgresTypeFilter::EqualToEncodedStringRepresentation => gen_equal_to_encoded_string_representation_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneEqualToEncodedStringRepresentation => gen_equal_to_encoded_string_representation_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::FindRangesWithinGivenRange { .. } => gen_find_ranges_within_given_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneFindRangesWithinGivenRange { .. } => gen_find_ranges_within_given_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => gen_find_ranges_that_fully_contain_the_given_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => gen_find_ranges_that_fully_contain_the_given_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::StrictlyToLeftOfRange { .. } => gen_strictly_to_left_of_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneStrictlyToLeftOfRange { .. } => gen_strictly_to_left_of_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::StrictlyToRightOfRange { .. } => gen_strictly_to_right_of_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneStrictlyToRightOfRange { .. } => gen_strictly_to_right_of_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::IncludedLowerBound { .. } => gen_included_lower_bound_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneIncludedLowerBound { .. } => gen_included_lower_bound_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::ExcludedUpperBound { .. } => gen_excluded_upper_bound_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneExcludedUpperBound { .. } => gen_excluded_upper_bound_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThanIncludedLowerBound { .. } => gen_greater_than_included_lower_bound_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanIncludedLowerBound { .. } => gen_greater_than_included_lower_bound_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::GreaterThanExcludedUpperBound { .. } => gen_greater_than_excluded_upper_bound_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanExcludedUpperBound { .. } => gen_greater_than_excluded_upper_bound_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::OverlapWithRange { .. } => gen_overlap_with_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneOverlapWithRange { .. } => gen_overlap_with_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::AdjacentWithRange { .. } => gen_adjacent_with_range_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneAdjacentWithRange { .. } => gen_adjacent_with_range_ts(&postgres_type_pattern_handle_array_dimension1),
                    postgres_crud_macros_common::PostgresTypeFilter::RangeLength => gen_range_length_ts(&postgres_type_pattern_handle_standart),
                    postgres_crud_macros_common::PostgresTypeFilter::DimensionOneRangeLength => gen_range_length_ts(&postgres_type_pattern_handle_array_dimension1),
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
            let impl_postgres_type_where_filter_ts = gen_impl_postgres_type_where_filter_ts(
                &FilterType::PostgresType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &increment_parameter_underscore,
                &postgres_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_postgres_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts = postgres_crud_macros_common::PostgresTypeFilter::into_array()
            .map(|el_7cfb1929| gen_filters_ts(&el_7cfb1929));
        let gend = quote! {#(#filter_array_ts)*};
        macros_helpers::maybe_write_ts_into_file(
            gen_where_filters_config
                .postgres_types_content_write_into_gen_where_filters_postgres_types,
            "gen_where_filters_postgres_types",
            &gend,
            &macros_helpers::FormatWithCargofmt::True,
        );
        gend
    };
    let postgres_json_type_ts = {
        let gen_filters_ts = |filter: &postgres_crud_macros_common::PostgresJsonTypeFilter| {
            let ident = PostgresJsonTypeWhereSelfUcc::from_display(&filter);
            let pub_value_postgres_json_type_not_empty_unique_vec_t_ts = quote! {
                pub #ValueSc: PostgresJsonTypeNotEmptyUniqueVec<T>
            };
            let query_bind_sqlx_types_json_self_value_ts = quote! {
                if let Err(#ErrorSc) = #QuerySc.try_bind(sqlx::types::Json(#SelfSc.#ValueSc)) {
                    return Err(#ErrorSc.to_string());
                }
                Ok(#QuerySc)
            };
            let gen_postgres_json_type_dimensions_helpers =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    gen_postgres_type_dimensions_helpers(postgres_type_pattern_handle, &postgres_crud_macros_common::PostgresTypeOrPostgresJsonType::PostgresJsonType)
                };
            let gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                 gen_format_handle_str: &dyn Fn(&PostgresTypeKind) -> String| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
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
                                &gen_format_handle_str(&postgres_type_kind),
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                 operator: &dyn Display| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgres_type_pattern_handle,
                        &|postgres_type_kind: &PostgresTypeKind| {
                            format!(
                                "{{}}({{}}{} {operator} ${{}})",
                                postgres_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_equal_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(postgres_type_pattern_handle, &"=")
            };
            let gen_all_elements_equal_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgres_type_pattern_handle,
                        &|postgres_type_kind: &PostgresTypeKind| {
                            format!(
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))",
                                postgres_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle,
                 operation: &dyn Display| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
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
                                postgres_type_kind.format_argument()
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
            let gen_length_equal_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(postgres_type_pattern_handle, &"=")
            };
            let gen_length_greater_than_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(postgres_type_pattern_handle, &">")
                };
            let gen_greater_than_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(postgres_type_pattern_handle, &">")
            };
            let gen_contains_el_greater_than_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgres_type_pattern_handle,
                        &|postgres_type_kind: &PostgresTypeKind| {
                            format!(
                                "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))",
                                postgres_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_all_elements_greater_than_ts =
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        postgres_type_pattern_handle,
                        &|postgres_type_kind: &PostgresTypeKind| {
                            format!(
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))",
                                postgres_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_between_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgres_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_between_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                    {
                        let content_ts: &dyn quote::ToTokens = match postgres_type_pattern_handle {
                            PostgresTypePatternHandle::Standart => &quote!{
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
                            PostgresTypePatternHandle::ArrayDimension1 |
                            PostgresTypePatternHandle::ArrayDimension2 |
                            PostgresTypePatternHandle::ArrayDimension3 |
                            PostgresTypePatternHandle::ArrayDimension4 => &value_match_increment_checked_add_one_initialization_ts
                        };
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", postgres_type_kind.format_argument()));
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
                        let content_ts: &dyn quote::ToTokens = match postgres_type_pattern_handle {
                            PostgresTypePatternHandle::Standart => &quote!{
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
                            PostgresTypePatternHandle::ArrayDimension1 |
                            PostgresTypePatternHandle::ArrayDimension2 |
                            PostgresTypePatternHandle::ArrayDimension3 |
                            PostgresTypePatternHandle::ArrayDimension4 => &query_bind_sqlx_types_json_self_value_ts
                        };
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #content_ts
                        }
                    },
                )
            };
            let gen_in_ts = |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgres_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_postgres_json_type_not_empty_unique_vec_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(
                        &maybe_dimensions_default_initialization_ts,
                    ),
                    {
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                            "{{}}({{}}{} in ({{}}))",
                            postgres_type_kind.format_argument()
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    postgres_type_kind, maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts
                ) = DimensionNumber::try_from(postgres_type_pattern_handle).map_or_else(
                    |()| (Ts2::new(), Ts2::new(), Ts2::new(), PostgresTypeKind::Standart, Ts2::new(), Ts2::new()),
                    |dimension_number| (
                        gen_pub_dimensions_bounded_vec_unsigned_part_of_std_primitive_i32_comma_ts(&dimension_number),
                        dimensions_default_initialization_comma_ts.clone(),
                        {
                            let dimensions_indexes_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&DimensionsIndexesSc, &DimensionsSc, &quote! {postgres_json_type_query_part_minus_one});
                            let last_dimensions_index_intialization_ts = gen_match_increment_checked_add_one_initialization_ts(&quote! {last_dimensions_index});
                            quote! {
                                #dimensions_indexes_initialization_ts
                                #last_dimensions_index_intialization_ts
                            }
                        },
                        PostgresTypeKind::ArrayDimension,
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
                                match &postgres_type_kind {
                                    PostgresTypeKind::Standart => "",
                                    PostgresTypeKind::ArrayDimension => "{}->>${}",
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
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
                                postgres_type_kind.format_argument()
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
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
                                postgres_type_kind.format_argument()
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgres_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} @> {{}})",
                                postgres_type_kind.format_argument()
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
                |postgres_type_pattern_handle: &PostgresTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        postgres_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_postgres_json_type_dimensions_helpers(postgres_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_postgres_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))",
                                postgres_type_kind.format_argument()
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
                postgres_crud_macros_common::PostgresJsonTypeFilter::Equal { .. } => gen_equal_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneEqual { .. } => gen_equal_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoEqual { .. } => gen_equal_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeEqual { .. } => gen_equal_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourEqual { .. } => gen_equal_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::AllElementsEqual { .. } => gen_all_elements_equal_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourAllElementsEqual { .. } => gen_all_elements_equal_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::LengthEqual => gen_length_equal_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneLengthEqual => gen_length_equal_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoLengthEqual => gen_length_equal_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeLengthEqual => gen_length_equal_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourLengthEqual => gen_length_equal_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::LengthGreaterThan => gen_length_greater_than_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneLengthGreaterThan => gen_length_greater_than_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoLengthGreaterThan => gen_length_greater_than_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeLengthGreaterThan => gen_length_greater_than_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourLengthGreaterThan => gen_length_greater_than_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::GreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneGreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoGreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeGreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourGreaterThan { .. } => gen_greater_than_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::ContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourContainsElGreaterThan { .. } => gen_contains_el_greater_than_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::AllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourAllElementsGreaterThan { .. } => gen_all_elements_greater_than_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::Between { .. } => gen_between_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneBetween { .. } => gen_between_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoBetween { .. } => gen_between_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeBetween { .. } => gen_between_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourBetween { .. } => gen_between_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::In { .. } => gen_in_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneIn { .. } => gen_in_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoIn { .. } => gen_in_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeIn { .. } => gen_in_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourIn { .. } => gen_in_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::RegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneRegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoRegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeRegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourRegularExpression => gen_regular_expression_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::ContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourContainsElRegularExpression => gen_contains_el_regular_expression_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::AllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourAllElementsRegularExpression => gen_all_elements_regular_expression_ts(&postgres_type_pattern_handle_array_dimension4),
                postgres_crud_macros_common::PostgresJsonTypeFilter::ContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourContainsAllElementsOfArray { .. } => gen_contains_all_elements_of_array_ts(&postgres_type_pattern_handle_array_dimension4),
                // postgres_crud_macros_common::PostgresJsonTypeFilter::ContainedInArray => todo!(),
                postgres_crud_macros_common::PostgresJsonTypeFilter::OverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgres_type_pattern_handle_standart),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionOneOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgres_type_pattern_handle_array_dimension1),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionTwoOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgres_type_pattern_handle_array_dimension2),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionThreeOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgres_type_pattern_handle_array_dimension3),
                postgres_crud_macros_common::PostgresJsonTypeFilter::DimensionFourOverlapsWithArray { .. } => gen_overlaps_with_array_ts(&postgres_type_pattern_handle_array_dimension4),
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
            let impl_postgres_type_where_filter_ts = gen_impl_postgres_type_where_filter_ts(
                &FilterType::PostgresJsonType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &postgres_crud_macros_common::IncrementParameterUnderscore::False,
                &postgres_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_postgres_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts = postgres_crud_macros_common::PostgresJsonTypeFilter::into_array()
            .map(|el_6a4ac539| gen_filters_ts(&el_6a4ac539));
        let gend = quote! {#(#filter_array_ts)*};
        macros_helpers::maybe_write_ts_into_file(
            gen_where_filters_config
                .postgres_json_types_content_write_into_gen_where_filters_postgres_json_types,
            "gen_where_filters_postgres_json_types",
            &gend,
            &macros_helpers::FormatWithCargofmt::True,
        );
        gend
    };
    let gend = quote! {
        #postgres_type_ts
        #postgres_json_type_ts
    };
    macros_helpers::maybe_write_ts_into_file(
        gen_where_filters_config.whole_content_write_into_gen_where_filters,
        "gen_where_filters",
        &gend,
        &macros_helpers::FormatWithCargofmt::True,
    );
    gend.into()
}
