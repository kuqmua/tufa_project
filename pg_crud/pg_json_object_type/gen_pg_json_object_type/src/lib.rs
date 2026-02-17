use proc_macro::TokenStream as Ts;
#[proc_macro_attribute]
pub fn pg_json_object_type_config(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_derive(GenPgJsonObjectType)]
pub fn gen_pg_json_object_type(input_ts: Ts) -> Ts {
    gen_pg_json_object_type_source::gen_pg_json_object_type(input_ts.into()).into()
}
