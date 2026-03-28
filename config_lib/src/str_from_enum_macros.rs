pub fn impl_from_str_for_enum_helper<T>(v: &str, vrts: &[(&str, T)]) -> Result<T, String>
where
    T: Copy,
{
    for (str_vrt, enum_vrt) in vrts {
        if v.eq_ignore_ascii_case(str_vrt) {
            return Ok(*enum_vrt);
        }
    }
    Err(format!("Unknown value: {v}"))
}
