#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlTypeFilter {
    Equal { ident: proc_macro2::TokenStream },
    DimensionOneEqual { ident: proc_macro2::TokenStream },
    GreaterThan { ident: proc_macro2::TokenStream },
    DimensionOneGreaterThan { ident: proc_macro2::TokenStream },
    Between { ident: proc_macro2::TokenStream },
    DimensionOneBetween { ident: proc_macro2::TokenStream },
    In { ident: proc_macro2::TokenStream },
    DimensionOneIn { ident: proc_macro2::TokenStream },
    RegularExpression,
    DimensionOneRegularExpression,
    Before { ident: proc_macro2::TokenStream },
    DimensionOneBefore { ident: proc_macro2::TokenStream },
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
    DimensionOneLengthEqual,
    DimensionOneLengthGreaterThan,
    EqualToEncodedStringRepresentation,
    DimensionOneEqualToEncodedStringRepresentation,
    FindRangesWithinGivenRange { ident: proc_macro2::TokenStream },
    DimensionOneFindRangesWithinGivenRange { ident: proc_macro2::TokenStream },
    FindRangesThatFullyContainTheGivenRange { ident: proc_macro2::TokenStream },
    DimensionOneFindRangesThatFullyContainTheGivenRange { ident: proc_macro2::TokenStream },
    StrictlyToLeftOfRange { ident: proc_macro2::TokenStream },
    DimensionOneStrictlyToLeftOfRange { ident: proc_macro2::TokenStream },
    StrictlyToRightOfRange { ident: proc_macro2::TokenStream },
    DimensionOneStrictlyToRightOfRange { ident: proc_macro2::TokenStream },
    IncludedLowerBound { ident: proc_macro2::TokenStream },
    DimensionOneIncludedLowerBound { ident: proc_macro2::TokenStream },
    ExcludedUpperBound { ident: proc_macro2::TokenStream },
    DimensionOneExcludedUpperBound { ident: proc_macro2::TokenStream },
    GreaterThanIncludedLowerBound { ident: proc_macro2::TokenStream },
    DimensionOneGreaterThanIncludedLowerBound { ident: proc_macro2::TokenStream },
    GreaterThanExcludedUpperBound { ident: proc_macro2::TokenStream },
    DimensionOneGreaterThanExcludedUpperBound { ident: proc_macro2::TokenStream },
    OverlapWithRange { ident: proc_macro2::TokenStream },
    DimensionOneOverlapWithRange { ident: proc_macro2::TokenStream },
    AdjacentWithRange { ident: proc_macro2::TokenStream },
    DimensionOneAdjacentWithRange { ident: proc_macro2::TokenStream },
    RangeLength,
    DimensionOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PostgresqlFilter for PostgresqlTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual { .. } => &naming::DimensionOneEqualUpperCamelCase,
            Self::GreaterThan { .. } => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { .. } => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::Between { .. } => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween { .. } => &naming::DimensionOneBetweenUpperCamelCase,
            Self::In { .. } => &naming::InUpperCamelCase,
            Self::DimensionOneIn { .. } => &naming::DimensionOneInUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &naming::DimensionOneRegularExpressionUpperCamelCase,
            Self::Before { .. } => &naming::BeforeUpperCamelCase,
            Self::DimensionOneBefore { .. } => &naming::DimensionOneBeforeUpperCamelCase,
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
            Self::DimensionOneLengthEqual => &naming::DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionOneLengthGreaterThan => &naming::DimensionOneLengthGreaterThanUpperCamelCase,
            Self::EqualToEncodedStringRepresentation => &naming::EqualToEncodedStringRepresentationUpperCamelCase,
            Self::DimensionOneEqualToEncodedStringRepresentation => &naming::DimensionOneEqualToEncodedStringRepresentationUpperCamelCase,
            Self::FindRangesWithinGivenRange { .. } => &naming::FindRangesWithinGivenRangeUpperCamelCase,
            Self::DimensionOneFindRangesWithinGivenRange { .. } => &naming::DimensionOneFindRangesWithinGivenRangeUpperCamelCase,
            Self::FindRangesThatFullyContainTheGivenRange { .. } => &naming::FindRangesThatFullyContainTheGivenRangeUpperCamelCase,
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => &naming::DimensionOneFindRangesThatFullyContainTheGivenRangeUpperCamelCase,
            Self::StrictlyToLeftOfRange { .. } => &naming::StrictlyToLeftOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToLeftOfRange { .. } => &naming::DimensionOneStrictlyToLeftOfRangeUpperCamelCase,
            Self::StrictlyToRightOfRange { .. } => &naming::StrictlyToRightOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToRightOfRange { .. } => &naming::DimensionOneStrictlyToRightOfRangeUpperCamelCase,
            Self::IncludedLowerBound { .. } => &naming::IncludedLowerBoundUpperCamelCase,
            Self::DimensionOneIncludedLowerBound { .. } => &naming::DimensionOneIncludedLowerBoundUpperCamelCase,
            Self::ExcludedUpperBound { .. } => &naming::ExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneExcludedUpperBound { .. } => &naming::DimensionOneExcludedUpperBoundUpperCamelCase,
            Self::GreaterThanIncludedLowerBound { .. } => &naming::GreaterThanIncludedLowerBoundUpperCamelCase,
            Self::DimensionOneGreaterThanIncludedLowerBound { .. } => &naming::DimensionOneGreaterThanIncludedLowerBoundUpperCamelCase,
            Self::GreaterThanExcludedUpperBound { .. } => &naming::GreaterThanExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneGreaterThanExcludedUpperBound { .. } => &naming::DimensionOneGreaterThanExcludedUpperBoundUpperCamelCase,
            Self::OverlapWithRange { .. } => &naming::OverlapWithRangeUpperCamelCase,
            Self::DimensionOneOverlapWithRange { .. } => &naming::DimensionOneOverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange { .. } => &naming::AdjacentWithRangeUpperCamelCase,
            Self::DimensionOneAdjacentWithRange { .. } => &naming::DimensionOneAdjacentWithRangeUpperCamelCase,
            Self::RangeLength => &naming::RangeLengthUpperCamelCase,
            Self::DimensionOneRangeLength => &naming::DimensionOneRangeLengthUpperCamelCase,
        }
    }
    fn prefix_where_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlTypeWhereSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote! {#value}
    }
    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream> {
        match &self {
            Self::Equal { ident }
            | Self::DimensionOneEqual { ident }
            | Self::GreaterThan { ident }
            | Self::DimensionOneGreaterThan { ident }
            | Self::Between { ident }
            | Self::DimensionOneBetween { ident }
            | Self::In { ident }
            | Self::DimensionOneIn { ident }
            | Self::Before { ident }
            | Self::DimensionOneBefore { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::DimensionOneFindRangesWithinGivenRange { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::DimensionOneFindRangesThatFullyContainTheGivenRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::DimensionOneStrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident }
            | Self::DimensionOneStrictlyToRightOfRange { ident }
            | Self::IncludedLowerBound { ident }
            | Self::DimensionOneIncludedLowerBound { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::DimensionOneExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::DimensionOneGreaterThanIncludedLowerBound { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::DimensionOneGreaterThanExcludedUpperBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::DimensionOneOverlapWithRange { ident }
            | Self::AdjacentWithRange { ident }
            | Self::DimensionOneAdjacentWithRange { ident } => Some(ident.clone()),
            Self::RegularExpression
            | Self::DimensionOneRegularExpression
            | Self::CurrentDate
            | Self::DimensionOneCurrentDate
            | Self::GreaterThanCurrentDate
            | Self::DimensionOneGreaterThanCurrentDate
            | Self::CurrentTimestamp
            | Self::DimensionOneCurrentTimestamp
            | Self::GreaterThanCurrentTimestamp
            | Self::DimensionOneGreaterThanCurrentTimestamp
            | Self::CurrentTime
            | Self::DimensionOneCurrentTime
            | Self::GreaterThanCurrentTime
            | Self::DimensionOneGreaterThanCurrentTime
            | Self::DimensionOneLengthEqual
            | Self::DimensionOneLengthGreaterThan
            | Self::EqualToEncodedStringRepresentation
            | Self::DimensionOneEqualToEncodedStringRepresentation
            | Self::RangeLength
            | Self::DimensionOneRangeLength => None,
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypeFilter {
    Equal { ident: proc_macro2::TokenStream },
    DimensionOneEqual { ident: proc_macro2::TokenStream },
    DimensionTwoEqual { ident: proc_macro2::TokenStream },
    DimensionThreeEqual { ident: proc_macro2::TokenStream },
    DimensionFourEqual { ident: proc_macro2::TokenStream },
    AllElementsEqual { ident: proc_macro2::TokenStream },
    DimensionOneAllElementsEqual { ident: proc_macro2::TokenStream },
    DimensionTwoAllElementsEqual { ident: proc_macro2::TokenStream },
    DimensionThreeAllElementsEqual { ident: proc_macro2::TokenStream },
    DimensionFourAllElementsEqual { ident: proc_macro2::TokenStream },
    LengthEqual,
    DimensionOneLengthEqual,
    DimensionTwoLengthEqual,
    DimensionThreeLengthEqual,
    DimensionFourLengthEqual,
    LengthGreaterThan,
    DimensionOneLengthGreaterThan,
    DimensionTwoLengthGreaterThan,
    DimensionThreeLengthGreaterThan,
    DimensionFourLengthGreaterThan,
    GreaterThan { ident: proc_macro2::TokenStream },
    DimensionOneGreaterThan { ident: proc_macro2::TokenStream },
    DimensionTwoGreaterThan { ident: proc_macro2::TokenStream },
    DimensionThreeGreaterThan { ident: proc_macro2::TokenStream },
    DimensionFourGreaterThan { ident: proc_macro2::TokenStream },
    ContainsElementGreaterThan { ident: proc_macro2::TokenStream },
    DimensionOneContainsElementGreaterThan { ident: proc_macro2::TokenStream },
    DimensionTwoContainsElementGreaterThan { ident: proc_macro2::TokenStream },
    DimensionThreeContainsElementGreaterThan { ident: proc_macro2::TokenStream },
    DimensionFourContainsElementGreaterThan { ident: proc_macro2::TokenStream },
    AllElementsGreaterThan { ident: proc_macro2::TokenStream },
    DimensionOneAllElementsGreaterThan { ident: proc_macro2::TokenStream },
    DimensionTwoAllElementsGreaterThan { ident: proc_macro2::TokenStream },
    DimensionThreeAllElementsGreaterThan { ident: proc_macro2::TokenStream },
    DimensionFourAllElementsGreaterThan { ident: proc_macro2::TokenStream },
    Between { ident: proc_macro2::TokenStream },
    DimensionOneBetween { ident: proc_macro2::TokenStream },
    DimensionTwoBetween { ident: proc_macro2::TokenStream },
    DimensionThreeBetween { ident: proc_macro2::TokenStream },
    DimensionFourBetween { ident: proc_macro2::TokenStream },
    In { ident: proc_macro2::TokenStream },
    DimensionOneIn { ident: proc_macro2::TokenStream },
    DimensionTwoIn { ident: proc_macro2::TokenStream },
    DimensionThreeIn { ident: proc_macro2::TokenStream },
    DimensionFourIn { ident: proc_macro2::TokenStream },
    RegularExpression,
    DimensionOneRegularExpression,
    DimensionTwoRegularExpression,
    DimensionThreeRegularExpression,
    DimensionFourRegularExpression,
    ContainsElementRegularExpression,
    DimensionOneContainsElementRegularExpression,
    DimensionTwoContainsElementRegularExpression,
    DimensionThreeContainsElementRegularExpression,
    DimensionFourContainsElementRegularExpression,
    AllElementsRegularExpression,
    DimensionOneAllElementsRegularExpression,
    DimensionTwoAllElementsRegularExpression,
    DimensionThreeAllElementsRegularExpression,
    DimensionFourAllElementsRegularExpression,
    ContainsAllElementsOfArray { ident: proc_macro2::TokenStream },
    DimensionOneContainsAllElementsOfArray { ident: proc_macro2::TokenStream },
    DimensionTwoContainsAllElementsOfArray { ident: proc_macro2::TokenStream },
    DimensionThreeContainsAllElementsOfArray { ident: proc_macro2::TokenStream },
    DimensionFourContainsAllElementsOfArray { ident: proc_macro2::TokenStream },
    // ContainedInArray,
    OverlapsWithArray { ident: proc_macro2::TokenStream },
    DimensionOneOverlapsWithArray { ident: proc_macro2::TokenStream },
    DimensionTwoOverlapsWithArray { ident: proc_macro2::TokenStream },
    DimensionThreeOverlapsWithArray { ident: proc_macro2::TokenStream },
    DimensionFourOverlapsWithArray { ident: proc_macro2::TokenStream },
}
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual { .. } => &naming::DimensionOneEqualUpperCamelCase,
            Self::DimensionTwoEqual { .. } => &naming::DimensionTwoEqualUpperCamelCase,
            Self::DimensionThreeEqual { .. } => &naming::DimensionThreeEqualUpperCamelCase,
            Self::DimensionFourEqual { .. } => &naming::DimensionFourEqualUpperCamelCase,
            Self::AllElementsEqual { .. } => &naming::AllElementsEqualUpperCamelCase,
            Self::DimensionOneAllElementsEqual { .. } => &naming::DimensionOneAllElementsEqualUpperCamelCase,
            Self::DimensionTwoAllElementsEqual { .. } => &naming::DimensionTwoAllElementsEqualUpperCamelCase,
            Self::DimensionThreeAllElementsEqual { .. } => &naming::DimensionThreeAllElementsEqualUpperCamelCase,
            Self::DimensionFourAllElementsEqual { .. } => &naming::DimensionFourAllElementsEqualUpperCamelCase,
            Self::LengthEqual => &naming::LengthEqualUpperCamelCase,
            Self::DimensionOneLengthEqual => &naming::DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionTwoLengthEqual => &naming::DimensionTwoLengthEqualUpperCamelCase,
            Self::DimensionThreeLengthEqual => &naming::DimensionThreeLengthEqualUpperCamelCase,
            Self::DimensionFourLengthEqual => &naming::DimensionFourLengthEqualUpperCamelCase,
            Self::GreaterThan { .. } => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { .. } => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::DimensionTwoGreaterThan { .. } => &naming::DimensionTwoGreaterThanUpperCamelCase,
            Self::DimensionThreeGreaterThan { .. } => &naming::DimensionThreeGreaterThanUpperCamelCase,
            Self::DimensionFourGreaterThan { .. } => &naming::DimensionFourGreaterThanUpperCamelCase,
            Self::ContainsElementGreaterThan { .. } => &naming::ContainsElementGreaterThanUpperCamelCase,
            Self::DimensionOneContainsElementGreaterThan { .. } => &naming::DimensionOneContainsElementGreaterThanUpperCamelCase,
            Self::DimensionTwoContainsElementGreaterThan { .. } => &naming::DimensionTwoContainsElementGreaterThanUpperCamelCase,
            Self::DimensionThreeContainsElementGreaterThan { .. } => &naming::DimensionThreeContainsElementGreaterThanUpperCamelCase,
            Self::DimensionFourContainsElementGreaterThan { .. } => &naming::DimensionFourContainsElementGreaterThanUpperCamelCase,
            Self::AllElementsGreaterThan { .. } => &naming::AllElementsGreaterThanUpperCamelCase,
            Self::DimensionOneAllElementsGreaterThan { .. } => &naming::DimensionOneAllElementsGreaterThanUpperCamelCase,
            Self::DimensionTwoAllElementsGreaterThan { .. } => &naming::DimensionTwoAllElementsGreaterThanUpperCamelCase,
            Self::DimensionThreeAllElementsGreaterThan { .. } => &naming::DimensionThreeAllElementsGreaterThanUpperCamelCase,
            Self::DimensionFourAllElementsGreaterThan { .. } => &naming::DimensionFourAllElementsGreaterThanUpperCamelCase,
            Self::Between { .. } => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween { .. } => &naming::DimensionOneBetweenUpperCamelCase,
            Self::DimensionTwoBetween { .. } => &naming::DimensionTwoBetweenUpperCamelCase,
            Self::DimensionThreeBetween { .. } => &naming::DimensionThreeBetweenUpperCamelCase,
            Self::DimensionFourBetween { .. } => &naming::DimensionFourBetweenUpperCamelCase,
            Self::In { .. } => &naming::InUpperCamelCase,
            Self::DimensionOneIn { .. } => &naming::DimensionOneInUpperCamelCase,
            Self::DimensionTwoIn { .. } => &naming::DimensionTwoInUpperCamelCase,
            Self::DimensionThreeIn { .. } => &naming::DimensionThreeInUpperCamelCase,
            Self::DimensionFourIn { .. } => &naming::DimensionFourInUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &naming::DimensionOneRegularExpressionUpperCamelCase,
            Self::DimensionTwoRegularExpression => &naming::DimensionTwoRegularExpressionUpperCamelCase,
            Self::DimensionThreeRegularExpression => &naming::DimensionThreeRegularExpressionUpperCamelCase,
            Self::DimensionFourRegularExpression => &naming::DimensionFourRegularExpressionUpperCamelCase,
            Self::ContainsElementRegularExpression => &naming::ContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionOneContainsElementRegularExpression => &naming::DimensionOneContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionTwoContainsElementRegularExpression => &naming::DimensionTwoContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionThreeContainsElementRegularExpression => &naming::DimensionThreeContainsElementRegularExpressionUpperCamelCase,
            Self::DimensionFourContainsElementRegularExpression => &naming::DimensionFourContainsElementRegularExpressionUpperCamelCase,
            Self::AllElementsRegularExpression => &naming::AllElementsRegularExpressionUpperCamelCase,
            Self::DimensionOneAllElementsRegularExpression => &naming::DimensionOneAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionTwoAllElementsRegularExpression => &naming::DimensionTwoAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionThreeAllElementsRegularExpression => &naming::DimensionThreeAllElementsRegularExpressionUpperCamelCase,
            Self::DimensionFourAllElementsRegularExpression => &naming::DimensionFourAllElementsRegularExpressionUpperCamelCase,
            Self::LengthGreaterThan => &naming::LengthGreaterThanUpperCamelCase,
            Self::DimensionOneLengthGreaterThan => &naming::DimensionOneLengthGreaterThanUpperCamelCase,
            Self::DimensionTwoLengthGreaterThan => &naming::DimensionTwoLengthGreaterThanUpperCamelCase,
            Self::DimensionThreeLengthGreaterThan => &naming::DimensionThreeLengthGreaterThanUpperCamelCase,
            Self::DimensionFourLengthGreaterThan => &naming::DimensionFourLengthGreaterThanUpperCamelCase,
            Self::ContainsAllElementsOfArray { .. } => &naming::ContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionOneContainsAllElementsOfArray { .. } => &naming::DimensionOneContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionTwoContainsAllElementsOfArray { .. } => &naming::DimensionTwoContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionThreeContainsAllElementsOfArray { .. } => &naming::DimensionThreeContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionFourContainsAllElementsOfArray { .. } => &naming::DimensionFourContainsAllElementsOfArrayUpperCamelCase,
            Self::OverlapsWithArray { .. } => &naming::OverlapsWithArrayUpperCamelCase,
            Self::DimensionOneOverlapsWithArray { .. } => &naming::DimensionOneOverlapsWithArrayUpperCamelCase,
            Self::DimensionTwoOverlapsWithArray { .. } => &naming::DimensionTwoOverlapsWithArrayUpperCamelCase,
            Self::DimensionThreeOverlapsWithArray { .. } => &naming::DimensionThreeOverlapsWithArrayUpperCamelCase,
            Self::DimensionFourOverlapsWithArray { .. } => &naming::DimensionFourOverlapsWithArrayUpperCamelCase,
        }
    }
    fn prefix_where_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlJsonTypeWhereSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote! {#value}
    }
    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream> {
        match &self {
            Self::Equal { ident }
            | Self::DimensionOneEqual { ident }
            | Self::DimensionTwoEqual { ident }
            | Self::DimensionThreeEqual { ident }
            | Self::DimensionFourEqual { ident }
            | Self::AllElementsEqual { ident }
            | Self::DimensionOneAllElementsEqual { ident }
            | Self::DimensionTwoAllElementsEqual { ident }
            | Self::DimensionThreeAllElementsEqual { ident }
            | Self::DimensionFourAllElementsEqual { ident }
            | Self::GreaterThan { ident }
            | Self::DimensionOneGreaterThan { ident }
            | Self::DimensionTwoGreaterThan { ident }
            | Self::DimensionThreeGreaterThan { ident }
            | Self::DimensionFourGreaterThan { ident }
            | Self::ContainsElementGreaterThan { ident }
            | Self::DimensionOneContainsElementGreaterThan { ident }
            | Self::DimensionTwoContainsElementGreaterThan { ident }
            | Self::DimensionThreeContainsElementGreaterThan { ident }
            | Self::DimensionFourContainsElementGreaterThan { ident }
            | Self::AllElementsGreaterThan { ident }
            | Self::DimensionOneAllElementsGreaterThan { ident }
            | Self::DimensionTwoAllElementsGreaterThan { ident }
            | Self::DimensionThreeAllElementsGreaterThan { ident }
            | Self::DimensionFourAllElementsGreaterThan { ident }
            | Self::Between { ident }
            | Self::DimensionOneBetween { ident }
            | Self::DimensionTwoBetween { ident }
            | Self::DimensionThreeBetween { ident }
            | Self::DimensionFourBetween { ident }
            | Self::In { ident }
            | Self::DimensionOneIn { ident }
            | Self::DimensionTwoIn { ident }
            | Self::DimensionThreeIn { ident }
            | Self::DimensionFourIn { ident }
            | Self::ContainsAllElementsOfArray { ident }
            | Self::DimensionOneContainsAllElementsOfArray { ident }
            | Self::DimensionTwoContainsAllElementsOfArray { ident }
            | Self::DimensionThreeContainsAllElementsOfArray { ident }
            | Self::DimensionFourContainsAllElementsOfArray { ident }
            | Self::OverlapsWithArray { ident }
            | Self::DimensionOneOverlapsWithArray { ident }
            | Self::DimensionTwoOverlapsWithArray { ident }
            | Self::DimensionThreeOverlapsWithArray { ident }
            | Self::DimensionFourOverlapsWithArray { ident } => Some(ident.clone()),
            Self::LengthEqual
            | Self::DimensionOneLengthEqual
            | Self::DimensionTwoLengthEqual
            | Self::DimensionThreeLengthEqual
            | Self::DimensionFourLengthEqual
            | Self::RegularExpression
            | Self::DimensionOneRegularExpression
            | Self::DimensionTwoRegularExpression
            | Self::DimensionThreeRegularExpression
            | Self::DimensionFourRegularExpression
            | Self::ContainsElementRegularExpression
            | Self::DimensionOneContainsElementRegularExpression
            | Self::DimensionTwoContainsElementRegularExpression
            | Self::DimensionThreeContainsElementRegularExpression
            | Self::DimensionFourContainsElementRegularExpression
            | Self::AllElementsRegularExpression
            | Self::DimensionOneAllElementsRegularExpression
            | Self::DimensionTwoAllElementsRegularExpression
            | Self::DimensionThreeAllElementsRegularExpression
            | Self::DimensionFourAllElementsRegularExpression
            | Self::LengthGreaterThan
            | Self::DimensionOneLengthGreaterThan
            | Self::DimensionTwoLengthGreaterThan
            | Self::DimensionThreeLengthGreaterThan
            | Self::DimensionFourLengthGreaterThan => None,
        }
    }
}

pub trait PostgresqlFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
    fn prefix_where_self_upper_camel_case(&self) -> proc_macro2::TokenStream;
    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream>;
}
