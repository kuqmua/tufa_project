use pg_crud_macros_common::gen_jsonb_build_obj_v;
#[must_use]
pub fn fi_jsonb_build_obj_v(fi: &str) -> String {
    let v = gen_jsonb_build_obj_v(&"'null'::jsonb");
    format!("'{fi}',{v},")
}
