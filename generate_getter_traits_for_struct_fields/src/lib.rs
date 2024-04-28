#[proc_macro_derive(GenerateGetterTraitsForStructFieldsHandle)]
pub fn generate_getter_traits_for_struct_fields_handle(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "GenerateGetterTraitsForStructFields";
    let ast: syn::DeriveInput =
        syn::parse(input).expect("{proc_macro_name_upper_camel_case_stringified} syn::parse(input) failed");
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
            let type_ident_wrapper_token_stream = {
                let element_type_stringified = {
                    let value_token_stream = quote::quote!{#type_ident};
                    value_token_stream.to_string()
                };
                let element_type_stringified  = format!(
                    "{element_type_stringified}",
                    // <naming_constants::Wrapper as naming_constants::Naming>::upper_camel_case_stringified(),
                );
                element_type_stringified .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {element_type_stringified } {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let path_trait_ident =
                format!("app_state::Get{upper_camel_case_field_ident}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path_trait_ident parse failed");
            let function_name_ident = format!("get_{field_ident}")
                .parse::<proc_macro2::TokenStream>()
                .expect("function_name_ident parse failed");
            quote::quote! {
                impl #path_trait_ident for #ident {
                    fn #function_name_ident (&self) -> &#type_ident_wrapper_token_stream {
                        &self.#field_ident
                    }
                }
                impl #path_trait_ident for &#ident {
                    fn #function_name_ident (&self) -> &#type_ident_wrapper_token_stream {
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