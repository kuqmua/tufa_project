use naming::{
    AdjacentWithRangeUcc, AllElementsEqualUcc, AllElementsGreaterThanUcc,
    AllElementsRegularExpressionUcc, BeforeUcc, BetweenUcc, ContainsAllElementsOfArrayUcc,
    ContainsElGreaterThanUcc, ContainsElRegularExpressionUcc, CurrentDateUcc, CurrentTimeUcc,
    CurrentTimestampUcc, DimensionFourAllElementsEqualUcc, DimensionFourAllElementsGreaterThanUcc,
    DimensionFourAllElementsRegularExpressionUcc, DimensionFourBetweenUcc,
    DimensionFourContainsAllElementsOfArrayUcc, DimensionFourContainsElGreaterThanUcc,
    DimensionFourContainsElRegularExpressionUcc, DimensionFourEqualUcc,
    DimensionFourGreaterThanUcc, DimensionFourInUcc, DimensionFourLengthEqualUcc,
    DimensionFourLengthGreaterThanUcc, DimensionFourOverlapsWithArrayUcc,
    DimensionFourRegularExpressionUcc, DimensionOneAdjacentWithRangeUcc,
    DimensionOneAllElementsEqualUcc, DimensionOneAllElementsGreaterThanUcc,
    DimensionOneAllElementsRegularExpressionUcc, DimensionOneBeforeUcc, DimensionOneBetweenUcc,
    DimensionOneContainsAllElementsOfArrayUcc, DimensionOneContainsElGreaterThanUcc,
    DimensionOneContainsElRegularExpressionUcc, DimensionOneCurrentDateUcc,
    DimensionOneCurrentTimeUcc, DimensionOneCurrentTimestampUcc,
    DimensionOneEqualToEncodedStringRepresentationUcc, DimensionOneEqualUcc,
    DimensionOneExcludedUpperBoundUcc, DimensionOneFindRangesThatFullyContainTheGivenRangeUcc,
    DimensionOneFindRangesWithinGivenRangeUcc, DimensionOneGreaterThanCurrentDateUcc,
    DimensionOneGreaterThanCurrentTimeUcc, DimensionOneGreaterThanCurrentTimestampUcc,
    DimensionOneGreaterThanExcludedUpperBoundUcc, DimensionOneGreaterThanIncludedLowerBoundUcc,
    DimensionOneGreaterThanUcc, DimensionOneInUcc, DimensionOneIncludedLowerBoundUcc,
    DimensionOneLengthEqualUcc, DimensionOneLengthGreaterThanUcc, DimensionOneOverlapWithRangeUcc,
    DimensionOneOverlapsWithArrayUcc, DimensionOneRangeLengthUcc, DimensionOneRegularExpressionUcc,
    DimensionOneStrictlyToLeftOfRangeUcc, DimensionOneStrictlyToRightOfRangeUcc,
    DimensionThreeAllElementsEqualUcc, DimensionThreeAllElementsGreaterThanUcc,
    DimensionThreeAllElementsRegularExpressionUcc, DimensionThreeBetweenUcc,
    DimensionThreeContainsAllElementsOfArrayUcc, DimensionThreeContainsElGreaterThanUcc,
    DimensionThreeContainsElRegularExpressionUcc, DimensionThreeEqualUcc,
    DimensionThreeGreaterThanUcc, DimensionThreeInUcc, DimensionThreeLengthEqualUcc,
    DimensionThreeLengthGreaterThanUcc, DimensionThreeOverlapsWithArrayUcc,
    DimensionThreeRegularExpressionUcc, DimensionTwoAllElementsEqualUcc,
    DimensionTwoAllElementsGreaterThanUcc, DimensionTwoAllElementsRegularExpressionUcc,
    DimensionTwoBetweenUcc, DimensionTwoContainsAllElementsOfArrayUcc,
    DimensionTwoContainsElGreaterThanUcc, DimensionTwoContainsElRegularExpressionUcc,
    DimensionTwoEqualUcc, DimensionTwoGreaterThanUcc, DimensionTwoInUcc,
    DimensionTwoLengthEqualUcc, DimensionTwoLengthGreaterThanUcc, DimensionTwoOverlapsWithArrayUcc,
    DimensionTwoRegularExpressionUcc, EqualToEncodedStringRepresentationUcc, EqualUcc,
    ExcludedUpperBoundUcc, FindRangesThatFullyContainTheGivenRangeUcc,
    FindRangesWithinGivenRangeUcc, GreaterThanCurrentDateUcc, GreaterThanCurrentTimeUcc,
    GreaterThanCurrentTimestampUcc, GreaterThanExcludedUpperBoundUcc,
    GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc, IncludedLowerBoundUcc, LengthEqualUcc,
    LengthGreaterThanUcc, OverlapWithRangeUcc, OverlapsWithArrayUcc, RangeLengthUcc,
    RegularExpressionUcc, StdFmtDisplayPlusQuoteToTokens, StrictlyToLeftOfRangeUcc,
    StrictlyToRightOfRangeUcc,
    parameter::{PostgresqlJsonTypeWhereSelfUcc, PostgresqlTypeWhereSelfUcc},
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
    fn prefix_where_self_ucc(&self) -> TokenStream {
        let value = PostgresqlTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#value}
    }
    fn ucc(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &EqualUcc,
            Self::DimensionOneEqual { .. } => &DimensionOneEqualUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimensionOneGreaterThan { .. } => &DimensionOneGreaterThanUcc,
            Self::Between { .. } => &BetweenUcc,
            Self::DimensionOneBetween { .. } => &DimensionOneBetweenUcc,
            Self::In { .. } => &InUcc,
            Self::DimensionOneIn { .. } => &DimensionOneInUcc,
            Self::RegularExpression => &RegularExpressionUcc,
            Self::DimensionOneRegularExpression => &DimensionOneRegularExpressionUcc,
            Self::Before { .. } => &BeforeUcc,
            Self::DimensionOneBefore { .. } => &DimensionOneBeforeUcc,
            Self::CurrentDate => &CurrentDateUcc,
            Self::DimensionOneCurrentDate => &DimensionOneCurrentDateUcc,
            Self::GreaterThanCurrentDate => &GreaterThanCurrentDateUcc,
            Self::DimensionOneGreaterThanCurrentDate => &DimensionOneGreaterThanCurrentDateUcc,
            Self::CurrentTimestamp => &CurrentTimestampUcc,
            Self::DimensionOneCurrentTimestamp => &DimensionOneCurrentTimestampUcc,
            Self::GreaterThanCurrentTimestamp => &GreaterThanCurrentTimestampUcc,
            Self::DimensionOneGreaterThanCurrentTimestamp => {
                &DimensionOneGreaterThanCurrentTimestampUcc
            }
            Self::CurrentTime => &CurrentTimeUcc,
            Self::DimensionOneCurrentTime => &DimensionOneCurrentTimeUcc,
            Self::GreaterThanCurrentTime => &GreaterThanCurrentTimeUcc,
            Self::DimensionOneGreaterThanCurrentTime => &DimensionOneGreaterThanCurrentTimeUcc,
            Self::DimensionOneLengthEqual => &DimensionOneLengthEqualUcc,
            Self::DimensionOneLengthGreaterThan => &DimensionOneLengthGreaterThanUcc,
            Self::EqualToEncodedStringRepresentation => &EqualToEncodedStringRepresentationUcc,
            Self::DimensionOneEqualToEncodedStringRepresentation => {
                &DimensionOneEqualToEncodedStringRepresentationUcc
            }
            Self::FindRangesWithinGivenRange { .. } => &FindRangesWithinGivenRangeUcc,
            Self::DimensionOneFindRangesWithinGivenRange { .. } => {
                &DimensionOneFindRangesWithinGivenRangeUcc
            }
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &FindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange { .. } => {
                &DimensionOneFindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::StrictlyToLeftOfRange { .. } => &StrictlyToLeftOfRangeUcc,
            Self::DimensionOneStrictlyToLeftOfRange { .. } => &DimensionOneStrictlyToLeftOfRangeUcc,
            Self::StrictlyToRightOfRange { .. } => &StrictlyToRightOfRangeUcc,
            Self::DimensionOneStrictlyToRightOfRange { .. } => {
                &DimensionOneStrictlyToRightOfRangeUcc
            }
            Self::IncludedLowerBound { .. } => &IncludedLowerBoundUcc,
            Self::DimensionOneIncludedLowerBound { .. } => &DimensionOneIncludedLowerBoundUcc,
            Self::ExcludedUpperBound { .. } => &ExcludedUpperBoundUcc,
            Self::DimensionOneExcludedUpperBound { .. } => &DimensionOneExcludedUpperBoundUcc,
            Self::GreaterThanIncludedLowerBound { .. } => &GreaterThanIncludedLowerBoundUcc,
            Self::DimensionOneGreaterThanIncludedLowerBound { .. } => {
                &DimensionOneGreaterThanIncludedLowerBoundUcc
            }
            Self::GreaterThanExcludedUpperBound { .. } => &GreaterThanExcludedUpperBoundUcc,
            Self::DimensionOneGreaterThanExcludedUpperBound { .. } => {
                &DimensionOneGreaterThanExcludedUpperBoundUcc
            }
            Self::OverlapWithRange { .. } => &OverlapWithRangeUcc,
            Self::DimensionOneOverlapWithRange { .. } => &DimensionOneOverlapWithRangeUcc,
            Self::AdjacentWithRange { .. } => &AdjacentWithRangeUcc,
            Self::DimensionOneAdjacentWithRange { .. } => &DimensionOneAdjacentWithRangeUcc,
            Self::RangeLength => &RangeLengthUcc,
            Self::DimensionOneRangeLength => &DimensionOneRangeLengthUcc,
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
    fn prefix_where_self_ucc(&self) -> TokenStream {
        let value = PostgresqlJsonTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#value}
    }
    fn ucc(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Equal { .. } => &EqualUcc,
            Self::DimensionOneEqual { .. } => &DimensionOneEqualUcc,
            Self::DimensionTwoEqual { .. } => &DimensionTwoEqualUcc,
            Self::DimensionThreeEqual { .. } => &DimensionThreeEqualUcc,
            Self::DimensionFourEqual { .. } => &DimensionFourEqualUcc,
            Self::AllElementsEqual { .. } => &AllElementsEqualUcc,
            Self::DimensionOneAllElementsEqual { .. } => &DimensionOneAllElementsEqualUcc,
            Self::DimensionTwoAllElementsEqual { .. } => &DimensionTwoAllElementsEqualUcc,
            Self::DimensionThreeAllElementsEqual { .. } => &DimensionThreeAllElementsEqualUcc,
            Self::DimensionFourAllElementsEqual { .. } => &DimensionFourAllElementsEqualUcc,
            Self::LengthEqual => &LengthEqualUcc,
            Self::DimensionOneLengthEqual => &DimensionOneLengthEqualUcc,
            Self::DimensionTwoLengthEqual => &DimensionTwoLengthEqualUcc,
            Self::DimensionThreeLengthEqual => &DimensionThreeLengthEqualUcc,
            Self::DimensionFourLengthEqual => &DimensionFourLengthEqualUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimensionOneGreaterThan { .. } => &DimensionOneGreaterThanUcc,
            Self::DimensionTwoGreaterThan { .. } => &DimensionTwoGreaterThanUcc,
            Self::DimensionThreeGreaterThan { .. } => &DimensionThreeGreaterThanUcc,
            Self::DimensionFourGreaterThan { .. } => &DimensionFourGreaterThanUcc,
            Self::ContainsElGreaterThan { .. } => &ContainsElGreaterThanUcc,
            Self::DimensionOneContainsElGreaterThan { .. } => &DimensionOneContainsElGreaterThanUcc,
            Self::DimensionTwoContainsElGreaterThan { .. } => &DimensionTwoContainsElGreaterThanUcc,
            Self::DimensionThreeContainsElGreaterThan { .. } => {
                &DimensionThreeContainsElGreaterThanUcc
            }
            Self::DimensionFourContainsElGreaterThan { .. } => {
                &DimensionFourContainsElGreaterThanUcc
            }
            Self::AllElementsGreaterThan { .. } => &AllElementsGreaterThanUcc,
            Self::DimensionOneAllElementsGreaterThan { .. } => {
                &DimensionOneAllElementsGreaterThanUcc
            }
            Self::DimensionTwoAllElementsGreaterThan { .. } => {
                &DimensionTwoAllElementsGreaterThanUcc
            }
            Self::DimensionThreeAllElementsGreaterThan { .. } => {
                &DimensionThreeAllElementsGreaterThanUcc
            }
            Self::DimensionFourAllElementsGreaterThan { .. } => {
                &DimensionFourAllElementsGreaterThanUcc
            }
            Self::Between { .. } => &BetweenUcc,
            Self::DimensionOneBetween { .. } => &DimensionOneBetweenUcc,
            Self::DimensionTwoBetween { .. } => &DimensionTwoBetweenUcc,
            Self::DimensionThreeBetween { .. } => &DimensionThreeBetweenUcc,
            Self::DimensionFourBetween { .. } => &DimensionFourBetweenUcc,
            Self::In { .. } => &InUcc,
            Self::DimensionOneIn { .. } => &DimensionOneInUcc,
            Self::DimensionTwoIn { .. } => &DimensionTwoInUcc,
            Self::DimensionThreeIn { .. } => &DimensionThreeInUcc,
            Self::DimensionFourIn { .. } => &DimensionFourInUcc,
            Self::RegularExpression => &RegularExpressionUcc,
            Self::DimensionOneRegularExpression => &DimensionOneRegularExpressionUcc,
            Self::DimensionTwoRegularExpression => &DimensionTwoRegularExpressionUcc,
            Self::DimensionThreeRegularExpression => &DimensionThreeRegularExpressionUcc,
            Self::DimensionFourRegularExpression => &DimensionFourRegularExpressionUcc,
            Self::ContainsElRegularExpression => &ContainsElRegularExpressionUcc,
            Self::DimensionOneContainsElRegularExpression => {
                &DimensionOneContainsElRegularExpressionUcc
            }
            Self::DimensionTwoContainsElRegularExpression => {
                &DimensionTwoContainsElRegularExpressionUcc
            }
            Self::DimensionThreeContainsElRegularExpression => {
                &DimensionThreeContainsElRegularExpressionUcc
            }
            Self::DimensionFourContainsElRegularExpression => {
                &DimensionFourContainsElRegularExpressionUcc
            }
            Self::AllElementsRegularExpression => &AllElementsRegularExpressionUcc,
            Self::DimensionOneAllElementsRegularExpression => {
                &DimensionOneAllElementsRegularExpressionUcc
            }
            Self::DimensionTwoAllElementsRegularExpression => {
                &DimensionTwoAllElementsRegularExpressionUcc
            }
            Self::DimensionThreeAllElementsRegularExpression => {
                &DimensionThreeAllElementsRegularExpressionUcc
            }
            Self::DimensionFourAllElementsRegularExpression => {
                &DimensionFourAllElementsRegularExpressionUcc
            }
            Self::LengthGreaterThan => &LengthGreaterThanUcc,
            Self::DimensionOneLengthGreaterThan => &DimensionOneLengthGreaterThanUcc,
            Self::DimensionTwoLengthGreaterThan => &DimensionTwoLengthGreaterThanUcc,
            Self::DimensionThreeLengthGreaterThan => &DimensionThreeLengthGreaterThanUcc,
            Self::DimensionFourLengthGreaterThan => &DimensionFourLengthGreaterThanUcc,
            Self::ContainsAllElementsOfArray { .. } => &ContainsAllElementsOfArrayUcc,
            Self::DimensionOneContainsAllElementsOfArray { .. } => {
                &DimensionOneContainsAllElementsOfArrayUcc
            }
            Self::DimensionTwoContainsAllElementsOfArray { .. } => {
                &DimensionTwoContainsAllElementsOfArrayUcc
            }
            Self::DimensionThreeContainsAllElementsOfArray { .. } => {
                &DimensionThreeContainsAllElementsOfArrayUcc
            }
            Self::DimensionFourContainsAllElementsOfArray { .. } => {
                &DimensionFourContainsAllElementsOfArrayUcc
            }
            Self::OverlapsWithArray { .. } => &OverlapsWithArrayUcc,
            Self::DimensionOneOverlapsWithArray { .. } => &DimensionOneOverlapsWithArrayUcc,
            Self::DimensionTwoOverlapsWithArray { .. } => &DimensionTwoOverlapsWithArrayUcc,
            Self::DimensionThreeOverlapsWithArray { .. } => &DimensionThreeOverlapsWithArrayUcc,
            Self::DimensionFourOverlapsWithArray { .. } => &DimensionFourOverlapsWithArrayUcc,
        }
    }
}

pub trait PostgresqlFilter {
    fn maybe_generic(&self) -> Option<TokenStream>;
    fn prefix_where_self_ucc(&self) -> TokenStream;
    fn ucc(&self) -> &'static dyn StdFmtDisplayPlusQuoteToTokens;
}
