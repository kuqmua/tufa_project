#[proc_macro_attribute]
pub fn pg_json_object_type_config(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_derive(GenPgJsonObjectType)]
pub fn gen_pg_json_object_type(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_pg_json_object_type_source::gen_pg_json_object_type(input_ts.into()).into()
}
