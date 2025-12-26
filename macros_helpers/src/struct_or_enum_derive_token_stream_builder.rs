generate_struct_or_enum_derive_token_stream_builder::generate_struct_or_enum_derive_token_stream_builder!([
    {
        "trait_name": "debug",
        "trait_type": "Debug"
    },
    {
        "trait_name": "default",
        "trait_type": "Default"
    },
    {
        "trait_name": "clone",
        "trait_type": "Clone"
    },
    {
        "trait_name": "copy",
        "trait_type": "Copy"
    },
    {
        "trait_name": "partial_eq",
        "trait_type": "PartialEq"
    },
    {
        "trait_name": "eq",
        "trait_type": "Eq"
    },
    {
        "trait_name": "partial_ord",
        "trait_type": "PartialOrd"
    },
    {
        "trait_name": "ord",
        "trait_type": "Ord"
    },
    {
        "trait_name": "serde_serialize",
        "trait_type": "serde::Serialize"
    },
    {
        "trait_name": "serde_deserialize",
        "trait_type": "serde::Deserialize"
    },
    {
        "trait_name": "utoipa_to_schema",
        "trait_type": "utoipa::ToSchema"
    },
    {
        "trait_name": "schemars_json_schema",
        "trait_type": "schemars::JsonSchema"
    },
    {
        "trait_name": "thiserror_error",
        "trait_type": "thiserror::Error"
    },
    {
        "trait_name": "error_occurence_lib_error_occurence",
        "trait_type": "error_occurence_lib::ErrorOccurence"
    }
]);
impl StructOrEnumDeriveTokenStreamBuilder {
            fn build_handle(
                self,
                struct_or_enum: StructOrEnum,
                current_ident: &dyn quote::ToTokens,
                content_token_stream: &dyn quote::ToTokens,
            ) -> proc_macro2::TokenStream {
                let maybe_pub_token_stream = self.make_pub.then(|| quote::quote!{pub});
                let derive_token_stream = {
                    let mut acc = vec![];
                    if self.derive_debug {
                        acc.push(quote::quote!{Debug});
                    }
                    if self.derive_default {
                        acc.push(quote::quote!{Default});
                    }
                    if self.derive_clone {
                        acc.push(quote::quote!{Clone});
                    }
                    if self.derive_copy {
                        acc.push(quote::quote!{Copy});
                    }
                    if self.derive_partial_eq {
                        acc.push(quote::quote!{PartialEq});
                    }
                    if self.derive_eq {
                        acc.push(quote::quote!{Eq});
                    }
                    if self.derive_partial_ord {
                        acc.push(quote::quote!{PartialOrd});
                    }
                    if self.derive_ord {
                        acc.push(quote::quote!{Ord});
                    }
                    if self.derive_serde_serialize {
                        acc.push(quote::quote!{serde::Serialize});
                    }
                    if self.derive_serde_deserialize {
                        acc.push(quote::quote!{serde::Deserialize});
                    }
                    if self.derive_utoipa_to_schema {
                        acc.push(quote::quote!{utoipa::ToSchema});
                    }
                    if self.derive_schemars_json_schema {
                        acc.push(quote::quote!{schemars::JsonSchema});
                    }
                    if self.derive_thiserror_error {
                        acc.push(quote::quote!{thiserror::Error});
                    }
                    if self.derive_error_occurence_lib_error_occurence {
                        acc.push(quote::quote!{error_occurence_lib::ErrorOccurence});
                    }
                    acc
                };
                let struct_or_enum_token_stream = match struct_or_enum {
                    StructOrEnum::Struct => quote::quote!{struct},
                    StructOrEnum::Enum => quote::quote!{enum},
                };
                // quote::quote! {
                //     #[derive(#(#derive_token_stream),*)]
                //     #maybe_pub_token_stream #struct_or_enum_token_stream #current_ident #content_token_stream
                // }
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
                                        let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                        #[allow(unused_mut)]
                                        let (mut derive_token_stream, i) = derive_token_stream
                                            .quote_into_iter();
                                        let has_iter = has_iter | i;
                                        let _: ::quote::__private::HasIterator = has_iter;
                                        loop {
                                            let derive_token_stream = match derive_token_stream.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::quote::ToTokens::to_tokens(&derive_token_stream, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::quote::ToTokens::to_tokens(&maybe_pub_token_stream, &mut _s);
                    ::quote::ToTokens::to_tokens(&struct_or_enum_token_stream, &mut _s);
                    ::quote::ToTokens::to_tokens(&current_ident, &mut _s);
                    ::quote::ToTokens::to_tokens(&content_token_stream, &mut _s);
                    _s
                }
            }
}