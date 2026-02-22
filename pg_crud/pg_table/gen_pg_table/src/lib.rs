use proc_macro::TokenStream as Ts;
#[proc_macro_attribute]
pub fn gen_pg_table_config(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn create_many_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn create_one_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn read_one_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn read_many_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn update_one_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn update_many_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn delete_one_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn delete_many_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn common_extra_er_vrts(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn create_many_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn create_one_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn read_many_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn read_one_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn update_many_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn update_one_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn delete_many_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn delete_one_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_attribute]
pub fn common_extra_logic(_attr: Ts, item: Ts) -> Ts {
    item
}
#[proc_macro_derive(GenPgTable, attributes(gen_pg_table_primary_k))]
pub fn gen_pg_table(input: Ts) -> Ts {
    gen_pg_table_source::gen_pg_table(input.into()).into()
}
