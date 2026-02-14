use proc_macro::TokenStream as Ts;

#[proc_macro]
pub fn gen_pg_types(input_ts: Ts) -> Ts {
    gen_pg_types_source::gen_pg_types(&input_ts.into()).into()
}
