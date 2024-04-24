#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(InitFromEnv)]
pub fn init_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput =
        syn::parse(input).expect("InitFromEnvWithPanicIfFailed syn::parse(input) failed");
    match ast.data {
        syn::Data::Struct(datastruct) => {
            let ident = &ast.ident;
            let error_ident = syn::Ident::new(&format!("{ident}Error"), ident.span());
            let error_enum_ident = syn::Ident::new(&format!("{ident}ErrorEnum"), ident.span());
            let error_std_env_var_ident =
                syn::Ident::new(&format!("{ident}StdEnvVar"), ident.span());
            let error_std_env_var_enum_ident =
                syn::Ident::new(&format!("{ident}ErrorStdEnvEnum"), ident.span());
            let error_parse_ident = syn::Ident::new(&format!("{ident}Parse"), ident.span());
            let error_parse_enum_ident =
                syn::Ident::new(&format!("{ident}ErrorParseEnum"), ident.span());
            let capacity = datastruct.fields.len();
            let mut generated_init_struct_fields: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(capacity);
            let mut generated_functions: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(capacity);
            let mut generated_enum_error_std_env_var_variants: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(capacity);
            let mut generated_enum_error_parse_variants: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(capacity);
            datastruct.fields.into_iter().for_each(|field| {
                let (
                    enum_variant_ident_value,
                    env_var_name,
                    enum_variant_ident
                ) = field.ident.as_ref().map_or_else(|| panic!(
                    "InitFromEnvWithPanicIfFailed {}",
                    naming_constants::FIELD_IDENT_IS_NONE
                ), |field_ident| {
                    generated_init_struct_fields.push(quote::quote! {
                        #field_ident: #field_ident
                    });
                    (
                        field_ident,
                        syn::Ident::new(
                            &convert_case::Casing::to_case(&format!("{field_ident}"), convert_case::Case::Snake)
                                .to_uppercase(),
                            ident.span(),
                        ),
                        syn::Ident::new(
                            &convert_case::Casing::to_case(&format!("{field_ident}"),convert_case::Case::UpperCamel),
                            ident.span(),
                        )
                    )
                });
                let (
                    enum_variant_type,
                    enum_variant_type_as_string,
                ) = match &field.ty {
                    //todo: add different types support
                    syn::Type::Path(type_path) => {
                        let enum_variant_type = &type_path.path;
                        let string_handle = {
                            let mut string_handle = std::string::String::from("");
                            if type_path.path.segments.len() == 1 {
                                let first_segment = type_path.path.segments.first().unwrap_or_else(||panic!("failed to get first type_path.path.segments element"));
                                string_handle = format!("{}", first_segment.ident);
                            } else {
                                for seg in &type_path.path.segments {
                                    string_handle.push_str(&format!("{}:", seg.ident));
                                }
                                if !string_handle.is_empty() {
                                    let _: std::option::Option<std::primitive::char> = string_handle.pop();
                                }
                            }
                            string_handle
                        };
                        (
                            enum_variant_type,
                            syn::LitStr::new(&string_handle, ident.span())
                        )
                    }
                    _ => panic!("InitFromEnvWithPanicIfFailed field.ty supports only syn::Type::Path!"),
                };
                let env_var_name_as_screaming_snake_case_string =
                    syn::LitStr::new(&format!("{env_var_name}"), ident.span());
                generated_functions.push(quote::quote! {
                    let #enum_variant_ident_value: #enum_variant_type;
                    match std::env::var(#env_var_name_as_screaming_snake_case_string) {
                        Err(error) => {
                            return Err(#error_ident {
                                source: #error_enum_ident::#error_std_env_var_ident(#error_std_env_var_enum_ident::#enum_variant_ident {
                                    source: error,
                                    env_var_name: #env_var_name_as_screaming_snake_case_string,
                                }),
                                was_dotenv_enable,
                            });
                        },
                        Ok(string_handle) => {
                            match string_handle.parse::<#enum_variant_type>() {
                                Err(_) => {
                                    return Err(#error_ident {
                                        source: #error_enum_ident::#error_parse_ident(#error_parse_enum_ident::#enum_variant_ident{
                                            env_var_name: #env_var_name_as_screaming_snake_case_string,
                                            expected_env_var_type: #enum_variant_type_as_string,
                                        }),
                                        was_dotenv_enable,
                                    });
                                },
                                Ok(handle) => {
                                    #enum_variant_ident_value = handle;
                                },
                            }
                        },
                    }
                });
                generated_enum_error_std_env_var_variants.push(quote::quote! {
                    #enum_variant_ident {
                        source: std::env::VarError,
                        env_var_name: &'static str,
                    },
                });
                generated_enum_error_parse_variants.push(quote::quote! {
                    #enum_variant_ident {
                        env_var_name: &'static str,
                        expected_env_var_type: &'static str,
                    },
                });
            });
            let gen = quote::quote! {
                #[derive(Debug)]
                pub struct #error_ident {
                    pub source: #error_enum_ident,
                    pub was_dotenv_enable: bool,
                }
                #[derive(Debug)]
                pub enum #error_enum_ident {
                    #error_std_env_var_ident(#error_std_env_var_enum_ident),
                    #error_parse_ident(#error_parse_enum_ident),
                }
                #[derive(Debug)]
                pub enum #error_std_env_var_enum_ident {
                    #(#generated_enum_error_std_env_var_variants)*
                }
                #[derive(Debug)]
                pub enum #error_parse_enum_ident {
                    #(#generated_enum_error_parse_variants)*
                }
                impl #ident {
                    #[must_use]
                    pub fn new() -> Result<Self, #error_ident> {
                        let was_dotenv_enable = dotenv::dotenv().is_ok();
                        #(#generated_functions)*
                        Ok(
                            Self {
                                #(#generated_init_struct_fields,)*
                            }
                        )
                    }
                }
            };
            // println!("{gen}");
            gen.into()
        }
        _ => panic!("InitFromEnvWithPanicIfFailed only works on Struct"),
    }
}
