use enum_extension_lib::EnumExtension;
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
    DimensionTwoRegularExpressionUcc, DisplayPlusToTokens, EqualToEncodedStringRepresentationUcc,
    EqualUcc, ExcludedUpperBoundUcc, FindRangesThatFullyContainTheGivenRangeUcc,
    FindRangesWithinGivenRangeUcc, GreaterThanCurrentDateUcc, GreaterThanCurrentTimeUcc,
    GreaterThanCurrentTimestampUcc, GreaterThanExcludedUpperBoundUcc,
    GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc, IncludedLowerBoundUcc, LengthEqualUcc,
    LengthGreaterThanUcc, OverlapWithRangeUcc, OverlapsWithArrayUcc, RangeLengthUcc,
    RegularExpressionUcc, StrictlyToLeftOfRangeUcc, StrictlyToRightOfRangeUcc,
    parameter::{PgJsonTypeWhereSelfUcc, PgTypeWhereSelfUcc},
};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use strum_macros::{Display, EnumIter};

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, EnumExtension)]
pub enum PgTypeFilter {
    Equal { ident: Ts2 },
    DimensionOneEqual { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    DimensionOneGreaterThan { ident: Ts2 },
    Between { ident: Ts2 },
    DimensionOneBetween { ident: Ts2 },
    In { ident: Ts2 },
    DimensionOneIn { ident: Ts2 },
    RegularExpression,
    DimensionOneRegularExpression,
    Before { ident: Ts2 },
    DimensionOneBefore { ident: Ts2 },
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
    FindRangesWithinGivenRange { ident: Ts2 },
    DimensionOneFindRangesWithinGivenRange { ident: Ts2 },
    FindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    DimensionOneFindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    StrictlyToLeftOfRange { ident: Ts2 },
    DimensionOneStrictlyToLeftOfRange { ident: Ts2 },
    StrictlyToRightOfRange { ident: Ts2 },
    DimensionOneStrictlyToRightOfRange { ident: Ts2 },
    IncludedLowerBound { ident: Ts2 },
    DimensionOneIncludedLowerBound { ident: Ts2 },
    ExcludedUpperBound { ident: Ts2 },
    DimensionOneExcludedUpperBound { ident: Ts2 },
    GreaterThanIncludedLowerBound { ident: Ts2 },
    DimensionOneGreaterThanIncludedLowerBound { ident: Ts2 },
    GreaterThanExcludedUpperBound { ident: Ts2 },
    DimensionOneGreaterThanExcludedUpperBound { ident: Ts2 },
    OverlapWithRange { ident: Ts2 },
    DimensionOneOverlapWithRange { ident: Ts2 },
    AdjacentWithRange { ident: Ts2 },
    DimensionOneAdjacentWithRange { ident: Ts2 },
    RangeLength,
    DimensionOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PgFilter for PgTypeFilter {
    fn maybe_generic(&self) -> Option<Ts2> {
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
    fn prefix_where_self_ucc(&self) -> Ts2 {
        let value = PgTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#value}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
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
#[derive(Debug, Clone, Display, EnumIter, EnumExtension)]
pub enum PgJsonTypeFilter {
    Equal { ident: Ts2 },
    DimensionOneEqual { ident: Ts2 },
    DimensionTwoEqual { ident: Ts2 },
    DimensionThreeEqual { ident: Ts2 },
    DimensionFourEqual { ident: Ts2 },
    AllElementsEqual { ident: Ts2 },
    DimensionOneAllElementsEqual { ident: Ts2 },
    DimensionTwoAllElementsEqual { ident: Ts2 },
    DimensionThreeAllElementsEqual { ident: Ts2 },
    DimensionFourAllElementsEqual { ident: Ts2 },
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
    GreaterThan { ident: Ts2 },
    DimensionOneGreaterThan { ident: Ts2 },
    DimensionTwoGreaterThan { ident: Ts2 },
    DimensionThreeGreaterThan { ident: Ts2 },
    DimensionFourGreaterThan { ident: Ts2 },
    ContainsElGreaterThan { ident: Ts2 },
    DimensionOneContainsElGreaterThan { ident: Ts2 },
    DimensionTwoContainsElGreaterThan { ident: Ts2 },
    DimensionThreeContainsElGreaterThan { ident: Ts2 },
    DimensionFourContainsElGreaterThan { ident: Ts2 },
    AllElementsGreaterThan { ident: Ts2 },
    DimensionOneAllElementsGreaterThan { ident: Ts2 },
    DimensionTwoAllElementsGreaterThan { ident: Ts2 },
    DimensionThreeAllElementsGreaterThan { ident: Ts2 },
    DimensionFourAllElementsGreaterThan { ident: Ts2 },
    Between { ident: Ts2 },
    DimensionOneBetween { ident: Ts2 },
    DimensionTwoBetween { ident: Ts2 },
    DimensionThreeBetween { ident: Ts2 },
    DimensionFourBetween { ident: Ts2 },
    In { ident: Ts2 },
    DimensionOneIn { ident: Ts2 },
    DimensionTwoIn { ident: Ts2 },
    DimensionThreeIn { ident: Ts2 },
    DimensionFourIn { ident: Ts2 },
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
    ContainsAllElementsOfArray { ident: Ts2 },
    DimensionOneContainsAllElementsOfArray { ident: Ts2 },
    DimensionTwoContainsAllElementsOfArray { ident: Ts2 },
    DimensionThreeContainsAllElementsOfArray { ident: Ts2 },
    DimensionFourContainsAllElementsOfArray { ident: Ts2 },
    // ContainedInArray,
    OverlapsWithArray { ident: Ts2 },
    DimensionOneOverlapsWithArray { ident: Ts2 },
    DimensionTwoOverlapsWithArray { ident: Ts2 },
    DimensionThreeOverlapsWithArray { ident: Ts2 },
    DimensionFourOverlapsWithArray { ident: Ts2 },
}
impl PgFilter for PgJsonTypeFilter {
    fn maybe_generic(&self) -> Option<Ts2> {
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
    fn prefix_where_self_ucc(&self) -> Ts2 {
        let value = PgJsonTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#value}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
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

pub trait PgFilter {
    fn maybe_generic(&self) -> Option<Ts2>;
    fn prefix_where_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
