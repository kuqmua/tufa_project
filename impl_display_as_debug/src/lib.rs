#[proc_macro_derive(ImplDisplayAsDebug)]
pub fn impl_display_as_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ImplDisplayAsDebug";
    let ast: syn::DeriveInput =
        syn::parse(input).expect("{proc_macro_name_upper_camel_case_stringified} syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:#?}", self)
            }
        }
    };
    // println!("{gen}");
    gen.into()
}