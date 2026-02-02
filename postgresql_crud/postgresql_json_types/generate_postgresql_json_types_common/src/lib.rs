#[must_use]
pub fn field_ident_jsonb_build_object_value(field_ident: &str) -> String {
    format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),")
}
