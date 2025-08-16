pub fn pagination_start_end_initialization_token_stream(
    value: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        let start = #value.pagination.start();
        let end = #value.pagination.end();
    }
}
