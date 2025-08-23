#[cfg(feature = "test-utils")]
pub trait PostgresqlTypeTestCases {
    type Element: crate::PostgresqlType;
    fn test_cases(read_only_ids: &<Self::Element as crate::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner>>; //todo maybe make it an array
    fn read_new_or_try_new_unwraped_for_test(value: <Self::Element as crate::PostgresqlType>::ReadInner) -> <Self::Element as crate::PostgresqlType>::Read;
    fn update_new_or_try_new_unwraped_for_test(value: <Self::Element as crate::PostgresqlType>::ReadInner) -> <Self::Element as crate::PostgresqlType>::Update;
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as crate::PostgresqlType>::ReadOnlyIds) ->  std::option::Option<crate::Value<<Self::Element as crate::PostgresqlType>::ReadInner>>;
    fn update_to_read_only_ids(value: <Self::Element as crate::PostgresqlType>::Update) -> <Self::Element as crate::PostgresqlType>::ReadOnlyIds;
}

#[cfg(feature = "test-utils")]
pub trait PostgresqlJsonTypeTestCases {
    type Element: crate::PostgresqlJsonType;
    fn test_cases(read_only_ids: &<Self::Element as crate::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as crate::PostgresqlJsonType>::ReadInner>>; //todo maybe make it an array
    fn read_new_or_try_new_unwraped_for_test(value: <Self::Element as crate::PostgresqlJsonType>::ReadInner) -> <Self::Element as crate::PostgresqlJsonType>::Read;
    fn update_new_or_try_new_unwraped_for_test(value: <Self::Element as crate::PostgresqlJsonType>::ReadInner) -> <Self::Element as crate::PostgresqlJsonType>::Update;
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as crate::PostgresqlJsonType>::ReadOnlyIds) ->  std::option::Option<crate::Value<<Self::Element as crate::PostgresqlJsonType>::ReadInner>>;
    fn update_to_read_only_ids(value: <Self::Element as crate::PostgresqlJsonType>::Update) -> <Self::Element as crate::PostgresqlJsonType>::ReadOnlyIds;
}