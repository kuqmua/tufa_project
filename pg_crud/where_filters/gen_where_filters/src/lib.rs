use gen_quotes::double_quotes_ts;
use macros_helpers::{DeriveSerdeDeserialize, StructOrEnumDeriveTokenStreamBuilder};
use macros_helpers::{
    FormatWithCargofmt, ShouldWriteTokenStreamIntoFile, gen_if_write_is_err_ts,
    maybe_write_ts_into_file,
};
use naming::{
    ColumnSc, DimsIndexesSc, DimsSc, ErrorSc, IncrementSc, PubSc, QuerySc, SelfSc, ValueSc,
    parameter::{PgJsonTypeWhereSelfUcc, PgTypeWhereSelfUcc},
};
use panic_location::panic_location;
use pg_crud_macros_common::{
    ColumnParameterUnderscore, ImportPath, IncrementParameterUnderscore,
    IsNeedToAddLogicalOperatorUnderscore, IsQueryBindMutable, PgJsonTypeFilter, PgTypeFilter,
    PgTypeOrPgJsonType, gen_impl_default_option_some_vec_one_el_ts,
    impl_pg_type_where_filter_for_ident_ts,
};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde_json::from_str;
use std::fmt::Display;
use token_patterns::{
    CoreDefaultDefaultDefault, PgCrudCommonDefaultOptionSomeVecOneEl,
    PgCrudCommonDefaultOptionSomeVecOneElCall,
};

