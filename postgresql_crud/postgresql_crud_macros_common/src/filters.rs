#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlTypeFilter {
    Equal,
    DimensionOneEqual,
    GreaterThan,
    DimensionOneGreaterThan,
    Between,
    DimensionOneBetween,
    In,
    DimensionOneIn,
    RegularExpression,
    DimensionOneRegularExpression,
    Before,
    DimensionOneBefore,
    CurrentDate,
    DimensionOneCurrentDate,
    GreaterThanCurrentDate,
    DimensionOneGreaterThanCurrentDate,
    CurrentTimestamp,
    DimensionOneCurrentTimestamp,
    GreaterThanCurrentTimestamp,
    DimensionOneGreaterThanCurrentTimestamp,
    CurrentTime,
    DimensionOneCurrentTime,
    GreaterThanCurrentTime,
    DimensionOneGreaterThanCurrentTime,
    ArrayLengthDimensionOne,
    DimensionOneArrayLengthDimensionOne,
    ArrayLengthMoreThanDimensionOne,
    DimensionOneArrayLengthMoreThanDimensionOne,
    EqualToEncodedStringRepresentation,
    DimensionOneEqualToEncodedStringRepresentation,
    ValueIsContainedWithinRange,
    DimensionOneValueIsContainedWithinRange,
    ContainsAnotherRange,
    DimensionOneContainsAnotherRange,
    StrictlyToLeftOfRange,
    DimensionOneStrictlyToLeftOfRange,
    StrictlyToRightOfRange,
    DimensionOneStrictlyToRightOfRange,
    IncludedLowerBound,
    DimensionOneIncludedLowerBound,
    ExcludedUpperBound,
    DimensionOneExcludedUpperBound,
    GreaterThanLowerBound,
    DimensionOneGreaterThanLowerBound,
    OverlapWithRange,
    DimensionOneOverlapWithRange,
    AdjacentWithRange,
    DimensionOneAdjacentWithRange,
    RangeLength,
    DimensionOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PostgresqlFilter for PostgresqlTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual => &naming::DimensionOneEqualUpperCamelCase,
            Self::GreaterThan => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::Between => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween => &naming::DimensionOneBetweenUpperCamelCase,
            Self::In => &naming::InUpperCamelCase,
            Self::DimensionOneIn => &naming::DimensionOneInUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &naming::DimensionOneRegularExpressionUpperCamelCase,
            Self::Before => &naming::BeforeUpperCamelCase,
            Self::DimensionOneBefore => &naming::DimensionOneBeforeUpperCamelCase,
            Self::CurrentDate => &naming::CurrentDateUpperCamelCase,
            Self::DimensionOneCurrentDate => &naming::DimensionOneCurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => &naming::GreaterThanCurrentDateUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentDate => &naming::DimensionOneGreaterThanCurrentDateUpperCamelCase,
            Self::CurrentTimestamp => &naming::CurrentTimestampUpperCamelCase,
            Self::DimensionOneCurrentTimestamp => &naming::DimensionOneCurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => &naming::GreaterThanCurrentTimestampUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentTimestamp => &naming::DimensionOneGreaterThanCurrentTimestampUpperCamelCase,
            Self::CurrentTime => &naming::CurrentTimeUpperCamelCase,
            Self::DimensionOneCurrentTime => &naming::DimensionOneCurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => &naming::GreaterThanCurrentTimeUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentTime => &naming::DimensionOneGreaterThanCurrentTimeUpperCamelCase,
            Self::ArrayLengthDimensionOne => &naming::ArrayLengthDimensionOneUpperCamelCase,
            Self::DimensionOneArrayLengthDimensionOne => &naming::DimensionOneArrayLengthDimensionOneUpperCamelCase,
            Self::ArrayLengthMoreThanDimensionOne => &naming::ArrayLengthMoreThanDimensionOneUpperCamelCase,
            Self::DimensionOneArrayLengthMoreThanDimensionOne => &naming::DimensionOneArrayLengthMoreThanDimensionOneUpperCamelCase,
            Self::EqualToEncodedStringRepresentation => &naming::EqualToEncodedStringRepresentationUpperCamelCase,
            Self::DimensionOneEqualToEncodedStringRepresentation => &naming::DimensionOneEqualToEncodedStringRepresentationUpperCamelCase,
            Self::ValueIsContainedWithinRange => &naming::ValueIsContainedWithinRangeUpperCamelCase,
            Self::DimensionOneValueIsContainedWithinRange => &naming::DimensionOneValueIsContainedWithinRangeUpperCamelCase,
            Self::ContainsAnotherRange => &naming::ContainsAnotherRangeUpperCamelCase,
            Self::DimensionOneContainsAnotherRange => &naming::DimensionOneContainsAnotherRangeUpperCamelCase,
            Self::StrictlyToLeftOfRange => &naming::StrictlyToLeftOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToLeftOfRange => &naming::DimensionOneStrictlyToLeftOfRangeUpperCamelCase,
            Self::StrictlyToRightOfRange => &naming::StrictlyToRightOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToRightOfRange => &naming::DimensionOneStrictlyToRightOfRangeUpperCamelCase,
            Self::IncludedLowerBound => &naming::IncludedLowerBoundUpperCamelCase,
            Self::DimensionOneIncludedLowerBound => &naming::DimensionOneIncludedLowerBoundUpperCamelCase,
            Self::ExcludedUpperBound => &naming::ExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneExcludedUpperBound => &naming::DimensionOneExcludedUpperBoundUpperCamelCase,
            Self::GreaterThanLowerBound => &naming::GreaterThanLowerBoundUpperCamelCase,
            Self::DimensionOneGreaterThanLowerBound => &naming::DimensionOneGreaterThanLowerBoundUpperCamelCase,
            Self::OverlapWithRange => &naming::OverlapWithRangeUpperCamelCase,
            Self::DimensionOneOverlapWithRange => &naming::DimensionOneOverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange => &naming::AdjacentWithRangeUpperCamelCase,
            Self::DimensionOneAdjacentWithRange => &naming::DimensionOneAdjacentWithRangeUpperCamelCase,
            Self::RangeLength => &naming::RangeLengthUpperCamelCase,
            Self::DimensionOneRangeLength => &naming::DimensionOneRangeLengthUpperCamelCase,
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
    DimensionOneEqual,
    GreaterThan,
    DimensionOneGreaterThan,
    Between,
    DimensionOneBetween,
    In,
    DimensionOneIn,
    CaseSensitiveRegularExpression,
    CaseInsensitiveRegularExpression,
    DimensionOneRegularExpression,
    Before,
    DimensionOneBefore,
    EqualToEncodedStringRepresentation,
    DimensionOneEqualToEncodedStringRepresentation,
    ValueIsContainedWithinRange,
    DimensionOneValueIsContainedWithinRange,
    ContainsAnotherRange,
    DimensionOneContainsAnotherRange,
    StrictlyToLeftOfRange,
    DimensionOneStrictlyToLeftOfRange,
    StrictlyToRightOfRange,
    DimensionOneStrictlyToRightOfRange,
    IncludedLowerBound,
    DimensionOneIncludedLowerBound,
    ExcludedUpperBound,
    DimensionOneExcludedUpperBound,
    GreaterThanLowerBound,
    DimensionOneGreaterThanLowerBound,
    OverlapWithRange,
    DimensionOneOverlapWithRange,
    AdjacentWithRange,
    DimensionOneAdjacentWithRange,
}
impl IsRelevantOnlyForNotNull for PostgresqlTypeFilterHasGeneric {
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::DimensionOneEqual => false,
            Self::GreaterThan => true,
            Self::DimensionOneGreaterThan => true,
            Self::Between => true,
            Self::DimensionOneBetween => true,
            Self::In => false,
            Self::DimensionOneIn => false,
            Self::CaseSensitiveRegularExpression => true,
            Self::CaseInsensitiveRegularExpression => true,
            Self::DimensionOneRegularExpression => true,
            Self::Before => true,
            Self::DimensionOneBefore => true,
            Self::EqualToEncodedStringRepresentation => true,
            Self::DimensionOneEqualToEncodedStringRepresentation => true,
            Self::ValueIsContainedWithinRange => true,
            Self::DimensionOneValueIsContainedWithinRange => true,
            Self::ContainsAnotherRange => true,
            Self::DimensionOneContainsAnotherRange => true,
            Self::StrictlyToLeftOfRange => true,
            Self::DimensionOneStrictlyToLeftOfRange => true,
            Self::StrictlyToRightOfRange => true,
            Self::DimensionOneStrictlyToRightOfRange => true,
            Self::IncludedLowerBound => true,
            Self::DimensionOneIncludedLowerBound => true,
            Self::ExcludedUpperBound => true,
            Self::DimensionOneExcludedUpperBound => true,
            Self::GreaterThanLowerBound => true,
            Self::DimensionOneGreaterThanLowerBound => true,
            Self::OverlapWithRange => true,
            Self::DimensionOneOverlapWithRange => true,
            Self::AdjacentWithRange => true,
            Self::DimensionOneAdjacentWithRange => true,
        }
    }
}
impl std::convert::TryFrom<&PostgresqlTypeFilter> for PostgresqlTypeFilterHasGeneric {
    type Error = ();
    fn try_from(value: &PostgresqlTypeFilter) -> Result<Self, Self::Error> {
        match &value {
            PostgresqlTypeFilter::Equal => Ok(Self::Equal),
            PostgresqlTypeFilter::DimensionOneEqual => Ok(Self::DimensionOneEqual),
            PostgresqlTypeFilter::GreaterThan => Ok(Self::GreaterThan),
            PostgresqlTypeFilter::DimensionOneGreaterThan => Ok(Self::DimensionOneGreaterThan),
            PostgresqlTypeFilter::Between => Ok(Self::Between),
            PostgresqlTypeFilter::DimensionOneBetween => Ok(Self::DimensionOneBetween),
            PostgresqlTypeFilter::In => Ok(Self::In),
            PostgresqlTypeFilter::DimensionOneIn => Ok(Self::DimensionOneIn),
            PostgresqlTypeFilter::RegularExpression => Err(()),
            PostgresqlTypeFilter::DimensionOneRegularExpression => Err(()),
            PostgresqlTypeFilter::Before => Ok(Self::Before),
            PostgresqlTypeFilter::DimensionOneBefore => Ok(Self::DimensionOneBefore),
            PostgresqlTypeFilter::CurrentDate => Err(()),
            PostgresqlTypeFilter::DimensionOneCurrentDate => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentDate => Err(()),
            PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate => Err(()),
            PostgresqlTypeFilter::CurrentTimestamp => Err(()),
            PostgresqlTypeFilter::DimensionOneCurrentTimestamp => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTimestamp => Err(()),
            PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp => Err(()),
            PostgresqlTypeFilter::CurrentTime => Err(()),
            PostgresqlTypeFilter::DimensionOneCurrentTime => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTime => Err(()),
            PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime => Err(()),
            PostgresqlTypeFilter::ArrayLengthDimensionOne => Err(()),
            PostgresqlTypeFilter::DimensionOneArrayLengthDimensionOne => Err(()),
            PostgresqlTypeFilter::ArrayLengthMoreThanDimensionOne => Err(()),
            PostgresqlTypeFilter::DimensionOneArrayLengthMoreThanDimensionOne => Err(()),
            PostgresqlTypeFilter::EqualToEncodedStringRepresentation => Ok(Self::EqualToEncodedStringRepresentation),
            PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation => Ok(Self::DimensionOneEqualToEncodedStringRepresentation),
            PostgresqlTypeFilter::ValueIsContainedWithinRange => Ok(Self::ValueIsContainedWithinRange),
            PostgresqlTypeFilter::DimensionOneValueIsContainedWithinRange => Ok(Self::DimensionOneValueIsContainedWithinRange),
            PostgresqlTypeFilter::ContainsAnotherRange => Ok(Self::ContainsAnotherRange),
            PostgresqlTypeFilter::DimensionOneContainsAnotherRange => Ok(Self::DimensionOneContainsAnotherRange),
            PostgresqlTypeFilter::StrictlyToLeftOfRange => Ok(Self::StrictlyToLeftOfRange),
            PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange => Ok(Self::DimensionOneStrictlyToLeftOfRange),
            PostgresqlTypeFilter::StrictlyToRightOfRange => Ok(Self::StrictlyToRightOfRange),
            PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange => Ok(Self::DimensionOneStrictlyToRightOfRange),
            PostgresqlTypeFilter::IncludedLowerBound => Ok(Self::IncludedLowerBound),
            PostgresqlTypeFilter::DimensionOneIncludedLowerBound => Ok(Self::DimensionOneIncludedLowerBound),
            PostgresqlTypeFilter::ExcludedUpperBound => Ok(Self::ExcludedUpperBound),
            PostgresqlTypeFilter::DimensionOneExcludedUpperBound => Ok(Self::DimensionOneExcludedUpperBound),
            PostgresqlTypeFilter::GreaterThanLowerBound => Ok(Self::GreaterThanLowerBound),
            PostgresqlTypeFilter::DimensionOneGreaterThanLowerBound => Ok(Self::DimensionOneGreaterThanLowerBound),
            PostgresqlTypeFilter::OverlapWithRange => Ok(Self::OverlapWithRange),
            PostgresqlTypeFilter::DimensionOneOverlapWithRange => Ok(Self::DimensionOneOverlapWithRange),
            PostgresqlTypeFilter::AdjacentWithRange => Ok(Self::AdjacentWithRange),
            PostgresqlTypeFilter::DimensionOneAdjacentWithRange => Ok(Self::DimensionOneAdjacentWithRange),
            PostgresqlTypeFilter::RangeLength => Err(()),
            PostgresqlTypeFilter::DimensionOneRangeLength => Err(()),
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypeFilter {
    Equal {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneAllElementsEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoAllElementsEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeAllElementsEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourAllElementsEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneLengthEqual,
    DimensionTwoLengthEqual,
    DimensionThreeLengthEqual,
    DimensionFourLengthEqual,
    GreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneContainsElementGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoContainsElementGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeContainsElementGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourContainsElementGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneAllElementsGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoAllElementsGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeAllElementsGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourAllElementsGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    Between {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneBetween {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoBetween {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeBetween {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourBetween {
        ident: proc_macro2::TokenStream,
    },
    In {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneIn {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoIn {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeIn {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourIn {
        ident: proc_macro2::TokenStream,
    },
    RegularExpression,
    DimensionOneRegularExpression,
    DimensionTwoRegularExpression,
    DimensionThreeRegularExpression,
    DimensionFourRegularExpression,
    DimensionOneContainsElementRegularExpression,
    DimensionTwoContainsElementRegularExpression,
    DimensionThreeContainsElementRegularExpression,
    DimensionFourContainsElementRegularExpression,
    DimensionOneAllElementsRegularExpression,
    DimensionTwoAllElementsRegularExpression,
    DimensionThreeAllElementsRegularExpression,
    DimensionFourAllElementsRegularExpression,
    DimensionOneLengthMoreThan,
    DimensionTwoLengthMoreThan,
    DimensionThreeLengthMoreThan,
    DimensionFourLengthMoreThan,
    DimensionOneContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
    // ContainedInArray,
    DimensionOneOverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoOverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreeOverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourOverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
}
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal {
                ident: _
            } => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual {
                ident: _
            } => &naming::DimensionOneEqualUpperCamelCase,
            Self::DimensionTwoEqual {
                ident: _
            } => &naming::DimensionTwoEqualUpperCamelCase,
            Self::DimensionThreeEqual {
                ident: _
            } => &naming::DimensionThreeEqualUpperCamelCase,
            Self::DimensionFourEqual {
                ident: _
            } => &naming::DimensionFourEqualUpperCamelCase,
            Self::DimensionOneAllElementsEqual {
                ident: _
            } => &naming::DimensionOneAllElementsEqualUpperCamelCase,
            Self::DimensionTwoAllElementsEqual {
                ident: _
            } => &naming::DimensionTwoAllElementsEqualUpperCamelCase,
            Self::DimensionThreeAllElementsEqual {
                ident: _
            } => &naming::DimensionThreeAllElementsEqualUpperCamelCase,
            Self::DimensionFourAllElementsEqual {
                ident: _
            } => &naming::DimensionFourAllElementsEqualUpperCamelCase,
            Self::DimensionOneLengthEqual => &naming::DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionTwoLengthEqual => &naming::DimensionTwoLengthEqualUpperCamelCase,
            Self::DimensionThreeLengthEqual => &naming::DimensionThreeLengthEqualUpperCamelCase,
            Self::DimensionFourLengthEqual => &naming::DimensionFourLengthEqualUpperCamelCase,
            Self::GreaterThan {
                ident: _
            } => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan {
                ident: _
            } => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::DimensionTwoGreaterThan {
                ident: _
            } => &naming::DimensionTwoGreaterThanUpperCamelCase,
            Self::DimensionThreeGreaterThan {
                ident: _
            } => &naming::DimensionThreeGreaterThanUpperCamelCase,
            Self::DimensionFourGreaterThan {
                ident: _
            } => &naming::DimensionFourGreaterThanUpperCamelCase,
            Self::DimensionOneContainsElementGreaterThan {
                ident: _
            } => &naming::DimensionOneContainsElementGreaterThanUpperCamelCase,
            Self::DimensionTwoContainsElementGreaterThan {
                ident: _
            } => &naming::DimensionTwoContainsElementGreaterThanUpperCamelCase,
            Self::DimensionThreeContainsElementGreaterThan {
                ident: _
            } => &naming::DimensionThreeContainsElementGreaterThanUpperCamelCase,
            Self::DimensionFourContainsElementGreaterThan {
                ident: _
            } => &naming::DimensionFourContainsElementGreaterThanUpperCamelCase,
            Self::DimensionOneAllElementsGreaterThan {
                ident: _
            } => &naming::DimensionOneAllElementsGreaterThanUpperCamelCase,
            Self::DimensionTwoAllElementsGreaterThan {
                ident: _
            } => &naming::DimensionTwoAllElementsGreaterThanUpperCamelCase,
            Self::DimensionThreeAllElementsGreaterThan {
                ident: _
            } => &naming::DimensionThreeAllElementsGreaterThanUpperCamelCase,
            Self::DimensionFourAllElementsGreaterThan {
                ident: _
            } => &naming::DimensionFourAllElementsGreaterThanUpperCamelCase,
            Self::Between {
                ident: _
            } => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween {
                ident: _
            } => &naming::DimensionOneBetweenUpperCamelCase,
            Self::DimensionTwoBetween {
                ident: _
            } => &naming::DimensionTwoBetweenUpperCamelCase,
            Self::DimensionThreeBetween {
                ident: _
            } => &naming::DimensionThreeBetweenUpperCamelCase,
            Self::DimensionFourBetween {
                ident: _
            } => &naming::DimensionFourBetweenUpperCamelCase,
            Self::In {
                ident: _
            } => &naming::InUpperCamelCase,
            Self::DimensionOneIn {
                ident: _
            } => &naming::DimensionOneInUpperCamelCase,
            Self::DimensionTwoIn {
                ident: _
            } => &naming::DimensionTwoInUpperCamelCase,
            Self::DimensionThreeIn {
                ident: _
            } => &naming::DimensionThreeInUpperCamelCase,
            Self::DimensionFourIn {
                ident: _
            } => &naming::DimensionFourInUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &naming::DimensionOneRegularExpressionUpperCamelCase,
            Self::DimensionTwoRegularExpression => &naming::DimensionTwoRegularExpressionUpperCamelCase,
            Self::DimensionThreeRegularExpression => &naming::DimensionThreeRegularExpressionUpperCamelCase,
            Self::DimensionFourRegularExpression => &naming::DimensionFourRegularExpressionUpperCamelCase,
            Self::DimensionOneContainsElementRegularExpression => &naming::DimensionOneContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionTwoContainsElementRegularExpression => &naming::DimensionTwoContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionThreeContainsElementRegularExpression => &naming::DimensionThreeContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionFourContainsElementRegularExpression => &naming::DimensionFourContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionOneAllElementsRegularExpression => &naming::DimensionOneAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionTwoAllElementsRegularExpression => &naming::DimensionTwoAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionThreeAllElementsRegularExpression => &naming::DimensionThreeAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionFourAllElementsRegularExpression => &naming::DimensionFourAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionOneLengthMoreThan => &naming::DimensionOneLengthMoreThanUpperCamelCase,
            Self::DimensionTwoLengthMoreThan => &naming::DimensionTwoLengthMoreThanUpperCamelCase,
            Self::DimensionThreeLengthMoreThan => &naming::DimensionThreeLengthMoreThanUpperCamelCase,
            Self::DimensionFourLengthMoreThan => &naming::DimensionFourLengthMoreThanUpperCamelCase,
            Self::DimensionOneContainsAllElementsOfArray {
                ident: _
            } => &naming::DimensionOneContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionTwoContainsAllElementsOfArray {
                ident: _
            } => &naming::DimensionTwoContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionThreeContainsAllElementsOfArray {
                ident: _
            } => &naming::DimensionThreeContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionFourContainsAllElementsOfArray {
                ident: _
            } => &naming::DimensionFourContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionOneOverlapsWithArray {
                ident: _
            } => &naming::DimensionOneOverlapsWithArrayUpperCamelCase,
            Self::DimensionTwoOverlapsWithArray {
                ident: _
            } => &naming::DimensionTwoOverlapsWithArrayUpperCamelCase,
            Self::DimensionThreeOverlapsWithArray {
                ident: _
            } => &naming::DimensionThreeOverlapsWithArrayUpperCamelCase,
            Self::DimensionFourOverlapsWithArray {
                ident: _
            } => &naming::DimensionFourOverlapsWithArrayUpperCamelCase,
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
//todo remove this
pub enum PostgresqlJsonTypeFilterHasGeneric {
    Equal,
    DimensionOneEqual,
    DimensionTwoEqual,
    DimensionThreeEqual,
    DimensionFourEqual,
    DimensionOneAllElementsEqual,
    DimensionTwoAllElementsEqual,
    DimensionThreeAllElementsEqual,
    DimensionFourAllElementsEqual,
    GreaterThan,
    DimensionOneGreaterThan,
    DimensionTwoGreaterThan,
    DimensionThreeGreaterThan,
    DimensionFourGreaterThan,
    DimensionOneContainsElementGreaterThan,
    DimensionTwoContainsElementGreaterThan,
    DimensionThreeContainsElementGreaterThan,
    DimensionFourContainsElementGreaterThan,
    DimensionOneAllElementsGreaterThan,
    DimensionTwoAllElementsGreaterThan,
    DimensionThreeAllElementsGreaterThan,
    DimensionFourAllElementsGreaterThan,
    Between,
    DimensionOneBetween,
    DimensionTwoBetween,
    DimensionThreeBetween,
    DimensionFourBetween,
    In,
    DimensionOneIn,
    DimensionTwoIn,
    DimensionThreeIn,
    DimensionFourIn,
    DimensionOneContainsAllElementsOfArray,
    DimensionTwoContainsAllElementsOfArray,
    DimensionThreeContainsAllElementsOfArray,
    DimensionFourContainsAllElementsOfArray,
    DimensionOneOverlapsWithArray,
    DimensionTwoOverlapsWithArray,
    DimensionThreeOverlapsWithArray,
    DimensionFourOverlapsWithArray,
}
impl IsRelevantOnlyForNotNull for PostgresqlJsonTypeFilterHasGeneric {
    //todo maybe not need
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::DimensionOneEqual => false,
            Self::DimensionTwoEqual => false,
            Self::DimensionThreeEqual => false,
            Self::DimensionFourEqual => false,
            Self::DimensionOneAllElementsEqual => false,
            Self::DimensionTwoAllElementsEqual => false,
            Self::DimensionThreeAllElementsEqual => false,
            Self::DimensionFourAllElementsEqual => false,
            Self::GreaterThan => true,
            Self::DimensionOneGreaterThan => true,
            Self::DimensionTwoGreaterThan => true,
            Self::DimensionThreeGreaterThan => true,
            Self::DimensionFourGreaterThan => true,
            Self::DimensionOneContainsElementGreaterThan => true,
            Self::DimensionTwoContainsElementGreaterThan => true,
            Self::DimensionThreeContainsElementGreaterThan => true,
            Self::DimensionFourContainsElementGreaterThan => true,
            Self::DimensionOneAllElementsGreaterThan => true,
            Self::DimensionTwoAllElementsGreaterThan => true,
            Self::DimensionThreeAllElementsGreaterThan => true,
            Self::DimensionFourAllElementsGreaterThan => true,
            Self::Between => true,
            Self::DimensionOneBetween => true,
            Self::DimensionTwoBetween => true,
            Self::DimensionThreeBetween => true,
            Self::DimensionFourBetween => true,
            Self::In => false,
            Self::DimensionOneIn => false,
            Self::DimensionTwoIn => false,
            Self::DimensionThreeIn => false,
            Self::DimensionFourIn => false,
            Self::DimensionOneContainsAllElementsOfArray => false,
            Self::DimensionTwoContainsAllElementsOfArray => false,
            Self::DimensionThreeContainsAllElementsOfArray => false,
            Self::DimensionFourContainsAllElementsOfArray => false,
            Self::DimensionOneOverlapsWithArray => false,
            Self::DimensionTwoOverlapsWithArray => false,
            Self::DimensionThreeOverlapsWithArray => false,
            Self::DimensionFourOverlapsWithArray => false,
        }
    }
}
impl std::convert::TryFrom<&PostgresqlJsonTypeFilter> for PostgresqlJsonTypeFilterHasGeneric {
    type Error = ();
    fn try_from(value: &PostgresqlJsonTypeFilter) -> Result<Self, Self::Error> {
        match &value {
            PostgresqlJsonTypeFilter::Equal {
                ident: _
            } => Ok(Self::Equal),
            PostgresqlJsonTypeFilter::DimensionOneEqual {
                ident: _
            } => Ok(Self::DimensionOneEqual),
            PostgresqlJsonTypeFilter::DimensionTwoEqual {
                ident: _
            } => Ok(Self::DimensionTwoEqual),
            PostgresqlJsonTypeFilter::DimensionThreeEqual {
                ident: _
            } => Ok(Self::DimensionThreeEqual),
            PostgresqlJsonTypeFilter::DimensionFourEqual {
                ident: _
            } => Ok(Self::DimensionFourEqual),
            PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                ident: _
            } => Ok(Self::DimensionOneAllElementsEqual),
            PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                ident: _
            } => Ok(Self::DimensionTwoAllElementsEqual),
            PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                ident: _
            } => Ok(Self::DimensionThreeAllElementsEqual),
            PostgresqlJsonTypeFilter::DimensionFourAllElementsEqual {
                ident: _
            } => Ok(Self::DimensionFourAllElementsEqual),
            PostgresqlJsonTypeFilter::DimensionOneLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::GreaterThan {
                ident: _
            } => Ok(Self::GreaterThan),
            PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                ident: _
            } => Ok(Self::DimensionOneGreaterThan),
            PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                ident: _
            } => Ok(Self::DimensionTwoGreaterThan),
            PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                ident: _
            } => Ok(Self::DimensionThreeGreaterThan),
            PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                ident: _
            } => Ok(Self::DimensionFourGreaterThan),
            PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                ident: _
            } => Ok(Self::DimensionOneContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                ident: _
            } => Ok(Self::DimensionTwoContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                ident: _
            } => Ok(Self::DimensionThreeContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::DimensionFourContainsElementGreaterThan {
                ident: _
            } => Ok(Self::DimensionFourContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                ident: _
            } => Ok(Self::DimensionOneAllElementsGreaterThan),
            PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                ident: _
            } => Ok(Self::DimensionTwoAllElementsGreaterThan),
            PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                ident: _
            } => Ok(Self::DimensionThreeAllElementsGreaterThan),
            PostgresqlJsonTypeFilter::DimensionFourAllElementsGreaterThan {
                ident: _
            } => Ok(Self::DimensionFourAllElementsGreaterThan),
            PostgresqlJsonTypeFilter::Between {
                ident: _
            } => Ok(Self::Between),
            PostgresqlJsonTypeFilter::DimensionOneBetween {
                ident: _
            } => Ok(Self::DimensionOneBetween),
            PostgresqlJsonTypeFilter::DimensionTwoBetween {
                ident: _
            } => Ok(Self::DimensionTwoBetween),
            PostgresqlJsonTypeFilter::DimensionThreeBetween {
                ident: _
            } => Ok(Self::DimensionThreeBetween),
            PostgresqlJsonTypeFilter::DimensionFourBetween {
                ident: _
            } => Ok(Self::DimensionFourBetween),
            PostgresqlJsonTypeFilter::In {
                ident: _
            } => Ok(Self::In),
            PostgresqlJsonTypeFilter::DimensionOneIn {
                ident: _
            } => Ok(Self::DimensionOneIn),
            PostgresqlJsonTypeFilter::DimensionTwoIn {
                ident: _
            } => Ok(Self::DimensionTwoIn),
            PostgresqlJsonTypeFilter::DimensionThreeIn {
                ident: _
            } => Ok(Self::DimensionThreeIn),
            PostgresqlJsonTypeFilter::DimensionFourIn {
                ident: _
            } => Ok(Self::DimensionFourIn),
            PostgresqlJsonTypeFilter::RegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                ident: _
            } => Ok(Self::DimensionOneContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                ident: _
            } => Ok(Self::DimensionTwoContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                ident: _
            } => Ok(Self::DimensionThreeContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                ident: _
            } => Ok(Self::DimensionFourContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                ident: _
            } => Ok(Self::DimensionOneOverlapsWithArray),
            PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                ident: _
            } => Ok(Self::DimensionTwoOverlapsWithArray),
            PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                ident: _
            } => Ok(Self::DimensionThreeOverlapsWithArray),
            PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                ident: _
            } => Ok(Self::DimensionFourOverlapsWithArray),
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
