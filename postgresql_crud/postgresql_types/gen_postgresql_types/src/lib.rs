#[proc_macro]
pub fn gen_postgresql_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_postgresql_types_source::gen_postgresql_types(&input_ts.into()).into()
}
