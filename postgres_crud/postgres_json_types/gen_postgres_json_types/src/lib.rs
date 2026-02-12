#[proc_macro]
pub fn gen_postgres_json_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_postgres_json_types_source::gen_postgres_json_types(&input_ts.into()).into()
}
