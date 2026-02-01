#[proc_macro_attribute]
pub fn postgresql_json_object_type_config(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_derive(GeneratePostgresqlJsonObjectType)]
pub fn generate_postgresql_json_object_type(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_postgresql_json_object_type_source::generate_postgresql_json_object_type(
        input_token_stream.into(),
    )
    .into()
}
