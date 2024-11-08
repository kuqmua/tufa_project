/*
only works if all enum variants without fields like this
#[derive(macros_assistants::ToUpperCamelCaseStringified)]
enum Operation {
     One,
     Two,
     Three,
}
*/
#[proc_macro_derive(EnumWithUnitFieldsToUpperCamelCaseStringified)]
pub fn enum_with_unit_fields_to_upper_camel_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToUpperCamelCaseStringified";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_stringified} {}: {error}", macros_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string = quote::quote! {std::string::String};
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_upper_camel_case_stringified = macros_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&variant_ident.to_string());
                let variant_ident_upper_camel_case_double_quotes_token_stream = macros_common::generate_quotes::double_quotes_token_stream(&variant_ident_upper_camel_case_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_upper_camel_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = macros_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let trait_path_token_stream = trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = macros_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let generated = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string {//todo maybe write duplicate Trait with &str instead of std::string::String
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}

/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::ToSnakeCaseStringified)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(EnumWithUnitFieldsToSnakeCaseStringified)]
pub fn enum_with_unit_fields_to_snake_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToSnakeCaseStringified";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_stringified} {}: {error}", macros_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_snake_case_stringified = macros_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_ident.to_string());
                let variant_ident_snake_case_double_quotes_token_stream = macros_common::generate_quotes::double_quotes_token_stream(&variant_ident_snake_case_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_snake_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = macros_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let trait_path_token_stream = trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = macros_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let generated = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}
/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::ToScreamingSnakeCaseStringified)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(EnumWithUnitFieldsToScreamingSnakeCaseStringified)]
pub fn enum_with_unit_fields_to_screaming_snake_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToScreamingSnakeCaseStringified";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_stringified} {}: {error}", macros_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_snake_case_stringified = macros_common::naming_conventions::ToScreamingSnakeCaseStringified::to_screaming_snake_case_stringified(&variant_ident.to_string());
                let variant_ident_snake_case_double_quotes_token_stream = macros_common::generate_quotes::double_quotes_token_stream(&variant_ident_snake_case_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_snake_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = macros_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let trait_path_token_stream = trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = macros_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&proc_macro_name_upper_camel_case_stringified);
    let generated = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}


fn trait_path_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {macros_common::naming_conventions}
}