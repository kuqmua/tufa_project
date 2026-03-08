use gen_quotes::dq_ts;
use macros_helpers::{DSerdeDeserialize, DTsBuilder};
use macros_helpers::{
    FormatWithCargofmt, ShouldWriteTsIntoFile, gen_if_write_is_err_ts, mb_write_ts_into_file,
};
use naming::{
    ColumnSc, DimsIesSc, DimsSc, ErSc, IncrSc, PubSc, QuerySc, SelfSc, VSc,
    param::{PgJsonWhereSelfUcc, PgTypeWhereSelfUcc},
};
use optml::Optml;
use panic_location::panic_location;
use pg_crud_macros_common::{
    AddOprtrUndrscr, ColumnParamUndrscr, Import, IncrParamUndrscr, IsQbMut, PgJsonFilter,
    PgTypeFilter, PgTypeOrPgJson, gen_impl_dflt_opt_some_vec_one_el_ts,
    impl_pg_type_where_filter_for_ident_ts,
};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde_json::from_str;
use std::fmt::Display;
use token_patterns::{
    CoreDefault, PgCrudCommonDfltOptSomeVecOneEl, PgCrudCommonDfltOptSomeVecOneElCall,
};
#[proc_macro]
pub fn gen_where_filters(input_ts: Ts) -> Ts {
    #[derive(Clone, Optml)]
    enum Generic {
        False,
        True { mb_extra_traits_ts: Option<Ts2> },
    }
    enum FilterType {
        PgJson,
        PgType,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, Optml)]
    enum PgTypePtrn {
        Stdrt,
        ArrDim1,
        ArrDim2,
        ArrDim3,
        ArrDim4,
    }
    impl TryFrom<&PgTypePtrn> for DimNbr {
        type Error = ();
        fn try_from(v: &PgTypePtrn) -> Result<Self, Self::Error> {
            match &v {
                PgTypePtrn::Stdrt => Err(()),
                PgTypePtrn::ArrDim1 => Ok(Self::One),
                PgTypePtrn::ArrDim2 => Ok(Self::Two),
                PgTypePtrn::ArrDim3 => Ok(Self::Three),
                PgTypePtrn::ArrDim4 => Ok(Self::Four),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, Optml)]
    enum DimNbr {
        One,
        Two,
        Three,
        Four,
    }
    impl DimNbr {
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
        Stdrt,
        ArrDim,
    }
    impl PgTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Stdrt => "",
                Self::ArrDim => "{}",
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, Optml)]
    struct GenWhereFiltersConfig {
        pg_types_write_into_file: ShouldWriteTsIntoFile,
        pg_json_write_into_file: ShouldWriteTsIntoFile,
        whole_write_into_file: ShouldWriteTsIntoFile,
    }
    panic_location();
    let gen_where_filters_config =
        from_str::<GenWhereFiltersConfig>(&input_ts.to_string()).expect("1217b73b");
    let import = Import::PgCrudCommon;
    let t_ts = quote! {T};
    let t_ann_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let pub_v_t_ts = quote! {pub #VSc: T};
    let unsigned_part_of_i32_ts = quote! {#import::UnsignedPartOfI32};
    let not_zero_unsigned_part_of_i32_ts = quote! {#import::NotZeroUnsignedPartOfI32};
    let v_dflt_opt_some_vec_one_el_ts = quote! {
        #VSc: #PgCrudCommonDfltOptSomeVecOneElCall
    };
    let gen_struct_ts = |filter_init_with_try_new_result_is_ok: bool,
                         generic: &Generic,
                         ident: &dyn ToTokens,
                         struct_extra_fields_ts: &dyn ToTokens| {
        let mb_pub_ts: &dyn ToTokens = if filter_init_with_try_new_result_is_ok {
            &proc_macro2_ts_new
        } else {
            &PubSc
        };
        DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_partial_eq()
            .d_serde_serialize()
            .d_serde_deserialize_if(if filter_init_with_try_new_result_is_ok {
                DSerdeDeserialize::False
            } else {
                DSerdeDeserialize::True
            })
            .d_schemars_json_schema()
            .build_struct(
                &ident,
                &match &generic {
                    Generic::False => proc_macro2_ts_new.clone(),
                    Generic::True { mb_extra_traits_ts } => {
                        mb_extra_traits_ts.as_ref().map_or_else(
                            || quote! {<#t_ts>},
                            |v_d05f3d4f| quote! {<#t_ts: #v_d05f3d4f>},
                        )
                    }
                },
                &quote::quote! {{
                    #mb_pub_ts oprtr: #import::Oprtr,
                    #struct_extra_fields_ts
                }},
            )
    };
    let gen_impl_dflt_opt_some_vec_one_el_ts =
        |generic: &Generic, ident: &dyn ToTokens, ts: &dyn ToTokens| {
            gen_impl_dflt_opt_some_vec_one_el_ts(
            &match &generic {
                Generic::False => Ts2::new(),
                Generic::True { mb_extra_traits_ts } => {
                    mb_extra_traits_ts.as_ref().map_or_else(
                        || quote! {<T: #PgCrudCommonDfltOptSomeVecOneEl>},
                        |v_29913af7| quote! {<T: #v_29913af7 + #PgCrudCommonDfltOptSomeVecOneEl>}
                    )
                }
            },
            &Import::PgCrudCommon,
            &ident,
            match &generic {
                Generic::False => &proc_macro2_ts_new,
                Generic::True { .. } => &t_ann_generic_ts,
            },
            &quote! {
                Self {
                    oprtr: #PgCrudCommonDfltOptSomeVecOneElCall,
                    #ts
                }
            },
        )
        };
    let gen_impl_pg_type_where_filter_ts =
        |filter_type: &FilterType,
         generic: &Generic,
         ident: &dyn ToTokens,
         incr_param_undrscr: &IncrParamUndrscr,
         add_oprtr_undrscr: &AddOprtrUndrscr,
         qp_ts: &dyn ToTokens,
         is_qb_mut: &IsQbMut,
         qb_ts: &dyn ToTokens| {
            impl_pg_type_where_filter_for_ident_ts(
                &{
                    let mb_t_extra_traits_for_pg_type_where_filter_ts: &dyn ToTokens =
                        match &generic {
                            Generic::False => &proc_macro2_ts_new,
                            Generic::True { mb_extra_traits_ts } => {
                                let send_and_lt_ts = quote! {Send + 'lt};
                                let ser_ts = quote! {serde::Serialize};
                                let ts = match (&filter_type, &mb_extra_traits_ts) {
                                    (FilterType::PgType, Some(v)) => &quote! {#v + #send_and_lt_ts},
                                    (FilterType::PgType, None) => &send_and_lt_ts,
                                    (FilterType::PgJson, Some(v)) => {
                                        &quote! {#v + #ser_ts + #send_and_lt_ts}
                                    }
                                    (FilterType::PgJson, None) => {
                                        &quote! {#ser_ts + #send_and_lt_ts}
                                    }
                                };
                                &quote! {, T: #ts}
                            }
                        };
                    quote! {<'lt #mb_t_extra_traits_for_pg_type_where_filter_ts>}
                },
                &ident,
                &match &generic {
                    Generic::False => &proc_macro2_ts_new,
                    Generic::True { .. } => &t_ann_generic_ts,
                },
                incr_param_undrscr,
                &ColumnParamUndrscr::False,
                add_oprtr_undrscr,
                &qp_ts,
                is_qb_mut,
                &qb_ts,
                &Import::PgCrudCommon,
            )
        };
    let add_regex_case_and_v_dcl_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            pub regex_case: RegexCase,
            pub #VSc: RegexRegex
        }
    };
    let add_regex_case_and_v_dflt_init_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            regex_case: #PgCrudCommonDfltOptSomeVecOneElCall,
            #v_dflt_opt_some_vec_one_el_ts
        }
    };
    let gen_match_incr_checked_add_one_init_ts = |ts: &dyn ToTokens| {
        quote! {
            let #ts = match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                Ok(v_25d59e01) => v_25d59e01,
                Err(#ErSc) => {
                    return Err(#ErSc);
                },
            };
        }
    };
    let v_match_incr_checked_add_one_init_ts = gen_match_incr_checked_add_one_init_ts(&VSc);
    let self_oprtr_to_qp_ts = quote! {&#SelfSc.oprtr.to_qp(add_oprtr),};
    let gen_ts_dbf9de6b =
        |v: &dyn Display, mb_dims_ies_init_ts: &dyn ToTokens, mb_extra_params_ts: &dyn ToTokens| {
            let format_ts = dq_ts(&v);
            quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #ColumnSc,
                    #mb_extra_params_ts
                    #SelfSc.regex_case.postgreql_syntax(),
                    #VSc
                ))
            }
        };
    let if_let_err_query_try_bind_self_v_to_string_ts = quote! {
        if let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc.to_string()) {
            return Err(#ErSc.to_string());
        }
        Ok(#QuerySc)
    };
    let if_let_err_query_try_bind_self_v_ts = quote! {
        if let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc) {
            return Err(#ErSc.to_string());
        }
    };
    let qb_one_v_ts = quote! {
        #if_let_err_query_try_bind_self_v_ts
        Ok(#QuerySc)
    };
    let generic_false = Generic::False;
    let generic_true_none = Generic::True {
        mb_extra_traits_ts: None,
    };
    let generic_true_debug_partial_eq_clone = Generic::True {
        mb_extra_traits_ts: Some(quote! {std::fmt::Debug + PartialEq + Clone}),
    };
    let generic_true_debug_partial_eq_partial_ord_clone_type_encode = Generic::True {
        mb_extra_traits_ts: Some(quote! {
            std::fmt::Debug
            + PartialEq
            + PartialOrd
            + Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
        }),
    };
    let pub_v_between_t_ts = quote! {pub #VSc: Between<T>};
    let query_self_v_qb_ts = quote! {
        match #SelfSc.#VSc.qb(#QuerySc) {
            Ok(v_f6d31bdd) => {
                #QuerySc = v_f6d31bdd;
            },
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
        Ok(#QuerySc)
    };
    let pg_type_ptrn_stdrt = PgTypePtrn::Stdrt;
    let pg_type_ptrn_arr_dim1 = PgTypePtrn::ArrDim1;
    let pg_type_ptrn_arr_dim2 = PgTypePtrn::ArrDim2;
    let pg_type_ptrn_arr_dim3 = PgTypePtrn::ArrDim3;
    let pg_type_ptrn_arr_dim4 = PgTypePtrn::ArrDim4;
    let gen_pub_dims_bounded_vec_ts =
        |vec_length_ts: &dyn ToTokens, kind_of_unsigned_part_of_i32: &KindOfUnsignedPartOfI32| {
            quote! {pub #DimsSc: BoundedVec<#import::#kind_of_unsigned_part_of_i32, #vec_length_ts>}
        };
    let gen_ident_match_field_fn_ok_v_return_err_ts =
        |ident_ts: &dyn ToTokens, field_ts: &dyn ToTokens, fn_ts: &dyn ToTokens| {
            quote! {
                let #ident_ts = match self.#field_ts.#fn_ts(#IncrSc, #ColumnSc, add_oprtr) {
                    Ok(v_0a22ee9a) => v_0a22ee9a,
                    Err(#ErSc) => {
                        return Err(#ErSc);
                    }
                };
            }
        };
    let v_match_self_v_qp_init_ts =
        gen_ident_match_field_fn_ok_v_return_err_ts(&VSc, &VSc, &quote! {qp});
    let dims_dflt_init_ts = quote! {
        #DimsSc: #PgCrudCommonDfltOptSomeVecOneElCall
    };
    let dims_dflt_init_comma_ts = quote! {#dims_dflt_init_ts,};
    let query_self_dims_qb_query_ts = quote! {
        match #SelfSc.#DimsSc.qb(#QuerySc) {
            Ok(v_ed6f1157) => {
                #QuerySc = v_ed6f1157;
            }
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
    };
    let dims_ies_comma_ts = quote! {#DimsIesSc,};
    let gen_mb_dims_dcl_pub_v_t_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            #pub_v_t_ts
        }
    };
    let gen_mb_dims_dflt_init_v_dflt_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            #v_dflt_opt_some_vec_one_el_ts
        }
    };
    let is_qb_mut_true = IsQbMut::True;
    let is_qb_mut_false = IsQbMut::False;
    let gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts = |dim_nbr: &DimNbr| {
        let pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_ts =
            gen_pub_dims_bounded_vec_ts(&dim_nbr.dim_ts(), &KindOfUnsignedPartOfI32::CanNotBeZero);
        quote! {#pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_ts,}
    };
    let gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts = |dim_nbr: &DimNbr| {
        let pub_dims_bounded_vec_unsigned_part_of_i32_ts =
            gen_pub_dims_bounded_vec_ts(&dim_nbr.dim_ts(), &KindOfUnsignedPartOfI32::CanBeZero);
        quote! {#pub_dims_bounded_vec_unsigned_part_of_i32_ts,}
    };
    let gen_pg_type_dims_helpers =
        |pg_type_ptrn: &PgTypePtrn, pg_type_or_pg_json: &PgTypeOrPgJson| {
            DimNbr::try_from(pg_type_ptrn).map_or_else(
                |()| {
                    (
                        Ts2::new(),
                        Ts2::new(),
                        Ts2::new(),
                        PgTypeKind::Stdrt,
                        Ts2::new(),
                        Ts2::new(),
                    )
                },
                |dim_nbr| {
                    (
                        match &pg_type_or_pg_json {
                            PgTypeOrPgJson::PgType => {
                                gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(
                                    &dim_nbr,
                                )
                            }
                            PgTypeOrPgJson::PgJson => {
                                gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts(&dim_nbr)
                            }
                        },
                        dims_dflt_init_comma_ts.clone(),
                        gen_ident_match_field_fn_ok_v_return_err_ts(
                            &DimsIesSc,
                            &DimsSc,
                            &match &pg_type_or_pg_json {
                                PgTypeOrPgJson::PgType => quote! {pg_type_qp},
                                PgTypeOrPgJson::PgJson => quote! {pg_json_qp},
                            },
                        ),
                        PgTypeKind::ArrDim,
                        dims_ies_comma_ts.clone(),
                        query_self_dims_qb_query_ts.clone(),
                    )
                },
            )
        };
    let pg_type_ts = {
        let gen_filters_ts = |filter: &PgTypeFilter| {
            let ident = PgTypeWhereSelfUcc::from_display(&filter);
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_opt_some_vec_one_el_extra_fields_ts,
                incr_param_undrscr,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = {
                let sqlx_type_pg_encode_ts = quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let generic_true_type_encode = Generic::True {
                    mb_extra_traits_ts: Some(sqlx_type_pg_encode_ts.clone()),
                };
                let pub_v_pg_type_not_empty_unique_vec_t_ts =
                    quote! {pub #VSc: PgTypeNotEmptyUniqueVec<T>};
                let gen_pg_type_dims_helpers_pg_type = |pg_type_ptrn: &PgTypePtrn| {
                    gen_pg_type_dims_helpers(pg_type_ptrn, &PgTypeOrPgJson::PgType)
                };
                let gen_32abfefc_ts =
                    |pg_type_ptrn: &PgTypePtrn,
                     gen_format_h_str: &dyn Fn(&PgTypeKind) -> String| {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            pg_type_kind,
                            mb_extra_params_ts,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                        (
                            generic_true_type_encode.clone(),
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrParamUndrscr::False,
                            {
                                let format_ts = dq_ts(&gen_format_h_str(&pg_type_kind));
                                quote! {
                                    #mb_dims_ies_init_ts
                                    #v_match_incr_checked_add_one_init_ts
                                    Ok(format!(
                                        #format_ts,
                                        #self_oprtr_to_qp_ts
                                        #ColumnSc,
                                        #mb_extra_params_ts
                                        #VSc
                                    ))
                                }
                            },
                            is_qb_mut_true,
                            quote! {
                                #mb_dims_qb_ts
                                #qb_one_v_ts
                            },
                        )
                    };
                let gen_a2ca84d5_ts = |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn Display| {
                    gen_32abfefc_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!("{{}}({{}}{} {oprtr} ${{}})", pg_type_kind.format_argument())
                    })
                };
                let gen_greater_than_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &">");
                let gen_between_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_between_t_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrParamUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} {{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_self_v_qp_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #query_self_v_qb_ts
                        },
                    )
                };
                let gen_in_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        Generic::True {
                            mb_extra_traits_ts: Some(
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_ts},
                            ),
                        },
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_pg_type_not_empty_unique_vec_t_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrParamUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote! {acc_14596a52, "${v_daedba9c},"},
                                &quote! {panic!("87f47f75");},
                            );
                            quote! {
                                #mb_dims_ies_init_ts
                                let #VSc = {
                                    let mut acc_14596a52 = String::default();
                                    for _ in #SelfSc.#VSc.to_vec() {
                                        match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                            Ok(v_daedba9c) => {
                                                #if_write_is_err_ts
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            },
                                        }
                                    }
                                    let _ = acc_14596a52.pop();
                                    acc_14596a52
                                };
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            for el in #SelfSc.#VSc.into_vec() {
                                if let Err(#ErSc) = #QuerySc.try_bind(el) {
                                    return Err(#ErSc.to_string());
                                }
                            }
                            Ok(#QuerySc)
                        },
                    )
                };
                let gen_regex_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        add_regex_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                        add_regex_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                        IncrParamUndrscr::False,
                        gen_ts_dbf9de6b(
                            &format!("{{}}({{}}{} {{}} ${{}})", pg_type_kind.format_argument()),
                            &mb_dims_ies_init_ts,
                            &mb_extra_params_ts,
                        ),
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #if_let_err_query_try_bind_self_v_to_string_ts
                        },
                    )
                };
                let gen_before_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_true_type_encode.clone(),
                        gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrParamUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} < ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #qb_one_v_ts
                        },
                    )
                };
                let gen_1fa0bbf4_ts = |pg_type_ptrn: &PgTypePtrn, pg_syntax: &dyn Display| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        match &pg_type_ptrn {
                            PgTypePtrn::Stdrt => IncrParamUndrscr::True,
                            PgTypePtrn::ArrDim1
                            | PgTypePtrn::ArrDim2
                            | PgTypePtrn::ArrDim3
                            | PgTypePtrn::ArrDim4 => IncrParamUndrscr::False,
                        },
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} {pg_syntax})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                ))
                            }
                        },
                        match &pg_type_ptrn {
                            PgTypePtrn::Stdrt => is_qb_mut_false,
                            PgTypePtrn::ArrDim1
                            | PgTypePtrn::ArrDim2
                            | PgTypePtrn::ArrDim3
                            | PgTypePtrn::ArrDim4 => is_qb_mut_true,
                        },
                        quote! {
                            #mb_dims_qb_ts
                            Ok(#QuerySc)
                        },
                    )
                };
                let gen_current_date_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_1fa0bbf4_ts(pg_type_ptrn, &"= current_date");
                let gen_greater_than_current_date_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_1fa0bbf4_ts(pg_type_ptrn, &"> current_date");
                let gen_current_timestamp_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_1fa0bbf4_ts(pg_type_ptrn, &"= current_timestamp")
                };
                let gen_greater_than_current_timestamp_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_1fa0bbf4_ts(pg_type_ptrn, &"> current_timestamp")
                };
                let gen_current_time_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_1fa0bbf4_ts(pg_type_ptrn, &"= current_time");
                let gen_greater_than_current_time_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_1fa0bbf4_ts(pg_type_ptrn, &"> current_time");
                let gen_equal_to_encoded_string_representation_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        quote! {
                            #mb_dims_dcl_ts
                            pub encode_format: EncodeFormat,
                            pub encoded_string_representation: String,
                        },
                        quote! {
                            #mb_dims_dflt_init_ts
                            encode_format: #PgCrudCommonDfltOptSomeVecOneElCall,
                            encoded_string_representation: #CoreDefault
                        },
                        IncrParamUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    &#SelfSc.encode_format,
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            if let Err(#ErSc) = #QuerySc.try_bind(self.encoded_string_representation) {
                                return Err(#ErSc.to_string());
                            }
                            Ok(#QuerySc)
                        },
                    )
                };
                let gen_find_ranges_within_given_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"<@");
                let gen_find_ranges_that_fully_contain_the_given_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"@>");
                let gen_strictly_to_left_of_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"&<");
                let gen_strictly_to_right_of_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"&>");
                let gen_overlap_with_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"&&");
                let gen_adjacent_with_range_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_a2ca84d5_ts(pg_type_ptrn, &"-|-");
                let pub_v_not_zero_unsigned_part_of_i32_dcl_ts =
                    quote! {pub #VSc: #not_zero_unsigned_part_of_i32_ts};
                let gen_length_filter_pattern_ts = |oprtr: &dyn Display| {
                    (
                        generic_false.clone(),
                        pub_v_not_zero_unsigned_part_of_i32_dcl_ts.clone(),
                        v_dflt_opt_some_vec_one_el_ts.clone(),
                        IncrParamUndrscr::False,
                        {
                            let format_ts =
                                dq_ts(&format!("{{}}(arr_length({{}}, 1) {oprtr} ${{}})"));
                            quote! {
                                match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                    Ok(v_f7988de8) => Ok(format!(#format_ts, &self.oprtr.to_qp(add_oprtr), #ColumnSc, v_f7988de8)),
                                    Err(#ErSc) => Err(#ErSc),
                                }
                            }
                        },
                        is_qb_mut_true,
                        qb_one_v_ts.clone(),
                    )
                };
                let gen_included_lower_bound_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_32abfefc_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(lower({{}}{}) = ${{}})",
                            pg_type_kind.format_argument()
                        )
                    })
                };
                let gen_excluded_upper_bound_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_32abfefc_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(upper({{}}{}) = ${{}})",
                            pg_type_kind.format_argument()
                        )
                    })
                };
                let gen_greater_than_included_lower_bound_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_32abfefc_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(lower({{}}{}) > ${{}})",
                            pg_type_kind.format_argument()
                        )
                    })
                };
                let gen_greater_than_excluded_upper_bound_ts = |pg_type_ptrn: &PgTypePtrn| {
                    gen_32abfefc_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}(upper({{}}{}) > ${{}})",
                            pg_type_kind.format_argument()
                        )
                    })
                };
                let gen_range_length_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = DimNbr::try_from(pg_type_ptrn).map_or_else(
                        |()| {
                            (
                                Ts2::new(),
                                Ts2::new(),
                                Ts2::new(),
                                PgTypeKind::Stdrt,
                                quote! {#ColumnSc,},
                                Ts2::new(),
                            )
                        },
                        |dim_nbr| {
                            (
                                gen_pub_dims_bounded_vec_not_zero_unsigned_part_of_i32_comma_ts(
                                    &dim_nbr,
                                ),
                                dims_dflt_init_comma_ts.clone(),
                                {
                                    let (dims_ies1_ts, dims_ies2_ts) = {
                                        let gen_ts = |ts: &dyn ToTokens| {
                                            gen_ident_match_field_fn_ok_v_return_err_ts(
                                                &ts,
                                                &DimsSc,
                                                &quote! {pg_type_qp},
                                            )
                                        };
                                        (gen_ts(&quote! {dims_ies1}), gen_ts(&quote! {dims_ies2}))
                                    };
                                    quote! {
                                        #dims_ies1_ts
                                        #dims_ies2_ts
                                    }
                                },
                                PgTypeKind::ArrDim,
                                quote! {
                                    dims_ies1,
                                    column,
                                    dims_ies2,
                                },
                                quote! {
                                    match #SelfSc.#DimsSc.clone().qb(#QuerySc) {
                                        Ok(v_6cb14cdc) => {
                                            #QuerySc = v_6cb14cdc;
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                    #query_self_dims_qb_query_ts
                                },
                            )
                        },
                    );
                    (
                        Generic::False,
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_not_zero_unsigned_part_of_i32_dcl_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrParamUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #qb_one_v_ts
                        },
                    )
                };
                let gen_ts_c7811da6 =
                    |mb_dims_ies_init_ts: &dyn ToTokens, format_ts: &dyn ToTokens| {
                        quote! {
                            #mb_dims_ies_init_ts
                            let oprtr = <T as #import::PgTypeEqualOprtr>::oprtr(&#SelfSc.#VSc);
                            let oprtr_query_str = oprtr.to_query_str();
                            Ok(format!(
                                #format_ts,
                                #self_oprtr_to_qp_ts
                                #ColumnSc,
                                match oprtr {
                                    #import::EqualOprtr::Equal => {
                                        #v_match_incr_checked_add_one_init_ts
                                        format!("{oprtr_query_str} ${v}")
                                    },
                                    #import::EqualOprtr::IsNull => oprtr_query_str.to_owned(),
                                }
                            ))
                        }
                    };
                let gen_ts_eeee6e79 = |ts: &dyn ToTokens| {
                    quote! {
                        #ts
                        if let #import::EqualOprtr::Equal = &<T as #import::PgTypeEqualOprtr>::oprtr(&#SelfSc.#VSc) {
                            #if_let_err_query_try_bind_self_v_ts
                        }
                        Ok(#QuerySc)
                    }
                };
                match &filter {
                    PgTypeFilter::Equal { .. } => {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            _,
                            _,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(&pg_type_ptrn_stdrt);
                        (
                            Generic::True {
                                mb_extra_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqualOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrParamUndrscr::False,
                            gen_ts_c7811da6(&mb_dims_ies_init_ts, &quote! {"{}({} {})"}),
                            is_qb_mut_true,
                            gen_ts_eeee6e79(&mb_dims_qb_ts),
                        )
                    }
                    PgTypeFilter::DimOneEqual { .. } => {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            _,
                            _,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(&pg_type_ptrn_arr_dim1);
                        (
                            Generic::True {
                                mb_extra_traits_ts: Some(
                                    quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqualOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrParamUndrscr::False,
                            gen_ts_c7811da6(&mb_dims_ies_init_ts, &quote! {"{}({}{dims_ies} {})"}),
                            is_qb_mut_true,
                            gen_ts_eeee6e79(&mb_dims_qb_ts),
                        )
                    }
                    PgTypeFilter::GreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneGreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::Between { .. } => gen_between_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneBetween { .. } => gen_between_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::Regex => gen_regex_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneRegex => gen_regex_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::Before { .. } => gen_before_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneBefore { .. } => gen_before_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::CurrentDate => gen_current_date_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneCurrentDate => gen_current_date_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::GreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentDate => {
                        gen_greater_than_current_date_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::CurrentTimestamp => gen_current_timestamp_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneCurrentTimestamp => {
                        gen_current_timestamp_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::GreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentTimestamp => {
                        gen_greater_than_current_timestamp_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::CurrentTime => gen_current_time_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneCurrentTime => gen_current_time_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFilter::GreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneGreaterThanCurrentTime => {
                        gen_greater_than_current_time_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::DimOneLengthEqual => gen_length_filter_pattern_ts(&"="),
                    PgTypeFilter::DimOneLengthGreaterThan => gen_length_filter_pattern_ts(&">"),
                    PgTypeFilter::EqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneEqualToEncodedStringRepresentation => {
                        gen_equal_to_encoded_string_representation_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::FindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneFindRangesWithinGivenRange { .. } => {
                        gen_find_ranges_within_given_range_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_find_ranges_that_fully_contain_the_given_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_find_ranges_that_fully_contain_the_given_range_ts(
                            &pg_type_ptrn_arr_dim1,
                        )
                    }
                    PgTypeFilter::StrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneStrictlyToLeftOfRange { .. } => {
                        gen_strictly_to_left_of_range_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::StrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneStrictlyToRightOfRange { .. } => {
                        gen_strictly_to_right_of_range_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::IncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneIncludedLowerBound { .. } => {
                        gen_included_lower_bound_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::ExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneExcludedUpperBound { .. } => {
                        gen_excluded_upper_bound_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::GreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneGreaterThanIncludedLowerBound { .. } => {
                        gen_greater_than_included_lower_bound_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::GreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneGreaterThanExcludedUpperBound { .. } => {
                        gen_greater_than_excluded_upper_bound_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::OverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneOverlapWithRange { .. } => {
                        gen_overlap_with_range_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::AdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFilter::DimOneAdjacentWithRange { .. } => {
                        gen_adjacent_with_range_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFilter::RangeLength => gen_range_length_ts(&pg_type_ptrn_stdrt),
                    PgTypeFilter::DimOneRangeLength => gen_range_length_ts(&pg_type_ptrn_arr_dim1),
                }
            };
            let struct_ts = gen_struct_ts(false, &generic, &ident, &struct_extra_fields_ts);
            let impl_dflt_opt_some_vec_one_el_ts = gen_impl_dflt_opt_some_vec_one_el_ts(
                &generic,
                &ident,
                &impl_dflt_opt_some_vec_one_el_extra_fields_ts,
            );
            let impl_pg_type_where_filter_ts = gen_impl_pg_type_where_filter_ts(
                &FilterType::PgType,
                &generic,
                &ident,
                &incr_param_undrscr,
                &AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_dflt_opt_some_vec_one_el_ts
                #impl_pg_type_where_filter_ts
            };
            gend
        };
        let filter_arr_ts = PgTypeFilter::into_arr().map(|el| gen_filters_ts(&el));
        let gend = quote! {#(#filter_arr_ts)*};
        mb_write_ts_into_file(
            gen_where_filters_config.pg_types_write_into_file,
            "gen_where_filters_pg_types",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let pg_json_ts = {
        let gen_filters_ts = |filter: &PgJsonFilter| {
            let ident = PgJsonWhereSelfUcc::from_display(&filter);
            let pub_v_pg_json_not_empty_unique_vec_t_ts = quote! {
                pub #VSc: PgJsonNotEmptyUniqueVec<T>
            };
            let qb_sqlx_types_json_self_v_ts = quote! {
                if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(#SelfSc.#VSc)) {
                    return Err(#ErSc.to_string());
                }
                Ok(#QuerySc)
            };
            let gen_pg_json_dims_helpers = |pg_type_ptrn: &PgTypePtrn| {
                gen_pg_type_dims_helpers(pg_type_ptrn, &PgTypeOrPgJson::PgJson)
            };
            let gen_1763ccf3_ts =
                |pg_type_ptrn: &PgTypePtrn, gen_format_h_str: &dyn Fn(&PgTypeKind) -> String| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_params_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                    (
                        generic_true_none.clone(),
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_t_ts
                        },
                        quote! {
                            #mb_dims_dflt_init_ts
                            #v_dflt_opt_some_vec_one_el_ts
                        },
                        {
                            let format_ts = dq_ts(&gen_format_h_str(&pg_type_kind));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColumnSc,
                                    #mb_extra_params_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #qb_sqlx_types_json_self_v_ts
                        },
                    )
                };
            let gen_7cc8e29b_ts = |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn Display| {
                gen_1763ccf3_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!("{{}}({{}}{} {oprtr} ${{}})", pg_type_kind.format_argument())
                })
            };
            let gen_equal_ts = |pg_type_ptrn: &PgTypePtrn| gen_7cc8e29b_ts(pg_type_ptrn, &"=");
            let gen_all_els_equal_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_1763ccf3_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_ts_e527a806 = |format_ts: &dyn ToTokens, mb_extra_params_ts: &dyn ToTokens| {
                quote! {
                    Ok(format!(
                        #format_ts,
                        #self_oprtr_to_qp_ts
                        #ColumnSc,
                        #mb_extra_params_ts
                        #VSc
                    ))
                }
            };
            let gen_ts_ae2fa44d = |pg_type_ptrn: &PgTypePtrn, op: &dyn Display| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_false.clone(),
                    quote! {
                        #mb_dims_dcl_ts
                        pub #VSc: #unsigned_part_of_i32_ts
                    },
                    quote! {
                        #mb_dims_dflt_init_ts
                        #VSc: #CoreDefault
                    },
                    {
                        let format_ts = dq_ts(&format!(
                            "{{}}(jsonb_array_length({{}}{}) {op} ${{}})",
                            pg_type_kind.format_argument()
                        ));
                        let ts = gen_ts_e527a806(&format_ts, &mb_extra_params_ts);
                        quote! {
                            #mb_dims_ies_init_ts
                            #v_match_incr_checked_add_one_init_ts
                            #ts
                        }
                    },
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #qb_one_v_ts
                    },
                )
            };
            let gen_length_equal_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_ts_ae2fa44d(pg_type_ptrn, &"=");
            let gen_length_greater_than_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_ts_ae2fa44d(pg_type_ptrn, &">");
            let gen_greater_than_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_7cc8e29b_ts(pg_type_ptrn, &">");
            let gen_contains_el_greater_than_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_1763ccf3_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_all_els_greater_than_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_1763ccf3_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_between_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    quote! {
                        #mb_dims_dcl_ts
                        #pub_v_between_t_ts
                    },
                    gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                    {
                        let ts: &dyn ToTokens = match pg_type_ptrn {
                            PgTypePtrn::Stdrt => &quote! {
                                let #VSc = match self.#VSc.qp(
                                    incr,
                                    column,
                                    add_oprtr
                                ) {
                                    Ok(v_cc8dda2f) => v_cc8dda2f,
                                    Err(er) => {
                                        return Err(er);
                                    }
                                };
                            },
                            PgTypePtrn::ArrDim1
                            | PgTypePtrn::ArrDim2
                            | PgTypePtrn::ArrDim3
                            | PgTypePtrn::ArrDim4 => &v_match_incr_checked_add_one_init_ts,
                        };
                        let ts0 = gen_ts_e527a806(
                            &dq_ts(&format!(
                                "{{}}({{}}{} {{}})",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_params_ts,
                        );
                        quote! {
                            #mb_dims_ies_init_ts
                            #ts
                            #ts0
                        }
                    },
                    is_qb_mut_true,
                    {
                        let ts: &dyn ToTokens = match pg_type_ptrn {
                            PgTypePtrn::Stdrt => &quote! {
                                match self.#VSc.qb(query) {
                                    Ok(v_b3d3fd36) => {
                                        query = v_b3d3fd36;
                                    },
                                    Err(er) => {
                                        return Err(er.to_string());
                                    }
                                }
                                Ok(query)
                            },
                            PgTypePtrn::ArrDim1
                            | PgTypePtrn::ArrDim2
                            | PgTypePtrn::ArrDim3
                            | PgTypePtrn::ArrDim4 => &qb_sqlx_types_json_self_v_ts,
                        };
                        quote! {
                            #mb_dims_qb_ts
                            #ts
                        }
                    },
                )
            };
            let gen_in_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_clone.clone(),
                    quote! {
                        #mb_dims_dcl_ts
                        #pub_v_pg_json_not_empty_unique_vec_t_ts
                    },
                    gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                    {
                        let v_init_ts = gen_ident_match_field_fn_ok_v_return_err_ts(
                            &VSc,
                            &VSc,
                            &quote! {qp_one_by_one},
                        );
                        let ts = gen_ts_e527a806(
                            &dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_params_ts,
                        );
                        quote! {
                            #mb_dims_ies_init_ts
                            #v_init_ts
                            #ts
                        }
                    },
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        match #SelfSc.#VSc.qb_one_by_one(#QuerySc) {
                            Ok(v_c79b2256) => {
                                #QuerySc = v_c79b2256;
                            }
                            Err(#ErSc) => {
                                return Err(#ErSc);
                            }
                        }
                        Ok(#QuerySc)
                    },
                )
            };
            let gen_regex_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = DimNbr::try_from(pg_type_ptrn).map_or_else(
                    |()| {
                        (
                            Ts2::new(),
                            Ts2::new(),
                            Ts2::new(),
                            PgTypeKind::Stdrt,
                            Ts2::new(),
                            Ts2::new(),
                        )
                    },
                    |dim_nbr| {
                        (
                            gen_pub_dims_bounded_vec_unsigned_part_of_i32_comma_ts(&dim_nbr),
                            dims_dflt_init_comma_ts.clone(),
                            {
                                let dims_ies_init_ts = gen_ident_match_field_fn_ok_v_return_err_ts(
                                    &DimsIesSc,
                                    &DimsSc,
                                    &quote! {pg_json_qp_minus_one},
                                );
                                let last_dims_i_intialization_ts =
                                    gen_match_incr_checked_add_one_init_ts(&quote! {last_dims_i});
                                quote! {
                                    #dims_ies_init_ts
                                    #last_dims_i_intialization_ts
                                }
                            },
                            PgTypeKind::ArrDim,
                            quote! {
                                last_dims_i,
                                #DimsIesSc,
                            },
                            query_self_dims_qb_query_ts.clone(),
                        )
                    },
                );
                (
                    generic_false.clone(),
                    add_regex_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                    add_regex_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                    gen_ts_dbf9de6b(
                        &format!(
                            "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                            match &pg_type_kind {
                                PgTypeKind::Stdrt => "",
                                PgTypeKind::ArrDim => "{}->>${}",
                            }
                        ),
                        &mb_dims_ies_init_ts,
                        &mb_extra_params_ts,
                    ),
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #if_let_err_query_try_bind_self_v_to_string_ts
                    },
                )
            };
            let gen_contains_el_regex_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_false.clone(),
                    add_regex_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                    add_regex_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                    gen_ts_dbf9de6b(
                        &format!(
                            //todo test it properly using all strange string vrts
                            "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') {{}} ${{}}))",
                            // "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) {{}} ${{}}))",
                            pg_type_kind.format_argument()
                        ),
                        &mb_dims_ies_init_ts,
                        &mb_extra_params_ts,
                    ),
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #if_let_err_query_try_bind_self_v_to_string_ts
                    },
                )
            };
            let gen_all_els_regex_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_false.clone(),
                    add_regex_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                    add_regex_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                    gen_ts_dbf9de6b(
                        &format!(
                            //todo test it properly using all strange string vrts
                            "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el #>> '{{{{}}}}') !{{}} ${{}}))",
                            // "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{{}} ${{}}))",
                            pg_type_kind.format_argument()
                        ),
                        &mb_dims_ies_init_ts,
                        &mb_extra_params_ts,
                    ),
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #if_let_err_query_try_bind_self_v_to_string_ts
                    },
                )
            };
            let gen_contains_all_els_of_arr_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_clone.clone(),
                    quote! {
                        #mb_dims_dcl_ts
                        #pub_v_pg_json_not_empty_unique_vec_t_ts
                    },
                    quote! {
                        #mb_dims_dflt_init_ts
                        #v_dflt_opt_some_vec_one_el_ts
                    },
                    {
                        let ts = gen_ts_e527a806(
                            &dq_ts(&format!(
                                "{{}}({{}}{} @> {{}})",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_params_ts,
                        );
                        quote! {
                            #mb_dims_ies_init_ts
                            #v_match_self_v_qp_init_ts
                            #ts
                        }
                    },
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #qb_sqlx_types_json_self_v_ts
                    },
                )
            };
            let gen_overlaps_with_arr_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_params_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_clone.clone(),
                    quote! {
                        #mb_dims_dcl_ts
                        #pub_v_pg_json_not_empty_unique_vec_t_ts
                    },
                    quote! {
                        #mb_dims_dflt_init_ts
                        #v_dflt_opt_some_vec_one_el_ts
                    },
                    {
                        let ts = gen_ts_e527a806(
                            &dq_ts(&format!(
                                "{{}}(exists (select 1 from jsonb_arr_els_text({{}}{}) as e1 join jsonb_arr_els_text({{}}) as e2 on e1.v = e2.v))",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_params_ts,
                        );
                        quote! {
                            #mb_dims_ies_init_ts
                            #v_match_self_v_qp_init_ts
                            #ts
                        }
                    },
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #qb_sqlx_types_json_self_v_ts
                    },
                )
            };
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_opt_some_vec_one_el_extra_fields_ts,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = match &filter {
                PgJsonFilter::Equal { .. } => gen_equal_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneEqual { .. } => gen_equal_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoEqual { .. } => gen_equal_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeEqual { .. } => gen_equal_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourEqual { .. } => gen_equal_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::AllElsEqual { .. } => gen_all_els_equal_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneAllElsEqual { .. } => {
                    gen_all_els_equal_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoAllElsEqual { .. } => {
                    gen_all_els_equal_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeAllElsEqual { .. } => {
                    gen_all_els_equal_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourAllElsEqual { .. } => {
                    gen_all_els_equal_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::LengthEqual => gen_length_equal_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneLengthEqual => gen_length_equal_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoLengthEqual => gen_length_equal_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeLengthEqual => gen_length_equal_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourLengthEqual => gen_length_equal_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::LengthGreaterThan => gen_length_greater_than_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourLengthGreaterThan => {
                    gen_length_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::GreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::ContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFilter::DimOneContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::AllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFilter::DimOneAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::Between { .. } => gen_between_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneBetween { .. } => gen_between_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoBetween { .. } => gen_between_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeBetween { .. } => gen_between_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourBetween { .. } => gen_between_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::Regex => gen_regex_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneRegex => gen_regex_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoRegex => gen_regex_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeRegex => gen_regex_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourRegex => gen_regex_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::ContainsElRegex => gen_contains_el_regex_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneContainsElRegex => {
                    gen_contains_el_regex_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoContainsElRegex => {
                    gen_contains_el_regex_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeContainsElRegex => {
                    gen_contains_el_regex_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourContainsElRegex => {
                    gen_contains_el_regex_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFilter::AllElsRegex => gen_all_els_regex_ts(&pg_type_ptrn_stdrt),
                PgJsonFilter::DimOneAllElsRegex => gen_all_els_regex_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFilter::DimTwoAllElsRegex => gen_all_els_regex_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFilter::DimThreeAllElsRegex => gen_all_els_regex_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFilter::DimFourAllElsRegex => gen_all_els_regex_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFilter::ContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFilter::DimOneContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim4)
                }
                // PgJsonFilter::ContainedInArr => todo!(),
                PgJsonFilter::OverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFilter::DimOneOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFilter::DimTwoOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFilter::DimThreeOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFilter::DimFourOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim4)
                }
            };
            let struct_ts = gen_struct_ts(false, &generic, &ident, &struct_extra_fields_ts);
            let impl_dflt_opt_some_vec_one_el_ts = gen_impl_dflt_opt_some_vec_one_el_ts(
                &generic,
                &ident,
                &impl_dflt_opt_some_vec_one_el_extra_fields_ts,
            );
            let impl_pg_type_where_filter_ts = gen_impl_pg_type_where_filter_ts(
                &FilterType::PgJson,
                &generic,
                &ident,
                &IncrParamUndrscr::False,
                &AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_dflt_opt_some_vec_one_el_ts
                #impl_pg_type_where_filter_ts
            };
            gend
        };
        let filter_arr_ts = PgJsonFilter::into_arr().map(|el| gen_filters_ts(&el));
        let gend = quote! {#(#filter_arr_ts)*};
        mb_write_ts_into_file(
            gen_where_filters_config.pg_json_write_into_file,
            "gen_where_filters_pg_json",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let gend = quote! {
        #pg_type_ts
        #pg_json_ts
    };
    mb_write_ts_into_file(
        gen_where_filters_config.whole_write_into_file,
        "gen_where_filters",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend.into()
}
