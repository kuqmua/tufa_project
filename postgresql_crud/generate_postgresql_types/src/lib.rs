#[proc_macro]
pub fn generate_postgresql_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generated = quote::quote! {
        
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypes",
    //     &generated,
    // );
    generated.into()
}