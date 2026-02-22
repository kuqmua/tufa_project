use gen_quotes::dq_ts;
use macros_helpers::{
    LocationFieldAttr, gen_if_write_is_err_ts, gen_impl_display_ts, gen_impl_to_err_string_ts,
    gen_serde_version_of_named_syn_vrt,
};
use naming::{IntoSerdeVersionSc, LocSc, ValueSc, WithSerdeUcc, parameter::SelfWithSerdeUcc};
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
        eo_location,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde,
        eo_vec_location,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_hashmap_k_string_v_location,
    )
)]
pub fn location(input: Ts) -> Ts {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SuportedEnumVrt {
        Named,
        Unnamed,
    }
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("d94f091a");
    let ident = &di.ident;
    let generic_params = &di
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
        let mut all_equal: Option<SuportedEnumVrt> = None;
        assert!(!data_enum.variants.is_empty(), "27275ae6");
        data_enum.variants.iter().for_each(|vrt| match &vrt.fields {
            Fields::Named(_) => match &all_equal {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Unnamed), "bf6be520");
                }
                None => {
                    all_equal = Some(SuportedEnumVrt::Named);
                }
            },
            Fields::Unnamed(_) => match &all_equal {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Named), "02090d85");
                }
                None => {
                    all_equal = Some(SuportedEnumVrt::Unnamed);
                }
            },
            Fields::Unit => panic!("2f2e9385"),
        });
        all_equal.expect("b9da972a")
    };
    let maybe_generic_params_ts = if generic_params.is_empty() {
        Ts2::new()
    } else {
        quote! {<#(#generic_params),*>}
    };
    let maybe_generic_params_location_lib_to_err_string_annotations_ts =
        if generic_params.is_empty() {
            Ts2::new()
        } else {
            let v = generic_params
                .iter()
                .map(|el| quote! {#el: location_lib::ToErrString});
            quote! {<#(#v),*>}
        };
    let gen_enum_ident_with_serde_ts = |vrts_ts: &dyn ToTokens| {
        quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serde_ucc #maybe_generic_params_ts {
                #vrts_ts
            }
        }
    };
    let gen_impl_ident_into_serde_version_ts = |vrts: &dyn ToTokens| {
        quote! {
            impl #maybe_generic_params_ts #ident #maybe_generic_params_ts {
                pub fn #IntoSerdeVersionSc(self) -> #ident_with_serde_ucc #maybe_generic_params_ts {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #vrts
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_vrt {
        SuportedEnumVrt::Named => {
            let loc_sc_str = LocSc.to_string();
            //todo maybe impl display was a bad idea. .to_string() casts is dangerous
            let impl_display_handle_ts = {
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
                        match LocationFieldAttr::try_from(el0).expect("8ff56aeb") {
                            LocationFieldAttr::EoToErrString | LocationFieldAttr::EoToErrStringSerde => {
                                quote! {
                                    location_lib::ToErrString::to_err_string(#el0_ident)
                                }
                            }
                            LocationFieldAttr::EoLocation => {
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
                            LocationFieldAttr::EoVecToErrString | LocationFieldAttr::EoVecToErrStringSerde => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_a9ba7521, "\n {el_6e4f53ad}"}, &quote! {panic!("b35ed9f5");});
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_ac447c4b, el| {
                                            acc_ac447c4b.push_str(
                                                &location_lib::ToErrString::to_err_string(el)
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
                            LocationFieldAttr::EoVecLocation => {
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
                            LocationFieldAttr::EoHashMapKeyStringValueToErrString | LocationFieldAttr::EoHashMapKeyStringValueToErrStringSerde => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_06473093, "\n {key}: {}", &location_lib::ToErrString::to_err_string(#ValueSc)}, &quote! {panic!("d030580a");});
                                quote! {
                                    #el0_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_06473093, (key, #ValueSc)| {
                                            #if_write_is_err_ts
                                            acc_06473093
                                        }
                                    )
                                }
                            }
                            LocationFieldAttr::EoHashMapKeyStringValueLocation => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(
                                    &{
                                        let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_addfc699, "\n  {el_8b8f577e}"}, &quote! {panic!("d0492fbf");});
                                        quote! {
                                            acc_a47e1ba7,
                                            "\n {key}: {}",
                                            #ValueSc.to_string().lines().fold(
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
                                        |mut acc_a47e1ba7, (key, #ValueSc)| {
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
                let loc_vrts_ts = data_enum.variants.iter().enumerate().map(|(index, el)| {
                    let el_ident = &el.ident;
                    if index == 0 {
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
                &maybe_generic_params_location_lib_to_err_string_annotations_ts,
                &ident,
                &maybe_generic_params_ts,
                &impl_display_handle_ts,
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
                    let fields_into_serde_version_excluding_loc_ts = fields.iter()
                    .filter(|el0| *el0.ident.as_ref().expect("0488fc4c") != *loc_sc_str)
                    .map(|el0| {
                        let el0_ident = &el0.ident.as_ref().expect("9a672ac2");
                        let conversion_ts = match LocationFieldAttr::try_from(el0).expect("449c3781") {
                            LocationFieldAttr::EoToErrString => {
                                quote! {
                                    #el0_ident: {
                                        location_lib::ToErrString::to_err_string(&#el0_ident)
                                    }
                                }
                            }
                            LocationFieldAttr::EoToErrStringSerde | LocationFieldAttr::EoVecToErrStringSerde | LocationFieldAttr::EoHashMapKeyStringValueToErrStringSerde => {
                                quote! {
                                    #el0_ident
                                }
                            }
                            LocationFieldAttr::EoLocation => {
                                quote! {
                                    #el0_ident: {
                                        #el0_ident.into_serde_version()
                                    }
                                }
                            }
                            LocationFieldAttr::EoVecToErrString => {
                                quote! {
                                    #el0_ident: {
                                        #el0_ident.into_iter().map(|el|location_lib::ToErrString::to_err_string(&el)).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoVecLocation => {
                                quote! {
                                    #el0_ident: {
                                        #el0_ident.into_iter().map(|el|el.into_serde_version()).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoHashMapKeyStringValueToErrString => {
                                quote! {
                                    #el0_ident: {
                                        #el0_ident.into_iter().map(|(k, v)|
                                            (k, location_lib::ToErrString::to_err_string(&v))
                                        ).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoHashMapKeyStringValueLocation => {
                                quote! {
                                    #el0_ident: {
                                        #el0_ident.into_iter().map(
                                            |(k, v)|(k, v.into_serde_version())
                                        ).collect()
                                    }
                                }
                            }
                        };
                        quote! {#conversion_ts,}
                    });
                    quote! {
                        Self::#el_ident {
                            #(#fields_idents_ts),*
                        } => #ident_with_serde_ucc::#el_ident {
                            #(#fields_into_serde_version_excluding_loc_ts)*
                            #LocSc,
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
                &maybe_generic_params_location_lib_to_err_string_annotations_ts,
                &ident_with_serde_ucc,
                &maybe_generic_params_ts,
                &impl_display_handle_ts,
            );
            let impl_location_lib_to_err_string_to_err_string_for_ident_with_serde_ts =
                gen_impl_to_err_string_ts(
                    &maybe_generic_params_location_lib_to_err_string_annotations_ts,
                    &ident_with_serde_ucc,
                    &maybe_generic_params_ts,
                    &quote! {format!("{self}")},
                );
            quote! {
                #impl_display_for_ident_ts
                #impl_ident_into_serde_version_ts
                #enum_ident_with_serde_ts
                #impl_display_for_ident_with_serde_ts
                #impl_location_lib_to_err_string_to_err_string_for_ident_with_serde_ts
            }
        }
        SuportedEnumVrt::Unnamed => {
            let gen_display_formatter_unnamed_ts = || {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    quote! {Self::#el_ident(v) => v}
                });
                quote! {match self { #(#vrts_ts),* }}
            };
            let impl_display_for_ident_ts = gen_impl_display_ts(
                &maybe_generic_params_location_lib_to_err_string_annotations_ts,
                &ident,
                &maybe_generic_params_ts,
                &{
                    let display_formatter_unnamed_ts = gen_display_formatter_unnamed_ts();
                    quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_ts
                        )
                    }
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
                    let inner_type_with_serde_ts = {
                        format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "d7a6b955");
                                let field_type = &fields.iter().next().expect("8a80c36d").ty;
                                quote! {#field_type}.to_string()
                            },
                            WithSerdeUcc
                        )
                        .parse::<Ts2>()
                        .expect("9ff40f7e")
                    };
                    quote! {#el_ident(#inner_type_with_serde_ts)}
                });
                gen_enum_ident_with_serde_ts(&quote! {#(#vrts_ts),*})
            };
            let impl_display_for_ident_with_serde_ts = gen_impl_display_ts(
                &maybe_generic_params_location_lib_to_err_string_annotations_ts,
                &ident_with_serde_ucc,
                &maybe_generic_params_ts,
                &{
                    let display_formatter_unnamed_ts = gen_display_formatter_unnamed_ts();
                    quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_ts
                        )
                    }
                },
            );
            //todo maybe make a trait?
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
    // if ident == "Er" {
    //     maybe_write_ts_into_file(
    //         ShouldWriteTokenStreamIntoFile::True,
    //         "location",
    //         &generated,
    //         &FormatWithCargofmt::True,
    //     );
    // }
    generated.into()
}
