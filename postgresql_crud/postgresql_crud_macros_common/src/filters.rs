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
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote!{#value}
    }
    fn has_generic(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => true,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => true,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::Before => true,
            Self::CurrentDate => false,
            Self::GreaterThanCurrentDate => false,
            Self::CurrentTimestamp => false,
            Self::GreaterThanCurrentTimestamp => false,
            Self::CurrentTime => false,
            Self::GreaterThanCurrentTime => false,
            Self::LengthEqual => false,
            Self::LengthMoreThan => false,
            Self::EqualToEncodedStringRepresentation => true,
            Self::ValueIsContainedWithinRange => true,
            Self::ContainsAnotherRange => true,
            Self::StrictlyToLeftOfRange => true,
            Self::StrictlyToRightOfRange => true,
            Self::IncludedLowerBound => true,
            Self::ExcludedUpperBound => true,
            Self::GreaterThanLowerBound => true,
            Self::OverlapWithRange => true,
            Self::AdjacentWithRange => true,
            Self::RangeLength => false,
        }
    }
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => true,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => true,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::Before => true,
            Self::CurrentDate => true,
            Self::GreaterThanCurrentDate => true,
            Self::CurrentTimestamp => true,
            Self::GreaterThanCurrentTimestamp => true,
            Self::CurrentTime => true,
            Self::GreaterThanCurrentTime => true,
            Self::LengthEqual => true,
            Self::LengthMoreThan => true,
            Self::EqualToEncodedStringRepresentation => true,
            Self::ValueIsContainedWithinRange => true,
            Self::ContainsAnotherRange => true,
            Self::StrictlyToLeftOfRange => true,
            Self::StrictlyToRightOfRange => true,
            Self::IncludedLowerBound => true,
            Self::ExcludedUpperBound => true,
            Self::GreaterThanLowerBound => true,
            Self::OverlapWithRange => true,
            Self::AdjacentWithRange => true,
            Self::RangeLength => true,
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
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlJsonTypeWhereElementSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote!{#value}
    }
    fn has_generic(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => true,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => true,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::LengthEqual => false,
            Self::LengthMoreThan => false,
            Self::PositionEqual => true,
            Self::PositionGreaterThan => true,
            Self::PositionCaseSensitiveRegularExpression => true,
            Self::PositionCaseInsensitiveRegularExpression => true,
            Self::ContainsAllElementsOfArray => true,
            Self::OverlapsWithArray => true,
            Self::AllElementsEqual => true,
            Self::ContainsElementGreaterThan => true,
            Self::AllElementsGreaterThan => true,
            Self::ContainsElementCaseSensitiveRegularExpression => true,
            Self::ContainsElementCaseInsensitiveRegularExpression => true,
            Self::AllElementsCaseSensitiveRegularExpression => true,
            Self::AllElementsCaseInsensitiveRegularExpression => true,
        }
    }
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => true,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => true,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::LengthEqual => true,
            Self::LengthMoreThan => true,
            Self::PositionEqual => true,
            Self::PositionGreaterThan => true,
            Self::PositionCaseSensitiveRegularExpression => true,
            Self::PositionCaseInsensitiveRegularExpression => true,
            Self::ContainsAllElementsOfArray => true,
            Self::OverlapsWithArray => true,
            Self::AllElementsEqual => true,
            Self::ContainsElementGreaterThan => true,
            Self::AllElementsGreaterThan => true,
            Self::ContainsElementCaseSensitiveRegularExpression => true,
            Self::ContainsElementCaseInsensitiveRegularExpression => true,
            Self::AllElementsCaseSensitiveRegularExpression => true,
            Self::AllElementsCaseInsensitiveRegularExpression => true,
        }
    }
}

pub trait WhereOperatorName {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream;
    fn has_generic(&self) -> std::primitive::bool;
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool;
}