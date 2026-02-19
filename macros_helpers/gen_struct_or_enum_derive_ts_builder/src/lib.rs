use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde_json::from_str;
#[proc_macro]
pub fn gen_struct_or_enum_derive_ts_builder(input_ts: Ts) -> Ts {
    use naming::parameter::{DeriveSelfIfSc, DeriveSelfSc, DeriveSelfUcc};
    #[derive(Clone)]
    struct Element {
        derive_trait_name_if_sc: Ts2,
        derive_trait_name_sc: Ts2,
        derive_trait_name_ucc: Ts2,
        trait_type: Ts2,
    }
    let make_pub_sc_ts = quote! {make_pub};
    let make_pub_if_sc_ts = quote! {make_pub_if};
    let make_pub_ucc_ts = quote! {MakePub};
    let el_vec = from_str::<Vec<String>>(&input_ts.to_string())
        .expect("c5d09740")
        .into_iter()
        .map(|el_4f4a2c74| {
            let sc = {
                let mut result = String::with_capacity(el_4f4a2c74.len());
                let mut prev_is_underscore = false;
                let mut prev_is_lowercase = false;
                for el_e5df7ee3 in el_4f4a2c74.chars() {
                    if el_e5df7ee3.is_alphabetic() {
                        if el_e5df7ee3.is_uppercase() {
                            if prev_is_lowercase && !prev_is_underscore {
                                result.push('_');
                            }
                            for el_8aa0e3a1 in el_e5df7ee3.to_lowercase() {
                                result.push(el_8aa0e3a1);
                            }
                            prev_is_lowercase = false;
                        } else {
                            result.push(el_e5df7ee3);
                            prev_is_lowercase = true;
                        }
                        prev_is_underscore = false;
                    } else {
                        if !prev_is_underscore && !result.is_empty() {
                            result.push('_');
                            prev_is_underscore = true;
                        }
                        prev_is_lowercase = false;
                    }
                }
                result.trim_matches('_').to_owned()
            };
            Element {
                derive_trait_name_ucc: {
                    let v = DeriveSelfUcc::from_display(&sc);
                    quote! {#v}
                },
                derive_trait_name_sc: {
                    let v = DeriveSelfSc::from_display(&sc);
                    quote! {#v}
                },
                derive_trait_name_if_sc: {
                    let v = DeriveSelfIfSc::from_display(&sc);
                    quote! {#v}
                },
                trait_type: el_4f4a2c74.parse::<Ts2>().expect("8672240f"),
            }
        })
        .collect::<Vec<Element>>();
    let (make_pub_pub_enum_ts, pub_enum_derive_vec_ts) = {
        fn gen_pun_enum_ts(ident: &dyn ToTokens) -> Ts2 {
            quote! {
                #[derive(Debug, Clone, Copy)]
                pub enum #ident {
                    True,
                    False
                }
            }
        }
        (
            gen_pun_enum_ts(&make_pub_ucc_ts),
            el_vec
                .iter()
                .map(|el_4f4a2c74| gen_pun_enum_ts(&el_4f4a2c74.derive_trait_name_ucc)),
        )
    };
    let (make_pub_derive_trait_name_bool_ts, field_vec_ts) = {
        fn gen_derive_trait_name_bool_ts(ident: &dyn ToTokens) -> Ts2 {
            quote! {#ident: bool,}
        }
        (
            gen_derive_trait_name_bool_ts(&make_pub_sc_ts),
            el_vec.iter().map(|el_03225620| {
                gen_derive_trait_name_bool_ts(&el_03225620.derive_trait_name_sc)
            }),
        )
    };
    let (make_pub_derive_and_derive_if_ts, derive_and_derive_if_vec_ts) = {
        (
            quote! {
                pub const fn #make_pub_sc_ts(mut self) -> Self {
                    self.#make_pub_sc_ts = true;
                    self
                }
                pub const fn #make_pub_if_sc_ts(mut self, condition: #make_pub_ucc_ts) -> Self {
                    if let #make_pub_ucc_ts::True = condition {
                        self.#make_pub_sc_ts = true;
                    }
                    self
                }
            },
            el_vec.iter().map(|el_09fab389|{
                let derive_trait_name_ucc = &el_09fab389.derive_trait_name_ucc;
                let derive_trait_name_sc = &el_09fab389.derive_trait_name_sc;
                let derive_trait_name_if_sc = &el_09fab389.derive_trait_name_if_sc;
                quote!{
                    pub const fn #derive_trait_name_sc(mut self) -> Self {
                        self.#derive_trait_name_sc = true;
                        self
                    }
                    pub const fn #derive_trait_name_if_sc(mut self, condition: #derive_trait_name_ucc) -> Self {
                        if let #derive_trait_name_ucc::True = condition {
                            self.#derive_trait_name_sc = true;
                        }
                        self
                    }
                }
            })
        )
    };
    let if_self_derive_acc_push_vec_ts = el_vec.iter().map(|el_09bcde51| {
        let derive_trait_name_sc = &el_09bcde51.derive_trait_name_sc;
        let trait_type = &el_09bcde51.trait_type;
        quote! {
            if self.#derive_trait_name_sc {
                acc_2a71375c.push(quote::quote!{#trait_type});
            }
        }
    });
    let struct_or_enum_derive_ts_builder_ucc = quote! {StructOrEnumDeriveTokenStreamBuilder};
    let struct_or_enum_ucc = quote! {StructOrEnum};
    let quote_to_tokens_ts = quote! {quote::ToTokens};
    let generated: Ts2 = quote! {
        #make_pub_pub_enum_ts
        #(#pub_enum_derive_vec_ts)*
        #[derive(Debug, Clone, Copy)]
        enum #struct_or_enum_ucc {
            Struct,
            Enum
        }
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #struct_or_enum_derive_ts_builder_ucc {
            #make_pub_derive_trait_name_bool_ts
            #(#field_vec_ts)*
        }
        impl #struct_or_enum_derive_ts_builder_ucc {
            pub fn new() -> Self {
                Self::default()
            }
            #make_pub_derive_and_derive_if_ts
            #(#derive_and_derive_if_vec_ts)*
            fn build_handle(
                self,
                struct_or_enum: #struct_or_enum_ucc,
                ident_d8cbb733: &dyn #quote_to_tokens_ts,
                generics_7d48c97a: &dyn #quote_to_tokens_ts,
                content_ts: &dyn #quote_to_tokens_ts,
            ) -> Ts2 {
                let maybe_pub_ts = self.#make_pub_sc_ts.then(|| quote::quote!{pub});
                let derive_ts = {
                    let mut acc_2a71375c = Vec::new();
                    #(#if_self_derive_acc_push_vec_ts)*
                    acc_2a71375c
                };
                let struct_or_enum_ts = match struct_or_enum {
                    #struct_or_enum_ucc::Struct => quote::quote!{struct},
                    #struct_or_enum_ucc::Enum => quote::quote!{enum},
                };
                // quote::quote! {
                //     #[derive(#(#derive_ts),*)]
                //     #maybe_pub_ts #struct_or_enum_ts #ident_d8cbb733 #content_ts
                // }
                // this is cargo expand ouput coz double quote::quote!{quote::quote!{}}
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_pound(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Bracket,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "derive");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    {
                                        use ::quote::__private::ext::*;
                                        let mut _i = 0usize;
                                        let has_iter = ::quote::__private::HasIterator::<false>;
                                        #[allow(unused_mut)]
                                        let (mut derive_ts, i) = derive_ts
                                            .quote_into_iter();
                                        let has_iter = has_iter | i;
                                        <_ as ::quote::__private::CheckHasIterator<
                                            true,
                                        >>::check(has_iter);
                                        loop {
                                            let derive_ts = match derive_ts.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::#quote_to_tokens_ts::to_tokens(&derive_ts, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::#quote_to_tokens_ts::to_tokens(&maybe_pub_ts, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&struct_or_enum_ts, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&ident_d8cbb733, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&generics_7d48c97a, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&content_ts, &mut _s);
                    _s
                }
            }
            pub fn build_struct(
                self,
                ident_d87c6809: &dyn #quote_to_tokens_ts,
                generics_c33a0ef2: &dyn #quote_to_tokens_ts,
                content_ts: &dyn #quote_to_tokens_ts,
            ) -> Ts2 {
                self.build_handle(
                    #struct_or_enum_ucc::Struct,
                    ident_d87c6809,
                    generics_c33a0ef2,
                    content_ts
                )
            }
            pub fn build_enum(
                self,
                ident_273dd063: &dyn #quote_to_tokens_ts,
                generics_84bc3f7f: &dyn #quote_to_tokens_ts,
                content_ts: &dyn #quote_to_tokens_ts,
            ) -> Ts2 {
                self.build_handle(
                    #struct_or_enum_ucc::Enum,
                    ident_273dd063,
                    generics_84bc3f7f,
                    content_ts
                )
            }
        }
    };
    generated.into()
}
