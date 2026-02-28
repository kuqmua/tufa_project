#[must_use]
pub fn fi_jsonb_build_object_v(fi: &str) -> String {
    format!("'{fi}',jsonb_build_object('value','null'::jsonb),")
}
