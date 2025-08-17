#[cfg(feature = "test-utils")]
pub trait PostgresqlTypeTestCases<T> {
    type Element: crate::PostgresqlType;
    fn test_cases(read_only_ids: &<Self::Element as crate::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner>; //todo maybe make it an array
    fn read_new_or_try_new_unwraped_for_test(value: T) -> <Self::Element as crate::PostgresqlType>::Read;
    fn update_new_or_try_new_unwraped_for_test(value: T) -> <Self::Element as crate::PostgresqlType>::Update;
}

#[cfg(feature = "test-utils")]
pub trait PostgresqlJsonTypeTestCases<T> {
    type Element: crate::PostgresqlJsonType;
    fn test_cases(read_only_ids: &<Self::Element as crate::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<<Self::Element as crate::PostgresqlJsonType>::ReadInner>; //todo maybe make it an array
    fn read_new_or_try_new_unwraped_for_test(value: T) -> <Self::Element as crate::PostgresqlJsonType>::Read;
    fn update_new_or_try_new_unwraped_for_test(value: T) -> <Self::Element as crate::PostgresqlJsonType>::Update;
}

//for multiple parameters initialization
// pub trait PostgresqlTypeTestCasesTwo {
//     type SelfHandle: crate::PostgresqlType;
//     type VecElement: std::convert::Into<<Self::SelfHandle as crate::PostgresqlType>::ReadInner>;
//     fn test_cases_two() -> std::vec::Vec<Self::VecElement>;
// }
// impl crate::tests::PostgresqlTypeTestCasesTwo for SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp {
//     type SelfHandle = SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp;
//     type VecElement = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization;
//     fn test_cases_two() -> std::vec::Vec<Self::VecElement> {
//         vec![
//             SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization {
//                 naive_date: crate::SqlxTypesChronoNaiveDate::try_new(sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).unwrap()).unwrap(),
//                 naive_time: crate::SqlxTypesChronoNaiveTime::try_new(crate::Hour::try_new(0).unwrap(), crate::Minute::try_new(0).unwrap(), crate::Second::try_new(0).unwrap(), crate::Microsecond::try_new(0).unwrap()).unwrap()
//             },
//             SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization {
//                 naive_date: crate::SqlxTypesChronoNaiveDate::try_new(sqlx::types::chrono::NaiveDate::MAX).unwrap(),
//                 naive_time: crate::SqlxTypesChronoNaiveTime::try_new(crate::Hour::try_new(23).unwrap(), crate::Minute::try_new(59).unwrap(), crate::Second::try_new(59).unwrap(), crate::Microsecond::try_new(999_999).unwrap()).unwrap()
//             }
//         ]
//     }
// }

// // SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization
// pub fn new_or_try_new_unwraped_for_test_two(value: <SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as crate::tests::PostgresqlTypeTestCasesTwo>::VecElement) -> Self {
//     Self(value.into())
// }
// pub fn new_or_try_new_unwraped_for_test_two(value: <SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as crate::tests::PostgresqlTypeTestCasesTwo>::VecElement) -> Self {
//     Self(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_or_try_new_unwraped_for_test_two(value))
// }
////
// #[derive(Clone)]
// pub struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization {
//     naive_date: crate::SqlxTypesChronoNaiveDate,
//     naive_time: crate::SqlxTypesChronoNaiveTime
// }
// impl std::convert::Into<sqlx::types::chrono::NaiveDateTime> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTestCasesInitialization {
//     fn into(self) -> sqlx::types::chrono::NaiveDateTime {
//         sqlx::types::chrono::NaiveDateTime::new(
//             self.naive_date.into(),
//             self.naive_time.into()
//         )
//     }
// }
