#[proc_macro_derive(InitFromEnv)]
pub fn derive_init_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use convert_case::Casing;
    let ast: syn::DeriveInput = syn::parse(input).expect("InitFromEnv syn::parse(input) failed");
    let ident = &ast.ident;
    let error_ident = syn::Ident::new(&format!("{ident}Error"), ident.span());
    let error_enum_ident = syn::Ident::new(&format!("{ident}ErrorEnum"), ident.span());
    let error_std_env_var_ident = syn::Ident::new(&format!("{ident}StdEnvVar"), ident.span());
    let error_std_env_var_enum_ident =
        syn::Ident::new(&format!("{ident}ErrorStdEnvEnum"), ident.span());
    let error_parse_ident = syn::Ident::new(&format!("{ident}Parse"), ident.span());
    let error_parse_enum_ident = syn::Ident::new(&format!("{ident}ErrorParseEnum"), ident.span());
    let value_suffix = "_value";
    let generated_init_struct_fields = match ast.data.clone() {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident: syn::Ident;
            let enum_variant_ident_value: syn::Ident;
            match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => {
                    enum_variant_ident = field_ident.clone();
                    enum_variant_ident_value =
                        syn::Ident::new(&format!("{field_ident}{value_suffix}"), ident.span());
                }
            };
            quote::quote! {
                #enum_variant_ident: #enum_variant_ident_value,
            }
        }),
        _ => panic!("InitFromEnv only works on Struct"),
    };
    let generated_functions = match ast.data.clone() {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident_pascal_case: syn::Ident;
            let enum_variant_ident_value: syn::Ident;
            let env_var_name: syn::Ident;
            let env_var_name_as_snake_case_string: syn::LitStr;
            match field.ident.clone() {
                None => panic!("field.ident is None"),
                Some(field_ident) => {
                    enum_variant_ident_pascal_case = syn::Ident::new(
                        &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                        ident.span(),
                    );
                    enum_variant_ident_value =
                        syn::Ident::new(&format!("{field_ident}{value_suffix}"), ident.span());
                    env_var_name = syn::Ident::new(
                        &format!("{field_ident}")
                            .to_case(convert_case::Case::Snake)
                            .to_uppercase(),
                        ident.span(),
                    );
                    env_var_name_as_snake_case_string =
                        syn::LitStr::new(&format!("{field_ident}"), ident.span());
                }
            };
            let enum_variant_type: syn::Path;
            let enum_variant_type_as_string: syn::LitStr;
            match field.ty {
                //todo: add different types support
                syn::Type::Path(type_path) => {
                    enum_variant_type = type_path.path.clone();
                    let mut string_handle = String::from("");
                    if type_path.path.segments.len() == 1 {
                        string_handle = format!("{}", type_path.path.segments[0].ident);
                    } else {
                        for seg in type_path.path.segments {
                            string_handle.push_str(&format!("{}:", seg.ident));
                        }
                        if !string_handle.is_empty() {
                            string_handle.pop();
                        }
                    }
                    enum_variant_type_as_string = syn::LitStr::new(&string_handle, ident.span());
                }
                _ => panic!("field.ty is not a syn::Type::Path!"),
            };
            let env_var_name_as_screaming_snake_case_string =
                syn::LitStr::new(&format!("{env_var_name}"), ident.span());
            //todo: add parsing error
            quote::quote! {
                let #enum_variant_ident_value: #enum_variant_type;
                match std::env::var(#env_var_name_as_screaming_snake_case_string) {
                    Err(e) => {
                        return Err(
                            #error_ident {
                                source: Box::new(
                                    #error_enum_ident::#error_std_env_var_ident(
                                        #error_std_env_var_enum_ident::#enum_variant_ident_pascal_case {
                                            source: e,
                                            env_var_name: #env_var_name_as_screaming_snake_case_string,
                                            field_name: #env_var_name_as_snake_case_string,
                                            where_was: tufa_common::where_was::WhereWas {
                                                time: chrono::DateTime::<chrono::Utc>::from_utc(chrono::Local::now().naive_utc(), chrono::Utc)
                                                  .with_timezone(&chrono::FixedOffset::east(crate::lazy_static::config::CONFIG.timezone)),
                                                  file: file!(),
                                                  line: line!(),
                                                  column: column!(),
                                                },
                                        }
                                    )
                                ),
                                was_dotenv_enable,
                            }
                        );
                    },
                    Ok(string_handle) => {
                        match string_handle.parse::<#enum_variant_type>() {
                            Err(e) => {
                                return Err(
                                    #error_ident {
                                        source: Box::new(
                                            #error_enum_ident::#error_parse_ident(
                                                #error_parse_enum_ident::#enum_variant_ident_pascal_case {
                                                    env_var_name: #env_var_name_as_screaming_snake_case_string,
                                                    field_name: #env_var_name_as_snake_case_string,
                                                    expected_env_var_type: #enum_variant_type_as_string,
                                                    where_was: tufa_common::where_was::WhereWas {
                                                        time: chrono::DateTime::<chrono::Utc>::from_utc(chrono::Local::now().naive_utc(), chrono::Utc)
                                                            .with_timezone(&chrono::FixedOffset::east(crate::lazy_static::config::CONFIG.timezone)),
                                                        file: file!(),
                                                        line: line!(),
                                                        column: column!(),
                                                    },
                                                }
                                            )
                                        ),
                                        was_dotenv_enable,
                                    }
                                );
                            },
                            Ok(handle) => {
                                #enum_variant_ident_value = handle;
                            },
                        }
                    },
                }
            }
        }),
        _ => panic!("InitFromEnv only works on Struct"),
    };
    let generated_enum_error_std_env_var_variants = match ast.data.clone() {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident = match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => syn::Ident::new(
                    &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                    ident.span(),
                ),
            };
            quote::quote! {
                #enum_variant_ident {
                    source: std::env::VarError,
                    env_var_name: &'static str,
                    field_name:  &'static str,
                    where_was: tufa_common::where_was::WhereWas,
                },
            }
        }),
        _ => panic!("InitFromEnv only works on Struct"),
    };
    let generated_enum_error_parse_variants = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident = match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => syn::Ident::new(
                    &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                    ident.span(),
                ),
            };
            quote::quote! {
                #enum_variant_ident {
                    env_var_name: &'static str,
                    field_name:  &'static str,
                    expected_env_var_type: &'static str,
                    where_was: tufa_common::where_was::WhereWas,
                },
            }
        }),
        _ => panic!("InitFromEnv only works on Struct"),
    };
    let gen = quote::quote! {
        #[derive(Debug)]
        pub struct #error_ident {
            pub source: Box<#error_enum_ident>,
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
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            pub fn new() -> Result<Self, #error_ident> {
                let was_dotenv_enable = dotenv::dotenv().is_ok();
                #(#generated_functions)*
                Ok(
                    Self {
                        #(#generated_init_struct_fields)*
                    }
                )
            }
        }
    };
    gen.into()
}
