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
    ArrayLengthDimensionOne,
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
impl PostgresqlFilter for PostgresqlTypeFilter {
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
            Self::ArrayLengthDimensionOne => &naming::ArrayLengthDimensionOneUpperCamelCase,
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
        quote::quote! {#value}
    }
    fn has_generic(&self) -> std::primitive::bool {
        PostgresqlTypeFilterHasGeneric::try_from(self).is_ok()
    }
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        if let Ok(value) = PostgresqlTypeFilterHasGeneric::try_from(self) {
            IsRelevantOnlyForNotNull::is_relevant_only_for_not_null(&value)
        } else {
            true //coz to not generate useless copies of generic types for optional types
        }
    }
}
pub enum PostgresqlTypeFilterHasGeneric {
    Equal,
    GreaterThan,
    Between,
    In,
    CaseSensitiveRegularExpression,
    CaseInsensitiveRegularExpression,
    Before,
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
}
impl IsRelevantOnlyForNotNull for PostgresqlTypeFilterHasGeneric {
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => false,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::Before => true,
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
        }
    }
}
impl std::convert::TryFrom<&PostgresqlTypeFilter> for PostgresqlTypeFilterHasGeneric {
    type Error = ();
    fn try_from(value: &PostgresqlTypeFilter) -> Result<Self, Self::Error> {
        match &value {
            PostgresqlTypeFilter::Equal => Ok(Self::Equal),
            PostgresqlTypeFilter::GreaterThan => Ok(Self::GreaterThan),
            PostgresqlTypeFilter::Between => Ok(Self::Between),
            PostgresqlTypeFilter::In => Ok(Self::In),
            PostgresqlTypeFilter::CaseSensitiveRegularExpression => Ok(Self::CaseSensitiveRegularExpression),
            PostgresqlTypeFilter::CaseInsensitiveRegularExpression => Ok(Self::CaseInsensitiveRegularExpression),
            PostgresqlTypeFilter::Before => Ok(Self::Before),
            PostgresqlTypeFilter::CurrentDate => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentDate => Err(()),
            PostgresqlTypeFilter::CurrentTimestamp => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTimestamp => Err(()),
            PostgresqlTypeFilter::CurrentTime => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTime => Err(()),
            PostgresqlTypeFilter::ArrayLengthDimensionOne => Err(()),
            PostgresqlTypeFilter::LengthMoreThan => Err(()),
            PostgresqlTypeFilter::EqualToEncodedStringRepresentation => Ok(Self::EqualToEncodedStringRepresentation),
            PostgresqlTypeFilter::ValueIsContainedWithinRange => Ok(Self::ValueIsContainedWithinRange),
            PostgresqlTypeFilter::ContainsAnotherRange => Ok(Self::ContainsAnotherRange),
            PostgresqlTypeFilter::StrictlyToLeftOfRange => Ok(Self::StrictlyToLeftOfRange),
            PostgresqlTypeFilter::StrictlyToRightOfRange => Ok(Self::StrictlyToRightOfRange),
            PostgresqlTypeFilter::IncludedLowerBound => Ok(Self::IncludedLowerBound),
            PostgresqlTypeFilter::ExcludedUpperBound => Ok(Self::ExcludedUpperBound),
            PostgresqlTypeFilter::GreaterThanLowerBound => Ok(Self::GreaterThanLowerBound),
            PostgresqlTypeFilter::OverlapWithRange => Ok(Self::OverlapWithRange),
            PostgresqlTypeFilter::AdjacentWithRange => Ok(Self::AdjacentWithRange),
            PostgresqlTypeFilter::RangeLength => Err(()),
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
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
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
        quote::quote! {#value}
    }
    fn has_generic(&self) -> std::primitive::bool {
        PostgresqlJsonTypeFilterHasGeneric::try_from(self).is_ok()
    }
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        if let Ok(value) = PostgresqlJsonTypeFilterHasGeneric::try_from(self) {
            IsRelevantOnlyForNotNull::is_relevant_only_for_not_null(&value)
        } else {
            true //coz to not generate useless copies of generic types for optional types
        }
    }
}
pub enum PostgresqlJsonTypeFilterHasGeneric {
    Equal,
    GreaterThan,
    Between,
    In,
    CaseSensitiveRegularExpression,
    CaseInsensitiveRegularExpression,
    PositionEqual,
    PositionGreaterThan,
    PositionCaseSensitiveRegularExpression,
    PositionCaseInsensitiveRegularExpression,
    ContainsAllElementsOfArray,
    OverlapsWithArray,
    AllElementsEqual,
    ContainsElementGreaterThan,
    AllElementsGreaterThan,
    ContainsElementCaseSensitiveRegularExpression,
    ContainsElementCaseInsensitiveRegularExpression,
    AllElementsCaseSensitiveRegularExpression,
    AllElementsCaseInsensitiveRegularExpression,
}
impl IsRelevantOnlyForNotNull for PostgresqlJsonTypeFilterHasGeneric {
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => false,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::PositionEqual => false,
            Self::PositionGreaterThan => true,
            Self::PositionCaseSensitiveRegularExpression => true,
            Self::PositionCaseInsensitiveRegularExpression => true,
            Self::ContainsAllElementsOfArray => false,
            Self::OverlapsWithArray => false,
            Self::AllElementsEqual => false,
            Self::ContainsElementGreaterThan => true,
            Self::AllElementsGreaterThan => true,
            Self::ContainsElementCaseSensitiveRegularExpression => true,
            Self::ContainsElementCaseInsensitiveRegularExpression => true,
            Self::AllElementsCaseSensitiveRegularExpression => true,
            Self::AllElementsCaseInsensitiveRegularExpression => true,
        }
    }
}
impl std::convert::TryFrom<&PostgresqlJsonTypeFilter> for PostgresqlJsonTypeFilterHasGeneric {
    type Error = ();
    fn try_from(value: &PostgresqlJsonTypeFilter) -> Result<Self, Self::Error> {
        match &value {
            PostgresqlJsonTypeFilter::Equal => Ok(Self::Equal),
            PostgresqlJsonTypeFilter::GreaterThan => Ok(Self::GreaterThan),
            PostgresqlJsonTypeFilter::Between => Ok(Self::Between),
            PostgresqlJsonTypeFilter::In => Ok(Self::In),
            PostgresqlJsonTypeFilter::CaseSensitiveRegularExpression => Ok(Self::CaseSensitiveRegularExpression),
            PostgresqlJsonTypeFilter::CaseInsensitiveRegularExpression => Ok(Self::CaseInsensitiveRegularExpression),
            PostgresqlJsonTypeFilter::LengthEqual => Err(()),
            PostgresqlJsonTypeFilter::LengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::PositionEqual => Ok(Self::PositionEqual),
            PostgresqlJsonTypeFilter::PositionGreaterThan => Ok(Self::PositionGreaterThan),
            PostgresqlJsonTypeFilter::PositionCaseSensitiveRegularExpression => Ok(Self::PositionCaseSensitiveRegularExpression),
            PostgresqlJsonTypeFilter::PositionCaseInsensitiveRegularExpression => Ok(Self::PositionCaseInsensitiveRegularExpression),
            PostgresqlJsonTypeFilter::ContainsAllElementsOfArray => Ok(Self::ContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::OverlapsWithArray => Ok(Self::OverlapsWithArray),
            PostgresqlJsonTypeFilter::AllElementsEqual => Ok(Self::AllElementsEqual),
            PostgresqlJsonTypeFilter::ContainsElementGreaterThan => Ok(Self::ContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::AllElementsGreaterThan => Ok(Self::AllElementsGreaterThan),
            PostgresqlJsonTypeFilter::ContainsElementCaseSensitiveRegularExpression => Ok(Self::ContainsElementCaseSensitiveRegularExpression),
            PostgresqlJsonTypeFilter::ContainsElementCaseInsensitiveRegularExpression => Ok(Self::ContainsElementCaseInsensitiveRegularExpression),
            PostgresqlJsonTypeFilter::AllElementsCaseSensitiveRegularExpression => Ok(Self::AllElementsCaseSensitiveRegularExpression),
            PostgresqlJsonTypeFilter::AllElementsCaseInsensitiveRegularExpression => Ok(Self::AllElementsCaseInsensitiveRegularExpression),
        }
    }
}

pub trait PostgresqlFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream;
    fn has_generic(&self) -> std::primitive::bool;
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool;
}
pub trait IsRelevantOnlyForNotNull {
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool;
}
