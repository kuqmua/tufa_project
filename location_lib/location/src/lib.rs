use gen_quotes::dq_ts;
use macros_helpers::{
    LocationFieldAttr, gen_if_write_is_err_ts, gen_impl_display_ts, gen_impl_to_err_string_ts,
    gen_serde_version_of_named_syn_variant,
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
    enum SuportedEnumVariant {
        Named,
        Unnamed,
    }
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("d94f091a");
    let ident = &di.ident;
    let generic_parameters = &di
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
    let supported_enum_variant = {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        assert!(!data_enum.variants.is_empty(), "27275ae6");
        data_enum
            .variants
            .iter()
            .for_each(|variant| match &variant.fields {
                Fields::Named(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Unnamed),
                            "bf6be520"
                        );
                    }
                    None => {
                        all_equal = Some(SuportedEnumVariant::Named);
                    }
                },
                Fields::Unnamed(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Named),
                            "02090d85"
                        );
                    }
                    None => {
                        all_equal = Some(SuportedEnumVariant::Unnamed);
                    }
                },
                Fields::Unit => panic!("2f2e9385"),
            });
        all_equal.expect("b9da972a")
    };
    let maybe_generic_parameters_ts = if generic_parameters.is_empty() {
        Ts2::new()
    } else {
        quote! {<#(#generic_parameters),*>}
    };
    let maybe_generic_parameters_location_lib_to_err_string_annotations_ts =
        if generic_parameters.is_empty() {
            Ts2::new()
        } else {
            let v = generic_parameters
                .iter()
                .map(|el_4ee89bd7| quote! {#el_4ee89bd7: location_lib::ToErrString});
            quote! {<#(#v),*>}
        };
    let gen_enum_ident_with_serde_ts = |variants_ts: &dyn ToTokens| {
        quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serde_ucc #maybe_generic_parameters_ts {
                #variants_ts
            }
        }
    };
    let gen_impl_ident_into_serde_version_ts = |variants: &dyn ToTokens| {
        quote! {
            impl #maybe_generic_parameters_ts #ident #maybe_generic_parameters_ts {
                pub fn #IntoSerdeVersionSc(self) -> #ident_with_serde_ucc #maybe_generic_parameters_ts {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #variants
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let loc_sc_str = LocSc.to_string();
            //todo maybe impl display was a bad idea. .to_string() casts is dangerous
            let impl_display_handle_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_f497ea11| {
                    let el_ident = &el_f497ea11.ident;
                    let fields = if let Fields::Named(fields) = &el_f497ea11.fields {
                        &fields.named
                    } else {
                        panic!("f64e0d21");
                    };
                    let fields_idents_excluding_loc_ts = {
                        let acc_ts = fields.iter()
                        .filter(|el_e26e2572| *el_e26e2572.ident.as_ref().expect("07504636") != *loc_sc_str)
                        .map(|el_e4070354| el_e4070354.ident.as_ref().expect("971ace15"))
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
                        .filter(|el_6ba47e94| *el_6ba47e94.ident.as_ref().expect("3d70a4f4") != *loc_sc_str)
                        .fold(
                            String::new(),
                            |mut acc_924ab1b3, el_e405984a| {
                                use std::fmt::Write as _;
                                let el_e405984a_ident = &el_e405984a.ident.as_ref().expect("2e7cd5fe");
                                assert!(
                                    writeln!(
                                        acc_924ab1b3,
                                        "{el_e405984a_ident}: {{}}"
                                    ).is_ok(),
                                    "ab44c70f"
                                );
                                acc_924ab1b3
                            }
                        )
                    );
                    let fields_format_vs_excluding_loc_ts = fields.iter()
                    .filter(|el_48337db8| *el_48337db8.ident.as_ref().expect("f6f6fb24") != *loc_sc_str)
                    .map(|el_f00312fe| {
                        let el_f00312fe_ident = &el_f00312fe.ident.as_ref().expect("e97b25b9");
                        match LocationFieldAttr::try_from(el_f00312fe).expect("8ff56aeb") {
                            LocationFieldAttr::EoToErrString | LocationFieldAttr::EoToErrStringSerde => {
                                quote! {
                                    location_lib::ToErrString::to_err_string(#el_f00312fe_ident)
                                }
                            }
                            LocationFieldAttr::EoLocation => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_52e70d22, "\n {element}"}, &quote! {panic!("c751d54a");});
                                quote! {
                                    #el_f00312fe_ident.to_string().lines().fold(
                                        #StringTs::new(),
                                        |mut acc_52e70d22, element| {
                                            #if_write_is_err_ts
                                            acc_52e70d22
                                        }
                                    )
                                }
                            }
                            LocationFieldAttr::EoVecToErrString | LocationFieldAttr::EoVecToErrStringSerde => {
                                let if_write_is_err_ts = gen_if_write_is_err_ts(&quote! {acc_a9ba7521, "\n {el_6e4f53ad}"}, &quote! {panic!("b35ed9f5");});
                                quote! {
                                    #el_f00312fe_ident.iter().fold(
                                        #StringTs::new(),
                                        |mut acc_ac447c4b, el_36630fcf| {
                                            acc_ac447c4b.push_str(
                                                &location_lib::ToErrString::to_err_string(el_36630fcf)
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
                                    #el_f00312fe_ident.iter().fold(
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
                                    #el_f00312fe_ident.iter().fold(
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
                                    #el_f00312fe_ident.iter().fold(
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
                let loc_variants_ts =
                    data_enum
                        .variants
                        .iter()
                        .enumerate()
                        .map(|(index, element)| {
                            let el_ident = &element.ident;
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
                            #(#variants_ts),*
                        },
                        match self {
                            #(#loc_variants_ts)*
                            => #LocSc
                        }
                    )
                }
            };
            let impl_display_for_ident_ts = gen_impl_display_ts(
                &maybe_generic_parameters_location_lib_to_err_string_annotations_ts,
                &ident,
                &maybe_generic_parameters_ts,
                &impl_display_handle_ts,
            );
            let impl_ident_into_serde_version_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_7d5a4c39| {
                    let el_ident = &el_7d5a4c39.ident;
                    let fields = if let Fields::Named(fields) = &el_7d5a4c39.fields {
                        &fields.named
                    } else {
                        panic!("238b402b");
                    };
                    let fields_idents_ts = fields.iter().map(|el_cddf556e| &el_cddf556e.ident);
                    let fields_into_serde_version_excluding_loc_ts = fields.iter()
                    .filter(|el_6a54951c| *el_6a54951c.ident.as_ref().expect("0488fc4c") != *loc_sc_str)
                    .map(|el_d7e120a3| {
                        let el_d7e120a3_ident = &el_d7e120a3.ident.as_ref().expect("9a672ac2");
                        let conversion_ts = match LocationFieldAttr::try_from(el_d7e120a3).expect("449c3781") {
                            LocationFieldAttr::EoToErrString => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        location_lib::ToErrString::to_err_string(&#el_d7e120a3_ident)
                                    }
                                }
                            }
                            LocationFieldAttr::EoToErrStringSerde | LocationFieldAttr::EoVecToErrStringSerde | LocationFieldAttr::EoHashMapKeyStringValueToErrStringSerde => {
                                quote! {
                                    #el_d7e120a3_ident
                                }
                            }
                            LocationFieldAttr::EoLocation => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        #el_d7e120a3_ident.into_serde_version()
                                    }
                                }
                            }
                            LocationFieldAttr::EoVecToErrString => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        #el_d7e120a3_ident.into_iter().map(|el_3c145d8e|location_lib::ToErrString::to_err_string(&el_3c145d8e)).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoVecLocation => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        #el_d7e120a3_ident.into_iter().map(|el_607695c6|el_607695c6.into_serde_version()).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoHashMapKeyStringValueToErrString => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        #el_d7e120a3_ident.into_iter().map(|(k, v)|
                                            (k, location_lib::ToErrString::to_err_string(&v))
                                        ).collect()
                                    }
                                }
                            }
                            LocationFieldAttr::EoHashMapKeyStringValueLocation => {
                                quote! {
                                    #el_d7e120a3_ident: {
                                        #el_d7e120a3_ident.into_iter().map(
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
                gen_impl_ident_into_serde_version_ts(&quote! {#(#variants_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let variants_ts = data_enum
                    .variants
                    .iter()
                    .map(gen_serde_version_of_named_syn_variant);
                gen_enum_ident_with_serde_ts(&quote! {#(#variants_ts),*})
            };
            let impl_display_for_ident_with_serde_ts = gen_impl_display_ts(
                &maybe_generic_parameters_location_lib_to_err_string_annotations_ts,
                &ident_with_serde_ucc,
                &maybe_generic_parameters_ts,
                &impl_display_handle_ts,
            );
            let impl_location_lib_to_err_string_to_err_string_for_ident_with_serde_ts =
                gen_impl_to_err_string_ts(
                    &maybe_generic_parameters_location_lib_to_err_string_annotations_ts,
                    &ident_with_serde_ucc,
                    &maybe_generic_parameters_ts,
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
        SuportedEnumVariant::Unnamed => {
            let gen_display_formatter_unnamed_ts = || {
                let variants_ts = data_enum.variants.iter().map(|el_f99bd80d| {
                    let el_ident = &el_f99bd80d.ident;
                    quote! {Self::#el_ident(v) => v}
                });
                quote! {match self { #(#variants_ts),* }}
            };
            let impl_display_for_ident_ts = gen_impl_display_ts(
                &maybe_generic_parameters_location_lib_to_err_string_annotations_ts,
                &ident,
                &maybe_generic_parameters_ts,
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
                let variants_ts = data_enum.variants.iter().map(|el_0e2b2f9c| {
                    let el_ident = &el_0e2b2f9c.ident;
                    quote! {
                        Self::#el_ident(v) => #ident_with_serde_ucc::#el_ident(
                            v.#IntoSerdeVersionSc(),
                        )
                    }
                });
                gen_impl_ident_into_serde_version_ts(&quote! {#(#variants_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_0f06fa87| {
                    let el_ident = &el_0f06fa87.ident;
                    let fields = if let Fields::Unnamed(fields) = &el_0f06fa87.fields {
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
                gen_enum_ident_with_serde_ts(&quote! {#(#variants_ts),*})
            };
            let impl_display_for_ident_with_serde_ts = gen_impl_display_ts(
                &maybe_generic_parameters_location_lib_to_err_string_annotations_ts,
                &ident_with_serde_ucc,
                &maybe_generic_parameters_ts,
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
