#[proc_macro_derive(GenerateGetterTraitsForStructFields)]
pub fn generate_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "GenerateGetterTraitsForStructFields";
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("{proc_macro_name_upper_camel_case_stringified} syn::parse(input) failed");
    let ident = &syn_derive_input.ident;
    let datastruct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("GenerateGetterTraitsForStructFields only works on Struct"),
    };
    let generated_traits_implementations = datastruct.fields.into_iter().map(|field| {
        let (field_ident, upper_camel_case_field_ident) = {
            let field_ident = field
                .ident
                .as_ref()
                .unwrap_or_else(|| panic!("{ident} {}", naming_conventions::FIELD_IDENT_IS_NONE));
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
    });
    let generated = quote::quote! {
        #(#generated_traits_implementations)*
    };
    // println!("{generated}");
    generated.into()
}

#[proc_macro_derive(GenerateGetterTrait)]
pub fn generate_getter_trait(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "GenerateGetterTrait";
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("{proc_macro_name_upper_camel_case_stringified} syn::parse(input) failed");
    let ident = &syn_derive_input.ident;
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("GenerateGetterTrait only works on Struct"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | 
        syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_stringified} only works with syn::Fields::Unnamed"),
    };
    assert!(fields_unnamed.len() == 1, "{proc_macro_name_upper_camel_case_stringified} fields_unnamed !== 1");
    let first_field_unnamed = fields_unnamed.iter().next().map_or_else(|| panic!("{proc_macro_name_upper_camel_case_stringified} fields_unnamed.iter().nth(0) is None"), |value| value);
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_ident_upper_camel_case_stringified = format!(
        "{}{ident}",
        naming_conventions::GetUpperCamelCase,
    );
    let get_ident_upper_camel_case_token_stream = {
        get_ident_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {get_ident_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let get_ident_snake_case_token_stream = {
        let value = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &get_ident_upper_camel_case_stringified
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generated = quote::quote! {
        pub trait #get_ident_upper_camel_case_token_stream {
            fn #get_ident_snake_case_token_stream(&self) -> &#first_field_unnamed_type;
        }
    };
    // println!("{generated}");
    generated.into()
}