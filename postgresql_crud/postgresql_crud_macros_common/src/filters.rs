#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlTypeFilter {
    Equal{
        ident: proc_macro2::TokenStream,
    },
    DimensionOneEqual{
        ident: proc_macro2::TokenStream,
    },
    GreaterThan{
        ident: proc_macro2::TokenStream,
    },
    DimensionOneGreaterThan {
        ident: proc_macro2::TokenStream,
    },
    Between {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneBetween {
        ident: proc_macro2::TokenStream,
    },
    In {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneIn {
        ident: proc_macro2::TokenStream,
    },
    RegularExpression,
    DimensionOneRegularExpression,
    Before {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneBefore {
        ident: proc_macro2::TokenStream,
    },
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
    DimensionOneLengthMoreThan,
    EqualToEncodedStringRepresentation,
    DimensionOneEqualToEncodedStringRepresentation,
    FindRangesWithinGivenRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneFindRangesWithinGivenRange {
        ident: proc_macro2::TokenStream,
    },
    FindRangesThatFullyContainTheGivenRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneFindRangesThatFullyContainTheGivenRange {
        ident: proc_macro2::TokenStream,
    },
    StrictlyToLeftOfRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneStrictlyToLeftOfRange {
        ident: proc_macro2::TokenStream,
    },
    StrictlyToRightOfRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneStrictlyToRightOfRange {
        ident: proc_macro2::TokenStream,
    },
    IncludedLowerBound {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneIncludedLowerBound {
        ident: proc_macro2::TokenStream,
    },
    ExcludedUpperBound {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneExcludedUpperBound {
        ident: proc_macro2::TokenStream,
    },
    GreaterThanIncludedLowerBound {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneGreaterThanIncludedLowerBound {
        ident: proc_macro2::TokenStream,
    },
    GreaterThanExcludedUpperBound {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneGreaterThanExcludedUpperBound {
        ident: proc_macro2::TokenStream,
    },
    OverlapWithRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneOverlapWithRange {
        ident: proc_macro2::TokenStream,
    },
    AdjacentWithRange {
        ident: proc_macro2::TokenStream,
    },
    DimensionOneAdjacentWithRange {
        ident: proc_macro2::TokenStream,
    },
    RangeLength,
    DimensionOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PostgresqlFilter for PostgresqlTypeFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { ident: _ } => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual { ident: _ } => &naming::DimensionOneEqualUpperCamelCase,
            Self::GreaterThan { ident: _ } => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { ident: _ } => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::Between { ident: _ } => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween { ident: _ } => &naming::DimensionOneBetweenUpperCamelCase,
            Self::In { ident: _ } => &naming::InUpperCamelCase,
            Self::DimensionOneIn { ident: _ } => &naming::DimensionOneInUpperCamelCase,
            Self::RegularExpression => &naming::RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &naming::DimensionOneRegularExpressionUpperCamelCase,
            Self::Before { ident: _ } => &naming::BeforeUpperCamelCase,
            Self::DimensionOneBefore { ident: _ } => &naming::DimensionOneBeforeUpperCamelCase,
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
            Self::DimensionOneLengthMoreThan => &naming::DimensionOneLengthMoreThanUpperCamelCase,
            Self::EqualToEncodedStringRepresentation => &naming::EqualToEncodedStringRepresentationUpperCamelCase,
            Self::DimensionOneEqualToEncodedStringRepresentation => &naming::DimensionOneEqualToEncodedStringRepresentationUpperCamelCase,
            Self::FindRangesWithinGivenRange { ident: _ } => &naming::FindRangesWithinGivenRangeUpperCamelCase,
            Self::DimensionOneFindRangesWithinGivenRange { ident: _ } => &naming::DimensionOneFindRangesWithinGivenRangeUpperCamelCase,
            Self::FindRangesThatFullyContainTheGivenRange { ident: _ } => &naming::FindRangesThatFullyContainTheGivenRangeUpperCamelCase,
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange { ident: _ } => &naming::DimensionOneFindRangesThatFullyContainTheGivenRangeUpperCamelCase,
            Self::StrictlyToLeftOfRange { ident: _ } => &naming::StrictlyToLeftOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToLeftOfRange { ident: _ } => &naming::DimensionOneStrictlyToLeftOfRangeUpperCamelCase,
            Self::StrictlyToRightOfRange { ident: _ } => &naming::StrictlyToRightOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToRightOfRange { ident: _ } => &naming::DimensionOneStrictlyToRightOfRangeUpperCamelCase,
            Self::IncludedLowerBound { ident: _ } => &naming::IncludedLowerBoundUpperCamelCase,
            Self::DimensionOneIncludedLowerBound { ident: _ } => &naming::DimensionOneIncludedLowerBoundUpperCamelCase,
            Self::ExcludedUpperBound { ident: _ } => &naming::ExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneExcludedUpperBound { ident: _ } => &naming::DimensionOneExcludedUpperBoundUpperCamelCase,
            Self::GreaterThanIncludedLowerBound { ident: _ } => &naming::GreaterThanIncludedLowerBoundUpperCamelCase,
            Self::DimensionOneGreaterThanIncludedLowerBound { ident: _ } => &naming::DimensionOneGreaterThanIncludedLowerBoundUpperCamelCase,
            Self::GreaterThanExcludedUpperBound { ident: _ } => &naming::GreaterThanExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneGreaterThanExcludedUpperBound { ident: _ } => &naming::DimensionOneGreaterThanExcludedUpperBoundUpperCamelCase,
            Self::OverlapWithRange { ident: _ } => &naming::OverlapWithRangeUpperCamelCase,
            Self::DimensionOneOverlapWithRange { ident: _ } => &naming::DimensionOneOverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange { ident: _ } => &naming::AdjacentWithRangeUpperCamelCase,
            Self::DimensionOneAdjacentWithRange { ident: _ } => &naming::DimensionOneAdjacentWithRangeUpperCamelCase,
            Self::RangeLength => &naming::RangeLengthUpperCamelCase,
            Self::DimensionOneRangeLength => &naming::DimensionOneRangeLengthUpperCamelCase,
        }
    }
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote! {#value}
    }
    fn maybe_generic(&self) -> std::option::Option<proc_macro2::TokenStream> {
        match &self {
            Self::Equal { ident } => Some(ident.clone()),
            Self::DimensionOneEqual { ident } => Some(ident.clone()),
            Self::GreaterThan { ident } => Some(ident.clone()),
            Self::DimensionOneGreaterThan { ident } => Some(ident.clone()),
            Self::Between { ident } => Some(ident.clone()),
            Self::DimensionOneBetween { ident } => Some(ident.clone()),
            Self::In { ident } => Some(ident.clone()),
            Self::DimensionOneIn { ident } => Some(ident.clone()),
            Self::RegularExpression => None,
            Self::DimensionOneRegularExpression => None,
            Self::Before { ident } => Some(ident.clone()),
            Self::DimensionOneBefore { ident } => Some(ident.clone()),
            Self::CurrentDate => None,
            Self::DimensionOneCurrentDate => None,
            Self::GreaterThanCurrentDate => None,
            Self::DimensionOneGreaterThanCurrentDate => None,
            Self::CurrentTimestamp => None,
            Self::DimensionOneCurrentTimestamp => None,
            Self::GreaterThanCurrentTimestamp => None,
            Self::DimensionOneGreaterThanCurrentTimestamp => None,
            Self::CurrentTime => None,
            Self::DimensionOneCurrentTime => None,
            Self::GreaterThanCurrentTime => None,
            Self::DimensionOneGreaterThanCurrentTime => None,
            Self::DimensionOneLengthEqual => None,
            Self::DimensionOneLengthMoreThan => None,
            Self::EqualToEncodedStringRepresentation => None,
            Self::DimensionOneEqualToEncodedStringRepresentation => None,
            Self::FindRangesWithinGivenRange { ident } => Some(ident.clone()),
            Self::DimensionOneFindRangesWithinGivenRange { ident } => Some(ident.clone()),
            Self::FindRangesThatFullyContainTheGivenRange { ident } => Some(ident.clone()),
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange { ident } => Some(ident.clone()),
            Self::StrictlyToLeftOfRange { ident } => Some(ident.clone()),
            Self::DimensionOneStrictlyToLeftOfRange { ident } => Some(ident.clone()),
            Self::StrictlyToRightOfRange { ident } => Some(ident.clone()),
            Self::DimensionOneStrictlyToRightOfRange { ident } => Some(ident.clone()),
            Self::IncludedLowerBound { ident } => Some(ident.clone()),
            Self::DimensionOneIncludedLowerBound { ident } => Some(ident.clone()),
            Self::ExcludedUpperBound { ident } => Some(ident.clone()),
            Self::DimensionOneExcludedUpperBound { ident } => Some(ident.clone()),
            Self::GreaterThanIncludedLowerBound { ident } => Some(ident.clone()),
            Self::DimensionOneGreaterThanIncludedLowerBound { ident } => Some(ident.clone()),
            Self::GreaterThanExcludedUpperBound { ident } => Some(ident.clone()),
            Self::DimensionOneGreaterThanExcludedUpperBound { ident } => Some(ident.clone()),
            Self::OverlapWithRange { ident } => Some(ident.clone()),
            Self::DimensionOneOverlapWithRange { ident } => Some(ident.clone()),
            Self::AdjacentWithRange { ident } => Some(ident.clone()),
            Self::DimensionOneAdjacentWithRange { ident } => Some(ident.clone()),
            Self::RangeLength => None,
            Self::DimensionOneRangeLength => None,
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
    AllElementsEqual {
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
    LengthEqual,
    DimensionOneLengthEqual,
    DimensionTwoLengthEqual,
    DimensionThreeLengthEqual,
    DimensionFourLengthEqual,
    LengthMoreThan,
    DimensionOneLengthMoreThan,
    DimensionTwoLengthMoreThan,
    DimensionThreeLengthMoreThan,
    DimensionFourLengthMoreThan,
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
    ContainsElementGreaterThan {
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
    AllElementsGreaterThan {
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
    ContainsAllElementsOfArray {
        ident: proc_macro2::TokenStream,
    },
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
    OverlapsWithArray {
        ident: proc_macro2::TokenStream,
    },
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
            Self::Equal { ident: _ } => &naming::EqualUpperCamelCase,
            Self::DimensionOneEqual { ident: _ } => &naming::DimensionOneEqualUpperCamelCase,
            Self::DimensionTwoEqual { ident: _ } => &naming::DimensionTwoEqualUpperCamelCase,
            Self::DimensionThreeEqual { ident: _ } => &naming::DimensionThreeEqualUpperCamelCase,
            Self::DimensionFourEqual { ident: _ } => &naming::DimensionFourEqualUpperCamelCase,
            Self::AllElementsEqual { ident: _ } => &naming::AllElementsEqualUpperCamelCase,
            Self::DimensionOneAllElementsEqual { ident: _ } => &naming::DimensionOneAllElementsEqualUpperCamelCase,
            Self::DimensionTwoAllElementsEqual { ident: _ } => &naming::DimensionTwoAllElementsEqualUpperCamelCase,
            Self::DimensionThreeAllElementsEqual { ident: _ } => &naming::DimensionThreeAllElementsEqualUpperCamelCase,
            Self::DimensionFourAllElementsEqual { ident: _ } => &naming::DimensionFourAllElementsEqualUpperCamelCase,
            Self::LengthEqual => &naming::LengthEqualUpperCamelCase,
            Self::DimensionOneLengthEqual => &naming::DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionTwoLengthEqual => &naming::DimensionTwoLengthEqualUpperCamelCase,
            Self::DimensionThreeLengthEqual => &naming::DimensionThreeLengthEqualUpperCamelCase,
            Self::DimensionFourLengthEqual => &naming::DimensionFourLengthEqualUpperCamelCase,
            Self::GreaterThan { ident: _ } => &naming::GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { ident: _ } => &naming::DimensionOneGreaterThanUpperCamelCase,
            Self::DimensionTwoGreaterThan { ident: _ } => &naming::DimensionTwoGreaterThanUpperCamelCase,
            Self::DimensionThreeGreaterThan { ident: _ } => &naming::DimensionThreeGreaterThanUpperCamelCase,
            Self::DimensionFourGreaterThan { ident: _ } => &naming::DimensionFourGreaterThanUpperCamelCase,
            Self::ContainsElementGreaterThan { ident: _ } => &naming::ContainsElementGreaterThanUpperCamelCase,
            Self::DimensionOneContainsElementGreaterThan { ident: _ } => &naming::DimensionOneContainsElementGreaterThanUpperCamelCase,
            Self::DimensionTwoContainsElementGreaterThan { ident: _ } => &naming::DimensionTwoContainsElementGreaterThanUpperCamelCase,
            Self::DimensionThreeContainsElementGreaterThan { ident: _ } => &naming::DimensionThreeContainsElementGreaterThanUpperCamelCase,
            Self::DimensionFourContainsElementGreaterThan { ident: _ } => &naming::DimensionFourContainsElementGreaterThanUpperCamelCase,
            Self::AllElementsGreaterThan { ident: _ } => &naming::AllElementsGreaterThanUpperCamelCase,
            Self::DimensionOneAllElementsGreaterThan { ident: _ } => &naming::DimensionOneAllElementsGreaterThanUpperCamelCase,
            Self::DimensionTwoAllElementsGreaterThan { ident: _ } => &naming::DimensionTwoAllElementsGreaterThanUpperCamelCase,
            Self::DimensionThreeAllElementsGreaterThan { ident: _ } => &naming::DimensionThreeAllElementsGreaterThanUpperCamelCase,
            Self::DimensionFourAllElementsGreaterThan { ident: _ } => &naming::DimensionFourAllElementsGreaterThanUpperCamelCase,
            Self::Between { ident: _ } => &naming::BetweenUpperCamelCase,
            Self::DimensionOneBetween { ident: _ } => &naming::DimensionOneBetweenUpperCamelCase,
            Self::DimensionTwoBetween { ident: _ } => &naming::DimensionTwoBetweenUpperCamelCase,
            Self::DimensionThreeBetween { ident: _ } => &naming::DimensionThreeBetweenUpperCamelCase,
            Self::DimensionFourBetween { ident: _ } => &naming::DimensionFourBetweenUpperCamelCase,
            Self::In { ident: _ } => &naming::InUpperCamelCase,
            Self::DimensionOneIn { ident: _ } => &naming::DimensionOneInUpperCamelCase,
            Self::DimensionTwoIn { ident: _ } => &naming::DimensionTwoInUpperCamelCase,
            Self::DimensionThreeIn { ident: _ } => &naming::DimensionThreeInUpperCamelCase,
            Self::DimensionFourIn { ident: _ } => &naming::DimensionFourInUpperCamelCase,
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
            Self::LengthMoreThan => &naming::LengthMoreThanUpperCamelCase,
            Self::DimensionOneLengthMoreThan => &naming::DimensionOneLengthMoreThanUpperCamelCase,
            Self::DimensionTwoLengthMoreThan => &naming::DimensionTwoLengthMoreThanUpperCamelCase,
            Self::DimensionThreeLengthMoreThan => &naming::DimensionThreeLengthMoreThanUpperCamelCase,
            Self::DimensionFourLengthMoreThan => &naming::DimensionFourLengthMoreThanUpperCamelCase,
            Self::ContainsAllElementsOfArray { ident: _ } => &naming::ContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionOneContainsAllElementsOfArray { ident: _ } => &naming::DimensionOneContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionTwoContainsAllElementsOfArray { ident: _ } => &naming::DimensionTwoContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionThreeContainsAllElementsOfArray { ident: _ } => &naming::DimensionThreeContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionFourContainsAllElementsOfArray { ident: _ } => &naming::DimensionFourContainsAllElementsOfArrayUpperCamelCase,
            Self::OverlapsWithArray { ident: _ } => &naming::OverlapsWithArrayUpperCamelCase,
            Self::DimensionOneOverlapsWithArray { ident: _ } => &naming::DimensionOneOverlapsWithArrayUpperCamelCase,
            Self::DimensionTwoOverlapsWithArray { ident: _ } => &naming::DimensionTwoOverlapsWithArrayUpperCamelCase,
            Self::DimensionThreeOverlapsWithArray { ident: _ } => &naming::DimensionThreeOverlapsWithArrayUpperCamelCase,
            Self::DimensionFourOverlapsWithArray { ident: _ } => &naming::DimensionFourOverlapsWithArrayUpperCamelCase,
        }
    }
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream {
        let value = naming::parameter::PostgresqlJsonTypeWhereElementSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote::quote! {#value}
    }
    fn maybe_generic(&self) -> std::option::Option<proc_macro2::TokenStream> {
        match &self {
            Self::Equal { ident } => Some(ident.clone()),
            Self::DimensionOneEqual { ident }=> Some(ident.clone()),
            Self::DimensionTwoEqual { ident }=> Some(ident.clone()),
            Self::DimensionThreeEqual { ident }=> Some(ident.clone()),
            Self::DimensionFourEqual { ident }=> Some(ident.clone()),
            Self::AllElementsEqual { ident }=> Some(ident.clone()),
            Self::DimensionOneAllElementsEqual { ident }=> Some(ident.clone()),
            Self::DimensionTwoAllElementsEqual { ident }=> Some(ident.clone()),
            Self::DimensionThreeAllElementsEqual { ident }=> Some(ident.clone()),
            Self::DimensionFourAllElementsEqual { ident }=> Some(ident.clone()),
            Self::LengthEqual => None,
            Self::DimensionOneLengthEqual => None,
            Self::DimensionTwoLengthEqual => None,
            Self::DimensionThreeLengthEqual => None,
            Self::DimensionFourLengthEqual => None,
            Self::GreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionOneGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionTwoGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionThreeGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionFourGreaterThan { ident }=> Some(ident.clone()),
            Self::ContainsElementGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionOneContainsElementGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionTwoContainsElementGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionThreeContainsElementGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionFourContainsElementGreaterThan { ident }=> Some(ident.clone()),
            Self::AllElementsGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionOneAllElementsGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionTwoAllElementsGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionThreeAllElementsGreaterThan { ident }=> Some(ident.clone()),
            Self::DimensionFourAllElementsGreaterThan { ident }=> Some(ident.clone()),
            Self::Between { ident }=> Some(ident.clone()),
            Self::DimensionOneBetween { ident }=> Some(ident.clone()),
            Self::DimensionTwoBetween { ident }=> Some(ident.clone()),
            Self::DimensionThreeBetween { ident }=> Some(ident.clone()),
            Self::DimensionFourBetween { ident }=> Some(ident.clone()),
            Self::In { ident }=> Some(ident.clone()),
            Self::DimensionOneIn { ident }=> Some(ident.clone()),
            Self::DimensionTwoIn { ident }=> Some(ident.clone()),
            Self::DimensionThreeIn { ident }=> Some(ident.clone()),
            Self::DimensionFourIn { ident }=> Some(ident.clone()),
            Self::RegularExpression => None,
            Self::DimensionOneRegularExpression => None,
            Self::DimensionTwoRegularExpression => None,
            Self::DimensionThreeRegularExpression => None,
            Self::DimensionFourRegularExpression => None,
            Self::ContainsElementRegularExpression => None,
            Self::DimensionOneContainsElementRegularExpression => None,
            Self::DimensionTwoContainsElementRegularExpression => None,
            Self::DimensionThreeContainsElementRegularExpression => None,
            Self::DimensionFourContainsElementRegularExpression => None,
            Self::AllElementsRegularExpression => None,
            Self::DimensionOneAllElementsRegularExpression => None,
            Self::DimensionTwoAllElementsRegularExpression => None,
            Self::DimensionThreeAllElementsRegularExpression => None,
            Self::DimensionFourAllElementsRegularExpression => None,
            Self::LengthMoreThan => None,
            Self::DimensionOneLengthMoreThan => None,
            Self::DimensionTwoLengthMoreThan => None,
            Self::DimensionThreeLengthMoreThan => None,
            Self::DimensionFourLengthMoreThan => None,
            Self::ContainsAllElementsOfArray { ident }=> Some(ident.clone()),
            Self::DimensionOneContainsAllElementsOfArray { ident }=> Some(ident.clone()),
            Self::DimensionTwoContainsAllElementsOfArray { ident }=> Some(ident.clone()),
            Self::DimensionThreeContainsAllElementsOfArray { ident }=> Some(ident.clone()),
            Self::DimensionFourContainsAllElementsOfArray { ident }=> Some(ident.clone()),
            Self::OverlapsWithArray { ident } => Some(ident.clone()),
            Self::DimensionOneOverlapsWithArray { ident }=> Some(ident.clone()),
            Self::DimensionTwoOverlapsWithArray { ident }=> Some(ident.clone()),
            Self::DimensionThreeOverlapsWithArray { ident }=> Some(ident.clone()),
            Self::DimensionFourOverlapsWithArray { ident }=> Some(ident.clone()),
        }
    }
}

pub trait PostgresqlFilter {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
    fn prefix_where_element_self_upper_camel_case(&self) -> proc_macro2::TokenStream;
    fn maybe_generic(&self) -> std::option::Option<proc_macro2::TokenStream>;
}