pub fn pagination_start_end_initialization_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{
        let start = self.pagination.start();
        let end = self.pagination.end();
    }
}