use gen_quotes::dq_ts;
use panic_location::panic_location;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use regex::Regex;
use serde_json::from_str;
use std::fmt::Write as _;
#[allow(unused_imports)]
use syn::{Data, DeriveInput, Fields, Ident, Type, parse};
use token_patterns::StringTs;
const REGEX_VALUE: &str = "^[a-zA-Z]+$";
#[proc_macro]
pub fn gen_ucc_and_sc_str_and_ts(input_ts: Ts) -> Ts {
    panic_location();
    let content_ts = from_str::<Vec<Vec<String>>>(&input_ts.to_string())
        .expect("90e5793b")
        .into_iter()
        .map(|el_020a8657| {
            {
                let regex = Regex::new(REGEX_VALUE).expect("20948d87");
                for el_d68254e8 in &el_020a8657 {
                    assert!(regex.is_match(el_d68254e8), "faadba8a");
                }
            }
            let phrase_part_ucc_str =
                el_020a8657
                    .iter()
                    .fold(String::new(), |mut acc_3d60efa0, el_132cd6b1| {
                        acc_3d60efa0.push_str(&naming_common::AsRefStrToUccStr::case(el_132cd6b1));
                        acc_3d60efa0
                    });
            let phrase_part_sc_str = el_020a8657.iter().enumerate().fold(
                String::new(),
                |mut acc_7a8bd950, (index, el_b9981760)| {
                    let el_sc_str = naming_common::AsRefStrToScStr::case(el_b9981760);
                    if index == 0 {
                        acc_7a8bd950.push_str(&el_sc_str);
                    } else {
                        assert!(write!(acc_7a8bd950, "_{el_sc_str}").is_ok(), "ef718915");
                    }
                    acc_7a8bd950
                },
            );
            let phrase_part_ucc_ucc_ts = format!("{phrase_part_ucc_str}Ucc")
                .parse::<Ts2>()
                .expect("4ab6a54c");
            let phrase_part_sc_ucc_ts = format!("{phrase_part_ucc_str}Sc")
                .parse::<Ts2>()
                .expect("0cc47b2e");
            let (ucc_struct_declaration_ts, sc_struct_declaration_ts) = {
                let gen_struct_declaration = |struct_name_ts: &dyn ToTokens| {
                    quote! {
                        #[derive(Debug)]
                        pub struct #struct_name_ts;
                    }
                };
                (
                    gen_struct_declaration(&phrase_part_ucc_ucc_ts),
                    gen_struct_declaration(&phrase_part_sc_ucc_ts),
                )
            };
            let (impl_display_ucc_ts, impl_display_sc_ts) = {
                let gen_impl_display_ts =
                    |struct_name_ts: &dyn ToTokens, write_content_ts: &dyn ToTokens| {
                        quote! {
                            impl std::fmt::Display for #struct_name_ts {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, #write_content_ts)
                                }
                            }
                        }
                    };
                (
                    gen_impl_display_ts(&phrase_part_ucc_ucc_ts, &dq_ts(&phrase_part_ucc_str)),
                    gen_impl_display_ts(&phrase_part_sc_ucc_ts, &dq_ts(&phrase_part_sc_str)),
                )
            };
            let (impl_to_tokens_ucc_ts, impl_to_tokens_snake_ts) = {
                let gen_impl_to_tokens_ts =
                    |struct_name_ts: &dyn ToTokens, quote_content_ts: &dyn ToTokens| {
                        quote! {
                            impl ToTokens for #struct_name_ts {
                                fn to_tokens(&self, tokens: &mut Ts2) {
                                    quote!{#quote_content_ts}.to_tokens(tokens);
                                }
                            }
                        }
                    };
                (
                    gen_impl_to_tokens_ts(
                        &phrase_part_ucc_ucc_ts,
                        &phrase_part_ucc_str.parse::<Ts2>().expect("7cf3ffc0"),
                    ),
                    gen_impl_to_tokens_ts(
                        &phrase_part_sc_ucc_ts,
                        &phrase_part_sc_str.parse::<Ts2>().expect("114a573a"),
                    ),
                )
            };
            quote! {
                #ucc_struct_declaration_ts
                #impl_display_ucc_ts
                #impl_to_tokens_ucc_ts
                #sc_struct_declaration_ts
                #impl_display_sc_ts
                #impl_to_tokens_snake_ts
            }
        });
    let generated = quote! {#(#content_ts)*};
    // println!("{generated}");
    generated.into()
}
#[proc_macro]
pub fn gen_self_ucc_and_sc_str_and_ts(input_ts: Ts) -> Ts {
    panic_location();
    let content_ts = from_str::<Vec<Vec<String>>>(&input_ts.to_string()).expect("9d6a20af").into_iter().map(|el_a5ccbaa7| {
        {
            let regex = Regex::new(REGEX_VALUE).expect("cba1b5fb");
            for el_6d4f29dd in &el_a5ccbaa7 {
                assert!(regex.is_match(el_6d4f29dd), "4a12d90f");
            }
        }
        let self_match_name = "self";
        {
            let mut is_self_exists_and_only_one = false;
            for el_3eac2cfb in &el_a5ccbaa7 {
                if el_3eac2cfb == self_match_name {
                    is_self_exists_and_only_one = true;
                    break;
                }
            }
            assert!(is_self_exists_and_only_one, "5680dd63");
        };
        let (elements_concat_v_ucc_dq_ts, elements_concat_v_sc_dq_ts, struct_ucc_ucc_ts, struct_sc_token_ucc_ts, trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
            let ucc_ucc_str = "Ucc";
            let sc_ucc_str = "Sc";
            let elements_concat_ucc_str = el_a5ccbaa7.iter().fold(String::new(), |mut acc_34997d76, el_98881b7d| {
                acc_34997d76.push_str(&naming_common::AsRefStrToUccStr::case(el_98881b7d));
                acc_34997d76
            });
            let elements_concat_v_ucc_dq_ts = dq_ts(&el_a5ccbaa7.iter().fold(String::new(), |mut acc_ae77cbd3, el_626f2b61| {
                if el_626f2b61 == "self" {
                    acc_ae77cbd3.push_str("{v}");
                } else {
                    acc_ae77cbd3.push_str(&naming_common::AsRefStrToUccStr::case(el_626f2b61));
                }
                acc_ae77cbd3
            }));
            let elements_concat_v_sc_dq_ts = dq_ts(&{
                let mut acc = el_a5ccbaa7.iter().fold(String::new(), |mut acc_cbcae5e1, el_73b0c851| {
                    let symbol = '_';
                    if el_73b0c851 == "self" {
                        assert!(write!(acc_cbcae5e1, "{{v}}{symbol}").is_ok(), "6a02a2ff");
                    } else {
                        assert!(write!(acc_cbcae5e1, "{}{symbol}", naming_common::AsRefStrToScStr::case(el_73b0c851)).is_ok(), "d915980a");
                    }
                    acc_cbcae5e1
                });
                let _: Option<char> = acc.pop();
                acc
            });
            let struct_ucc_ucc_ts = format!("{elements_concat_ucc_str}{ucc_ucc_str}").parse::<Ts2>().expect("82f4ac08");
            let struct_sc_token_ucc_ts = format!("{elements_concat_ucc_str}{sc_ucc_str}").parse::<Ts2>().expect("21044eba");
            let (trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
                let trait_ucc_str = "Trait";
                let trait_ucc_ucc_ts = format!("{elements_concat_ucc_str}{ucc_ucc_str}{trait_ucc_str}").parse::<Ts2>().expect("1066857a");
                let trait_sc_token_ucc_ts = format!("{elements_concat_ucc_str}{sc_ucc_str}{trait_ucc_str}").parse::<Ts2>().expect("8db74cfd");
                (trait_ucc_ucc_ts, trait_sc_token_ucc_ts)
            };
            (
                elements_concat_v_ucc_dq_ts,
                elements_concat_v_sc_dq_ts,
                struct_ucc_ucc_ts,
                struct_sc_token_ucc_ts,
                trait_ucc_ucc_ts,
                trait_sc_token_ucc_ts,
            )
        };
        let gen_struct_ts = |elements_concat_v_case_dq_ts: &dyn ToTokens, is_ucc: bool, trait_ident_ts: &dyn ToTokens| {
            let struct_ident_ts = if is_ucc {
                quote! {#struct_ucc_ucc_ts}
            } else {
                quote! {#struct_sc_token_ucc_ts}
            };
            let casing_ts = if is_ucc {
                quote! {naming_common::AsRefStrToUccStr::case}
            } else {
                quote! {naming_common::AsRefStrToScStr::case}
            };
            quote! {
                #[derive(Debug)]
                pub struct #struct_ident_ts(String);
                impl #struct_ident_ts {
                    fn wrap(v: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(v))
                    }
                    fn format(v: &dyn std::fmt::Display) -> String {
                        format!(#elements_concat_v_case_dq_ts)
                    }
                    pub fn from_display(v: &dyn std::fmt::Display) -> Self {
                        Self::wrap(&#casing_ts(&v.to_string()))
                    }
                    pub fn from_tokens(v: &dyn ToTokens) -> Self {
                        Self::wrap(&#casing_ts(&{
                            let mut tokens = Ts2::new();
                            ToTokens::to_tokens(&v, &mut tokens);
                            tokens
                        }.to_string()))
                    }
                    pub fn from_type_last_segment(v: &syn::Type) -> Self {
                        if let syn::Type::Path(type_path) = v {
                            let path_before_str = type_path.path.segments.iter().take(
                                type_path.path.segments.len().checked_sub(1).expect("e1f5a332")
                            )
                            .fold(String::new(), |mut acc_f0a77378, el_2b05e58f| {
                                use std::fmt::Write as _;
                                assert!(write!(acc_f0a77378, "{}::", el_2b05e58f.ident).is_ok(), "67c90ce9");
                                acc_f0a77378
                            });
                            let last = type_path.path.segments.iter().last().expect("19f6e1a6");
                            Self(format!("{path_before_str}{}", Self::format(&#casing_ts(&last.ident.to_string()))))
                        }
                        else {
                            panic!("518933f8");
                        }
                    }
                }
                impl std::fmt::Display for #struct_ident_ts {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.0)
                    }
                }
                impl ToTokens for #struct_ident_ts {
                    fn to_tokens(&self, tokens: &mut Ts2) {
                        self.to_string().parse::<Ts2>()
                            .expect("71c8d26b").to_tokens(tokens);
                    }
                }
                pub trait #trait_ident_ts: std::fmt::Display + ToTokens {}
                impl #trait_ident_ts for #struct_ident_ts {}
            }
        };
        let pub_struct_ucc_ts = gen_struct_ts(&elements_concat_v_ucc_dq_ts, true, &trait_ucc_ucc_ts);
        let pub_struct_sc_ts = gen_struct_ts(&elements_concat_v_sc_dq_ts, false, &trait_sc_token_ucc_ts);
        quote! {
            #pub_struct_ucc_ts
            #pub_struct_sc_ts
        }
    });
    let generated = quote! {#(#content_ts)*};
    // println!("{generated}");
    generated.into()
}
fn gen_impl_trait_for_ident_ts(
    name_ts: &dyn ToTokens,
    ident: &Ident,
    variants_matching_ts: &[Ts2],
) -> Ts2 {
    quote! {
        impl naming::#name_ts for #ident {
            fn case(&self) -> #StringTs {//todo maybe write duplicate Trait with &str instead of String
                match self {#(#variants_matching_ts),*}
            }
        }
    }
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUccStr)]
pub fn as_ref_str_enum_with_unit_fields_to_ucc_str(input_ts: Ts) -> Ts {
    panic_location();
    let syn_derive_input: DeriveInput = parse(input_ts).expect("a8f22481");
    let ident = &syn_derive_input.ident;
    let Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("d26bf85e")
    };
    let generated = gen_impl_trait_for_ident_ts(
        &quote! {AsRefStrToUccStr},
        ident,
        &data_enum
            .variants
            .iter()
            .map(|el| match el.fields {
                Fields::Unit => {
                    let el_ident = &el.ident;
                    let el_ident_ucc_dq_ts =
                        dq_ts(&naming_common::ToTokensToUccStr::case(&el_ident));
                    quote! {Self::#el_ident => #StringTs::from(#el_ident_ucc_dq_ts)}
                }
                Fields::Named(_) | Fields::Unnamed(_) => {
                    panic!("4955c50d")
                }
            })
            .collect::<Vec<Ts2>>(),
    );
    // println!("{generated}");
    generated.into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_sc_str(input_ts: Ts) -> Ts {
    panic_location();
    let syn_derive_input: DeriveInput = parse(input_ts).expect("dea5cbcf");
    let ident = &syn_derive_input.ident;
    let Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("ed6efe2e");
    };
    let generated = gen_impl_trait_for_ident_ts(
        &quote! {AsRefStrToScStr},
        ident,
        &data_enum
            .variants
            .iter()
            .map(|el| match el.fields {
                Fields::Unit => {
                    let el_ident = &el.ident;
                    let el_ident_sc_dq_ts = dq_ts(&naming_common::ToTokensToScStr::case(&el_ident));
                    quote! {Self::#el_ident => #StringTs::from(#el_ident_sc_dq_ts)}
                }
                Fields::Named(_) | Fields::Unnamed(_) => {
                    panic!("b3ef2657")
                }
            })
            .collect::<Vec<Ts2>>(),
    );
    // println!("{generated}");
    generated.into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_sc_str(input_ts: Ts) -> Ts {
    panic_location();
    let syn_derive_input: DeriveInput = parse(input_ts).expect("edabbc24");
    let ident = &syn_derive_input.ident;
    let Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("b2263e7e");
    };
    let generated = gen_impl_trait_for_ident_ts(
        &quote! {ToUpperScStr},
        ident,
        &data_enum
            .variants
            .iter()
            .map(|el| match el.fields {
                Fields::Unit => {
                    let el_ident = &el.ident;
                    let el_ident_sc_dq_ts =
                        dq_ts(&naming_common::ToTokensToUpperScStr::case(&el_ident));
                    quote! {Self::#el_ident => #StringTs::from(#el_ident_sc_dq_ts)}
                }
                Fields::Named(_) | Fields::Unnamed(_) => panic!("b6fedcff"),
            })
            .collect::<Vec<Ts2>>(),
    );
    // println!("{generated}");
    generated.into()
}
