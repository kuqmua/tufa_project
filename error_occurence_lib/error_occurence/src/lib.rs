use naming::{
    CodeOccurenceSc, IntoSerializeDeserializeVersionSc, ValueSc, WithSerializeDeserializeUcc,
    parameter::SelfWithSerializeDeserializeUcc,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use token_patterns::StdStringString;
#[proc_macro_derive(
    ErrorOccurence,
    attributes(
        eo_to_std_string_string,
        eo_to_std_string_string_serialize_deserialize,
        eo_error_occurence,
        eo_vec_to_std_string_string,
        eo_vec_to_std_string_string_serialize_deserialize,
        eo_vec_error_occurence,
        eo_hashmap_key_std_string_string_value_to_std_string_string,
        eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize,
        eo_hashmap_key_std_string_string_value_error_occurence,
    )
)]
pub fn error_occurence(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SuportedEnumVariant {
        Named,
        Unnamed,
    }
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("d94f091a-ed2a-48d5-ba75-5e47502f3bef");
    let ident = &syn_derive_input.ident;
    let generic_parameters = &syn_derive_input
        .generics
        .params
        .iter()
        .map(|el_a6a747c1| match &el_a6a747c1 {
            syn::GenericParam::Type(value) => &value.ident,
            syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => {
                panic!("3ce82d11-36be-49ab-b521-21486f3fe22a")
            }
        })
        .collect::<Vec<&syn::Ident>>();
    let ident_with_serialize_deserialize_ucc = SelfWithSerializeDeserializeUcc::from_tokens(&ident);
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("d98214f7-c406-44c7-9cca-b98192949a95");
    };
    let supported_enum_variant = {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        assert!(
            !data_enum.variants.is_empty(),
            "27275ae6-f36d-4aa0-ac0b-6e678e1ccfe3"
        );
        data_enum
            .variants
            .iter()
            .for_each(|variant| match &variant.fields {
                syn::Fields::Named(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Unnamed),
                            "bf6be520-cc31-4c54-a5ee-707df114e247"
                        );
                    }
                    None => {
                        all_equal = Some(SuportedEnumVariant::Named);
                    }
                },
                syn::Fields::Unnamed(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Named),
                            "02090d85-7ffb-41f2-9b8c-9ce6792bb3d4"
                        );
                    }
                    None => {
                        all_equal = Some(SuportedEnumVariant::Unnamed);
                    }
                },
                syn::Fields::Unit => panic!("2f2e9385-59e4-43b7-a230-2adaa2bfc38a"),
            });
        all_equal.expect("b9da972a-f38b-4217-939c-54ffd56f0301")
    };
    let maybe_generic_parameters_ts = if generic_parameters.is_empty() {
        Ts2::new()
    } else {
        quote! {<#(#generic_parameters),*>}
    };
    let maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts =
        if generic_parameters.is_empty() {
            Ts2::new()
        } else {
            let value = generic_parameters
                .iter()
                .map(|el_4ee89bd7| quote! {#el_4ee89bd7: error_occurence_lib::ToStdStringString});
            quote! {<#(#value),*>}
        };
    let gen_enum_ident_with_serialize_deserialize_ts = |variants_ts: &dyn ToTokens| {
        quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serialize_deserialize_ucc #maybe_generic_parameters_ts {
                #variants_ts
            }
        }
    };
    let gen_impl_ident_into_serialize_deserialize_version_ts = |variants: &dyn ToTokens| {
        quote! {
            impl #maybe_generic_parameters_ts #ident #maybe_generic_parameters_ts {
                pub fn #IntoSerializeDeserializeVersionSc(self) -> #ident_with_serialize_deserialize_ucc #maybe_generic_parameters_ts {
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
            let code_occurence_sc_str = CodeOccurenceSc.to_string();
            //todo maybe impl display was a bad idea. .to_string() casts is dangerous
            let impl_std_fmt_display_handle_content_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_f497ea11| {
                    let el_ident = &el_f497ea11.ident;
                    let fields = if let syn::Fields::Named(fields) = &el_f497ea11.fields {
                        &fields.named
                    } else {
                        panic!("f64e0d21-349c-48db-83ef-b06064333b3d");
                    };
                    let fields_idents_excluding_code_occurence_ts = {
                        let acc_ts = fields.iter()
                        .filter(|el_e26e2572| *el_e26e2572.ident.as_ref().expect("07504636-310e-43cf-aa3b-afd7443987f0") != *code_occurence_sc_str)
                        .map(|el_e4070354| el_e4070354.ident.as_ref().expect("971ace15-e8cb-4780-8589-2da5e99e5587"))
                        .collect::<Vec<&syn::Ident>>();
                        if acc_ts.is_empty() {
                            Ts2::new()
                        }
                        else {
                            quote!{#(#acc_ts),*,}
                        }
                    };
                    let fields_format_excluding_code_occurence_ts = gen_quotes::double_quotes_ts(
                        &fields.iter()
                        .filter(|el_6ba47e94| *el_6ba47e94.ident.as_ref().expect("3d70a4f4-046d-4f37-af70-d6c7b1c46b9d") != *code_occurence_sc_str)
                        .fold(
                            String::new(),
                            |mut acc_924ab1b3, el_e405984a| {
                                use std::fmt::Write as _;
                                let current_ident = &el_e405984a.ident.as_ref().expect("2e7cd5fe-7653-4c10-8977-526b061d6748");
                                assert!(writeln!(acc_924ab1b3, "{current_ident}: {{}}").is_ok(), "ab44c70f-092e-46a0-8daa-56fe44395228");
                                acc_924ab1b3
                            }
                        )
                    );
                    let fields_format_values_excluding_code_occurence_ts = fields.iter()
                    .filter(|el_48337db8| *el_48337db8.ident.as_ref().expect("f6f6fb24-bdf2-4bb6-a2be-32462758dba9") != *code_occurence_sc_str)
                    .map(|el_f00312fe| {
                        let current_ident = &el_f00312fe.ident.as_ref().expect("e97b25b9-49d3-4f89-a18a-4e77355c4c9c");
                        match macros_helpers::ErrorOccurenceFieldAttribute::try_from(el_f00312fe).expect("8ff56aeb-8636-43d6-b8c1-f8fb0486f817") {
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
                                quote! {
                                    error_occurence_lib::ToStdStringString::to_std_string_string(#current_ident)
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(&quote! {acc_52e70d22, "\n {element}"}, &quote! {panic!("c751d54a-b008-493f-a97d-2f8e381780d5");});
                                quote! {
                                    #current_ident.to_string().lines().fold(
                                        #StdStringString::new(),
                                        |mut acc_52e70d22, element| {
                                            #if_write_is_err_ts
                                            acc_52e70d22
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(&quote! {acc_a9ba7521, "\n {el_6e4f53ad}"}, &quote! {panic!("b35ed9f5-525b-4287-9d6e-0be1d72a0874");});
                                quote! {
                                    #current_ident.iter().fold(
                                        #StdStringString::new(),
                                        |mut acc_ac447c4b, el_36630fcf| {
                                            acc_ac447c4b.push_str(
                                                &error_occurence_lib::ToStdStringString::to_std_string_string(el_36630fcf)
                                                .lines()
                                                .fold(
                                                    #StdStringString::new(),
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
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(&quote! {acc_1bbd5ef3, "\n {el_3f2fe01d}"}, &quote! {panic!("4dfdd18d-5fca-41ba-b556-36ceb1b18b60");});
                                quote! {
                                    #current_ident.iter().fold(
                                        #StdStringString::new(),
                                        |mut acc_c5adba93, el_37c46c8a| {
                                            acc_c5adba93.push_str(&el_37c46c8a.to_string().lines().fold(
                                                #StdStringString::new(),
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
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(&quote! {acc_06473093, "\n {key}: {}", &error_occurence_lib::ToStdStringString::to_std_string_string(#ValueSc)}, &quote! {panic!("d030580a-6c03-4913-9088-b77316b9f285");});
                                quote! {
                                    #current_ident.iter().fold(
                                        #StdStringString::new(),
                                        |mut acc_06473093, (key, #ValueSc)| {
                                            #if_write_is_err_ts
                                            acc_06473093
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                                    &{
                                        let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(&quote! {acc_addfc699, "\n  {el_8b8f577e}"}, &quote! {panic!("d0492fbf-2da0-4b02-bec3-9d011bf08999");});
                                        quote! {
                                            acc_a47e1ba7,
                                            "\n {key}: {}",
                                            #ValueSc.to_string().lines().fold(
                                                #StdStringString::new(),
                                                |mut acc_addfc699, el_8b8f577e| {
                                                    #if_write_is_err_ts
                                                    acc_addfc699
                                                }
                                            )
                                        }
                                    },
                                    &quote! {panic!("75f6432a-9854-48cc-9a0d-c1dbf6774433");},
                                );
                                quote! {
                                    #current_ident.iter().fold(
                                        #StdStringString::new(),
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
                            #fields_idents_excluding_code_occurence_ts
                            ..
                        } => {
                            format!(
                                #fields_format_excluding_code_occurence_ts,
                                #(#fields_format_values_excluding_code_occurence_ts),*
                            )
                        }
                    }
                });
                let code_occurence_variants_ts =
                    data_enum
                        .variants
                        .iter()
                        .enumerate()
                        .map(|(index, element)| {
                            let el_ident = &element.ident;
                            if index == 0 {
                                quote! {
                                    Self::#el_ident {
                                        #CodeOccurenceSc,
                                        ..
                                    }
                                }
                            } else {
                                quote! {
                                    | Self::#el_ident {
                                        #CodeOccurenceSc,
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
                            #(#code_occurence_variants_ts)*
                            => #CodeOccurenceSc
                        }
                    )
                }
            };
            let impl_std_fmt_display_for_ident_ts = macros_helpers::gen_impl_std_fmt_display_ts(
                &maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts,
                &ident,
                &maybe_generic_parameters_ts,
                &impl_std_fmt_display_handle_content_ts,
            );
            let impl_ident_into_serialize_deserialize_version_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_7d5a4c39| {
                    let el_ident = &el_7d5a4c39.ident;
                    let fields = if let syn::Fields::Named(fields) = &el_7d5a4c39.fields {
                        &fields.named
                    } else {
                        panic!("238b402b-0407-417a-bce7-21bf0d4fe4d6");
                    };
                    let fields_idents_ts = fields.iter().map(|el_cddf556e| &el_cddf556e.ident);
                    let fields_into_serialize_deserialize_version_excluding_code_occurence_ts = fields.iter()
                    .filter(|el_6a54951c| *el_6a54951c.ident.as_ref().expect("0488fc4c-be15-431b-904b-8bf6a5b2e8e6") != *code_occurence_sc_str)
                    .map(|el_d7e120a3| {
                        let current_ident = &el_d7e120a3.ident.as_ref().expect("9a672ac2-5184-4427-9d88-70cb2a0bd199");
                        let conversion_ts = match macros_helpers::ErrorOccurenceFieldAttribute::try_from(el_d7e120a3).expect("449c3781-1900-4ed4-b784-485db5a08508") {
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote! {
                                    #current_ident: {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(&#current_ident)
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize | macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                quote! {
                                    #current_ident
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                quote! {
                                    #current_ident: {
                                        #current_ident.into_serialize_deserialize_version()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote! {
                                    #current_ident: {
                                        #current_ident.into_iter().map(|el_3c145d8e|error_occurence_lib::ToStdStringString::to_std_string_string(&el_3c145d8e)).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                quote! {
                                    #current_ident: {
                                        #current_ident.into_iter().map(|el_607695c6|el_607695c6.into_serialize_deserialize_version()).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                                quote! {
                                    #current_ident: {
                                        #current_ident.into_iter().map(|(key, value)|
                                            (key, error_occurence_lib::ToStdStringString::to_std_string_string(&value))
                                        ).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                quote! {
                                    #current_ident: {
                                        #current_ident.into_iter().map(
                                            |(key, value)|(key, value.into_serialize_deserialize_version())
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
                        } => #ident_with_serialize_deserialize_ucc::#el_ident {
                            #(#fields_into_serialize_deserialize_version_excluding_code_occurence_ts)*
                            #CodeOccurenceSc,
                        }
                    }
                });
                gen_impl_ident_into_serialize_deserialize_version_ts(&quote! {#(#variants_ts),*})
            };
            let enum_ident_with_serialize_deserialize_ts = {
                let variants_ts = data_enum
                    .variants
                    .iter()
                    .map(macros_helpers::gen_serialize_deserialize_version_of_named_syn_variant);
                gen_enum_ident_with_serialize_deserialize_ts(&quote! {#(#variants_ts),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_ts = macros_helpers::gen_impl_std_fmt_display_ts(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts, &ident_with_serialize_deserialize_ucc, &maybe_generic_parameters_ts, &impl_std_fmt_display_handle_content_ts);
            let impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_ts =
                macros_helpers::gen_impl_error_occurence_lib_to_std_string_string_ts(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts, &ident_with_serialize_deserialize_ucc, &maybe_generic_parameters_ts, &quote! {format!("{self}")});
            quote! {
                #impl_std_fmt_display_for_ident_ts
                #impl_ident_into_serialize_deserialize_version_ts
                #enum_ident_with_serialize_deserialize_ts
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_ts
                #impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_ts
            }
        }
        SuportedEnumVariant::Unnamed => {
            let gen_display_formatter_unnamed_ts = || {
                let variants_ts = data_enum.variants.iter().map(|el_f99bd80d| {
                    let el_ident = &el_f99bd80d.ident;
                    quote! {Self::#el_ident(value) => value}
                });
                quote! {match self { #(#variants_ts),* }}
            };
            let impl_std_fmt_display_for_ident_ts = macros_helpers::gen_impl_std_fmt_display_ts(
                &maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts,
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
            let impl_ident_into_serialize_deserialize_version_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_0e2b2f9c| {
                    let el_ident = &el_0e2b2f9c.ident;
                    quote! {
                        Self::#el_ident(value) => #ident_with_serialize_deserialize_ucc::#el_ident(
                            value.#IntoSerializeDeserializeVersionSc(),
                        )
                    }
                });
                gen_impl_ident_into_serialize_deserialize_version_ts(&quote! {#(#variants_ts),*})
            };
            let enum_ident_with_serialize_deserialize_ts = {
                let variants_ts = data_enum.variants.iter().map(|el_0f06fa87| {
                    let el_ident = &el_0f06fa87.ident;
                    let fields = if let syn::Fields::Unnamed(fields) = &el_0f06fa87.fields {
                        &fields.unnamed
                    } else {
                        panic!("5749e920-0ec8-480a-a16b-3b48e6ddb29f");
                    };
                    let inner_type_with_serialize_deserialize_ts = {
                        format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "d7a6b955-3795-4e0c-a990-b06734e9d923");
                                let field_type = &fields
                                    .iter()
                                    .next()
                                    .expect("8a80c36d-0b80-4ade-a4aa-2febb8079bd9")
                                    .ty;
                                quote! {#field_type}.to_string()
                            },
                            WithSerializeDeserializeUcc
                        )
                        .parse::<Ts2>()
                        .expect("9ff40f7e-b7b0-4226-8663-d2de6d5e05ed")
                    };
                    quote! {#el_ident(#inner_type_with_serialize_deserialize_ts)}
                });
                gen_enum_ident_with_serialize_deserialize_ts(&quote! {#(#variants_ts),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_ts = macros_helpers::gen_impl_std_fmt_display_ts(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_ts, &ident_with_serialize_deserialize_ucc, &maybe_generic_parameters_ts, &{
                let display_formatter_unnamed_ts = gen_display_formatter_unnamed_ts();
                quote! {
                    write!(
                        f,
                        "{}",
                        #display_formatter_unnamed_ts
                    )
                }
            });
            //todo maybe make a trait?
            quote! {
                #impl_std_fmt_display_for_ident_ts
                #impl_ident_into_serialize_deserialize_version_ts
                #enum_ident_with_serialize_deserialize_ts
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_ts
            }
        }
    };
    let generated = quote! {#tokens};
    // println!("{generated} ");
    // if ident == "" {
    //     macros_helpers::maybe_write_ts_into_file(
    //         macros_helpers::ShouldWriteTokenStreamIntoFile::True,
    //         "error_occurence",
    //         &generated,
    //         &macros_helpers::FormatWithCargofmt::True
    //     );
    // }
    generated.into()
}
