#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlTypeFilter {
    Equal,
    GreaterThan,
    Between,
    In,
    RegularExpression,
    Before,
    CurrentDate,
    GreaterThanCurrentDate,
    CurrentTimestamp,
    GreaterThanCurrentTimestamp,
    CurrentTime,
    GreaterThanCurrentTime,
    ArrayLengthDimensionOne,
    ArrayLengthMoreThanDimensionOne,
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
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::Before => &naming::BeforeUpperCamelCase,
            Self::CurrentDate => &naming::CurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => &naming::GreaterThanCurrentDateUpperCamelCase,
            Self::CurrentTimestamp => &naming::CurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => &naming::GreaterThanCurrentTimestampUpperCamelCase,
            Self::CurrentTime => &naming::CurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => &naming::GreaterThanCurrentTimeUpperCamelCase,
            Self::ArrayLengthDimensionOne => &naming::ArrayLengthDimensionOneUpperCamelCase,
            Self::ArrayLengthMoreThanDimensionOne => &naming::ArrayLengthMoreThanDimensionOneUpperCamelCase,
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
            PostgresqlTypeFilter::RegularExpression => Err(()),
            PostgresqlTypeFilter::Before => Ok(Self::Before),
            PostgresqlTypeFilter::CurrentDate => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentDate => Err(()),
            PostgresqlTypeFilter::CurrentTimestamp => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTimestamp => Err(()),
            PostgresqlTypeFilter::CurrentTime => Err(()),
            PostgresqlTypeFilter::GreaterThanCurrentTime => Err(()),
            PostgresqlTypeFilter::ArrayLengthDimensionOne => Err(()),
            PostgresqlTypeFilter::ArrayLengthMoreThanDimensionOne => Err(()),
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
    Equal {
        ident: proc_macro2::TokenStream,
    },
    DimensionOnePositionEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoPositionEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreePositionEqual {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourPositionEqual {
        ident: proc_macro2::TokenStream,
    },
    GreaterThan {
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
    DimensionOneLengthEqual,
    DimensionTwoLengthEqual,
    DimensionThreeLengthEqual,
    DimensionFourLengthEqual,
    DimensionOneLengthMoreThan,
    DimensionTwoLengthMoreThan,
    DimensionThreeLengthMoreThan,
    DimensionFourLengthMoreThan,
    DimensionOnePositionGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionTwoPositionGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionThreePositionGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionFourPositionGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    DimensionOnePositionRegularExpression,
    DimensionTwoPositionRegularExpression,
    DimensionThreePositionRegularExpression,
    DimensionFourPositionRegularExpression,
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
    DimensionOneContainsElementRegularExpression,
    DimensionTwoContainsElementRegularExpression,
    DimensionThreeContainsElementRegularExpression,
    DimensionFourContainsElementRegularExpression,
    DimensionOneAllElementsRegularExpression,
    DimensionTwoAllElementsRegularExpression,
    DimensionThreeAllElementsRegularExpression,
    DimensionFourAllElementsRegularExpression,
}
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal {
                ident: _
            } => &naming::EqualUpperCamelCase,
            Self::DimensionOnePositionEqual {
                ident: _
            } => &naming::DimensionOnePositionEqualUpperCamelCase,
            Self::DimensionTwoPositionEqual {
                ident: _
            } => &naming::DimensionTwoPositionEqualUpperCamelCase,
            Self::DimensionThreePositionEqual {
                ident: _
            } => &naming::DimensionThreePositionEqualUpperCamelCase,
            Self::DimensionFourPositionEqual {
                ident: _
            } => &naming::DimensionFourPositionEqualUpperCamelCase,
            Self::GreaterThan {
                ident: _
            } => &naming::GreaterThanUpperCamelCase,
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
            Self::DimensionOneLengthEqual => &naming::DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionTwoLengthEqual => &naming::DimensionTwoLengthEqualUpperCamelCase,
            Self::DimensionThreeLengthEqual => &naming::DimensionThreeLengthEqualUpperCamelCase,
            Self::DimensionFourLengthEqual => &naming::DimensionFourLengthEqualUpperCamelCase,
            Self::DimensionOneLengthMoreThan => &naming::DimensionOneLengthMoreThanUpperCamelCase,
            Self::DimensionTwoLengthMoreThan => &naming::DimensionTwoLengthMoreThanUpperCamelCase,
            Self::DimensionThreeLengthMoreThan => &naming::DimensionThreeLengthMoreThanUpperCamelCase,
            Self::DimensionFourLengthMoreThan => &naming::DimensionFourLengthMoreThanUpperCamelCase,
            Self::DimensionOnePositionGreaterThan {
                ident: _
            } => &naming::DimensionOnePositionGreaterThanUpperCamelCase,
            Self::DimensionTwoPositionGreaterThan {
                ident: _
            } => &naming::DimensionTwoPositionGreaterThanUpperCamelCase,
            Self::DimensionThreePositionGreaterThan {
                ident: _
            } => &naming::DimensionThreePositionGreaterThanUpperCamelCase,
            Self::DimensionFourPositionGreaterThan {
                ident: _
            } => &naming::DimensionFourPositionGreaterThanUpperCamelCase,
            Self::DimensionOnePositionRegularExpression => &naming::DimensionOnePositionRegularExpressionUpperCamelCase,
            Self::DimensionTwoPositionRegularExpression => &naming::DimensionTwoPositionRegularExpressionUpperCamelCase,
            Self::DimensionThreePositionRegularExpression => &naming::DimensionThreePositionRegularExpressionUpperCamelCase,
            Self::DimensionFourPositionRegularExpression => &naming::DimensionFourPositionRegularExpressionUpperCamelCase,
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
            Self::DimensionOneContainsElementRegularExpression => &naming::DimensionOneContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionTwoContainsElementRegularExpression => &naming::DimensionTwoContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionThreeContainsElementRegularExpression => &naming::DimensionThreeContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionFourContainsElementRegularExpression => &naming::DimensionFourContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionOneAllElementsRegularExpression => &naming::DimensionOneAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionTwoAllElementsRegularExpression => &naming::DimensionTwoAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionThreeAllElementsRegularExpression => &naming::DimensionThreeAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionFourAllElementsRegularExpression => &naming::DimensionFourAllElementsRegularExpressionUpperCamelCase,
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
    GreaterThan,
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
    DimensionOnePositionEqual,
    DimensionTwoPositionEqual,
    DimensionThreePositionEqual,
    DimensionFourPositionEqual,
    DimensionOnePositionGreaterThan,
    DimensionTwoPositionGreaterThan,
    DimensionThreePositionGreaterThan,
    DimensionFourPositionGreaterThan,
    DimensionOneContainsAllElementsOfArray,
    DimensionTwoContainsAllElementsOfArray,
    DimensionThreeContainsAllElementsOfArray,
    DimensionFourContainsAllElementsOfArray,
    DimensionOneOverlapsWithArray,
    DimensionTwoOverlapsWithArray,
    DimensionThreeOverlapsWithArray,
    DimensionFourOverlapsWithArray,
    DimensionOneAllElementsEqual,
    DimensionTwoAllElementsEqual,
    DimensionThreeAllElementsEqual,
    DimensionFourAllElementsEqual,
    DimensionOneContainsElementGreaterThan,
    DimensionTwoContainsElementGreaterThan,
    DimensionThreeContainsElementGreaterThan,
    DimensionFourContainsElementGreaterThan,
    DimensionOneAllElementsGreaterThan,
    DimensionTwoAllElementsGreaterThan,
    DimensionThreeAllElementsGreaterThan,
    DimensionFourAllElementsGreaterThan,
}
impl IsRelevantOnlyForNotNull for PostgresqlJsonTypeFilterHasGeneric {
    //todo maybe not need
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::GreaterThan => true,
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
            Self::DimensionOnePositionEqual => false,
            Self::DimensionTwoPositionEqual => false,
            Self::DimensionThreePositionEqual => false,
            Self::DimensionFourPositionEqual => false,
            Self::DimensionOnePositionGreaterThan => true,
            Self::DimensionTwoPositionGreaterThan => true,
            Self::DimensionThreePositionGreaterThan => true,
            Self::DimensionFourPositionGreaterThan => true,
            Self::DimensionOneContainsAllElementsOfArray => false,
            Self::DimensionTwoContainsAllElementsOfArray => false,
            Self::DimensionThreeContainsAllElementsOfArray => false,
            Self::DimensionFourContainsAllElementsOfArray => false,
            Self::DimensionOneOverlapsWithArray => false,
            Self::DimensionTwoOverlapsWithArray => false,
            Self::DimensionThreeOverlapsWithArray => false,
            Self::DimensionFourOverlapsWithArray => false,
            Self::DimensionOneAllElementsEqual => false,
            Self::DimensionTwoAllElementsEqual => false,
            Self::DimensionThreeAllElementsEqual => false,
            Self::DimensionFourAllElementsEqual => false,
            Self::DimensionOneContainsElementGreaterThan => true,
            Self::DimensionTwoContainsElementGreaterThan => true,
            Self::DimensionThreeContainsElementGreaterThan => true,
            Self::DimensionFourContainsElementGreaterThan => true,
            Self::DimensionOneAllElementsGreaterThan => true,
            Self::DimensionTwoAllElementsGreaterThan => true,
            Self::DimensionThreeAllElementsGreaterThan => true,
            Self::DimensionFourAllElementsGreaterThan => true,
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
            PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                ident: _
            } => Ok(Self::DimensionOnePositionEqual),
            PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                ident: _
            } => Ok(Self::DimensionTwoPositionEqual),
            PostgresqlJsonTypeFilter::DimensionThreePositionEqual {
                ident: _
            } => Ok(Self::DimensionThreePositionEqual),
            PostgresqlJsonTypeFilter::DimensionFourPositionEqual {
                ident: _
            } => Ok(Self::DimensionFourPositionEqual),
            PostgresqlJsonTypeFilter::GreaterThan {
                ident: _
            } => Ok(Self::GreaterThan),
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
            PostgresqlJsonTypeFilter::DimensionOneLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourLengthEqual => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan => Err(()),
            PostgresqlJsonTypeFilter::DimensionOnePositionGreaterThan {
                ident: _
            } => Ok(Self::DimensionOnePositionGreaterThan),
            PostgresqlJsonTypeFilter::DimensionTwoPositionGreaterThan {
                ident: _
            } => Ok(Self::DimensionTwoPositionGreaterThan),
            PostgresqlJsonTypeFilter::DimensionThreePositionGreaterThan {
                ident: _
            } => Ok(Self::DimensionThreePositionGreaterThan),
            PostgresqlJsonTypeFilter::DimensionFourPositionGreaterThan {
                ident: _
            } => Ok(Self::DimensionFourPositionGreaterThan),
            PostgresqlJsonTypeFilter::DimensionOnePositionRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoPositionRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreePositionRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourPositionRegularExpression => Err(()),
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
            PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::DimensionFourAllElementsRegularExpression => Err(()),
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