#[proc_macro]
pub fn gen_where_filters(input_ts: Ts) -> Ts {
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
        ArrayDim1,
        ArrayDim2,
        ArrayDim3,
        ArrayDim4,
    }
    impl TryFrom<&PgTypePatternHandle> for DimNumber {
        type Error = ();
        fn try_from(value: &PgTypePatternHandle) -> Result<Self, Self::Error> {
            match &value {
                PgTypePatternHandle::Standart => Err(()),
                PgTypePatternHandle::ArrayDim1 => Ok(Self::One),
                PgTypePatternHandle::ArrayDim2 => Ok(Self::Two),
                PgTypePatternHandle::ArrayDim3 => Ok(Self::Three),
                PgTypePatternHandle::ArrayDim4 => Ok(Self::Four),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone)]
    enum DimNumber {
        One,
        Two,
        Three,
        Four,
    }
    impl DimNumber {
        fn dim_ts(&self) -> Ts2 {
            self.dim_u8().to_string().parse::<Ts2>().expect("18c32bc0")
        }
        const fn dim_u8(&self) -> u8 {
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
    impl ToTokens for KindOfUnsignedPartOfI32 {
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
        ArrayDim,
    }
    impl PgTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Standart => "",
                Self::ArrayDim => "{}",
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
    panic_location();
    let gen_where_filters_config =
        from_str::<GenWhereFiltersConfig>(&input_ts.to_string()).expect("1217b73b");
    let import_path = ImportPath::PgCrudCommon;
    let t_ts = quote! {T};
    let t_annotation_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let core_default_default_default_ts = CoreDefaultDefaultDefault;
    let pg_crud_common_default_option_some_vec_one_el_ts = PgCrudCommonDefaultOptionSomeVecOneEl;
    let pg_crud_common_default_option_some_vec_one_el_call_ts =
        PgCrudCommonDefaultOptionSomeVecOneElCall;
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
    let gen_struct_ts = |filter_initialized_with_try_new_result_is_ok: bool, should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn ToTokens, struct_additional_fields_ts: &dyn ToTokens| {
        let maybe_pub_ts: &dyn ToTokens = if filter_initialized_with_try_new_result_is_ok { &proc_macro2_ts_new } else { &PubSc };
        let maybe_declaration_of_struct_ident_generic_ts: &dyn ToTokens = match &should_add_declaration_of_struct_ident_generic {
            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_ts_new,
            ShouldAddDeclarationOfStructIdentGeneric::True { maybe_additional_traits_ts } => {
                &maybe_additional_traits_ts.as_ref().map_or_else(|| quote! {<#t_ts>}, |value_d05f3d4f| quote! {<#t_ts: #value_d05f3d4f>})
            }
        };
        StructOrEnumDeriveTokenStreamBuilder::new()
        .make_pub()
        .derive_debug()
        .derive_clone()
        .derive_partial_eq()
        .derive_serde_serialize()
        .derive_serde_deserialize_if(
            if filter_initialized_with_try_new_result_is_ok {
                DeriveSerdeDeserialize::False
            } else {
                DeriveSerdeDeserialize::True
            }
        )
        .derive_schemars_json_schema()
        .build_struct(
            &ident,
            &maybe_declaration_of_struct_ident_generic_ts,
            &quote::quote!{{
                #maybe_pub_ts logical_operator: #import_path::LogicalOperator,
                #struct_additional_fields_ts
            }}
        )
    };
    let gen_impl_default_option_some_vec_one_el_ts = |should_add_declaration_of_struct_ident_generic: &ShouldAddDeclarationOfStructIdentGeneric, ident: &dyn ToTokens, impl_default_option_some_vec_one_el_additional_fields_ts: &dyn ToTokens| {
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
        ident: &dyn ToTokens,
        increment_parameter_underscore: &IncrementParameterUnderscore,
        is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
        query_part_content_ts: &dyn ToTokens,
        is_query_bind_mutable: &IsQueryBindMutable,
        query_bind_content_ts: &dyn ToTokens
    | {
        impl_pg_type_where_filter_for_ident_ts(
            &{
                let maybe_t_additional_traits_for_pg_type_where_filter_ts: &dyn ToTokens = match &should_add_declaration_of_struct_ident_generic {
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
    let gen_match_increment_checked_add_one_initialization_ts = |ident_ts: &dyn ToTokens| {
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
    let pg_type_pattern_handle_array_dim1 = PgTypePatternHandle::ArrayDim1;
    let pg_type_pattern_handle_array_dim2 = PgTypePatternHandle::ArrayDim2;
    let pg_type_pattern_handle_array_dim3 = PgTypePatternHandle::ArrayDim3;
    let pg_type_pattern_handle_array_dim4 = PgTypePatternHandle::ArrayDim4;
    let gen_pub_dims_bounded_vec_ts =
        |vec_length_ts: &dyn ToTokens, kind_of_unsigned_part_of_i32: &KindOfUnsignedPartOfI32| {
            quote! {pub #DimsSc: BoundedStdVecVec<pg_crud_common::#kind_of_unsigned_part_of_i32, #vec_length_ts>}
        };
    let value_match_increment_checked_add_one_initialization_ts =
        gen_match_increment_checked_add_one_initialization_ts(&ValueSc);
    let gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts =
        |ident_ts: &dyn ToTokens, field_ts: &dyn ToTokens, function_ts: &dyn ToTokens| {
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
    let dims_default_initialization_ts = quote! {
        #DimsSc: #pg_crud_common_default_option_some_vec_one_el_call_ts
    };
    let dims_default_initialization_comma_ts = quote! {#dims_default_initialization_ts,};
    let query_self_dims_query_bind_query_ts = quote! {
        match #SelfSc.#DimsSc.query_bind(#QuerySc) {
            Ok(value_ed6f1157) => {
                #QuerySc = value_ed6f1157;
            }
            Err(#ErrorSc) => {
                return Err(#ErrorSc);
            }
        }
    };
    let dims_indexes_comma_ts = quote! {#DimsIndexesSc,};
    let gen_maybe_dims_declaration_pub_value_t_ts = |maybe_dims_declaration_ts: &dyn ToTokens| {
        quote! {
            #maybe_dims_declaration_ts
            #pub_value_t_ts
        }
    };
    let gen_maybe_dims_default_initialization_value_default_ts =
        |maybe_dims_default_initialization_ts: &dyn ToTokens| {
            quote! {
                #maybe_dims_default_initialization_ts
                #value_default_option_some_vec_one_el_ts
            }
        };
    let is_query_bind_mutable_true = IsQueryBindMutable::True;
    let is_query_bind_mutable_false = IsQueryBindMutable::False;
    let gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts =
        |dim_number: &DimNumber| {
            let pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_ts = gen_pub_dims_bounded_vec_ts(
                &dim_number.dim_ts(),
                &KindOfUnsignedPartOfI32::CanNotBeZero,
            );
            quote! {#pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_ts,}
        };
    let gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts = |dim_number: &DimNumber| {
        let pub_dims_bounded_vec_unsigned_part_of_i32_ts =
            gen_pub_dims_bounded_vec_ts(&dim_number.dim_ts(), &KindOfUnsignedPartOfI32::CanBeZero);
        quote! {#pub_dims_bounded_vec_unsigned_part_of_i32_ts,}
    };
    let gen_pg_type_dims_helpers =
        |pg_type_pattern_handle: &PgTypePatternHandle,
         pg_type_or_pg_json_type: &PgTypeOrPgJsonType| {
            DimNumber::try_from(pg_type_pattern_handle).map_or_else(
            |()| (Ts2::new(),Ts2::new(), Ts2::new(), PgTypeKind::Standart, Ts2::new(), Ts2::new()),
            |dim_number| (
                match &pg_type_or_pg_json_type {
                    PgTypeOrPgJsonType::PgType => gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(&dim_number),
                    PgTypeOrPgJsonType::PgJsonType => gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts(&dim_number),
                },
                dims_default_initialization_comma_ts.clone(),
                gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(
                    &DimsIndexesSc,
                    &DimsSc,
                    &match &pg_type_or_pg_json_type {
                        PgTypeOrPgJsonType::PgType => quote! {pg_type_query_part},
                        PgTypeOrPgJsonType::PgJsonType => quote! {pg_json_type_query_part},
                    },
                ),
                PgTypeKind::ArrayDim,
                dims_indexes_comma_ts.clone(),
                query_self_dims_query_bind_query_ts.clone(),
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
                let gen_pg_type_dims_helpers_pg_type =
                    |pg_type_pattern_handle: &PgTypePatternHandle| {
                        gen_pg_type_dims_helpers(
                            pg_type_pattern_handle,
                            &PgTypeOrPgJsonType::PgType,
                        )
                    };
                let gen_32abfefc_c087_480b_b502_cb78533dafb0_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle,
                     gen_format_handle_str: &dyn Fn(&PgTypeKind) -> String| {
                        let (
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            maybe_dims_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dims_query_bind_content_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                            gen_maybe_dims_declaration_pub_value_t_ts(&maybe_dims_declaration_ts),
                            gen_maybe_dims_default_initialization_value_default_ts(
                                &maybe_dims_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts =
                                    double_quotes_ts(&gen_format_handle_str(&pg_type_kind));
                                quote! {
                                    #maybe_dims_indexes_initialization_ts
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
                                #maybe_dims_query_bind_content_ts
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
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #pub_value_between_t_ts
                        },
                        gen_maybe_dims_default_initialization_value_default_ts(&maybe_dims_default_initialization_ts),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = double_quotes_ts(&format!("{{}}({{}}{} {{}})", pg_type_kind.format_argument()));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #query_self_value_query_bind_ts
                        },
                    )
                };
                let gen_in_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::True {
                            maybe_additional_traits_ts: Some(
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_ts},
                            ),
                        },
                        quote! {
                            #maybe_dims_declaration_ts
                            #pub_value_pg_type_not_empty_unique_vec_t_ts
                        },
                        gen_maybe_dims_default_initialization_value_default_ts(
                            &maybe_dims_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote! {acc_14596a52, "${value_daedba9c},"},
                                &quote! {panic!("87f47f75");},
                            );
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
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
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}({{}}{} {{}} ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
                let gen_before_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_type_encode.clone(),
                        gen_maybe_dims_declaration_pub_value_t_ts(&maybe_dims_declaration_ts),
                        gen_maybe_dims_default_initialization_value_default_ts(
                            &maybe_dims_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}({{}}{} < ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #query_bind_one_value_ts
                        },
                    )
                };
                let gen_1fa0bbf4_908e_421b_ae0a_fc9e7ff95034_ts =
                    |pg_type_pattern_handle: &PgTypePatternHandle, pg_syntax: &dyn Display| {
                        let (
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            maybe_dims_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dims_query_bind_content_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            match &pg_type_pattern_handle {
                                PgTypePatternHandle::Standart => IncrementParameterUnderscore::True,
                                PgTypePatternHandle::ArrayDim1
                                | PgTypePatternHandle::ArrayDim2
                                | PgTypePatternHandle::ArrayDim3
                                | PgTypePatternHandle::ArrayDim4 => {
                                    IncrementParameterUnderscore::False
                                }
                            },
                            {
                                let format_handle_ts = double_quotes_ts(&format!(
                                    "{{}}({{}}{} {pg_syntax})",
                                    pg_type_kind.format_argument()
                                ));
                                quote! {
                                    #maybe_dims_indexes_initialization_ts
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
                                PgTypePatternHandle::ArrayDim1
                                | PgTypePatternHandle::ArrayDim2
                                | PgTypePatternHandle::ArrayDim3
                                | PgTypePatternHandle::ArrayDim4 => is_query_bind_mutable_true,
                            },
                            quote! {
                                #maybe_dims_query_bind_content_ts
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
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            maybe_dims_indexes_initialization_ts,
                            pg_type_kind,
                            maybe_additional_parameters_ts,
                            maybe_dims_query_bind_content_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_pattern_handle);
                        (
                            should_add_declaration_of_struct_ident_generic_false.clone(),
                            quote! {
                                #maybe_dims_declaration_ts
                                pub encode_format: EncodeFormat,
                                pub encoded_string_representation: String,
                            },
                            quote! {
                                #maybe_dims_default_initialization_ts
                                encode_format: #pg_crud_common_default_option_some_vec_one_el_call_ts,
                                encoded_string_representation: #core_default_default_default_ts
                            },
                            IncrementParameterUnderscore::False,
                            {
                                let format_handle_ts = double_quotes_ts(&format!(
                                    "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                    pg_type_kind.format_argument()
                                ));
                                quote! {
                                    #maybe_dims_indexes_initialization_ts
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
                                #maybe_dims_query_bind_content_ts
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
                            let format_handle_ts = double_quotes_ts(&format!(
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
                        maybe_dims_declaration_ts, maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts
                    ) = DimNumber::try_from(pg_type_pattern_handle).map_or_else(
                        |()| (Ts2::new(), Ts2::new(), Ts2::new(), PgTypeKind::Standart, quote! {#ColumnSc,}, Ts2::new()),
                        |dim_number| (
                            gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(&dim_number),
                            dims_default_initialization_comma_ts.clone(),
                            {
                                let dims_indexes1_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dims_indexes1}, &DimsSc, &quote! {pg_type_query_part});
                                let dims_indexes2_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&quote! {dims_indexes2}, &DimsSc, &quote! {pg_type_query_part});
                                quote! {
                                    #dims_indexes1_ts
                                    #dims_indexes2_ts
                                }
                            },
                            PgTypeKind::ArrayDim,
                            quote! {
                                dims_indexes1,
                                column,
                                dims_indexes2,
                            },
                            quote! {
                                match #SelfSc.#DimsSc.clone().query_bind(#QuerySc) {
                                    Ok(value_6cb14cdc) => {
                                        #QuerySc = value_6cb14cdc;
                                    },
                                    Err(#ErrorSc) => {
                                        return Err(#ErrorSc);
                                    }
                                }
                                #query_self_dims_query_bind_query_ts
                            },
                        )
                    );
                    (
                        ShouldAddDeclarationOfStructIdentGeneric::False,
                        quote! {
                            #maybe_dims_declaration_ts
                            #pub_value_not_zero_unsigned_part_of_i32_declaration_ts
                        },
                        gen_maybe_dims_default_initialization_value_default_ts(
                            &maybe_dims_default_initialization_ts,
                        ),
                        IncrementParameterUnderscore::False,
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #query_bind_one_value_ts
                        },
                    )
                };
                match &filter {
                    PgTypeFilter::Equal { .. } => {
                        let (
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            maybe_dims_indexes_initialization_ts,
                            _,
                            _,
                            maybe_dims_query_bind_content_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(&pg_type_pattern_handle_standart);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + pg_crud_common::PgTypeEqualOperator},
                                ),
                            },
                            gen_maybe_dims_declaration_pub_value_t_ts(&maybe_dims_declaration_ts),
                            gen_maybe_dims_default_initialization_value_default_ts(
                                &maybe_dims_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                                #maybe_dims_query_bind_content_ts
                                if let pg_crud_common::EqualOperator::Equal = &<T as pg_crud_common::PgTypeEqualOperator>::operator(&#SelfSc.#ValueSc) {
                                    #if_let_err_query_try_bind_self_value_ts
                                }
                                Ok(#QuerySc)
                            },
                        )
                    }
                    PgTypeFilter::DimOneEqual { .. } => {
                        let (
                            maybe_dims_declaration_ts,
                            maybe_dims_default_initialization_ts,
                            maybe_dims_indexes_initialization_ts,
                            _,
                            _,
                            maybe_dims_query_bind_content_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(&pg_type_pattern_handle_array_dim1);
                        (
                            ShouldAddDeclarationOfStructIdentGeneric::True {
                                maybe_additional_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + pg_crud_common::PgTypeEqualOperator},
                                ),
                            },
                            gen_maybe_dims_declaration_pub_value_t_ts(&maybe_dims_declaration_ts),
                            gen_maybe_dims_default_initialization_value_default_ts(
                                &maybe_dims_default_initialization_ts,
                            ),
                            IncrementParameterUnderscore::False,
                            quote! {
                                #maybe_dims_indexes_initialization_ts
                                let operator = <T as pg_crud_common::PgTypeEqualOperator>::operator(&#SelfSc.#ValueSc);
                                let operator_query_str = operator.to_query_str();
                                let content = match operator {
                                    pg_crud_common::EqualOperator::Equal => {
                                        #value_match_increment_checked_add_one_initialization_ts
                                        format!("{operator_query_str} ${value}")
                                    }
                                    pg_crud_common::EqualOperator::IsNull => operator_query_str.to_owned(),
                                };
                                Ok(format!("{}({}{dims_indexes} {content})", &#SelfSc.logical_operator.to_query_part(is_need_to_add_logical_operator), #ColumnSc))
                            },
                            is_query_bind_mutable_true,
                            quote! {
                                #maybe_dims_query_bind_content_ts
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
                    PgTypeFilter::DimOneGreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::Between { .. } => {
                        gen_between_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneBetween { .. } => {
                        gen_between_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::In { .. } => gen_in_ts(&pg_type_pattern_handle_standart),
                    PgTypeFilter::DimOneIn { .. } => gen_in_ts(&pg_type_pattern_handle_array_dim1),
                    PgTypeFilter::RegularExpression => {
                        gen_regular_expression_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneRegularExpression => {
                        gen_regular_expression_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::Before { .. } => gen_before_ts(&pg_type_pattern_handle_standart),
                    PgTypeFilter::DimOneBefore { .. } => {
                        gen_before_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::CurrentDate => {
                        gen_current_date_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneCurrentDate => {
                        gen_current_date_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::GreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::CurrentTimestamp => {
                        gen_current_timestamp_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneCurrentTimestamp => {
                        gen_current_timestamp_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::GreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::CurrentTime => {
                        gen_current_time_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneCurrentTime => {
                        gen_current_time_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::GreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::DimOneLengthEqual => gen_length_filter_pattern_ts(&"="),
                    PgTypeFilter::DimOneLengthGreaterThan => gen_length_filter_pattern_ts(&">"),
                    PgTypeFilter::EqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(
                            &pg_type_pattern_handle_standart,
                        )
                    }
                    PgTypeFilter::DimOneEqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(
                            &pg_type_pattern_handle_array_dim1,
                        )
                    }
                    PgTypeFilter::FindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneFindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_find_ranges_that_fully_contain_the_given_range_ts(
                            &pg_type_pattern_handle_standart,
                        )
                    }
                    PgTypeFilter::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_find_ranges_that_fully_contain_the_given_range_ts(
                            &pg_type_pattern_handle_array_dim1,
                        )
                    }
                    PgTypeFilter::StrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneStrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::StrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneStrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::IncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneIncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::ExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::GreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneGreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::GreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneGreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::OverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneOverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::AdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneAdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_pattern_handle_array_dim1)
                    }
                    PgTypeFilter::RangeLength => {
                        gen_range_length_ts(&pg_type_pattern_handle_standart)
                    }
                    PgTypeFilter::DimOneRangeLength => {
                        gen_range_length_ts(&pg_type_pattern_handle_array_dim1)
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
            let gen_pg_json_type_dims_helpers = |pg_type_pattern_handle: &PgTypePatternHandle| {
                gen_pg_type_dims_helpers(pg_type_pattern_handle, &PgTypeOrPgJsonType::PgJsonType)
            };
            let gen_1763ccf3_10be_4527_912b_363d8ea05f4b_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle,
                 gen_format_handle_str: &dyn Fn(&PgTypeKind) -> String| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_none.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #pub_value_t_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts =
                                double_quotes_ts(&gen_format_handle_str(&pg_type_kind));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
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
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            pub #ValueSc: #unsigned_part_of_i32_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #ValueSc: #core_default_default_default_ts
                        },
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}(jsonb_array_length({{}}{}) {operation} ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
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
                    maybe_dims_declaration_ts,
                    maybe_dims_default_initialization_ts,
                    maybe_dims_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dims_query_bind_content_ts,
                ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote! {
                        #maybe_dims_declaration_ts
                        #pub_value_between_t_ts
                    },
                    gen_maybe_dims_default_initialization_value_default_ts(&maybe_dims_default_initialization_ts),
                    {
                        let content_ts: &dyn ToTokens = match pg_type_pattern_handle {
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
                            PgTypePatternHandle::ArrayDim1 |
                            PgTypePatternHandle::ArrayDim2 |
                            PgTypePatternHandle::ArrayDim3 |
                            PgTypePatternHandle::ArrayDim4 => &value_match_increment_checked_add_one_initialization_ts
                        };
                        let format_handle_ts = double_quotes_ts(&format!("{{}}({{}}{} {{}})", pg_type_kind.format_argument()));
                        quote! {
                            #maybe_dims_indexes_initialization_ts
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
                        let content_ts: &dyn ToTokens = match pg_type_pattern_handle {
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
                            PgTypePatternHandle::ArrayDim1 |
                            PgTypePatternHandle::ArrayDim2 |
                            PgTypePatternHandle::ArrayDim3 |
                            PgTypePatternHandle::ArrayDim4 => &query_bind_sqlx_types_json_self_value_ts
                        };
                        quote! {
                            #maybe_dims_query_bind_content_ts
                            #content_ts
                        }
                    },
                )
            };
            let gen_in_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dims_declaration_ts,
                    maybe_dims_default_initialization_ts,
                    maybe_dims_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dims_query_bind_content_ts,
                ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dims_declaration_ts
                        #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                    },
                    gen_maybe_dims_default_initialization_value_default_ts(
                        &maybe_dims_default_initialization_ts,
                    ),
                    {
                        let format_handle_ts = double_quotes_ts(&format!(
                            "{{}}({{}}{} in ({{}}))",
                            pg_type_kind.format_argument()
                        ));
                        let value_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&ValueSc, &ValueSc, &quote! {query_part_one_by_one});
                        quote! {
                            #maybe_dims_indexes_initialization_ts
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
                        #maybe_dims_query_bind_content_ts
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
                    maybe_dims_declaration_ts,
                    maybe_dims_default_initialization_ts,
                    maybe_dims_indexes_initialization_ts,
                    pg_type_kind, maybe_additional_parameters_ts,
                    maybe_dims_query_bind_content_ts
                ) = DimNumber::try_from(pg_type_pattern_handle).map_or_else(
                    |()| (Ts2::new(), Ts2::new(), Ts2::new(), PgTypeKind::Standart, Ts2::new(), Ts2::new()),
                    |dim_number| (
                        gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts(&dim_number),
                        dims_default_initialization_comma_ts.clone(),
                        {
                            let dims_indexes_initialization_ts = gen_ident_match_self_field_function_increment_column_is_need_to_add_logical_operator_initialization_ts(&DimsIndexesSc, &DimsSc, &quote! {pg_json_type_query_part_minus_one});
                            let last_dims_index_intialization_ts = gen_match_increment_checked_add_one_initialization_ts(&quote! {last_dims_index});
                            quote! {
                                #dims_indexes_initialization_ts
                                #last_dims_index_intialization_ts
                            }
                        },
                        PgTypeKind::ArrayDim,
                        quote! {
                            last_dims_index,
                            #DimsIndexesSc,
                        },
                        query_self_dims_query_bind_query_ts.clone(),
                    )
                );
                (
                    should_add_declaration_of_struct_ident_generic_false.clone(),
                    quote! {
                        #maybe_dims_declaration_ts
                        #regular_expression_case_and_value_declaration_ts
                    },
                    quote! {
                        #maybe_dims_default_initialization_ts
                        #regular_expression_case_and_value_default_initialization_ts
                    },
                    {
                        let format_handle_ts = double_quotes_ts(&format!(
                            "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                            match &pg_type_kind {
                                PgTypeKind::Standart => "",
                                PgTypeKind::ArrayDim => "{}->>${}",
                            }
                        ));
                        quote! {
                            #maybe_dims_indexes_initialization_ts
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
                        #maybe_dims_query_bind_content_ts
                        #if_let_err_query_try_bind_self_value_to_string_ts
                    },
                )
            };
            let gen_contains_el_regular_expression_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                //todo test it properly using all strange string variants
                                "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') {{}} ${{}}))",
                                // "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) {{}} ${{}}))",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
            let gen_all_elements_regular_expression_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_false.clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #regular_expression_case_and_value_declaration_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #regular_expression_case_and_value_default_initialization_ts
                        },
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                //todo test it properly using all strange string variants
                                "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') !{{}} ${{}}))",
                                // "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{{}} ${{}}))",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #if_let_err_query_try_bind_self_value_to_string_ts
                        },
                    )
                };
            let gen_contains_all_elements_of_array_ts =
                |pg_type_pattern_handle: &PgTypePatternHandle| {
                    let (
                        maybe_dims_declaration_ts,
                        maybe_dims_default_initialization_ts,
                        maybe_dims_indexes_initialization_ts,
                        pg_type_kind,
                        maybe_additional_parameters_ts,
                        maybe_dims_query_bind_content_ts,
                    ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                    (
                        should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                            .clone(),
                        quote! {
                            #maybe_dims_declaration_ts
                            #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                        },
                        quote! {
                            #maybe_dims_default_initialization_ts
                            #value_default_option_some_vec_one_el_ts
                        },
                        {
                            let format_handle_ts = double_quotes_ts(&format!(
                                "{{}}({{}}{} @> {{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #maybe_dims_indexes_initialization_ts
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
                            #maybe_dims_query_bind_content_ts
                            #query_bind_sqlx_types_json_self_value_ts
                        },
                    )
                };
            let gen_overlaps_with_array_ts = |pg_type_pattern_handle: &PgTypePatternHandle| {
                let (
                    maybe_dims_declaration_ts,
                    maybe_dims_default_initialization_ts,
                    maybe_dims_indexes_initialization_ts,
                    pg_type_kind,
                    maybe_additional_parameters_ts,
                    maybe_dims_query_bind_content_ts,
                ) = gen_pg_json_type_dims_helpers(pg_type_pattern_handle);
                (
                    should_add_declaration_of_struct_ident_generic_true_debug_partial_eq_clone
                        .clone(),
                    quote! {
                        #maybe_dims_declaration_ts
                        #pub_value_pg_json_type_not_empty_unique_vec_t_ts
                    },
                    quote! {
                        #maybe_dims_default_initialization_ts
                        #value_default_option_some_vec_one_el_ts
                    },
                    {
                        let format_handle_ts = double_quotes_ts(&format!(
                            "{{}}(exists (select 1 from jsonb_array_elements_text({{}}{}) as e1 join jsonb_array_elements_text({{}}) as e2 on e1.value = e2.value))",
                            pg_type_kind.format_argument()
                        ));
                        quote! {
                            #maybe_dims_indexes_initialization_ts
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
                        #maybe_dims_query_bind_content_ts
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
                PgJsonTypeFilter::DimOneEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourEqual { .. } => {
                    gen_equal_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::AllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourAllElementsEqual { .. } => {
                    gen_all_elements_equal_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::LengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourLengthEqual => {
                    gen_length_equal_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::LengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::GreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::ContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::AllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourAllElementsGreaterThan { .. } => {
                    gen_all_elements_greater_than_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::Between { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourBetween { .. } => {
                    gen_between_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::In { .. } => gen_in_ts(&pg_type_pattern_handle_standart),
                PgJsonTypeFilter::DimOneIn { .. } => gen_in_ts(&pg_type_pattern_handle_array_dim1),
                PgJsonTypeFilter::DimTwoIn { .. } => gen_in_ts(&pg_type_pattern_handle_array_dim2),
                PgJsonTypeFilter::DimThreeIn { .. } => {
                    gen_in_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourIn { .. } => gen_in_ts(&pg_type_pattern_handle_array_dim4),
                PgJsonTypeFilter::RegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourRegularExpression => {
                    gen_regular_expression_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::ContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourContainsElRegularExpression => {
                    gen_contains_el_regular_expression_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::AllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourAllElementsRegularExpression => {
                    gen_all_elements_regular_expression_ts(&pg_type_pattern_handle_array_dim4)
                }
                PgJsonTypeFilter::ContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourContainsAllElementsOfArray { .. } => {
                    gen_contains_all_elements_of_array_ts(&pg_type_pattern_handle_array_dim4)
                }
                // PgJsonTypeFilter::ContainedInArray => todo!(),
                PgJsonTypeFilter::OverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_standart)
                }
                PgJsonTypeFilter::DimOneOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dim1)
                }
                PgJsonTypeFilter::DimTwoOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dim2)
                }
                PgJsonTypeFilter::DimThreeOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dim3)
                }
                PgJsonTypeFilter::DimFourOverlapsWithArray { .. } => {
                    gen_overlaps_with_array_ts(&pg_type_pattern_handle_array_dim4)
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
