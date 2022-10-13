#[proc_macro_derive(GitInfo)]
pub fn derive_git_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("GitInfo syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl tufa_common::traits::git_info_trait::GitInfo for #ident {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn git_info(&self) -> String {
                crate::lazy_static::git_info::GIT_INFO.data.get_commit_link()
            }
        }
    };
    gen.into()
}
