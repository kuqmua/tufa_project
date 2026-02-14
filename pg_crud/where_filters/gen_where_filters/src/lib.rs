use macros_helpers::{
    FormatWithCargofmt, ShouldWriteTokenStreamIntoFile, gen_if_write_is_err_ts,
    maybe_write_ts_into_file,
};
use naming::{
    ColumnSc, DimensionsIndexesSc, DimensionsSc, ErrorSc, IncrementSc, PubSc, QuerySc, SelfSc,
    ValueSc,
    parameter::{PgJsonTypeWhereSelfUcc, PgTypeWhereSelfUcc},
};
use pg_crud_macros_common::{
    ColumnParameterUnderscore, ImportPath, IncrementParameterUnderscore,
    IsNeedToAddLogicalOperatorUnderscore, IsQueryBindMutable, PgJsonTypeFilter, PgTypeFilter,
    PgTypeOrPgJsonType, gen_impl_default_option_some_vec_one_el_ts,
    impl_pg_type_where_filter_for_ident_ts,
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
        PgJsonType,
        PgType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone)]
    enum PgTypePatternHandle {
        Standart,
        ArrayDimension1,
        ArrayDimension2,
        ArrayDimension3,
        ArrayDimension4,
    }
    impl TryFrom<&PgTypePatternHandle> for DimensionNumber {
        type Error = ();
        fn try_from(value: &PgTypePatternHandle) -> Result<Self, Self::Error> {
            match &value {
                PgTypePatternHandle::Standart => Err(()),
                PgTypePatternHandle::ArrayDimension1 => Ok(Self::One),
                PgTypePatternHandle::ArrayDimension2 => Ok(Self::Two),
                PgTypePatternHandle::ArrayDimension3 => Ok(Self::Three),
                PgTypePatternHandle::ArrayDimension4 => Ok(Self::Four),
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
        fn dimension_ts(&self) -> Ts2 {
            self.dimension_u8()
                .to_string()
                .parse::<Ts2>()
                .expect("18c32bc0")
        }
        const fn dimension_u8(&self) -> u8 {
            match &self {
                Self::One => 1,
                Self::Two => 2,
                Self::Three => 3,
                Self::Four => 4,
            }
        }
    }
    enum KindOfUnsignedPartOfI32 {
        CanBeZero,
        CanNotBeZero,
    }
    impl quote::ToTokens for KindOfUnsignedPartOfI32 {
        fn to_tokens(&self, tokens: &mut Ts2) {
            match &self {
                Self::CanBeZero => quote! {UnsignedPartOfI32}.to_tokens(tokens),
                Self::CanNotBeZero => {
                    quote! {NotZeroUnsignedPartOfI32}.to_tokens(tokens);
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PgTypeKind {
        Standart,
        ArrayDimension,
    }
    impl PgTypeKind {
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
        pg_types_content_write_into_gen_where_filters_pg_types: ShouldWriteTokenStreamIntoFile,
        pg_json_types_content_write_into_gen_where_filters_pg_json_types:
            ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_where_filters: ShouldWriteTokenStreamIntoFile,
    }
    panic_location::panic_location();
    let gen_where_filters_config =
        serde_json::from_str::<GenWhereFiltersConfig>(&input_ts.to_string()).expect("1217b73b");
    let import_path = ImportPath::PgCrudCommon;
    let t_ts = quote! {T};
    let t_annotation_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let core_default_default_default_ts = token_patterns::CoreDefaultDefaultDefault;
    let pg_crud_common_default_option_some_vec_one_el_ts =
        token_patterns::PgCrudCommonDefaultOptionSomeVecOneEl;
    let pg_crud_common_default_option_some_vec_one_el_call_ts =
        token_patterns::PgCrudCommonDefaultOptionSomeVecOneElCall;
    let pub_value_t_ts = quote! {pub #ValueSc: T};
    let unsigned_part_of_i32_ts = quote! {pg_crud_common::UnsignedPartOfI32};
    let not_zero_unsigned_part_of_i32_ts = quote! {pg_crud_common::NotZeroUnsignedPartOfI32};
    let value_not_zero_unsigned_part_of_i32_declaration_ts =
        quote! {#ValueSc: #not_zero_unsigned_part_of_i32_ts};
    let pub_value_not_zero_unsigned_part_of_i32_declaration_ts =
        quote! {pub #value_not_zero_unsigned_part_of_i32_declaration_ts};
    let value_default_option_some_vec_one_el_ts = quote! {
        #ValueSc: #pg_crud_common_default_option_some_vec_one_el_call_ts
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
        gen_impl_default_option_some_vec_one_el_ts(
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => Ts2::new(),
                ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                    maybe_additional_traits_ts.as_ref().map_or_else(
                        || quote! {<T: #pg_crud_common_default_option_some_vec_one_el_ts>},
                        |value_29913af7| quote! {<T: #value_29913af7 + #pg_crud_common_default_option_some_vec_one_el_ts>}
                    )
                }
            },
            &ImportPath::PgCrudCommon,
            &ident,
            match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            &quote! {
                Self {
                    logical_operator: #pg_crud_common_default_option_some_vec_one_el_call_ts,
                    #impl_default_option_some_vec_one_el_additional_fields_ts
                }
            },
        )
    };
    let gen_impl_pg_type_where_filter_ts = |
        filter_type: &FilterType,
        should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric,
        ident: &dyn quote::ToTokens,
        increment_parameter_underscore: &IncrementParameterUnderscore,
        is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
        query_part_content_ts: &dyn quote::ToTokens,
        is_query_bind_mutable: &IsQueryBindMutable,
        query_bind_content_ts: &dyn quote::ToTokens
    | {
        impl_pg_type_where_filter_for_ident_ts(
            &{
                let maybe_t_additional_traits_for_pg_type_where_filter_ts: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                    ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                        let send_and_lifetime_ts = quote! {Send + 'lifetime};
                        let serde_serialize_ts = quote! {serde::Serialize};
                        let content_ts = match (&filter_type, &maybe_additional_traits_ts) {
                            (FilterType::PgType, Some(value)) => &quote! {#value + #send_and_lifetime_ts},
                            (FilterType::PgType, None) => &send_and_lifetime_ts,
                            (FilterType::PgJsonType, Some(value)) => &quote! {#value + #serde_serialize_ts + #send_and_lifetime_ts},
                            (FilterType::PgJsonType, None) => &quote! {#serde_serialize_ts + #send_and_lifetime_ts},
                        };
                        &quote! {, T: #content_ts}
                    }
                };
                quote! {<'lifetime #maybe_t_additional_traits_for_pg_type_where_filter_ts>}
            },
            &ident,
            &match &should_add_declaration_of_struct_ident_generic {
                ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
                ShouldAddDeclarationOfStructIdentGeneric::True { .. } => &t_annotation_generic_ts,
            },
            increment_parameter_underscore,
            &ColumnParameterUnderscore::False,
            is_need_to_add_logical_operator_underscore,
            &query_part_content_ts,
            is_query_bind_mutable,
            &query_bind_content_ts,
            &ImportPath::PgCrudCommon,
        )
    };
    let regular_expression_case_and_value_declaration_ts = quote! {
        pub regular_expression_case: RegularExpressionCase,
        pub value: RegexRegex
    };
    let regular_expression_case_and_value_default_initialization_ts = quote! {
        regular_expression_case: #pg_crud_common_default_option_some_vec_one_el_call_ts,
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
            let #ident_ts = match pg_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
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
    let pg_type_pattern_handle_standart = PgTypePatternHandle::Standart;
    let pg_type_pattern_handle_array_dimension1 = PgTypePatternHandle::ArrayDimension1;
    let pg_type_pattern_handle_array_dimension2 = PgTypePatternHandle::ArrayDimension2;
    let pg_type_pattern_handle_array_dimension3 = PgTypePatternHandle::ArrayDimension3;
    let pg_type_pattern_handle_array_dimension4 = PgTypePatternHandle::ArrayDimension4;
    let gen_pub_dimensions_bounded_vec_ts =
        |vec_length_ts: &dyn quote::ToTokens,
         kind_of_unsigned_part_of_i32: &KindOfUnsignedPartOfI32| {
            quote! {pub #DimensionsSc: BoundedStdVecVec<pg_crud_common::#kind_of_unsigned_part_of_i32, #vec_length_ts>}
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
        #DimensionsSc: #pg_crud_common_default_option_some_vec_one_el_call_ts
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
    let is_query_bind_mutable_true = IsQueryBindMutable::True;
    let is_query_bind_mutable_false = IsQueryBindMutable::False;
    let gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts =
        |dimension_number: &DimensionNumber| {
            let pub_dimensions_bounded_vec_not_zero_unsigned_part_of_i32_ts =
                gen_pub_dimensions_bounded_vec_ts(
                    &dimension_number.dimension_ts(),
                    &KindOfUnsignedPartOfI32::CanNotBeZero,
                );
            quote! {#pub_dimensions_bounded_vec_not_zero_unsigned_part_of_i32_ts,}
        };
    let gen_pub_dimensions_bounded_vec_unsigned_part_of_i32_comma_ts =
        |dimension_number: &DimensionNumber| {
            let pub_dimensions_bounded_vec_unsigned_part_of_i32_ts =
                gen_pub_dimensions_bounded_vec_ts(
                    &dimension_number.dimension_ts(),
                    &KindOfUnsignedPartOfI32::CanBeZero,
                );
            quote! {#pub_dimensions_bounded_vec_unsigned_part_of_i32_ts,}
        };
    let gen_pg_type_dimensions_helpers =
        |pg_type_pattern_handle: &PgTypePatternHandle,
         pg_type_or_pg_json_type: &PgTypeOrPgJsonType| {
            DimensionNumber::try_from(pg_type_pattern_handle).map_or_else(
            |()| (Ts2::new(),Ts2::new(), Ts2::new(), PgTypeKind::Standart, Ts2::new(), Ts2::new()),
            |dimension_number| (
                match &pg_type_or_pg_json_type {
                    PgTypeOrPgJsonType::PgType => gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(&dimension_number),
                    PgTypeOrPgJsonType::PgJsonType => gen_pub_dimensions_bounded_vec_unsigned_part_of_i32_comma_ts(&dimension_number),
                },
                dimensions_default_initialization_comma_ts.clone(),
                gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(
                    &DimensionsIndexesSc,
                    &DimensionsSc,
                    &match &pg_type_or_pg_json_type {
                        PgTypeOrPgJsonType::PgType => quote! {pg_type_query_part},
                        PgTypeOrPgJsonType::PgJsonType => quote! {pg_json_type_query_part},
                    },
                ),
                PgTypeKind::ArrayDimension,
                dimensions_indexes_comma_ts.clone(),
                query_self_dimensions_query_bind_query_ts.clone(),
            )
        )
        };
    let pg_type_ts = {
        let gen_filters_ts = |filter: &PgTypeFilter| {
            let ident = PgTypeWhereSelfUcc::from_display(&filter);
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_ts,
                impl_default_option_some_vec_one_el_additional_fields_ts,
                increment_parameter_underscore,
                query_part_content_ts,
                is_query_bind_mutable,
                query_bind_content_ts,
            ) = {
                let sqlx_type_pg_encode_ts = quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let should_add_declaration_of_struct_ident_generic_true_type_encode =
                    ShouldAddDeclarationOfStructIdentGeneric::True {
                        maybe_additional_traits_ts: Some(sqlx_type_pg_encode_ts.clone()),
                    };
                let pub_value_pg_type_not_empty_unique_vec_t_ts =
                    quote! {pub #ValueSc: PgTypeNotEmptyUniqueVec<T>};
                let gen_pg_type_dimensions_helpers_pg_type =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_pg_type_dimensions_helpers(
                            pg_type_pattern_handle,
                            &PgTypeOrPgJsonType::PgType,
                        )
                    };
                let gen_32abfefc_c087_480b_b502_cb78533dafb0_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle,
                     gen_format_handle_str: &dyn Fn(&PgTypeKind) -> String| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                            gen_maybe_dimensions_declaration_pub_value_t_ts(
                                &maybe_dimensions_declaration_ts,
                            ),
                            gen_maybe_dimensions_default_initialization_value_default_ts(
                                &maybe_dimensions_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(
                                    &gen_format_handle_str(&pg_type_kind),
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
                                #query_bind_one_value_ts
                            },
                        )
                    };
                let gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle, operator: &dyn Display| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            pg_type_pattern_handle,
                            &|pg_type_kind: &PgTypeKind| {
                                format!(
                                    "{{}}({{}}{} {operator} ${{}})",
                                    pg_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &">")
                };
                let gen_between_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_between_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", pg_type_kind.format_argument()));
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
                let gen_in_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::True {
                            maybe_additional_traits_ts: Some(
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_ts},
                            ),
                        },
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_pg_type_not_empty_unique_vec_t_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote! {acc_14596a52, "${value_daedba9c},"},
                                &quote! {panic!("87f47f75");},
                            );
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let #ValueSc = {
                                    let mut acc_14596a52 = String::default();
                                    for _ in #SelfSc.#ValueSc.to_vec() {
                                        match pg_crud_common::increment_checked_add_one_returning_increment(#IncrementSc) {
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
                let gen_regular_expression_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
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
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} {{}} ${{}})",
                                pg_type_kind.format_argument()
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
                let gen_before_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        gen_maybe_dimensions_declaration_pub_value_t_ts(
                            &maybe_dimensions_declaration_ts,
                        ),
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} < ${{}})",
                                pg_type_kind.format_argument()
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
                    |pg_type_pattern_handle: &PgTypePatternHandle, pg_syntax: &dyn Display| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            match &pg_type_pattern_handle {
                                PgTypePatternHandle::Standart => IncrementParameterUnderscore::True,
                                PgTypePatternHandle::ArrayDimension1
                                | PgTypePatternHandle::ArrayDimension2
                                | PgTypePatternHandle::ArrayDimension3
                                | PgTypePatternHandle::ArrayDimension4 => {
                                    IncrementParameterUnderscore::False
                                }
                            },
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}({{}}{} {pg_syntax})",
                                    pg_type_kind.format_argument()
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
                            match &pg_type_pattern_handle {
                                PgTypePatternHandle::Standart => is_query_bind_mutable_false,
                                PgTypePatternHandle::ArrayDimension1
                                | PgTypePatternHandle::ArrayDimension2
                                | PgTypePatternHandle::ArrayDimension3
                                | PgTypePatternHandle::ArrayDimension4 => {
                                    is_query_bind_mutable_true
                                }
                            },
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                Ok(#QuerySc)
                            },
                        )
                    };
                let gen_current_date_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                        pg_type_pattern_handle,
                        &"= current_date",
                    )
                };
                let gen_greater_than_current_date_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            pg_type_pattern_handle,
                            &"> current_date",
                        )
                    };
                let gen_current_timestamp_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                        pg_type_pattern_handle,
                        &"= current_timestamp",
                    )
                };
                let gen_greater_than_current_timestamp_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            pg_type_pattern_handle,
                            &"> current_timestamp",
                        )
                    };
                let gen_current_time_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                        pg_type_pattern_handle,
                        &"= current_time",
                    )
                };
                let gen_greater_than_current_time_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts(
                            pg_type_pattern_handle,
                            &"> current_time",
                        )
                    };
                let gen_equal_to_encoded_string_representation_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_pg_type_dimensions_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            quote! {
                                #maybe_dimensions_declaration_ts
                                pub encode_format: EncodeFormat,
                                pub encoded_string_representation: String,
                            },
                            quote! {
                                #maybe_dimensions_default_initialization_ts
                                encode_format: #pg_crud_common_default_option_some_vec_one_el_call_ts,
                                encoded_string_representation: #core_default_default_default_ts
                            },
                            IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                    "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                    pg_type_kind.format_argument()
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
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"<@")
                    };
                let gen_find_ranges_that_fully_contain_the_given_range_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"@>")
                    };
                let gen_strictly_to_left_of_range_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"&<")
                    };
                let gen_strictly_to_right_of_range_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"&>")
                    };
                let gen_overlap_with_range_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"&&")
                };
                let gen_adjacent_with_range_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_a2ca84d5_03cc_48b6_9eb5_81b2939181d6_ts(pg_type_pattern_handle, &"-|-")
                };
                let gen_length_filter_pattern_ts = |operator: &dyn Display| {
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        pub_value_not_zero_unsigned_part_of_i32_declaration_ts.clone(),
                        value_default_option_some_vec_one_el_ts.clone(),
                        IncrementParameterUnderscore::False,
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
                let gen_included_lower_bound_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                        pg_type_pattern_handle,
                        &|pg_type_kind: &PgTypeKind| {
                            format!(
                                "{{}}(lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument()
                            )
                        },
                    )
                };
                let gen_excluded_upper_bound_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                        pg_type_pattern_handle,
                        &|pg_type_kind: &PgTypeKind| {
                            format!(
                                "{{}}(upper({{}}{}) = ${{}})",
                                pg_type_kind.format_argument()
                            )
                        },
                    )
                };
                let gen_greater_than_included_lower_bound_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            pg_type_pattern_handle,
                            &|pg_type_kind: &PgTypeKind| {
                                format!(
                                    "{{}}(lower({{}}{}) > ${{}})",
                                    pg_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_greater_than_excluded_upper_bound_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_32abfefc_c087_480b_b502_cb78533dafb0_ts(
                            pg_type_pattern_handle,
                            &|pg_type_kind: &PgTypeKind| {
                                format!(
                                    "{{}}(upper({{}}{}) > ${{}})",
                                    pg_type_kind.format_argument()
                                )
                            },
                        )
                    };
                let gen_range_length_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts, maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts
                    ) = DimensionNumber::try_from(pg_type_pattern_handle).map_or_else(
                        |()| (Ts2::new(), Ts2::new(), Ts2::new(), PgTypeKind::Standart, quote! {#ColumnSc,}, Ts2::new()),
                        |dimension_number| (
                            gen_pub_dimensions_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(&dimension_number),
                            dimensions_default_initialization_comma_ts.clone(),
                            {
                                let dimensions_indexes1_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes1}, &DimensionsSc, &quote! {pg_type_query_part});
                                let dimensions_indexes2_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dimensions_indexes2}, &DimensionsSc, &quote! {pg_type_query_part});
                                quote! {
                                    #dimensions_indexes1_ts
                                    #dimensions_indexes2_ts
                                }
                            },
                            PgTypeKind::ArrayDimension,
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
                            #pub_value_not_zero_unsigned_part_of_i32_declaration_ts
                        },
                        gen_maybe_dimensions_default_initialization_value_default_ts(
                            &maybe_dimensions_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
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
                    PgTypeFilter::Equal { .. } => {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            _,
                            _,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_pg_type_dimensions_helpers_pg_type(
                            &pg_type_pattern_handle_standart,
                        );
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + pg_crud_common::PgTypeEqualOperator},
                                ),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(
                                &maybe_dimensions_declaration_ts,
                            ),
                            gen_maybe_dimensions_default_initialization_value_default_ts(
                                &maybe_dimensions_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as pg_crud_common::PgTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    pg_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    },
                                    pg_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let pg_crud_common::EqualOperator::Equal = &<T as pg_crud_common::PgTypeEqualOperator>::operator(&#SelfSc.#ValueSc) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    PgTypeFilter::DimensionOneEqual { .. } => {
                        let (
                            maybe_dimensions_declaration_ts,
                            maybe_dimensions_default_initialization_ts,
                            maybe_dimensions_indexes_initialization_ts,
                            _,
                            _,
                            maybe_dimensions_query_bind_content_ts,
                        ) = gen_pg_type_dimensions_helpers_pg_type(
                            &pg_type_pattern_handle_array_dimension1,
                        );
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + pg_crud_common::PgTypeEqualOperator},
                                ),
                            },
                            gen_maybe_dimensions_declaration_pub_value_t_ts(
                                &maybe_dimensions_declaration_ts,
                            ),
                            gen_maybe_dimensions_default_initialization_value_default_ts(
                                &maybe_dimensions_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dimensions_indexes_initialization_ts
                                let operator = <T as pg_crud_common::PgTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    pg_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    }
                                    pg_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({}{dimensions_indexes} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dimensions_query_bind_content_ts
                                if let pg_crud_common::EqualOperator::Equal = &<T as pg_crud_common::PgTypeEqualOperator>::operator(
                                    &#SelfSc.#ValueSc
                                ) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    PgTypeFilter::GreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::Between { .. } => {
                        gen_between_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneBetween { .. } => {
                        gen_between_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::In { .. } => gen_in_ts(&pg_type_pattern_handle_standart),
                    PgTypeFilter::DimensionOneIn { .. } => {
                        gen_in_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::RegularExpression => {
                        gen_regular_expression_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneRegularExpression => {
                        gen_regular_expression_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::Before { .. } => gen_before_ts(&pg_type_pattern_handle_standart),
                    PgTypeFilter::DimensionOneBefore { .. } => {
                        gen_before_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::CurrentDate => {
                        gen_current_date_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneCurrentDate => {
                        gen_current_date_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::GreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::CurrentTimestamp => {
                        gen_current_timestamp_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneCurrentTimestamp => {
                        gen_current_timestamp_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::GreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(
                            &pg_type_pattern_handle_array_dimension1,
                        )
                    }
                    PgTypeFilter::CurrentTime => {
                        gen_current_time_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneCurrentTime => {
                        gen_current_time_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::GreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::DimensionOneLengthEqual => gen_length_filter_pattern_ts(&"="),
                    PgTypeFilter::DimensionOneLengthGreaterThan => {
                        gen_length_filter_pattern_ts(&">")
                    }
                    PgTypeFilter::EqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(
                            &pg_type_pattern_handle_standart,
                        )
                    }
                    PgTypeFilter::DimensionOneEqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(
                            &pg_type_pattern_handle_array_dimension1,
                        )
                    }
                    PgTypeFilter::FindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneFindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(
                            &pg_type_pattern_handle_array_dimension1,
                        )
                    }
                    PgTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_find_ranges_that_fully_contain_the_given_range_ts(
                            &pg_type_pattern_handle_standart,
                        )
                    }
                    PgTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange {
                        ..
                    } => gen_find_ranges_that_fully_contain_the_given_range_ts(
                        &pg_type_pattern_handle_array_dimension1,
                    ),
                    PgTypeFilter::StrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneStrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::StrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneStrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::IncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneIncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::ExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::GreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(
                            &pg_type_pattern_handle_array_dimension1,
                        )
                    }
                    PgTypeFilter::GreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneGreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(
                            &pg_type_pattern_handle_array_dimension1,
                        )
                    }
                    PgTypeFilter::OverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneOverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::AdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneAdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_pattern_handle_array_dimension1)
                    }
                    PgTypeFilter::RangeLength => {
                        gen_range_length_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimensionOneRangeLength => {
                        gen_range_length_ts(&pg_type_pattern_handle_array_dimension1)
                    }
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
            let impl_pg_type_where_filter_ts = gen_impl_pg_type_where_filter_ts(
                &FilterType::PgType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &increment_parameter_underscore,
                &IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_pg_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts =
            PgTypeFilter::into_array().map(|el_7cfb1929| gen_filters_ts(&el_7cfb1929));
        let gend = quote! {#(#filter_array_ts)*};
        maybe_write_ts_into_file(
            gen_where_filters_config.pg_types_content_write_into_gen_where_filters_pg_types,
            "gen_where_filters_pg_types",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let pg_json_type_ts = {
        let gen_filters_ts = |filter: &PgJsonTypeFilter| {
            let ident = PgJsonTypeWhereSelfUcc::from_display(&filter);
            let pub_value_pg_json_type_not_empty_unique_vec_t_ts = quote! {
                pub #ValueSc: PgJsonTypeNotEmptyUniqueVec<T>
            };
            let query_bind_sqlx_types_json_self_value_ts = quote! {
                if let Err(#ErrorSc) = #QuerySc.try_bind(sqlx::types::Json(#SelfSc.#ValueSc)) {
                    return Err(#ErrorSc.to_string());
                }
                Ok(#QuerySc)
            };
            let gen_pg_json_type_dimensions_helpers =
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_pg_type_dimensions_helpers(
                        pg_type_pattern_handle,
                        &PgTypeOrPgJsonType::PgJsonType,
                    )
                };
            let gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle,
                 gen_format_handle_str: &dyn Fn(&PgTypeKind) -> String| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
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
                            let format_handle_ts =
                                gen_quotes::double_quotes_ts(&gen_format_handle_str(&pg_type_kind));
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
                |pg_type_pattern_handle: &PgTypePatternHandle, operator: &dyn Display| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        pg_type_pattern_handle,
                        &|pg_type_kind: &PgTypeKind| {
                            format!(
                                "{{}}({{}}{} {operator} ${{}})",
                                pg_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_equal_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(pg_type_pattern_handle, &"=")
            };
            let gen_all_elements_equal_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                    pg_type_pattern_handle,
                    &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))",
                            pg_type_kind.format_argument()
                        )
                    },
                )
            };
            let gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle, operation: &dyn Display| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            pub #ValueSc: #unsigned_part_of_i32_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #ValueSc: #core_default_default_default_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}(jsonb_array_length({{}}{}) {operation} ${{}})",
                                pg_type_kind.format_argument()
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
            let gen_length_equal_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(pg_type_pattern_handle, &"=")
            };
            let gen_length_greater_than_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_ae2fa44d_9035_49fd_ba20_eed1bd4680d4_ts(pg_type_pattern_handle, &">")
            };
            let gen_greater_than_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_7cc8e29b_53e1_4bee_9947_71987439148c_ts(pg_type_pattern_handle, &">")
            };
            let gen_contains_el_greater_than_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                    pg_type_pattern_handle,
                    &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))",
                            pg_type_kind.format_argument()
                        )
                    },
                )
            };
            let gen_all_elements_greater_than_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts(
                        pg_type_pattern_handle,
                        &|pg_type_kind: &PgTypeKind| {
                            format!(
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))",
                                pg_type_kind.format_argument()
                            )
                        },
                    )
                };
            let gen_between_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_between_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(&maybe_dimensions_default_initialization_ts),
                    {
                        let content_ts: &dyn quote::ToTokens = match pg_type_pattern_handle {
                            PgTypePatternHandle::Standart => &quote!{
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
                            PgTypePatternHandle::ArrayDimension1 |
                            PgTypePatternHandle::ArrayDimension2 |
                            PgTypePatternHandle::ArrayDimension3 |
                            PgTypePatternHandle::ArrayDimension4 => &value_match_increment_checked_add_one_initialization_ts
                        };
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{{}}({{}}{} {{}})", pg_type_kind.format_argument()));
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
                        let content_ts: &dyn quote::ToTokens = match pg_type_pattern_handle {
                            PgTypePatternHandle::Standart => &quote!{
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
                            PgTypePatternHandle::ArrayDimension1 |
                            PgTypePatternHandle::ArrayDimension2 |
                            PgTypePatternHandle::ArrayDimension3 |
                            PgTypePatternHandle::ArrayDimension4 => &query_bind_sqlx_types_json_self_value_ts
                        };
                        quote! {
                            #maybe_dimensions_query_bind_content_ts
                            #content_ts
                        }
                    },
                )
            };
            let gen_in_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                    },
                    gen_maybe_dimensions_default_initialization_value_default_ts(
                        &maybe_dimensions_default_initialization_ts,
                    ),
                    {
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                            "{{}}({{}}{} in ({{}}))",
                            pg_type_kind.format_argument()
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
            let gen_regular_expression_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    pg_type_kind, maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts
                ) = DimensionNumber::try_from(pg_type_pattern_handle).map_or_else(
                    |()| (Ts2::new(), Ts2::new(), Ts2::new(), PgTypeKind::Standart, Ts2::new(), Ts2::new()),
                    |dimension_number| (
                        gen_pub_dimensions_bounded_vec_unsigned_part_of_i32_comma_ts(&dimension_number),
                        dimensions_default_initialization_comma_ts.clone(),
                        {
                            let dimensions_indexes_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&DimensionsIndexesSc, &DimensionsSc, &quote! {pg_json_type_query_part_minus_one});
                            let last_dimensions_index_intialization_ts = gen_match_increment_checked_add_one_initialization_ts(&quote! {last_dimensions_index});
                            quote! {
                                #dimensions_indexes_initialization_ts
                                #last_dimensions_index_intialization_ts
                            }
                        },
                        PgTypeKind::ArrayDimension,
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
                            match &pg_type_kind {
                                PgTypeKind::Standart => "",
                                PgTypeKind::ArrayDimension => "{}->>${}",
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
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
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
                                pg_type_kind.format_argument()
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
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
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
                                pg_type_kind.format_argument()
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
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dimensions_declaration_ts,
                        maybe_dimensions_default_initialization_ts,
                        maybe_dimensions_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dimensions_query_bind_content_ts,
                    ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dimensions_declaration_ts
                            #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dimensions_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                                "{{}}({{}}{} @> {{}})",
                                pg_type_kind.format_argument()
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
            let gen_overlaps_with_array_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dimensions_declaration_ts,
                    maybe_dimensions_default_initialization_ts,
                    maybe_dimensions_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dimensions_query_bind_content_ts,
                ) = gen_pg_json_type_dimensions_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dimensions_declaration_ts
                        #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                    },
                    quote! {
                        #maybe_dimensions_default_initialization_ts
                        #value_default_option_some_vec_one_el_ts
                    },
                    {
                        let format_handle_ts = gen_quotes::double_quotes_ts(&format!(
                            "{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))",
                            pg_type_kind.format_argument()
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
                query_bind_content_ts,
            ) = match &filter {
                PgJsonTypeFilter::Equal { .. } => gen_equal_ts(&pg_type_pattern_handle_standart),
                PgJsonTypeFilter::DimensionOneEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::AllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::LengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::LengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::GreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::ContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::AllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::Between { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::In { .. } => gen_in_ts(&pg_type_pattern_handle_standart),
                PgJsonTypeFilter::DimensionOneIn { .. } => {
                    gen_in_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoIn { .. } => {
                    gen_in_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeIn { .. } => {
                    gen_in_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourIn { .. } => {
                    gen_in_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::RegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::ContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::AllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dimension4)
                }
                PgJsonTypeFilter::ContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dimension4)
                }
                // PgJsonTypeFilter::ContainedInArray => todo!(),
                PgJsonTypeFilter::OverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimensionOneOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dimension1)
                }
                PgJsonTypeFilter::DimensionTwoOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dimension2)
                }
                PgJsonTypeFilter::DimensionThreeOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dimension3)
                }
                PgJsonTypeFilter::DimensionFourOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dimension4)
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
            let impl_pg_type_where_filter_ts = gen_impl_pg_type_where_filter_ts(
                &FilterType::PgJsonType,
                &should_add_declaration_of_struct_ident_generic,
                &ident,
                &IncrementParameterUnderscore::False,
                &IsNeedToAddLogicalOperatorUnderscore::False,
                &query_part_content_ts,
                &is_query_bind_mutable,
                &query_bind_content_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_default_option_some_vec_one_el_ts
                #impl_pg_type_where_filter_ts
            };
            gend
        };
        let filter_array_ts =
            PgJsonTypeFilter::into_array().map(|el_6a4ac539| gen_filters_ts(&el_6a4ac539));
        let gend = quote! {#(#filter_array_ts)*};
        maybe_write_ts_into_file(
            gen_where_filters_config
                .pg_json_types_content_write_into_gen_where_filters_pg_json_types,
            "gen_where_filters_pg_json_types",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let gend = quote! {
        #pg_type_ts
        #pg_json_type_ts
    };
    maybe_write_ts_into_file(
        gen_where_filters_config.whole_content_write_into_gen_where_filters,
        "gen_where_filters",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend.into()
}
