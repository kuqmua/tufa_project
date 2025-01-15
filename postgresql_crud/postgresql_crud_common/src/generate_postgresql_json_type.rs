pub trait StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self;
}
pub trait AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self>;
}

pub fn wrap_into_jsonb_build_object(field: &std::primitive::str, value: &std::primitive::str) -> std::string::String {
    format!("jsonb_build_object('{field}',{value})||")
}