#[proc_macro]
pub fn generate_struct_or_enum_derive_ts_builder(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[derive(Clone)]
    struct Element {
        derive_trait_name_if_snake_case: proc_macro2::TokenStream,
        derive_trait_name_snake_case: proc_macro2::TokenStream,
        derive_trait_name_upper_camel_case: proc_macro2::TokenStream,
        trait_type: proc_macro2::TokenStream,
    }
    let make_pub_snake_case_ts = quote::quote! {make_pub};
    let make_pub_if_snake_case_ts = quote::quote! {make_pub_if};
    let make_pub_upper_camel_case_ts = quote::quote! {MakePub};
    let element_vec = serde_json::from_str::<Vec<String>>(&input_ts.to_string())
        .expect("c5d09740-51b2-4c11-9b66-931622d1a053")
        .into_iter()
        .map(|element_4f4a2c74| {
            let snake_case = {
                let mut result = String::with_capacity(element_4f4a2c74.len());
                let mut prev_is_underscore = false;
                let mut prev_is_lowercase = false;
                for element_e5df7ee3 in element_4f4a2c74.chars() {
                    if element_e5df7ee3.is_alphabetic() {
                        if element_e5df7ee3.is_uppercase() {
                            if prev_is_lowercase && !prev_is_underscore {
                                result.push('_');
                            }
                            for element_8aa0e3a1 in element_e5df7ee3.to_lowercase() {
                                result.push(element_8aa0e3a1);
                            }
                            prev_is_lowercase = false;
                        } else {
                            result.push(element_e5df7ee3);
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
                derive_trait_name_upper_camel_case: {
                    let value =
                        naming::parameter::DeriveSelfUpperCamelCase::from_display(&snake_case);
                    quote::quote! {#value}
                },
                derive_trait_name_snake_case: {
                    let value = naming::parameter::DeriveSelfSnakeCase::from_display(&snake_case);
                    quote::quote! {#value}
                },
                derive_trait_name_if_snake_case: {
                    let value = naming::parameter::DeriveSelfIfSnakeCase::from_display(&snake_case);
                    quote::quote! {#value}
                },
                trait_type: element_4f4a2c74
                    .parse::<proc_macro2::TokenStream>()
                    .expect("8672240f-97b3-40f5-bf14-dc4b13af528f"),
            }
        })
        .collect::<Vec<Element>>();
    let (make_pub_pub_enum_ts, pub_enum_derive_vec_ts) = {
        fn generate_pun_enum_ts(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {
                #[derive(Debug, Clone, Copy)]
                pub enum #ident {
                    True,
                    False
                }
            }
        }
        (
            generate_pun_enum_ts(&make_pub_upper_camel_case_ts),
            element_vec.iter().map(|element_4f4a2c74| {
                generate_pun_enum_ts(&element_4f4a2c74.derive_trait_name_upper_camel_case)
            }),
        )
    };
    let (make_pub_derive_trait_name_bool_ts, field_vec_ts) = {
        fn generate_derive_trait_name_bool_ts(
            ident: &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream {
            quote::quote! {#ident: bool,}
        }
        (
            generate_derive_trait_name_bool_ts(&make_pub_snake_case_ts),
            element_vec.iter().map(|element_03225620| {
                generate_derive_trait_name_bool_ts(&element_03225620.derive_trait_name_snake_case)
            }),
        )
    };
    let (make_pub_derive_and_derive_if_ts, derive_and_derive_if_vec_ts) = {
        (
            quote::quote! {
                pub const fn #make_pub_snake_case_ts(mut self) -> Self {
                    self.#make_pub_snake_case_ts = true;
                    self
                }
                pub const fn #make_pub_if_snake_case_ts(mut self, condition: #make_pub_upper_camel_case_ts) -> Self {
                    if let #make_pub_upper_camel_case_ts::True = condition {
                        self.#make_pub_snake_case_ts = true;
                    }
                    self
                }
            },
            element_vec.iter().map(|element_09fab389|{
                let derive_trait_name_upper_camel_case = &element_09fab389.derive_trait_name_upper_camel_case;
                let derive_trait_name_snake_case = &element_09fab389.derive_trait_name_snake_case;
                let derive_trait_name_if_snake_case = &element_09fab389.derive_trait_name_if_snake_case;
                quote::quote!{
                    pub const fn #derive_trait_name_snake_case(mut self) -> Self {
                        self.#derive_trait_name_snake_case = true;
                        self
                    }
                    pub const fn #derive_trait_name_if_snake_case(mut self, condition: #derive_trait_name_upper_camel_case) -> Self {
                        if let #derive_trait_name_upper_camel_case::True = condition {
                            self.#derive_trait_name_snake_case = true;
                        }
                        self
                    }
                }
            })
        )
    };
    let if_self_derive_acc_push_vec_ts = element_vec.iter().map(|element_09bcde51| {
        let derive_trait_name_snake_case = &element_09bcde51.derive_trait_name_snake_case;
        let trait_type = &element_09bcde51.trait_type;
        quote::quote! {
            if self.#derive_trait_name_snake_case {
                acc_2a71375c.push(quote::quote!{#trait_type});
            }
        }
    });
    let struct_or_enum_derive_ts_builder_upper_camel_case =
        quote::quote! {StructOrEnumDeriveTokenStreamBuilder};
    let struct_or_enum_upper_camel_case = quote::quote! {StructOrEnum};
    let generated: proc_macro2::TokenStream = quote::quote! {
        #make_pub_pub_enum_ts
        #(#pub_enum_derive_vec_ts)*
        #[derive(Debug, Clone, Copy)]
        enum #struct_or_enum_upper_camel_case {
            Struct,
            Enum
        }
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #struct_or_enum_derive_ts_builder_upper_camel_case {
            #make_pub_derive_trait_name_bool_ts
            #(#field_vec_ts)*
        }
        impl #struct_or_enum_derive_ts_builder_upper_camel_case {
            pub fn new() -> Self {
                Self::default()
            }
            #make_pub_derive_and_derive_if_ts
            #(#derive_and_derive_if_vec_ts)*
            fn build_handle(
                self,
                struct_or_enum: #struct_or_enum_upper_camel_case,
                current_ident: &dyn quote::ToTokens,
                content_ts: &dyn quote::ToTokens,
            ) -> proc_macro2::TokenStream {
                let maybe_pub_ts = self.#make_pub_snake_case_ts.then(|| quote::quote!{pub});
                let derive_ts = {
                    let mut acc_2a71375c = Vec::new();
                    #(#if_self_derive_acc_push_vec_ts)*
                    acc_2a71375c
                };
                let struct_or_enum_ts = match struct_or_enum {
                    #struct_or_enum_upper_camel_case::Struct => quote::quote!{struct},
                    #struct_or_enum_upper_camel_case::Enum => quote::quote!{enum},
                };
                // quote::quote! {
                //     #[derive(#(#derive_ts),*)]
                //     #maybe_pub_ts #struct_or_enum_ts #current_ident #content_ts
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
                                            ::quote::ToTokens::to_tokens(&derive_ts, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::quote::ToTokens::to_tokens(&maybe_pub_ts, &mut _s);
                    ::quote::ToTokens::to_tokens(&struct_or_enum_ts, &mut _s);
                    ::quote::ToTokens::to_tokens(&current_ident, &mut _s);
                    ::quote::ToTokens::to_tokens(&content_ts, &mut _s);
                    _s
                }
            }
            pub fn build_struct(
                self,
                current_ident: &dyn quote::ToTokens,
                content_ts: &dyn quote::ToTokens,
            ) -> proc_macro2::TokenStream {
                self.build_handle(#struct_or_enum_upper_camel_case::Struct, current_ident, content_ts)
            }
            pub fn build_enum(
                self,
                current_ident: &dyn quote::ToTokens,
                content_ts: &dyn quote::ToTokens,
            ) -> proc_macro2::TokenStream {
                self.build_handle(#struct_or_enum_upper_camel_case::Enum, current_ident, content_ts)
            }
        }
    };
    generated.into()
}
