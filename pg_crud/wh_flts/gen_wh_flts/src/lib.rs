use proc_macro::TokenStream as Ts;
#[proc_macro]
pub fn gen_wh_flts(input_ts: Ts) -> Ts {
    gen_wh_flts_src::gen_wh_flts(&input_ts.into()).into()
}
