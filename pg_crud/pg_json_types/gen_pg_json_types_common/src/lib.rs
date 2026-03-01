use pg_crud_macros_common::gen_jsonb_build_object;
#[must_use]
pub fn fi_jsonb_build_object_v(fi: &str) -> String {
    let v = gen_jsonb_build_object(&"'value','null'::jsonb");
    format!("'{fi}',{v},")
}
