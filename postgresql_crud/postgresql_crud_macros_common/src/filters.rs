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
    GreaterThan {
        ident: proc_macro2::TokenStream,
    },
    Between {
        ident: proc_macro2::TokenStream,
    },
    In {
        ident: proc_macro2::TokenStream,
    },
    RegularExpression,
    LengthEqual,
    LengthMoreThan,
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
    DimensionOnePositionGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    PositionRegularExpression,
    ContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
    // ContainedInArray,
    OverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
    AllElementsEqual {
        ident: proc_macro2::TokenStream,
    },
    ContainsElementGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    AllElementsGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    ContainsElementRegularExpression,
    AllElementsRegularExpression,
}
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal {
                ident: _
            } => &naming::EqualUpperCamelCase,
            Self::GreaterThan {
                ident: _
            } => &naming::GreaterThanUpperCamelCase,
            Self::Between {
                ident: _
            } => &naming::BetweenUpperCamelCase,
            Self::In {
                ident: _
            } => &naming::InUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::LengthEqual => &naming::LengthEqualUpperCamelCase,
            Self::LengthMoreThan => &naming::LengthMoreThanUpperCamelCase,
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
            Self::DimensionOnePositionGreaterThan {
                ident: _
            } => &naming::DimensionOnePositionGreaterThanUpperCamelCase,
            Self::PositionRegularExpression => &naming::PositionRegularExpressionUpperCamelCase,
            Self::ContainsAllElementsOfArray {
                ident: _
            } => &naming::ContainsAllElementsOfArrayUpperCamelCase,
            Self::OverlapsWithArray {
                ident: _
            } => &naming::OverlapsWithArrayUpperCamelCase,
            Self::AllElementsEqual {
                ident: _
            } => &naming::AllElementsEqualUpperCamelCase,
            Self::ContainsElementGreaterThan {
                ident: _
            } => &naming::ContainsElementGreaterThanUpperCamelCase,
            Self::AllElementsGreaterThan {
                ident: _
            } => &naming::AllElementsGreaterThanUpperCamelCase,
            Self::ContainsElementRegularExpression => &naming::ContainsElementRegularExpressionUpperCamelCase,
            Self::AllElementsRegularExpression => &naming::AllElementsRegularExpressionUpperCamelCase,
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
    In,
    DimensionOnePositionEqual,
    DimensionTwoPositionEqual,
    DimensionThreePositionEqual,
    DimensionFourPositionEqual,
    DimensionOnePositionGreaterThan,
    ContainsAllElementsOfArray,
    OverlapsWithArray,
    AllElementsEqual,
    ContainsElementGreaterThan,
    AllElementsGreaterThan,
    AllElementsCaseSensitiveRegularExpression,
    AllElementsCaseInsensitiveRegularExpression,
}
impl IsRelevantOnlyForNotNull for PostgresqlJsonTypeFilterHasGeneric {
    //todo maybe not need
    fn is_relevant_only_for_not_null(&self) -> std::primitive::bool {
        match &self {
            Self::Equal => false,
            Self::GreaterThan => true,
            Self::Between => true,
            Self::In => false,
            Self::DimensionOnePositionEqual => false,
            Self::DimensionTwoPositionEqual => false,
            Self::DimensionThreePositionEqual => false,
            Self::DimensionFourPositionEqual => false,
            Self::DimensionOnePositionGreaterThan => true,
            Self::ContainsAllElementsOfArray => false,
            Self::OverlapsWithArray => false,
            Self::AllElementsEqual => false,
            Self::ContainsElementGreaterThan => true,
            Self::AllElementsGreaterThan => true,
            Self::AllElementsCaseSensitiveRegularExpression => true,
            Self::AllElementsCaseInsensitiveRegularExpression => true,
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
            PostgresqlJsonTypeFilter::GreaterThan {
                ident: _
            } => Ok(Self::GreaterThan),
            PostgresqlJsonTypeFilter::Between {
                ident: _
            } => Ok(Self::Between),
            PostgresqlJsonTypeFilter::In {
                ident: _
            } => Ok(Self::In),
            PostgresqlJsonTypeFilter::RegularExpression => Err(()),
            PostgresqlJsonTypeFilter::LengthEqual => Err(()),
            PostgresqlJsonTypeFilter::LengthMoreThan => Err(()),
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
            PostgresqlJsonTypeFilter::DimensionOnePositionGreaterThan {
                ident: _
            } => Ok(Self::DimensionOnePositionGreaterThan),
            PostgresqlJsonTypeFilter::PositionRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::ContainsAllElementsOfArray {
                ident: _
            } => Ok(Self::ContainsAllElementsOfArray),
            PostgresqlJsonTypeFilter::OverlapsWithArray {
                ident: _
            } => Ok(Self::OverlapsWithArray),
            PostgresqlJsonTypeFilter::AllElementsEqual {
                ident: _
            } => Ok(Self::AllElementsEqual),
            PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                ident: _
            } => Ok(Self::ContainsElementGreaterThan),
            PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                ident: _
            } => Ok(Self::AllElementsGreaterThan),
            PostgresqlJsonTypeFilter::ContainsElementRegularExpression => Err(()),
            PostgresqlJsonTypeFilter::AllElementsRegularExpression => Err(()),
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
