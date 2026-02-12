use quote::quote;

#[proc_macro_derive(FromStr)]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("f83fcd2d-667d-4bdc-872f-e8a0afcb4388");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("d35db256-2db6-42d9-8ac9-24aebe6b7ec4");
    };
    let variant_idents = data_enum
        .variants
        .into_iter()
        .map(|variant| match variant.fields {
            syn::Fields::Unit => variant.ident,
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                panic!("23575b02-186f-4f9f-84bb-3973c3952d66")
            }
        })
        .collect::<Vec<syn::Ident>>();
    let variants_ts = variant_idents.iter().map(|variant_ident| {
        let variant_ident_snake_case_ts = {
            let variant_ident_snake_case_stringified = convert_case::Casing::to_case(
                &format!("\"{variant_ident}\""),
                convert_case::Case::Snake,
            );
            variant_ident_snake_case_stringified
                .parse::<proc_macro2::TokenStream>()
                .expect("791603c1-e547-4486-898e-631abb15afc5")
        };
        quote! {
            #variant_ident_snake_case_ts => Ok(Self::#variant_ident),
        }
    });
    let error_variants_stringified =
        variant_idents
            .iter()
            .fold(String::default(), |mut acc_d6966473, variant_ident| {
                use std::fmt::Write as _;
                let variant_ident_snake_case_stringified = convert_case::Casing::to_case(
                    &format!("{variant_ident}"),
                    convert_case::Case::Snake,
                );
                assert!(
                    write!(acc_d6966473, "\'{variant_ident_snake_case_stringified}\',").is_ok(),
                    "09c49558-9d46-41d1-86a5-f76c1460a21e"
                );
                acc_d6966473
            });
    let error_ts = {
        let error_stringified = format!(
            "\"Invalid {ident}, expected one of {error_variants_stringified} found {{value}}\""
        );
        error_stringified
            .parse::<proc_macro2::TokenStream>()
            .expect("1b778757-4118-4419-bb33-a2f677afa169")
    };
    let generated = quote! {
        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    #(#variants_ts)*
                    _ => Err(format!(#error_ts)),
                }
            }
        }
    };
    // if ident == "" {
    //    println!("{generated}");
    // }
    generated.into()
}
