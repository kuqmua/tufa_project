generate_postgresql_type_where_element_filters::generate_postgresql_type_where_element_filters!();

//todo ExactSizeIterator now is not a solution. error[E0658]: use of unstable library feature `exact_size_is_empty`. maybe rewrite it later
pub trait IsEmpty {
    fn is_empty(&self) -> std::primitive::bool;
}