pub trait PostgresqlTypeTestCases {
    type Element: crate::PostgresqlType;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner>;//todo maybe make it an array
}