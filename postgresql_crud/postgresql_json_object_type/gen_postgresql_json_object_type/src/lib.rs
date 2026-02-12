#[proc_macro_attribute]
pub fn postgresql_json_object_type_config(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_derive(GenPostgresqlJsonObjectType)]
pub fn gen_postgresql_json_object_type(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    gen_postgresql_json_object_type_source::gen_postgresql_json_object_type(input_ts.into()).into()
}
