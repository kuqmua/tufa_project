#[proc_macro_derive(GenerateGetterTraitsForStructFields)]
pub fn generate_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput =
        syn::parse(input).expect("GenerateGetterTraitsForStructFields syn::parse(input) failed");
    let ident = &ast.ident;
    let generated_traits_implementations = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let (field_ident, upper_camel_case_field_ident) = {
                let field_ident = field
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| panic!("{ident} {}", naming_constants::FIELD_IDENT_IS_NONE));
                (
                    field_ident,
                    syn::Ident::new(
                        &convert_case::Casing::to_case(
                            &format!("{field_ident}"),
                            convert_case::Case::UpperCamel,
                        ),
                        ident.span(),
                    ),
                )
            };
            let type_ident = field.ty;
            let path_trait_ident =
                format!("app_state::Get{upper_camel_case_field_ident}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path_trait_ident parse failed");
            let function_name_ident = format!("get_{field_ident}")
                .parse::<proc_macro2::TokenStream>()
                .expect("function_name_ident parse failed");
            quote::quote! {
                impl #path_trait_ident for #ident {
                    fn #function_name_ident (&self) -> &#type_ident {
                        &self.#field_ident
                    }
                }
                impl #path_trait_ident for &#ident {
                    fn #function_name_ident (&self) -> &#type_ident {
                        &self.#field_ident
                    }
                }
            }
        }),
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("GenerateGetterTraitsForStructFields only works on Struct"),
    };
    let gen = quote::quote! {
        #(#generated_traits_implementations)*
    };
    // println!("{gen}");
    gen.into()
}
////////////////////////
#[proc_macro_derive(GenerateGetterTraitsForStructFieldsHandle)]
pub fn generate_getter_traits_for_struct_fields_handle(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput =
        syn::parse(input).expect("GenerateGetterTraitsForStructFields syn::parse(input) failed");
    let ident = &ast.ident;
    let generated_traits_implementations = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let (field_ident, upper_camel_case_field_ident) = {
                let field_ident = field
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| panic!("{ident} {}", naming_constants::FIELD_IDENT_IS_NONE));
                (
                    field_ident,
                    syn::Ident::new(
                        &convert_case::Casing::to_case(
                            &format!("{field_ident}"),
                            convert_case::Case::UpperCamel,
                        ),
                        ident.span(),
                    ),
                )
            };
            let type_ident = field.ty;
            let path_trait_ident =
                format!("app_state::Get{upper_camel_case_field_ident}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path_trait_ident parse failed");
            let function_name_ident = format!("get_{field_ident}")
                .parse::<proc_macro2::TokenStream>()
                .expect("function_name_ident parse failed");
            quote::quote! {
                impl #path_trait_ident for #ident {
                    fn #function_name_ident (&self) -> &#type_ident {
                        &self.#field_ident.into_inner()
                    }
                }
                impl #path_trait_ident for &#ident {
                    fn #function_name_ident (&self) -> &#type_ident {
                        &self.#field_ident.into_inner()
                    }
                }
            }
        }),
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("GenerateGetterTraitsForStructFields only works on Struct"),
    };
    let gen = quote::quote! {
        #(#generated_traits_implementations)*
    };
    println!("{gen}");
    gen.into()
}