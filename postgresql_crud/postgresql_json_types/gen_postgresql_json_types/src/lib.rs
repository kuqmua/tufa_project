#[proc_macro]
pub fn gen_postgresql_json_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_postgresql_json_types_source::gen_postgresql_json_types(&input_ts.into()).into()
}
