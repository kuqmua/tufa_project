use gen_quotes::dq_ts;
use macros_helpers::{
    LocFieldAttr, gen_if_write_is_err_ts, gen_impl_display_ts, gen_impl_to_err_string_ts,
    gen_serde_version_of_named_syn_vrt,
};
use naming::{IntoSerdeVersionSc, LocSc, VSc, WithSerdeUcc, prm::SelfWithSerdeUcc};
use optml::Optml;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, GenericParam, Ident, parse};
use token_patterns::StringTs;
#[proc_macro_derive(
    Location,
    attributes(
        eo_to_err_string,
        eo_to_err_string_serde,
        eo_loc,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde,
        eo_vec_loc,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_hashmap_k_string_v_loc,
    )
)]
pub fn loc(input: Ts) -> Ts {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Optml)]
    enum SuportedEnumVrt {
        Named,
        Unnamed,
    }
    panic_loc::panic_loc();
    let di: DeriveInput = parse(input).expect("d94f091a");
    let ident = &di.ident;
    let generic_prms = &di
        .generics
        .params
        .iter()
        .map(|el_a6a747c1| match &el_a6a747c1 {
            GenericParam::Type(v) => &v.ident,
            GenericParam::Lifetime(_) | GenericParam::Const(_) => {
                panic!("3ce82d11")
            }
        })
        .collect::<Vec<&Ident>>();
    let ident_with_serde_ucc = SelfWithSerdeUcc::from_tokens(&ident);
    let Data::Enum(data_enum) = di.data else {
        panic!("d98214f7");
    };
    let supported_enum_vrt = {
        let mut all_eq: Option<SuportedEnumVrt> = None;
        assert!(!data_enum.variants.is_empty(), "27275ae6");
        data_enum.variants.iter().for_each(|vrt| match &vrt.fields {
            Fields::Named(_) => match &all_eq {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Unnamed), "bf6be520");
                }
                None => {
                    all_eq = Some(SuportedEnumVrt::Named);
                }
            },
            Fields::Unnamed(_) => match &all_eq {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Named), "02090d85");
                }
                None => {
                    all_eq = Some(SuportedEnumVrt::Unnamed);
                }
            },
            Fields::Unit => panic!("2f2e9385"),
        });
        all_eq.expect("b9da972a")
    };
    let mb_generic_prms_ts = if generic_prms.is_empty() {
        Ts2::new()
    } else {
        quote! {<#(#generic_prms),*>}
    };
    let mb_generic_prms_loc_lib_to_err_string_anns_ts = if generic_prms.is_empty() {
        Ts2::new()
    } else {
        let v = generic_prms
            .iter()
            .map(|el| quote! {#el: loc_lib::ToErrString});
        quote! {<#(#v),*>}
    };
    let gen_enum_ident_with_serde_ts = |ts: &dyn ToTokens| {
        quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize, loc_lib::Optml)]
            pub enum #ident_with_serde_ucc #mb_generic_prms_ts {
                #ts
            }
        }
    };
    let gen_impl_ident_into_serde_version_ts = |ts: &dyn ToTokens| {
        quote! {
            impl #mb_generic_prms_ts #ident #mb_generic_prms_ts {
                pub fn #IntoSerdeVersionSc(self) -> #ident_with_serde_ucc #mb_generic_prms_ts {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #ts
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_vrt {
        SuportedEnumVrt::Named => {
            let loc_sc_str = LocSc.to_string();
            //todo mb impl display was a bad idea. .to_string() casts is dangerous
            let impl_display_h_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let Fields::Named(fields) = &el.fields {
                        &fields.named
                    } else {
                        panic!("f64e0d21");
                    };
                    let fields_idents_excluding_loc_ts = {
                        let acc_ts = fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("07504636") != *loc_sc_str)
                        .map(|el0| el0.ident.as_ref().expect("971ace15"))
                        .collect::<Vec<&Ident>>();
                        if acc_ts.is_empty() {
                            Ts2::new()
                        }
                        else {
                            quote!{#(#acc_ts),*,}
                        }
                    };
                    let fields_format_excluding_loc_ts = dq_ts(
                        &fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("3d70a4f4") != *loc_sc_str)
                        .fold(
                            String::new(),
                            |mut acc, el0| {
                                use std::fmt::Write as _;
                                let el0_ident = &el0.ident.as_ref().expect("2e7cd5fe");
                                assert!(
                                    writeln!(
                                        acc,
                                        "{el0_ident}: {{}}"
                                    ).is_ok(),
                                    "ab44c70f"
                                );
                                acc
                            }
                        )
                    );
                    let fields_format_vs_excluding_loc_ts = fields.iter()
                    .filter(|el0| *el0.ident.as_ref().expect("f6f6fb24") != *loc_sc_str)
                    .map(|el0| {
                        let el0_ident = &el0.ident.as_ref().expect("e97b25b9");
                        match LocFieldAttr::try_from(el0).expect("8ff56aeb") {
                            LocFieldAttr::EoToErrString | LocFieldAttr::EoToErrStringSerde => {
                                quote! {
                                    loc_lib::ToErrString::to_err_string(#el0_ident)
                                }
                            }
                            LocFieldAttr::EoLoc => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_52e70d22, "\n {el}"}, &quote! {panic!("c751d54a");});
                                quote! {
                                    #el0_ident.to_string().lines().fold(
                                        #StringTs::new(),
                                        |mut acc_52e70d22, el| {
                                            #if_write_is_err_ts
                                            acc_52e70d22
                                        }
                                    )
                                }
                            }
                            LocFieldAttr::EoVecToErrString | LocFieldAttr::EoVecToErrStringSerde => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_a9ba7521, "\n {el_6e4f53ad}"}, &quote! {panic!("b35ed9f5");});
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_ac447c4b, el| {
                                            acc_ac447c4b.push_str(
                                                &loc_lib::ToErrString::to_err_string(el)
                                                .lines()
                                                .fold(
                                                    #StringTs::new(),
                                                    |mut acc_a9ba7521, el_6e4f53ad| {
                                                        #if_write_is_err_ts
                                                        acc_a9ba7521
                                                    }
                                                )
                                            );
                                            acc_ac447c4b
                                        }
                                    )
                                }
                            }
                            LocFieldAttr::EoVecLoc => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_1bbd5ef3, "\n {el_3f2fe01d}"}, &quote! {panic!("4dfdd18d");});
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_c5adba93, el_37c46c8a| {
                                            acc_c5adba93.push_str(&el_37c46c8a.to_string().lines().fold(
                                                #StringTs::new(),
                                                |mut acc_1bbd5ef3, el_3f2fe01d| {
                                                    #if_write_is_err_ts
                                                    acc_1bbd5ef3
                                                },
                                            ));
                                            acc_c5adba93
                                        }
                                    )
                                }
                            }
                            LocFieldAttr::EoHashMapKStringVToErrString | LocFieldAttr::EoHashMapKStringVToErrStringSerde => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_06473093, "\n {k}: {}", &loc_lib::ToErrString::to_err_string(#VSc)}, &quote! {panic!("d030580a");});
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_06473093, (k, #VSc)| {
                                            #if_write_is_err_ts
                                            acc_06473093
                                        }
                                    )
                                }
                            }
                            LocFieldAttr::EoHashMapKStringVLoc => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(
                                    &{
                                        let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_addfc699, "\n  {el_8b8f577e}"}, &quote! {panic!("d0492fbf");});
                                        quote! {
                                            acc_a47e1ba7,
                                            "\n {k}: {}",
                                            #VSc.to_string().lines().fold(
                                                #StringTs::new(),
                                                |mut acc_addfc699, el_8b8f577e| {
                                                    #if_write_is_err_ts
                                                    acc_addfc699
                                                }
                                            )
                                        }
                                    },
                                    &quote! {panic!("75f6432a");},
                                );
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_a47e1ba7, (k, #VSc)| {
                                            #if_write_is_err_ts
                                            acc_a47e1ba7
                                        }
                                    )
                                }
                            }
                        }
                    });
                    quote! {
                        Self::#el_ident {
                            #fields_idents_excluding_loc_ts
                            ..
                        } => {
                            format!(
                                #fields_format_excluding_loc_ts,
                                #(#fields_format_vs_excluding_loc_ts),*
                            )
                        }
                    }
                });
                let loc_vrts_ts = data_enum.variants.iter().enumerate().map(|(i, el)| {
                    let el_ident = &el.ident;
                    if i == 0 {
                        quote! {
                            Self::#el_ident {
                                #LocSc,
                                ..
                            }
                        }
                    } else {
                        quote! {
                            | Self::#el_ident {
                                #LocSc,
                                ..
                            }
                        }
                    }
                });
                quote! {
                    write!(
                        f,
                        "{}{}",
                        match self {
                            #(#vrts_ts),*
                        },
                        match self {
                            #(#loc_vrts_ts)*
                            => #LocSc
                        }
                    )
                }
            };
            let impl_display_for_ident_ts = gen_impl_display_ts(
                &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                &ident,
                &mb_generic_prms_ts,
                &impl_display_h_ts,
            );
            let impl_ident_into_serde_version_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let Fields::Named(fields) = &el.fields {
                        &fields.named
                    } else {
                        panic!("238b402b");
                    };
                    let fields_idents_ts = fields.iter().map(|el0| &el0.ident);
                    let fields_ts = fields.iter()
                    .map(|el0| {
                        let el0_ident = &el0.ident.as_ref().expect("9a672ac2");
                        if **el0_ident == *loc_sc_str {
                            quote! {#el0_ident}
                        }
                        else {
                            let gen_field_ts = |ts: &dyn ToTokens|quote!{#el0_ident: {#ts}};
                            match LocFieldAttr::try_from(el0).expect("449c3781") {
                                LocFieldAttr::EoToErrString => gen_field_ts(&quote!{
                                    loc_lib::ToErrString::to_err_string(&#el0_ident)
                                }),
                                LocFieldAttr::EoToErrStringSerde | LocFieldAttr::EoVecToErrStringSerde | LocFieldAttr::EoHashMapKStringVToErrStringSerde => {
                                    quote! {#el0_ident}
                                }
                                LocFieldAttr::EoLoc => gen_field_ts(&quote!{
                                    #el0_ident.into_serde_version()
                                }),
                                LocFieldAttr::EoVecToErrString => gen_field_ts(&quote!{
                                    #el0_ident.into_iter().map(|el|loc_lib::ToErrString::to_err_string(&el)).collect()
                                }),
                                LocFieldAttr::EoVecLoc => gen_field_ts(&quote!{
                                    #el0_ident.into_iter().map(|el|el.into_serde_version()).collect()
                                }),
                                LocFieldAttr::EoHashMapKStringVToErrString => gen_field_ts(&quote!{
                                    #el0_ident.into_iter().map(
                                        |(k, v)|(k, loc_lib::ToErrString::to_err_string(&v))
                                    ).collect()
                                }),
                                LocFieldAttr::EoHashMapKStringVLoc => gen_field_ts(&quote!{
                                    #el0_ident.into_iter().map(
                                        |(k, v)|(k, v.into_serde_version())
                                    ).collect()
                                }),
                            }
                        }
                    });
                    quote! {
                        Self::#el_ident {
                            #(#fields_idents_ts),*
                        } => #ident_with_serde_ucc::#el_ident {
                            #(#fields_ts),*
                        }
                    }
                });
                gen_impl_ident_into_serde_version_ts(&quote! {#(#vrts_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let vrts_ts = data_enum
                    .variants
                    .iter()
                    .map(gen_serde_version_of_named_syn_vrt);
                gen_enum_ident_with_serde_ts(&quote! {#(#vrts_ts),*})
            };
            let impl_display_for_ident_with_serde_ts = gen_impl_display_ts(
                &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                &ident_with_serde_ucc,
                &mb_generic_prms_ts,
                &impl_display_h_ts,
            );
            let impl_loc_lib_to_err_string_to_err_string_for_ident_with_serde_ts =
                gen_impl_to_err_string_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident_with_serde_ucc,
                    &mb_generic_prms_ts,
                    &quote! {format!("{self}")},
                );
            quote! {
                #impl_display_for_ident_ts
                #impl_ident_into_serde_version_ts
                #enum_ident_with_serde_ts
                #impl_display_for_ident_with_serde_ts
                #impl_loc_lib_to_err_string_to_err_string_for_ident_with_serde_ts
            }
        }
        SuportedEnumVrt::Unnamed => {
            let display_formatter_unnamed_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    quote! {Self::#el_ident(v) => v}
                });
                quote! {match self { #(#vrts_ts),* }}
            };
            let impl_display_for_ident_ts = gen_impl_display_ts(
                &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                &ident,
                &mb_generic_prms_ts,
                &quote! {
                    write!(
                        f,
                        "{}",
                        #display_formatter_unnamed_ts
                    )
                },
            );
            let impl_ident_into_serde_version_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    quote! {
                        Self::#el_ident(v) => #ident_with_serde_ucc::#el_ident(
                            v.#IntoSerdeVersionSc(),
                        )
                    }
                });
                gen_impl_ident_into_serde_version_ts(&quote! {#(#vrts_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let Fields::Unnamed(fields) = &el.fields {
                        &fields.unnamed
                    } else {
                        panic!("5749e920");
                    };
                    let inn_type_with_serde_ts = {
                        format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "d7a6b955");
                                let ft = &fields.iter().next().expect("8a80c36d").ty;
                                quote! {#ft}.to_string()
                            },
                            WithSerdeUcc
                        )
                        .parse::<Ts2>()
                        .expect("9ff40f7e")
                    };
                    quote! {#el_ident(#inn_type_with_serde_ts)}
                });
                gen_enum_ident_with_serde_ts(&quote! {#(#vrts_ts),*})
            };
            let impl_display_for_ident_with_serde_ts = gen_impl_display_ts(
                &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                &ident_with_serde_ucc,
                &mb_generic_prms_ts,
                &quote! {
                    write!(
                        f,
                        "{}",
                        #display_formatter_unnamed_ts
                    )
                },
            );
            //todo mb make a trait?
            quote! {
                #impl_display_for_ident_ts
                #impl_ident_into_serde_version_ts
                #enum_ident_with_serde_ts
                #impl_display_for_ident_with_serde_ts
            }
        }
    };
    let generated = quote! {#tokens};
    // println!("{generated} ");
    // if ident == "" {
    //     macros_helpers::mb_write_ts_into_file(
    //         macros_helpers::ShouldWriteTsIntoFile::True,
    //         "loc",
    //         &generated,
    //         &macros_helpers::FormatWithCargofmt::True,
    //     );
    // }
    generated.into()
}
