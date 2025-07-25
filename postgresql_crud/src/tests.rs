pub trait PostgresqlTypeTestCases {
    type Element: crate::PostgresqlType;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner>;//todo maybe make it an array
}

pub trait PostgresqlTypeTestCasesTwo {
    type SelfHandle: crate::PostgresqlType;
    type VecElement: std::convert::Into<<Self::SelfHandle as crate::PostgresqlType>::ReadInner>;
    fn test_cases_two() -> std::vec::Vec<Self::VecElement>;
}