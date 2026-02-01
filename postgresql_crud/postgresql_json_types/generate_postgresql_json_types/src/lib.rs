#[proc_macro]
pub fn generate_postgresql_json_types(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_postgresql_json_types_source::generate_postgresql_json_types(input_token_stream.into())
        .into()
}
