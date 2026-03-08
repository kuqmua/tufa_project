use proc_macro::TokenStream as Ts;
#[proc_macro]
pub fn gen_pg_json(input_ts: Ts) -> Ts {
    gen_pg_json_src::gen_pg_json(&input_ts.into()).into()
}
