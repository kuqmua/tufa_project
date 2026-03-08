use proc_macro::TokenStream as Ts;
#[proc_macro_attribute]
pub fn pg_json_object_config(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_derive(GenPgJsonObject)]
pub fn gen_pg_json_object(input_ts: Ts) -> Ts {
    gen_pg_json_obj_src::gen_pg_json_object(input_ts.into()).into()
}
