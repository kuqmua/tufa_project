#[proc_macro_derive(FromStr)]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| panic!("let syn_derive_input: syn::DeriveInput = syn::parse(input) failed"));
    let ident = &syn_derive_input.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = syn_derive_input.data {
        data_enum
    } else {
        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
    };
    let variant_idents = data_enum
        .variants
        .into_iter()
        .map(|variant| match variant.fields {
            syn::Fields::Unit => variant.ident,
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("expected fields would be unit"),
        })
        .collect::<Vec<syn::Ident>>();
    let variants_token_stream = variant_idents.iter().map(|variant_ident| {
        let variant_ident_snake_case_token_stream = {
            let variant_ident_snake_case_stringified = convert_case::Casing::to_case(&format!("\"{variant_ident}\""), convert_case::Case::Snake);
            variant_ident_snake_case_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{variant_ident_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            #variant_ident_snake_case_token_stream => Ok(Self::#variant_ident),
        }
    });
    let error_variants_stringified = variant_idents.iter().fold(std::string::String::default(), |mut acc, variant_ident| {
        let variant_ident_snake_case_stringified = convert_case::Casing::to_case(&format!("{variant_ident}"), convert_case::Case::Snake);
        acc.push_str(&format!("\'{variant_ident_snake_case_stringified}\',"));
        acc
    });
    let error_token_stream = {
        let error_stringified = format!("\"Invalid {ident}, expected one of {error_variants_stringified} found {{value}}\"");
        error_stringified.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{error_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generated = quote::quote! {
        impl std::str::FromStr for #ident {
            type Err = std::string::String;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    #(#variants_token_stream)*
                    _ => Err(format!(#error_token_stream)),
                }
            }
        }
    };
    // if ident == "" {
    //    println!("{generated}");
    // }
    generated.into()
}
