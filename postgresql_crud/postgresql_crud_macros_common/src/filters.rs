use naming::{
    AdjacentWithRangeUpperCamelCase, AllElementsEqualUpperCamelCase,
    AllElementsGreaterThanUpperCamelCase, AllElementsRegularExpressionUpperCamelCase,
    BeforeUpperCamelCase, BetweenUpperCamelCase, ContainsAllElementsOfArrayUpperCamelCase,
    ContainsElGreaterThanUpperCamelCase, ContainsElRegularExpressionUpperCamelCase,
    CurrentDateUpperCamelCase, CurrentTimeUpperCamelCase, CurrentTimestampUpperCamelCase,
    DimensionFourAllElementsEqualUpperCamelCase, DimensionFourAllElementsGreaterThanUpperCamelCase,
    DimensionFourAllElementsRegularExpressionUpperCamelCase, DimensionFourBetweenUpperCamelCase,
    DimensionFourContainsAllElementsOfArrayUpperCamelCase,
    DimensionFourContainsElGreaterThanUpperCamelCase,
    DimensionFourContainsElRegularExpressionUpperCamelCase, DimensionFourEqualUpperCamelCase,
    DimensionFourGreaterThanUpperCamelCase, DimensionFourInUpperCamelCase,
    DimensionFourLengthEqualUpperCamelCase, DimensionFourLengthGreaterThanUpperCamelCase,
    DimensionFourOverlapsWithArrayUpperCamelCase, DimensionFourRegularExpressionUpperCamelCase,
    DimensionOneAdjacentWithRangeUpperCamelCase, DimensionOneAllElementsEqualUpperCamelCase,
    DimensionOneAllElementsGreaterThanUpperCamelCase,
    DimensionOneAllElementsRegularExpressionUpperCamelCase, DimensionOneBeforeUpperCamelCase,
    DimensionOneBetweenUpperCamelCase, DimensionOneContainsAllElementsOfArrayUpperCamelCase,
    DimensionOneContainsElGreaterThanUpperCamelCase,
    DimensionOneContainsElRegularExpressionUpperCamelCase, DimensionOneCurrentDateUpperCamelCase,
    DimensionOneCurrentTimeUpperCamelCase, DimensionOneCurrentTimestampUpperCamelCase,
    DimensionOneEqualToEncodedStringRepresentationUpperCamelCase, DimensionOneEqualUpperCamelCase,
    DimensionOneExcludedUpperBoundUpperCamelCase,
    DimensionOneFindRangesThatFullyContainTheGivenRangeUpperCamelCase,
    DimensionOneFindRangesWithinGivenRangeUpperCamelCase,
    DimensionOneGreaterThanCurrentDateUpperCamelCase,
    DimensionOneGreaterThanCurrentTimeUpperCamelCase,
    DimensionOneGreaterThanCurrentTimestampUpperCamelCase,
    DimensionOneGreaterThanExcludedUpperBoundUpperCamelCase,
    DimensionOneGreaterThanIncludedLowerBoundUpperCamelCase, DimensionOneGreaterThanUpperCamelCase,
    DimensionOneInUpperCamelCase, DimensionOneIncludedLowerBoundUpperCamelCase,
    DimensionOneLengthEqualUpperCamelCase, DimensionOneLengthGreaterThanUpperCamelCase,
    DimensionOneOverlapWithRangeUpperCamelCase, DimensionOneOverlapsWithArrayUpperCamelCase,
    DimensionOneRangeLengthUpperCamelCase, DimensionOneRegularExpressionUpperCamelCase,
    DimensionOneStrictlyToLeftOfRangeUpperCamelCase,
    DimensionOneStrictlyToRightOfRangeUpperCamelCase, DimensionThreeAllElementsEqualUpperCamelCase,
    DimensionThreeAllElementsGreaterThanUpperCamelCase,
    DimensionThreeAllElementsRegularExpressionUpperCamelCase, DimensionThreeBetweenUpperCamelCase,
    DimensionThreeContainsAllElementsOfArrayUpperCamelCase,
    DimensionThreeContainsElGreaterThanUpperCamelCase,
    DimensionThreeContainsElRegularExpressionUpperCamelCase, DimensionThreeEqualUpperCamelCase,
    DimensionThreeGreaterThanUpperCamelCase, DimensionThreeInUpperCamelCase,
    DimensionThreeLengthEqualUpperCamelCase, DimensionThreeLengthGreaterThanUpperCamelCase,
    DimensionThreeOverlapsWithArrayUpperCamelCase, DimensionThreeRegularExpressionUpperCamelCase,
    DimensionTwoAllElementsEqualUpperCamelCase, DimensionTwoAllElementsGreaterThanUpperCamelCase,
    DimensionTwoAllElementsRegularExpressionUpperCamelCase, DimensionTwoBetweenUpperCamelCase,
    DimensionTwoContainsAllElementsOfArrayUpperCamelCase,
    DimensionTwoContainsElGreaterThanUpperCamelCase,
    DimensionTwoContainsElRegularExpressionUpperCamelCase, DimensionTwoEqualUpperCamelCase,
    DimensionTwoGreaterThanUpperCamelCase, DimensionTwoInUpperCamelCase,
    DimensionTwoLengthEqualUpperCamelCase, DimensionTwoLengthGreaterThanUpperCamelCase,
    DimensionTwoOverlapsWithArrayUpperCamelCase, DimensionTwoRegularExpressionUpperCamelCase,
    EqualToEncodedStringRepresentationUpperCamelCase, EqualUpperCamelCase,
    ExcludedUpperBoundUpperCamelCase, FindRangesThatFullyContainTheGivenRangeUpperCamelCase,
    FindRangesWithinGivenRangeUpperCamelCase, GreaterThanCurrentDateUpperCamelCase,
    GreaterThanCurrentTimeUpperCamelCase, GreaterThanCurrentTimestampUpperCamelCase,
    GreaterThanExcludedUpperBoundUpperCamelCase, GreaterThanIncludedLowerBoundUpperCamelCase,
    GreaterThanUpperCamelCase, InUpperCamelCase, IncludedLowerBoundUpperCamelCase,
    LengthEqualUpperCamelCase, LengthGreaterThanUpperCamelCase, OverlapWithRangeUpperCamelCase,
    OverlapsWithArrayUpperCamelCase, RangeLengthUpperCamelCase, RegularExpressionUpperCamelCase,
    StdFmtDisplayPlusQuoteToTokens, StrictlyToLeftOfRangeUpperCamelCase,
    StrictlyToRightOfRangeUpperCamelCase,
    parameter::{PostgresqlJsonTypeWhereSelfUpperCamelCase, PostgresqlTypeWhereSelfUpperCamelCase},
};
use proc_macro2::TokenStream;
use quote::quote;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension,
)]
pub enum PostgresqlTypeFilter {
    Equal { ident: TokenStream },
    DimensionOneEqual { ident: TokenStream },
    GreaterThan { ident: TokenStream },
    DimensionOneGreaterThan { ident: TokenStream },
    Between { ident: TokenStream },
    DimensionOneBetween { ident: TokenStream },
    In { ident: TokenStream },
    DimensionOneIn { ident: TokenStream },
    RegularExpression,
    DimensionOneRegularExpression,
    Before { ident: TokenStream },
    DimensionOneBefore { ident: TokenStream },
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
    FindRangesWithinGivenRange { ident: TokenStream },
    DimensionOneFindRangesWithinGivenRange { ident: TokenStream },
    FindRangesThatFullyContainTheGivenRange { ident: TokenStream },
    DimensionOneFindRangesThatFullyContainTheGivenRange { ident: TokenStream },
    StrictlyToLeftOfRange { ident: TokenStream },
    DimensionOneStrictlyToLeftOfRange { ident: TokenStream },
    StrictlyToRightOfRange { ident: TokenStream },
    DimensionOneStrictlyToRightOfRange { ident: TokenStream },
    IncludedLowerBound { ident: TokenStream },
    DimensionOneIncludedLowerBound { ident: TokenStream },
    ExcludedUpperBound { ident: TokenStream },
    DimensionOneExcludedUpperBound { ident: TokenStream },
    GreaterThanIncludedLowerBound { ident: TokenStream },
    DimensionOneGreaterThanIncludedLowerBound { ident: TokenStream },
    GreaterThanExcludedUpperBound { ident: TokenStream },
    DimensionOneGreaterThanExcludedUpperBound { ident: TokenStream },
    OverlapWithRange { ident: TokenStream },
    DimensionOneOverlapWithRange { ident: TokenStream },
    AdjacentWithRange { ident: TokenStream },
    DimensionOneAdjacentWithRange { ident: TokenStream },
    RangeLength,
    DimensionOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PostgresqlFilter for PostgresqlTypeFilter {
    fn maybe_generic(&self) -> Option<TokenStream> {
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
    fn prefix_where_self_upper_camel_case(&self) -> TokenStream {
        let value = PostgresqlTypeWhereSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote! {#value}
    }
    fn upper_camel_case(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &EqualUpperCamelCase,
            Self::DimensionOneEqual { .. } => &DimensionOneEqualUpperCamelCase,
            Self::GreaterThan { .. } => &GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { .. } => &DimensionOneGreaterThanUpperCamelCase,
            Self::Between { .. } => &BetweenUpperCamelCase,
            Self::DimensionOneBetween { .. } => &DimensionOneBetweenUpperCamelCase,
            Self::In { .. } => &InUpperCamelCase,
            Self::DimensionOneIn { .. } => &DimensionOneInUpperCamelCase,
            Self::RegularExpression => &RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &DimensionOneRegularExpressionUpperCamelCase,
            Self::Before { .. } => &BeforeUpperCamelCase,
            Self::DimensionOneBefore { .. } => &DimensionOneBeforeUpperCamelCase,
            Self::CurrentDate => &CurrentDateUpperCamelCase,
            Self::DimensionOneCurrentDate => &DimensionOneCurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => &GreaterThanCurrentDateUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentDate => {
                &DimensionOneGreaterThanCurrentDateUpperCamelCase
            }
            Self::CurrentTimestamp => &CurrentTimestampUpperCamelCase,
            Self::DimensionOneCurrentTimestamp => &DimensionOneCurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => &GreaterThanCurrentTimestampUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentTimestamp => {
                &DimensionOneGreaterThanCurrentTimestampUpperCamelCase
            }
            Self::CurrentTime => &CurrentTimeUpperCamelCase,
            Self::DimensionOneCurrentTime => &DimensionOneCurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => &GreaterThanCurrentTimeUpperCamelCase,
            Self::DimensionOneGreaterThanCurrentTime => {
                &DimensionOneGreaterThanCurrentTimeUpperCamelCase
            }
            Self::DimensionOneLengthEqual => &DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionOneLengthGreaterThan => &DimensionOneLengthGreaterThanUpperCamelCase,
            Self::EqualToEncodedStringRepresentation => {
                &EqualToEncodedStringRepresentationUpperCamelCase
            }
            Self::DimensionOneEqualToEncodedStringRepresentation => {
                &DimensionOneEqualToEncodedStringRepresentationUpperCamelCase
            }
            Self::FindRangesWithinGivenRange { .. } => &FindRangesWithinGivenRangeUpperCamelCase,
            Self::DimensionOneFindRangesWithinGivenRange { .. } => {
                &DimensionOneFindRangesWithinGivenRangeUpperCamelCase
            }
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &FindRangesThatFullyContainTheGivenRangeUpperCamelCase
            }
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => {
                &DimensionOneFindRangesThatFullyContainTheGivenRangeUpperCamelCase
            }
            Self::StrictlyToLeftOfRange { .. } => &StrictlyToLeftOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToLeftOfRange { .. } => {
                &DimensionOneStrictlyToLeftOfRangeUpperCamelCase
            }
            Self::StrictlyToRightOfRange { .. } => &StrictlyToRightOfRangeUpperCamelCase,
            Self::DimensionOneStrictlyToRightOfRange { .. } => {
                &DimensionOneStrictlyToRightOfRangeUpperCamelCase
            }
            Self::IncludedLowerBound { .. } => &IncludedLowerBoundUpperCamelCase,
            Self::DimensionOneIncludedLowerBound { .. } => {
                &DimensionOneIncludedLowerBoundUpperCamelCase
            }
            Self::ExcludedUpperBound { .. } => &ExcludedUpperBoundUpperCamelCase,
            Self::DimensionOneExcludedUpperBound { .. } => {
                &DimensionOneExcludedUpperBoundUpperCamelCase
            }
            Self::GreaterThanIncludedLowerBound { .. } => {
                &GreaterThanIncludedLowerBoundUpperCamelCase
            }
            Self::DimensionOneGreaterThanIncludedLowerBound { .. } => {
                &DimensionOneGreaterThanIncludedLowerBoundUpperCamelCase
            }
            Self::GreaterThanExcludedUpperBound { .. } => {
                &GreaterThanExcludedUpperBoundUpperCamelCase
            }
            Self::DimensionOneGreaterThanExcludedUpperBound { .. } => {
                &DimensionOneGreaterThanExcludedUpperBoundUpperCamelCase
            }
            Self::OverlapWithRange { .. } => &OverlapWithRangeUpperCamelCase,
            Self::DimensionOneOverlapWithRange { .. } => {
                &DimensionOneOverlapWithRangeUpperCamelCase
            }
            Self::AdjacentWithRange { .. } => &AdjacentWithRangeUpperCamelCase,
            Self::DimensionOneAdjacentWithRange { .. } => {
                &DimensionOneAdjacentWithRangeUpperCamelCase
            }
            Self::RangeLength => &RangeLengthUpperCamelCase,
            Self::DimensionOneRangeLength => &DimensionOneRangeLengthUpperCamelCase,
        }
    }
}

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension,
)]
pub enum PostgresqlJsonTypeFilter {
    Equal { ident: TokenStream },
    DimensionOneEqual { ident: TokenStream },
    DimensionTwoEqual { ident: TokenStream },
    DimensionThreeEqual { ident: TokenStream },
    DimensionFourEqual { ident: TokenStream },
    AllElementsEqual { ident: TokenStream },
    DimensionOneAllElementsEqual { ident: TokenStream },
    DimensionTwoAllElementsEqual { ident: TokenStream },
    DimensionThreeAllElementsEqual { ident: TokenStream },
    DimensionFourAllElementsEqual { ident: TokenStream },
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
    GreaterThan { ident: TokenStream },
    DimensionOneGreaterThan { ident: TokenStream },
    DimensionTwoGreaterThan { ident: TokenStream },
    DimensionThreeGreaterThan { ident: TokenStream },
    DimensionFourGreaterThan { ident: TokenStream },
    ContainsElGreaterThan { ident: TokenStream },
    DimensionOneContainsElGreaterThan { ident: TokenStream },
    DimensionTwoContainsElGreaterThan { ident: TokenStream },
    DimensionThreeContainsElGreaterThan { ident: TokenStream },
    DimensionFourContainsElGreaterThan { ident: TokenStream },
    AllElementsGreaterThan { ident: TokenStream },
    DimensionOneAllElementsGreaterThan { ident: TokenStream },
    DimensionTwoAllElementsGreaterThan { ident: TokenStream },
    DimensionThreeAllElementsGreaterThan { ident: TokenStream },
    DimensionFourAllElementsGreaterThan { ident: TokenStream },
    Between { ident: TokenStream },
    DimensionOneBetween { ident: TokenStream },
    DimensionTwoBetween { ident: TokenStream },
    DimensionThreeBetween { ident: TokenStream },
    DimensionFourBetween { ident: TokenStream },
    In { ident: TokenStream },
    DimensionOneIn { ident: TokenStream },
    DimensionTwoIn { ident: TokenStream },
    DimensionThreeIn { ident: TokenStream },
    DimensionFourIn { ident: TokenStream },
    RegularExpression,
    DimensionOneRegularExpression,
    DimensionTwoRegularExpression,
    DimensionThreeRegularExpression,
    DimensionFourRegularExpression,
    ContainsElRegularExpression,
    DimensionOneContainsElRegularExpression,
    DimensionTwoContainsElRegularExpression,
    DimensionThreeContainsElRegularExpression,
    DimensionFourContainsElRegularExpression,
    AllElementsRegularExpression,
    DimensionOneAllElementsRegularExpression,
    DimensionTwoAllElementsRegularExpression,
    DimensionThreeAllElementsRegularExpression,
    DimensionFourAllElementsRegularExpression,
    ContainsAllElementsOfArray { ident: TokenStream },
    DimensionOneContainsAllElementsOfArray { ident: TokenStream },
    DimensionTwoContainsAllElementsOfArray { ident: TokenStream },
    DimensionThreeContainsAllElementsOfArray { ident: TokenStream },
    DimensionFourContainsAllElementsOfArray { ident: TokenStream },
    // ContainedInArray,
    OverlapsWithArray { ident: TokenStream },
    DimensionOneOverlapsWithArray { ident: TokenStream },
    DimensionTwoOverlapsWithArray { ident: TokenStream },
    DimensionThreeOverlapsWithArray { ident: TokenStream },
    DimensionFourOverlapsWithArray { ident: TokenStream },
}
impl PostgresqlFilter for PostgresqlJsonTypeFilter {
    fn maybe_generic(&self) -> Option<TokenStream> {
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
            | Self::ContainsElGreaterThan { ident }
            | Self::DimensionOneContainsElGreaterThan { ident }
            | Self::DimensionTwoContainsElGreaterThan { ident }
            | Self::DimensionThreeContainsElGreaterThan { ident }
            | Self::DimensionFourContainsElGreaterThan { ident }
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
            | Self::ContainsElRegularExpression
            | Self::DimensionOneContainsElRegularExpression
            | Self::DimensionTwoContainsElRegularExpression
            | Self::DimensionThreeContainsElRegularExpression
            | Self::DimensionFourContainsElRegularExpression
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
    fn prefix_where_self_upper_camel_case(&self) -> TokenStream {
        let value =
            PostgresqlJsonTypeWhereSelfUpperCamelCase::from_display(&self.upper_camel_case());
        quote! {#value}
    }
    fn upper_camel_case(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &EqualUpperCamelCase,
            Self::DimensionOneEqual { .. } => &DimensionOneEqualUpperCamelCase,
            Self::DimensionTwoEqual { .. } => &DimensionTwoEqualUpperCamelCase,
            Self::DimensionThreeEqual { .. } => &DimensionThreeEqualUpperCamelCase,
            Self::DimensionFourEqual { .. } => &DimensionFourEqualUpperCamelCase,
            Self::AllElementsEqual { .. } => &AllElementsEqualUpperCamelCase,
            Self::DimensionOneAllElementsEqual { .. } => {
                &DimensionOneAllElementsEqualUpperCamelCase
            }
            Self::DimensionTwoAllElementsEqual { .. } => {
                &DimensionTwoAllElementsEqualUpperCamelCase
            }
            Self::DimensionThreeAllElementsEqual { .. } => {
                &DimensionThreeAllElementsEqualUpperCamelCase
            }
            Self::DimensionFourAllElementsEqual { .. } => {
                &DimensionFourAllElementsEqualUpperCamelCase
            }
            Self::LengthEqual => &LengthEqualUpperCamelCase,
            Self::DimensionOneLengthEqual => &DimensionOneLengthEqualUpperCamelCase,
            Self::DimensionTwoLengthEqual => &DimensionTwoLengthEqualUpperCamelCase,
            Self::DimensionThreeLengthEqual => &DimensionThreeLengthEqualUpperCamelCase,
            Self::DimensionFourLengthEqual => &DimensionFourLengthEqualUpperCamelCase,
            Self::GreaterThan { .. } => &GreaterThanUpperCamelCase,
            Self::DimensionOneGreaterThan { .. } => &DimensionOneGreaterThanUpperCamelCase,
            Self::DimensionTwoGreaterThan { .. } => &DimensionTwoGreaterThanUpperCamelCase,
            Self::DimensionThreeGreaterThan { .. } => &DimensionThreeGreaterThanUpperCamelCase,
            Self::DimensionFourGreaterThan { .. } => &DimensionFourGreaterThanUpperCamelCase,
            Self::ContainsElGreaterThan { .. } => &ContainsElGreaterThanUpperCamelCase,
            Self::DimensionOneContainsElGreaterThan { .. } => {
                &DimensionOneContainsElGreaterThanUpperCamelCase
            }
            Self::DimensionTwoContainsElGreaterThan { .. } => {
                &DimensionTwoContainsElGreaterThanUpperCamelCase
            }
            Self::DimensionThreeContainsElGreaterThan { .. } => {
                &DimensionThreeContainsElGreaterThanUpperCamelCase
            }
            Self::DimensionFourContainsElGreaterThan { .. } => {
                &DimensionFourContainsElGreaterThanUpperCamelCase
            }
            Self::AllElementsGreaterThan { .. } => &AllElementsGreaterThanUpperCamelCase,
            Self::DimensionOneAllElementsGreaterThan { .. } => {
                &DimensionOneAllElementsGreaterThanUpperCamelCase
            }
            Self::DimensionTwoAllElementsGreaterThan { .. } => {
                &DimensionTwoAllElementsGreaterThanUpperCamelCase
            }
            Self::DimensionThreeAllElementsGreaterThan { .. } => {
                &DimensionThreeAllElementsGreaterThanUpperCamelCase
            }
            Self::DimensionFourAllElementsGreaterThan { .. } => {
                &DimensionFourAllElementsGreaterThanUpperCamelCase
            }
            Self::Between { .. } => &BetweenUpperCamelCase,
            Self::DimensionOneBetween { .. } => &DimensionOneBetweenUpperCamelCase,
            Self::DimensionTwoBetween { .. } => &DimensionTwoBetweenUpperCamelCase,
            Self::DimensionThreeBetween { .. } => &DimensionThreeBetweenUpperCamelCase,
            Self::DimensionFourBetween { .. } => &DimensionFourBetweenUpperCamelCase,
            Self::In { .. } => &InUpperCamelCase,
            Self::DimensionOneIn { .. } => &DimensionOneInUpperCamelCase,
            Self::DimensionTwoIn { .. } => &DimensionTwoInUpperCamelCase,
            Self::DimensionThreeIn { .. } => &DimensionThreeInUpperCamelCase,
            Self::DimensionFourIn { .. } => &DimensionFourInUpperCamelCase,
            Self::RegularExpression => &RegularExpressionUpperCamelCase,
            Self::DimensionOneRegularExpression => &DimensionOneRegularExpressionUpperCamelCase,
            Self::DimensionTwoRegularExpression => &DimensionTwoRegularExpressionUpperCamelCase,
            Self::DimensionThreeRegularExpression => &DimensionThreeRegularExpressionUpperCamelCase,
            Self::DimensionFourRegularExpression => &DimensionFourRegularExpressionUpperCamelCase,
            Self::ContainsElRegularExpression => &ContainsElRegularExpressionUpperCamelCase,
            Self::DimensionOneContainsElRegularExpression => {
                &DimensionOneContainsElRegularExpressionUpperCamelCase
            }
            Self::DimensionTwoContainsElRegularExpression => {
                &DimensionTwoContainsElRegularExpressionUpperCamelCase
            }
            Self::DimensionThreeContainsElRegularExpression => {
                &DimensionThreeContainsElRegularExpressionUpperCamelCase
            }
            Self::DimensionFourContainsElRegularExpression => {
                &DimensionFourContainsElRegularExpressionUpperCamelCase
            }
            Self::AllElementsRegularExpression => &AllElementsRegularExpressionUpperCamelCase,
            Self::DimensionOneAllElementsRegularExpression => {
                &DimensionOneAllElementsRegularExpressionUpperCamelCase
            }
            Self::DimensionTwoAllElementsRegularExpression => {
                &DimensionTwoAllElementsRegularExpressionUpperCamelCase
            }
            Self::DimensionThreeAllElementsRegularExpression => {
                &DimensionThreeAllElementsRegularExpressionUpperCamelCase
            }
            Self::DimensionFourAllElementsRegularExpression => {
                &DimensionFourAllElementsRegularExpressionUpperCamelCase
            }
            Self::LengthGreaterThan => &LengthGreaterThanUpperCamelCase,
            Self::DimensionOneLengthGreaterThan => &DimensionOneLengthGreaterThanUpperCamelCase,
            Self::DimensionTwoLengthGreaterThan => &DimensionTwoLengthGreaterThanUpperCamelCase,
            Self::DimensionThreeLengthGreaterThan => &DimensionThreeLengthGreaterThanUpperCamelCase,
            Self::DimensionFourLengthGreaterThan => &DimensionFourLengthGreaterThanUpperCamelCase,
            Self::ContainsAllElementsOfArray { .. } => &ContainsAllElementsOfArrayUpperCamelCase,
            Self::DimensionOneContainsAllElementsOfArray { .. } => {
                &DimensionOneContainsAllElementsOfArrayUpperCamelCase
            }
            Self::DimensionTwoContainsAllElementsOfArray { .. } => {
                &DimensionTwoContainsAllElementsOfArrayUpperCamelCase
            }
            Self::DimensionThreeContainsAllElementsOfArray { .. } => {
                &DimensionThreeContainsAllElementsOfArrayUpperCamelCase
            }
            Self::DimensionFourContainsAllElementsOfArray { .. } => {
                &DimensionFourContainsAllElementsOfArrayUpperCamelCase
            }
            Self::OverlapsWithArray { .. } => &OverlapsWithArrayUpperCamelCase,
            Self::DimensionOneOverlapsWithArray { .. } => {
                &DimensionOneOverlapsWithArrayUpperCamelCase
            }
            Self::DimensionTwoOverlapsWithArray { .. } => {
                &DimensionTwoOverlapsWithArrayUpperCamelCase
            }
            Self::DimensionThreeOverlapsWithArray { .. } => {
                &DimensionThreeOverlapsWithArrayUpperCamelCase
            }
            Self::DimensionFourOverlapsWithArray { .. } => {
                &DimensionFourOverlapsWithArrayUpperCamelCase
            }
        }
    }
}

pub trait PostgresqlFilter {
    fn maybe_generic(&self) -> Option<TokenStream>;
    fn prefix_where_self_upper_camel_case(&self) -> TokenStream;
    fn upper_camel_case(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens;
}
