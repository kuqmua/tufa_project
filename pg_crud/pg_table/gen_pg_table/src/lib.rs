use proc_macro::TokenStream as Ts;
#[proc_macro_attribute]
pub fn gen_pg_table_config(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn cm_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn co_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn ro_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn rm_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn uo_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn um_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn dlo_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn dm_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn common_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn cm_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn co_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn rm_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn ro_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn um_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn uo_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn dm_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn dlo_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn common_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_derive(GenPgTable, attributes(gen_pg_table_pk))]
pub fn gen_pg_table(input: Ts) -> Ts {
    gen_pg_table_source::gen_pg_table(input.into()).into()
}
