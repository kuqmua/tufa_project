use gen_quotes::dq_ts;
use macros_helpers::{DSerdeDeserialize, DTsBuilder};
use macros_helpers::{
    FormatWithCargofmt, ShouldWriteTsIntoFile, gen_if_write_is_err_ts, mb_write_ts_into_file,
};
use naming::{
    ColSc, DimsIesSc, DimsSc, ErSc, IncrSc, PubSc, QuerySc, SelfSc, VSc,
    prm::{PgJsonWhSelfUcc, PgTypeWhSelfUcc},
};
use optml::Optml;
use panic_loc::panic_loc;
use pg_crud_macros_cmn::{
    AddOprtrUndrscr, ColPrmUndrscr, Import, IncrPrmUndrscr, IsQbMut, PgJsonFlt, PgTypeFlt,
    PgTypeOrPgJson, gen_impl_dflt_some_one_el_ts, impl_pg_type_wh_flt_for_ident_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde_json::from_str;
use std::fmt::Display;
use token_patterns::{PgCrudCmnDfltSomeOneEl, PgCrudCmnDfltSomeOneElCall};
#[must_use]
pub fn gen_wh_flts(input_ts: &Ts2) -> Ts2 {
    #[derive(Clone, Optml)]
    enum Generic {
        False,
        True { mb_extra_traits_ts: Option<Ts2> },
    }
    enum FltType {
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
    struct GenWhFltsConfig {
        pg_types_write_into_file: ShouldWriteTsIntoFile,
        pg_json_write_into_file: ShouldWriteTsIntoFile,
        whole_write_into_file: ShouldWriteTsIntoFile,
    }
    panic_loc();
    let gen_wh_flts_config = match from_str::<GenWhFltsConfig>(&input_ts.to_string()) {
        Ok(v) => v,
        Err(er) => {
            let msg = format!("failed to parse GenWhFltsConfig: {er}");
            return quote! { compile_error!(#msg); };
        }
    };
    let import = Import::PgCrudCmn;
    let t_ts = quote! {T};
    let t_ann_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let pub_v_t_ts = quote! {pub #VSc: T};
    let unsigned_part_of_i32_ts = quote! {#import::UnsignedPartOfI32};
    let not_zero_unsigned_part_of_i32_ts = quote! {#import::NotZeroUnsignedPartOfI32};
    let v_dflt_some_one_el_ts = quote! {
        #VSc: #PgCrudCmnDfltSomeOneElCall
    };
    let gen_struct_ts = |flt_init_with_try_new_result_is_ok: bool,
                         generic: &Generic,
                         ident: &dyn ToTokens,
                         struct_extra_fields_ts: &dyn ToTokens| {
        let mb_pub_ts: &dyn ToTokens = if flt_init_with_try_new_result_is_ok {
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
            .d_serde_deserialize_if(if flt_init_with_try_new_result_is_ok {
                DSerdeDeserialize::False
            } else {
                DSerdeDeserialize::True
            })
            .d_schemars_json_schema()
            .build_struct(
                &Ts2::new(),
                &ident,
                &match &generic {
                    Generic::False => proc_macro2_ts_new.clone(),
                    Generic::True { mb_extra_traits_ts } => mb_extra_traits_ts
                        .as_ref()
                        .map_or_else(|| quote! {<#t_ts>}, |v| quote! {<#t_ts: #v>}),
                },
                &quote::quote! {{
                    #mb_pub_ts oprtr: #import::Oprtr,
                    #struct_extra_fields_ts
                }},
            )
    };
    let gen_impl_dflt_some_one_el_ts = |generic: &Generic,
                                        ident: &dyn ToTokens,
                                        ts: &dyn ToTokens| {
        gen_impl_dflt_some_one_el_ts(
            &match &generic {
                Generic::False => Ts2::new(),
                Generic::True { mb_extra_traits_ts } => mb_extra_traits_ts.as_ref().map_or_else(
                    || quote! {<T: #PgCrudCmnDfltSomeOneEl>},
                    |v| quote! {<T: #v + #PgCrudCmnDfltSomeOneEl>},
                ),
            },
            &Import::PgCrudCmn,
            &ident,
            match &generic {
                Generic::False => &proc_macro2_ts_new,
                Generic::True { .. } => &t_ann_generic_ts,
            },
            &quote! {
                Self {
                    oprtr: #PgCrudCmnDfltSomeOneElCall,
                    #ts
                }
            },
        )
    };
    let gen_impl_pg_type_wh_flt_ts = |flt_type: &FltType,
                                      generic: &Generic,
                                      ident: &dyn ToTokens,
                                      incr_prm_undrscr: &IncrPrmUndrscr,
                                      add_oprtr_undrscr: &AddOprtrUndrscr,
                                      qp_ts: &dyn ToTokens,
                                      is_qb_mut: &IsQbMut,
                                      qb_ts: &dyn ToTokens| {
        impl_pg_type_wh_flt_for_ident_ts(
            &{
                let mb_t_extra_traits_for_pg_type_wh_flt_ts: &dyn ToTokens = match &generic {
                    Generic::False => &proc_macro2_ts_new,
                    Generic::True { mb_extra_traits_ts } => {
                        let send_and_lt_ts = quote! {Send + 'lt};
                        let ser_ts = quote! {serde::Serialize};
                        let ts = match (&flt_type, &mb_extra_traits_ts) {
                            (FltType::PgType, Some(v)) => &quote! {#v + #send_and_lt_ts},
                            (FltType::PgType, None) => &send_and_lt_ts,
                            (FltType::PgJson, Some(v)) => &quote! {#v + #ser_ts + #send_and_lt_ts},
                            (FltType::PgJson, None) => &quote! {#ser_ts + #send_and_lt_ts},
                        };
                        &quote! {, T: #ts}
                    }
                };
                quote! {<'lt #mb_t_extra_traits_for_pg_type_wh_flt_ts>}
            },
            &ident,
            &match &generic {
                Generic::False => &proc_macro2_ts_new,
                Generic::True { .. } => &t_ann_generic_ts,
            },
            incr_prm_undrscr,
            &ColPrmUndrscr::False,
            add_oprtr_undrscr,
            &qp_ts,
            is_qb_mut,
            &qb_ts,
            &Import::PgCrudCmn,
        )
    };
    let add_rgx_case_and_v_dcl_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            pub rgx_case: RgxCase,
            pub #VSc: RgxRgx
        }
    };
    let add_rgx_case_and_v_dflt_init_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            rgx_case: #PgCrudCmnDfltSomeOneElCall,
            #v_dflt_some_one_el_ts
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
    let gen_rgx_qp_format_ts =
        |v: &dyn Display, mb_dims_ies_init_ts: &dyn ToTokens, mb_extra_prms_ts: &dyn ToTokens| {
            let format_ts = dq_ts(&v);
            quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #ColSc,
                    #mb_extra_prms_ts
                    #SelfSc.rgx_case.postgreql_syntax(),
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
    let pub_v_btwn_t_ts = quote! {pub #VSc: Btwn<T>};
    let gen_match_qb_ts = |field_ts: &dyn ToTokens| {
        quote! {
            match #field_ts.qb(#QuerySc) {
                Ok(v_f6d31bdd) => {
                    #QuerySc = v_f6d31bdd;
                },
                Err(#ErSc) => {
                    return Err(#ErSc);
                }
            }
        }
    };
    let query_self_v_qb_ts = {
        let ts = gen_match_qb_ts(&quote! {#SelfSc.#VSc});
        quote! {
            #ts
            Ok(#QuerySc)
        }
    };
    let pg_type_ptrn_stdrt = PgTypePtrn::Stdrt;
    let pg_type_ptrn_arr_dim1 = PgTypePtrn::ArrDim1;
    let pg_type_ptrn_arr_dim2 = PgTypePtrn::ArrDim2;
    let pg_type_ptrn_arr_dim3 = PgTypePtrn::ArrDim3;
    let pg_type_ptrn_arr_dim4 = PgTypePtrn::ArrDim4;
    let gen_pub_dims_bounded_vec_ts =
        |vec_len_ts: &dyn ToTokens, kind_of_unsigned_part_of_i32: &KindOfUnsignedPartOfI32| {
            quote! {pub #DimsSc: BoundedVec<#import::#kind_of_unsigned_part_of_i32, #vec_len_ts>}
        };
    let gen_ident_match_field_fn_ok_v_return_err_ts =
        |ident_ts: &dyn ToTokens, field_ts: &dyn ToTokens, fn_ts: &dyn ToTokens| {
            quote! {
                let #ident_ts = match self.#field_ts.#fn_ts(#IncrSc, #ColSc, add_oprtr) {
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
        #DimsSc: #PgCrudCmnDfltSomeOneElCall
    };
    let dims_dflt_init_comma_ts = quote! {#dims_dflt_init_ts,};
    let query_self_dims_qb_query_ts = gen_match_qb_ts(&quote! {#SelfSc.#DimsSc});
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
            #v_dflt_some_one_el_ts
        }
    };
    let gen_two_ts = |mb_dims_qb_ts: &dyn ToTokens, trailing_ts: &dyn ToTokens| {
        quote! {
            #mb_dims_qb_ts
            #trailing_ts
        }
    };
    let is_qb_mut_true = IsQbMut::True;
    let is_qb_mut_false = IsQbMut::False;
    let gen_qp_format_with_v_ts =
        |mb_dims_ies_init_ts: &dyn ToTokens,
         format_ts: &dyn ToTokens,
         mb_extra_prms_ts: &dyn ToTokens| {
            quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #ColSc,
                    #mb_extra_prms_ts
                    #VSc
                ))
            }
        };
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
        let gen_flts_ts = |flt: &PgTypeFlt| {
            let ident = PgTypeWhSelfUcc::from_display(&flt);
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_some_one_el_extra_fields_ts,
                incr_prm_undrscr,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = {
                let sqlx_type_pg_encode_ts = quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let generic_true_type_encode = Generic::True {
                    mb_extra_traits_ts: Some(sqlx_type_pg_encode_ts.clone()),
                };
                let gen_pg_type_dims_helpers_pg_type = |pg_type_ptrn: &PgTypePtrn| {
                    gen_pg_type_dims_helpers(pg_type_ptrn, &PgTypeOrPgJson::PgType)
                };
                let gen_cmp_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn,
                     gen_format_h_str: &dyn Fn(&PgTypeKind) -> String| {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            pg_type_kind,
                            mb_extra_prms_ts,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                        (
                            generic_true_type_encode.clone(),
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrPrmUndrscr::False,
                            gen_qp_format_with_v_ts(
                                &mb_dims_ies_init_ts,
                                &dq_ts(&gen_format_h_str(&pg_type_kind)),
                                &mb_extra_prms_ts,
                            ),
                            is_qb_mut_true,
                            gen_two_ts(&mb_dims_qb_ts, &qb_one_v_ts),
                        )
                    };
                let gen_oprtr_cmp_flt_ts = |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn Display| {
                    gen_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!("{{}}({{}}{} {oprtr} ${{}})", pg_type_kind.format_argument())
                    })
                };
                let gen_greater_than_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_oprtr_cmp_flt_ts(pg_type_ptrn, &">");
                let gen_btwn_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_btwn_t_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
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
                                    #ColSc,
                                    #mb_extra_prms_ts
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
                        mb_extra_prms_ts,
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
                            pub #VSc: PgTypeNotEmptyUnqVec<T>
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote! {acc, "${v_daedba9c},"},
                                &quote! {return Err(#import::QpEr::WriteIntoBuffer { loc: loc_lib::loc!() });},
                            );
                            quote! {
                                #mb_dims_ies_init_ts
                                let #VSc = {
                                    let mut acc = String::default();
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
                                    let _: Option<char> = acc.pop();
                                    acc
                                };
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColSc,
                                    #mb_extra_prms_ts
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
                let gen_rgx_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        add_rgx_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                        add_rgx_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
                        gen_rgx_qp_format_ts(
                            &format!("{{}}({{}}{} {{}} ${{}})", pg_type_kind.format_argument()),
                            &mb_dims_ies_init_ts,
                            &mb_extra_prms_ts,
                        ),
                        is_qb_mut_true,
                        gen_two_ts(
                            &mb_dims_qb_ts,
                            &if_let_err_query_try_bind_self_v_to_string_ts,
                        ),
                    )
                };
                let gen_before_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_oprtr_cmp_flt_ts(pg_type_ptrn, &"<");
                let gen_pg_syntax_flt_ts = |pg_type_ptrn: &PgTypePtrn, pg_syntax: &dyn Display| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        match &pg_type_ptrn {
                            PgTypePtrn::Stdrt => IncrPrmUndrscr::True,
                            PgTypePtrn::ArrDim1
                            | PgTypePtrn::ArrDim2
                            | PgTypePtrn::ArrDim3
                            | PgTypePtrn::ArrDim4 => IncrPrmUndrscr::False,
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
                                    #ColSc,
                                    #mb_extra_prms_ts
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
                let gen_eq_to_encoded_string_representation_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
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
                            encode_format: #PgCrudCmnDfltSomeOneElCall,
                            encoded_string_representation: String::default()
                        },
                        IncrPrmUndrscr::False,
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
                                    #ColSc,
                                    #mb_extra_prms_ts
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
                let pub_v_not_zero_unsigned_part_of_i32_dcl_ts =
                    quote! {pub #VSc: #not_zero_unsigned_part_of_i32_ts};
                let gen_len_flt_pattern_ts = |oprtr: &dyn Display| {
                    (
                        generic_false.clone(),
                        pub_v_not_zero_unsigned_part_of_i32_dcl_ts.clone(),
                        v_dflt_some_one_el_ts.clone(),
                        IncrPrmUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!("{{}}(arr_len({{}}, 1) {oprtr} ${{}})"));
                            quote! {
                                match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                    Ok(v_f7988de8) => Ok(format!(#format_ts, &self.oprtr.to_qp(add_oprtr), #ColSc, v_f7988de8)),
                                    Err(#ErSc) => Err(#ErSc),
                                }
                            }
                        },
                        is_qb_mut_true,
                        qb_one_v_ts.clone(),
                    )
                };
                let gen_range_bound_cmp_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn, bound_fn: &str, oprtr: &str| {
                        gen_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                            format!(
                                "{{}}({bound_fn}({{}}{}) {oprtr} ${{}})",
                                pg_type_kind.format_argument()
                            )
                        })
                    };
                let gen_range_len_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = DimNbr::try_from(pg_type_ptrn).map_or_else(
                        |()| {
                            (
                                Ts2::new(),
                                Ts2::new(),
                                Ts2::new(),
                                PgTypeKind::Stdrt,
                                quote! {#ColSc,},
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
                                    col,
                                    dims_ies2,
                                },
                                {
                                    let dims_clone_qb_ts =
                                        gen_match_qb_ts(&quote! {#SelfSc.#DimsSc.clone()});
                                    quote! {
                                        #dims_clone_qb_ts
                                        #query_self_dims_qb_query_ts
                                    }
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
                        IncrPrmUndrscr::False,
                        gen_qp_format_with_v_ts(
                            &mb_dims_ies_init_ts,
                            &dq_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
                            )),
                            &mb_extra_prms_ts,
                        ),
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #qb_one_v_ts
                        },
                    )
                };
                let gen_eq_oprtr_qp_ts =
                    |mb_dims_ies_init_ts: &dyn ToTokens, format_ts: &dyn ToTokens| {
                        quote! {
                            #mb_dims_ies_init_ts
                            let oprtr = <T as #import::PgTypeEqOprtr>::oprtr(&#SelfSc.#VSc);
                            let oprtr_query_str = oprtr.to_query_str();
                            Ok(format!(
                                #format_ts,
                                #self_oprtr_to_qp_ts
                                #ColSc,
                                match oprtr {
                                    #import::EqOprtr::Eq => {
                                        #v_match_incr_checked_add_one_init_ts
                                        format!("{oprtr_query_str} ${v}")
                                    },
                                    #import::EqOprtr::IsNull => oprtr_query_str.to_owned(),
                                }
                            ))
                        }
                    };
                let gen_eq_oprtr_qb_ts = |ts: &dyn ToTokens| {
                    quote! {
                        #ts
                        if matches!(&<T as #import::PgTypeEqOprtr>::oprtr(&#SelfSc.#VSc), #import::EqOprtr::Eq)
                            && let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc)
                        {
                            return Err(#ErSc.to_string());
                        }
                        Ok(#QuerySc)
                    }
                };
                match &flt {
                    PgTypeFlt::Eq { .. } => {
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
                                    quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrPrmUndrscr::False,
                            gen_eq_oprtr_qp_ts(&mb_dims_ies_init_ts, &quote! {"{}({} {})"}),
                            is_qb_mut_true,
                            gen_eq_oprtr_qb_ts(&mb_dims_qb_ts),
                        )
                    }
                    PgTypeFlt::DimOneEq { .. } => {
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
                                    quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrPrmUndrscr::False,
                            gen_eq_oprtr_qp_ts(
                                &mb_dims_ies_init_ts,
                                &quote! {"{}({}{dims_ies} {})"},
                            ),
                            is_qb_mut_true,
                            gen_eq_oprtr_qb_ts(&mb_dims_qb_ts),
                        )
                    }
                    PgTypeFlt::GreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneGreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFlt::Btwn { .. } => gen_btwn_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneBtwn { .. } => gen_btwn_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFlt::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFlt::Rgx => gen_rgx_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneRgx => gen_rgx_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFlt::Before { .. } => gen_before_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneBefore { .. } => gen_before_ts(&pg_type_ptrn_arr_dim1),
                    PgTypeFlt::CrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_date")
                    }
                    PgTypeFlt::DimOneCrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"= current_date")
                    }
                    PgTypeFlt::GreaterThanCrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_date")
                    }
                    PgTypeFlt::DimOneGreaterThanCrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"> current_date")
                    }
                    PgTypeFlt::CrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_timestamp")
                    }
                    PgTypeFlt::DimOneCrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"= current_timestamp")
                    }
                    PgTypeFlt::GreaterThanCrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_timestamp")
                    }
                    PgTypeFlt::DimOneGreaterThanCrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"> current_timestamp")
                    }
                    PgTypeFlt::CrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_time")
                    }
                    PgTypeFlt::DimOneCrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"= current_time")
                    }
                    PgTypeFlt::GreaterThanCrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_time")
                    }
                    PgTypeFlt::DimOneGreaterThanCrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_arr_dim1, &"> current_time")
                    }
                    PgTypeFlt::DimOneLenEq => gen_len_flt_pattern_ts(&"="),
                    PgTypeFlt::DimOneLenGreaterThan => gen_len_flt_pattern_ts(&">"),
                    PgTypeFlt::EqToEncodedStringRepresentation => {
                        gen_eq_to_encoded_string_representation_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFlt::DimOneEqToEncodedStringRepresentation => {
                        gen_eq_to_encoded_string_representation_ts(&pg_type_ptrn_arr_dim1)
                    }
                    PgTypeFlt::FindRangesWithinGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"<@")
                    }
                    PgTypeFlt::DimOneFindRangesWithinGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"<@")
                    }
                    PgTypeFlt::FindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"@>")
                    }
                    PgTypeFlt::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"@>")
                    }
                    PgTypeFlt::StrictlyToLeftOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&<")
                    }
                    PgTypeFlt::DimOneStrictlyToLeftOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"&<")
                    }
                    PgTypeFlt::StrictlyToRightOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&>")
                    }
                    PgTypeFlt::DimOneStrictlyToRightOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"&>")
                    }
                    PgTypeFlt::IncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", "=")
                    }
                    PgTypeFlt::DimOneIncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_arr_dim1, "lower", "=")
                    }
                    PgTypeFlt::ExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", "=")
                    }
                    PgTypeFlt::DimOneExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_arr_dim1, "upper", "=")
                    }
                    PgTypeFlt::GreaterThanIncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", ">")
                    }
                    PgTypeFlt::DimOneGreaterThanIncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_arr_dim1, "lower", ">")
                    }
                    PgTypeFlt::GreaterThanExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", ">")
                    }
                    PgTypeFlt::DimOneGreaterThanExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_arr_dim1, "upper", ">")
                    }
                    PgTypeFlt::OverlapWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&&")
                    }
                    PgTypeFlt::DimOneOverlapWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"&&")
                    }
                    PgTypeFlt::AdjacentWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"-|-")
                    }
                    PgTypeFlt::DimOneAdjacentWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_arr_dim1, &"-|-")
                    }
                    PgTypeFlt::RangeLen => gen_range_len_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::DimOneRangeLen => gen_range_len_ts(&pg_type_ptrn_arr_dim1),
                }
            };
            let struct_ts = gen_struct_ts(false, &generic, &ident, &struct_extra_fields_ts);
            let impl_dflt_some_one_el_ts = gen_impl_dflt_some_one_el_ts(
                &generic,
                &ident,
                &impl_dflt_some_one_el_extra_fields_ts,
            );
            let impl_pg_type_wh_flt_ts = gen_impl_pg_type_wh_flt_ts(
                &FltType::PgType,
                &generic,
                &ident,
                &incr_prm_undrscr,
                &AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_dflt_some_one_el_ts
                #impl_pg_type_wh_flt_ts
            };
            gend
        };
        let flt_arr_ts = PgTypeFlt::into_arr().map(|el| gen_flts_ts(&el));
        let gend = quote! {#(#flt_arr_ts)*};
        mb_write_ts_into_file(
            gen_wh_flts_config.pg_types_write_into_file,
            "gen_wh_flts_pg_types",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let pg_json_ts = {
        let gen_flts_ts = |flt: &PgJsonFlt| {
            let ident = PgJsonWhSelfUcc::from_display(&flt);
            let pub_v_pg_json_not_empty_unq_vec_t_ts = quote! {
                pub #VSc: PgJsonNotEmptyUnqVec<T>
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
            let gen_json_cmp_flt_ts =
                |pg_type_ptrn: &PgTypePtrn, gen_format_h_str: &dyn Fn(&PgTypeKind) -> String| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                    (
                        generic_true_none.clone(),
                        gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        gen_qp_format_with_v_ts(
                            &mb_dims_ies_init_ts,
                            &dq_ts(&gen_format_h_str(&pg_type_kind)),
                            &mb_extra_prms_ts,
                        ),
                        is_qb_mut_true,
                        gen_two_ts(&mb_dims_qb_ts, &qb_sqlx_types_json_self_v_ts),
                    )
                };
            let gen_json_oprtr_cmp_flt_ts = |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn Display| {
                gen_json_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!("{{}}({{}}{} {oprtr} ${{}})", pg_type_kind.format_argument())
                })
            };
            let gen_eq_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_json_oprtr_cmp_flt_ts(pg_type_ptrn, &"=");
            let gen_all_els_eq_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_json_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <> ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_format_qp_ts = |format_ts: &dyn ToTokens, mb_extra_prms_ts: &dyn ToTokens| {
                quote! {
                    Ok(format!(
                        #format_ts,
                        #self_oprtr_to_qp_ts
                        #ColSc,
                        #mb_extra_prms_ts
                        #VSc
                    ))
                }
            };
            let gen_json_arr_len_flt_ts = |pg_type_ptrn: &PgTypePtrn, op: &dyn Display| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_prms_ts,
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
                        #VSc: #unsigned_part_of_i32_ts::default()
                    },
                    {
                        let format_ts = dq_ts(&format!(
                            "{{}}(jsonb_array_length({{}}{}) {op} ${{}})",
                            pg_type_kind.format_argument()
                        ));
                        let ts = gen_format_qp_ts(&format_ts, &mb_extra_prms_ts);
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
            let gen_len_eq_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_json_arr_len_flt_ts(pg_type_ptrn, &"=");
            let gen_len_greater_than_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_json_arr_len_flt_ts(pg_type_ptrn, &">");
            let gen_greater_than_ts =
                |pg_type_ptrn: &PgTypePtrn| gen_json_oprtr_cmp_flt_ts(pg_type_ptrn, &">");
            let gen_contains_el_greater_than_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_json_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) > ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_all_els_greater_than_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_json_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                    format!(
                        "{{}}(not exists(select 1 from jsonb_array_elements({{}}{}) as el where (el) <= ${{}}))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let gen_btwn_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_prms_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                    gen_two_ts(&mb_dims_dcl_ts, &pub_v_btwn_t_ts),
                    gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                    {
                        let ts: &dyn ToTokens = match pg_type_ptrn {
                            PgTypePtrn::Stdrt => &quote! {
                                let #VSc = match self.#VSc.qp(
                                    incr,
                                    col,
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
                        let ts0 = gen_format_qp_ts(
                            &dq_ts(&format!(
                                "{{}}({{}}{} {{}})",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_prms_ts,
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
                                        return Err(er);
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
                    mb_extra_prms_ts,
                    mb_dims_qb_ts,
                ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                (
                    generic_true_debug_partial_eq_clone.clone(),
                    gen_two_ts(&mb_dims_dcl_ts, &pub_v_pg_json_not_empty_unq_vec_t_ts),
                    gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                    {
                        let v_init_ts = gen_ident_match_field_fn_ok_v_return_err_ts(
                            &VSc,
                            &VSc,
                            &quote! {qp_one_by_one},
                        );
                        let ts = gen_format_qp_ts(
                            &dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            )),
                            &mb_extra_prms_ts,
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
            let gen_rgx_ts = |pg_type_ptrn: &PgTypePtrn| {
                let (
                    mb_dims_dcl_ts,
                    mb_dims_dflt_init_ts,
                    mb_dims_ies_init_ts,
                    pg_type_kind,
                    mb_extra_prms_ts,
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
                    add_rgx_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                    add_rgx_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                    gen_rgx_qp_format_ts(
                        &format!(
                            "{{}}(trim(both '\\\"' from ({{}}{})::text) {{}} ${{}})",
                            match &pg_type_kind {
                                PgTypeKind::Stdrt => "",
                                PgTypeKind::ArrDim => "{}->>${}",
                            }
                        ),
                        &mb_dims_ies_init_ts,
                        &mb_extra_prms_ts,
                    ),
                    is_qb_mut_true,
                    quote! {
                        #mb_dims_qb_ts
                        #if_let_err_query_try_bind_self_v_to_string_ts
                    },
                )
            };
            let gen_jsonb_arr_el_rgx_ts =
                |pg_type_ptrn: &PgTypePtrn, fmt_fn: &dyn Fn(&'static str) -> String| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        add_rgx_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                        add_rgx_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                        gen_rgx_qp_format_ts(
                            &fmt_fn(pg_type_kind.format_argument()),
                            &mb_dims_ies_init_ts,
                            &mb_extra_prms_ts,
                        ),
                        is_qb_mut_true,
                        gen_two_ts(
                            &mb_dims_qb_ts,
                            &if_let_err_query_try_bind_self_v_to_string_ts,
                        ),
                    )
                };
            //todo test it properly using all strange string vrts
            let gen_contains_el_rgx_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_jsonb_arr_el_rgx_ts(pg_type_ptrn, &|fa| {
                    format!(
                        "{{}}(exists(select 1 from jsonb_array_elements({{}}{fa}) as el where (el #>> '{{{{}}}}') {{}} ${{}}))"
                    )
                })
            };
            //todo reuse select it
            //todo test it properly using all strange string vrts
            let gen_all_els_rgx_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_jsonb_arr_el_rgx_ts(pg_type_ptrn, &|fa| {
                    format!(
                        "{{}}(not exists(select 1 from jsonb_array_elements({{}}{fa}) as el where (el #>> '{{{{}}}}') !{{}} ${{}}))"
                    )
                })
            };
            let gen_pg_json_vec_with_qp_ts =
                |pg_type_ptrn: &PgTypePtrn, gen_format_str: &dyn Fn(&PgTypeKind) -> String| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_json_dims_helpers(pg_type_ptrn);
                    (
                        generic_true_debug_partial_eq_clone.clone(),
                        gen_two_ts(&mb_dims_dcl_ts, &pub_v_pg_json_not_empty_unq_vec_t_ts),
                        gen_two_ts(&mb_dims_dflt_init_ts, &v_dflt_some_one_el_ts),
                        {
                            let ts = gen_format_qp_ts(
                                &dq_ts(&gen_format_str(&pg_type_kind)),
                                &mb_extra_prms_ts,
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
            let gen_contains_all_els_of_arr_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_pg_json_vec_with_qp_ts(pg_type_ptrn, &|pg_type_kind| {
                    format!("{{}}({{}}{} @> {{}})", pg_type_kind.format_argument())
                })
            };
            let gen_overlaps_with_arr_ts = |pg_type_ptrn: &PgTypePtrn| {
                gen_pg_json_vec_with_qp_ts(pg_type_ptrn, &|pg_type_kind| {
                    format!(
                        "{{}}(exists (select 1 from jsonb_arr_els_text({{}}{}) as e1 join jsonb_arr_els_text({{}}) as e2 on e1.v = e2.v))",
                        pg_type_kind.format_argument()
                    )
                })
            };
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_some_one_el_extra_fields_ts,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = match &flt {
                PgJsonFlt::Eq { .. } => gen_eq_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneEq { .. } => gen_eq_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoEq { .. } => gen_eq_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeEq { .. } => gen_eq_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourEq { .. } => gen_eq_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::AllElsEq { .. } => gen_all_els_eq_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneAllElsEq { .. } => gen_all_els_eq_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoAllElsEq { .. } => gen_all_els_eq_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeAllElsEq { .. } => gen_all_els_eq_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourAllElsEq { .. } => gen_all_els_eq_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::LenEq => gen_len_eq_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneLenEq => gen_len_eq_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoLenEq => gen_len_eq_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeLenEq => gen_len_eq_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourLenEq => gen_len_eq_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::LenGreaterThan => gen_len_greater_than_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneLenGreaterThan => gen_len_greater_than_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoLenGreaterThan => gen_len_greater_than_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeLenGreaterThan => {
                    gen_len_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourLenGreaterThan => gen_len_greater_than_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::GreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneGreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoGreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeGreaterThan { .. } => {
                    gen_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourGreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::ContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFlt::DimOneContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFlt::DimTwoContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFlt::DimThreeContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourContainsElGreaterThan { .. } => {
                    gen_contains_el_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFlt::AllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFlt::DimOneAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFlt::DimTwoAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFlt::DimThreeAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourAllElsGreaterThan { .. } => {
                    gen_all_els_greater_than_ts(&pg_type_ptrn_arr_dim4)
                }
                PgJsonFlt::Btwn { .. } => gen_btwn_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneBtwn { .. } => gen_btwn_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoBtwn { .. } => gen_btwn_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeBtwn { .. } => gen_btwn_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourBtwn { .. } => gen_btwn_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourIn { .. } => gen_in_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::Rgx => gen_rgx_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneRgx => gen_rgx_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoRgx => gen_rgx_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeRgx => gen_rgx_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourRgx => gen_rgx_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::ContainsElRgx => gen_contains_el_rgx_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneContainsElRgx => gen_contains_el_rgx_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoContainsElRgx => gen_contains_el_rgx_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeContainsElRgx => gen_contains_el_rgx_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourContainsElRgx => gen_contains_el_rgx_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::AllElsRgx => gen_all_els_rgx_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneAllElsRgx => gen_all_els_rgx_ts(&pg_type_ptrn_arr_dim1),
                PgJsonFlt::DimTwoAllElsRgx => gen_all_els_rgx_ts(&pg_type_ptrn_arr_dim2),
                PgJsonFlt::DimThreeAllElsRgx => gen_all_els_rgx_ts(&pg_type_ptrn_arr_dim3),
                PgJsonFlt::DimFourAllElsRgx => gen_all_els_rgx_ts(&pg_type_ptrn_arr_dim4),
                PgJsonFlt::ContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_stdrt)
                }
                PgJsonFlt::DimOneContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFlt::DimTwoContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFlt::DimThreeContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourContainsAllElsOfArr { .. } => {
                    gen_contains_all_els_of_arr_ts(&pg_type_ptrn_arr_dim4)
                }
                // PgJsonFlt::ContainedInArr => todo!(),
                PgJsonFlt::OverlapsWithArr { .. } => gen_overlaps_with_arr_ts(&pg_type_ptrn_stdrt),
                PgJsonFlt::DimOneOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim1)
                }
                PgJsonFlt::DimTwoOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim2)
                }
                PgJsonFlt::DimThreeOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim3)
                }
                PgJsonFlt::DimFourOverlapsWithArr { .. } => {
                    gen_overlaps_with_arr_ts(&pg_type_ptrn_arr_dim4)
                }
            };
            let struct_ts = gen_struct_ts(false, &generic, &ident, &struct_extra_fields_ts);
            let impl_dflt_some_one_el_ts = gen_impl_dflt_some_one_el_ts(
                &generic,
                &ident,
                &impl_dflt_some_one_el_extra_fields_ts,
            );
            let impl_pg_type_wh_flt_ts = gen_impl_pg_type_wh_flt_ts(
                &FltType::PgJson,
                &generic,
                &ident,
                &IncrPrmUndrscr::False,
                &AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_dflt_some_one_el_ts
                #impl_pg_type_wh_flt_ts
            };
            gend
        };
        let flt_arr_ts = PgJsonFlt::into_arr().map(|el| gen_flts_ts(&el));
        let gend = quote! {#(#flt_arr_ts)*};
        mb_write_ts_into_file(
            gen_wh_flts_config.pg_json_write_into_file,
            "gen_wh_flts_pg_json",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let imports_ts = quote! {
        #[allow(clippy::wildcard_imports)]
        use super::*;
    };
    let gend = pg_crud_macros_cmn::gen_mod_with_pub_use_ts(
        &quote::format_ident!("gen_wh_flts_mod"),
        &[imports_ts, pg_type_ts, pg_json_ts],
    );
    mb_write_ts_into_file(
        gen_wh_flts_config.whole_write_into_file,
        "gen_wh_flts",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend
}
