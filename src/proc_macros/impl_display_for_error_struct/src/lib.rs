#[proc_macro_derive(ImplDisplayForErrorStruct)]
pub fn derive_impl_display_for_error_struct(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplDisplayForErrorStruct syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.source)
            }
        }
    };
    gen.into()
}
