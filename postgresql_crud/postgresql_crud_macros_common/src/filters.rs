#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlTypeFilter {
    Equal,
    GreaterThan,
    Between,
    In,
    CaseSensitiveRegularExpression,
    CaseInsensitiveRegularExpression,
    Before,
    CurrentDate,
    GreaterThanCurrentDate,
    CurrentTimestamp,
    GreaterThanCurrentTimestamp,
    CurrentTime,
    GreaterThanCurrentTime,
    LengthEqual,
    LengthMoreThan,
    EqualToEncodedStringRepresentation,
    ValueIsContainedWithinRange,
    ContainsAnotherRange,
    StrictlyToLeftOfRange,
    StrictlyToRightOfRange,
    IncludedLowerBound,
    ExcludedUpperBound,
    GreaterThanLowerBound,
    OverlapWithRange,
    AdjacentWithRange,
    RangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl WhereOperatorName for PostgresqlTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal => &naming::EqualUpperCamelCase,
            Self::GreaterThan => &naming::GreaterThanUpperCamelCase,
            Self::Between => &naming::BetweenUpperCamelCase,
            Self::In => &naming::InUpperCamelCase,
            Self::CaseSensitiveRegularExpression => &naming::CaseSensitiveRegularExpressionUpperCamelCase,
            Self::CaseInsensitiveRegularExpression => &naming::CaseInsensitiveRegularExpressionUpperCamelCase,
            Self::Before => &naming::BeforeUpperCamelCase,
            Self::CurrentDate => &naming::CurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => &naming::GreaterThanCurrentDateUpperCamelCase,
            Self::CurrentTimestamp => &naming::CurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => &naming::GreaterThanCurrentTimestampUpperCamelCase,
            Self::CurrentTime => &naming::CurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => &naming::GreaterThanCurrentTimeUpperCamelCase,
            Self::LengthEqual => &naming::LengthEqualUpperCamelCase,
            Self::LengthMoreThan => &naming::LengthMoreThanUpperCamelCase,
            Self::EqualToEncodedStringRepresentation => &naming::EqualToEncodedStringRepresentationUpperCamelCase,
            Self::ValueIsContainedWithinRange => &naming::ValueIsContainedWithinRangeUpperCamelCase,
            Self::ContainsAnotherRange => &naming::ContainsAnotherRangeUpperCamelCase,
            Self::StrictlyToLeftOfRange => &naming::StrictlyToLeftOfRangeUpperCamelCase,
            Self::StrictlyToRightOfRange => &naming::StrictlyToRightOfRangeUpperCamelCase,
            Self::IncludedLowerBound => &naming::IncludedLowerBoundUpperCamelCase,
            Self::ExcludedUpperBound => &naming::ExcludedUpperBoundUpperCamelCase,
            Self::GreaterThanLowerBound => &naming::GreaterThanLowerBoundUpperCamelCase,
            Self::OverlapWithRange => &naming::OverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange => &naming::AdjacentWithRangeUpperCamelCase,
            Self::RangeLength => &naming::RangeLengthUpperCamelCase,
        }
    }
}
#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypeFilter {
    Equal,
    GreaterThan,
    Between,
    In,
    CaseSensitiveRegularExpression,
    CaseInsensitiveRegularExpression,
    LengthEqual,
    LengthMoreThan,
    PositionEqual,
    PositionGreaterThan,
    PositionCaseSensitiveRegularExpression,
    PositionCaseInsensitiveRegularExpression,
    ContainsAllElementsOfArray,
    // ContainedInArray,
    OverlapsWithArray,
    AllElementsEqual,
    ContainsElementGreaterThan,
    AllElementsGreaterThan,
    ContainsElementCaseSensitiveRegularExpression,
    ContainsElementCaseInsensitiveRegularExpression,
    AllElementsCaseSensitiveRegularExpression,
    AllElementsCaseInsensitiveRegularExpression,
}
impl WhereOperatorName for PostgresqlJsonTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal => &naming::EqualUpperCamelCase,
            Self::GreaterThan => &naming::GreaterThanUpperCamelCase,
            Self::Between => &naming::BetweenUpperCamelCase,
            Self::In => &naming::InUpperCamelCase,
            Self::CaseSensitiveRegularExpression => &naming::CaseSensitiveRegularExpressionUpperCamelCase,
            Self::CaseInsensitiveRegularExpression => &naming::CaseInsensitiveRegularExpressionUpperCamelCase,
            Self::LengthEqual => &naming::LengthEqualUpperCamelCase,
            Self::LengthMoreThan => &naming::LengthMoreThanUpperCamelCase,
            Self::PositionEqual => &naming::PositionEqualUpperCamelCase,
            Self::PositionGreaterThan => &naming::PositionGreaterThanUpperCamelCase,
            Self::PositionCaseSensitiveRegularExpression => &naming::PositionCaseSensitiveRegularExpressionUpperCamelCase,
            Self::PositionCaseInsensitiveRegularExpression => &naming::PositionCaseInsensitiveRegularExpressionUpperCamelCase,
            Self::ContainsAllElementsOfArray => &naming::ContainsAllElementsOfArrayUpperCamelCase,
            Self::OverlapsWithArray => &naming::OverlapsWithArrayUpperCamelCase,
            Self::AllElementsEqual => &naming::AllElementsEqualUpperCamelCase,
            Self::ContainsElementGreaterThan => &naming::ContainsElementGreaterThanUpperCamelCase,
            Self::AllElementsGreaterThan => &naming::AllElementsGreaterThanUpperCamelCase,
            Self::ContainsElementCaseSensitiveRegularExpression => &naming::ContainsElementCaseSensitiveRegularExpressionUpperCamelCase,
            Self::ContainsElementCaseInsensitiveRegularExpression => &naming::ContainsElementCaseInsensitiveRegularExpressionUpperCamelCase,
            Self::AllElementsCaseSensitiveRegularExpression => &naming::AllElementsCaseSensitiveRegularExpressionUpperCamelCase,
            Self::AllElementsCaseInsensitiveRegularExpression => &naming::AllElementsCaseInsensitiveRegularExpressionUpperCamelCase,
        }
    }
}

pub trait WhereOperatorName {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
}