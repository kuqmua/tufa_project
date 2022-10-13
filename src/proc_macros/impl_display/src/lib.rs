#[proc_macro_derive(ImplDisplay)]
pub fn derive_impl_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("ImplDisplay syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
    gen.into()
}
