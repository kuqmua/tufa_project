#[proc_macro]
pub fn gen_pg_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_pg_types_source::gen_pg_types(&input_ts.into()).into()
}
