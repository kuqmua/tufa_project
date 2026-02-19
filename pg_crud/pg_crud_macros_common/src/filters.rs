use enum_extension_lib::EnumExtension;
use naming::{
    AdjacentWithRangeUcc, AllElementsEqualUcc, AllElementsGreaterThanUcc,
    AllElementsRegularExpressionUcc, BeforeUcc, BetweenUcc, ContainsAllElementsOfArrayUcc,
    ContainsElGreaterThanUcc, ContainsElRegularExpressionUcc, CurrentDateUcc, CurrentTimeUcc,
    CurrentTimestampUcc, DimFourAllElementsEqualUcc, DimFourAllElementsGreaterThanUcc,
    DimFourAllElementsRegularExpressionUcc, DimFourBetweenUcc,
    DimFourContainsAllElementsOfArrayUcc, DimFourContainsElGreaterThanUcc,
    DimFourContainsElRegularExpressionUcc, DimFourEqualUcc, DimFourGreaterThanUcc, DimFourInUcc,
    DimFourLengthEqualUcc, DimFourLengthGreaterThanUcc, DimFourOverlapsWithArrayUcc,
    DimFourRegularExpressionUcc, DimOneAdjacentWithRangeUcc, DimOneAllElementsEqualUcc,
    DimOneAllElementsGreaterThanUcc, DimOneAllElementsRegularExpressionUcc, DimOneBeforeUcc,
    DimOneBetweenUcc, DimOneContainsAllElementsOfArrayUcc, DimOneContainsElGreaterThanUcc,
    DimOneContainsElRegularExpressionUcc, DimOneCurrentDateUcc, DimOneCurrentTimeUcc,
    DimOneCurrentTimestampUcc, DimOneEqualToEncodedStringRepresentationUcc, DimOneEqualUcc,
    DimOneExcludedUpperBoundUcc, DimOneFindRangesThatFullyContainTheGivenRangeUcc,
    DimOneFindRangesWithinGivenRangeUcc, DimOneGreaterThanCurrentDateUcc,
    DimOneGreaterThanCurrentTimeUcc, DimOneGreaterThanCurrentTimestampUcc,
    DimOneGreaterThanExcludedUpperBoundUcc, DimOneGreaterThanIncludedLowerBoundUcc,
    DimOneGreaterThanUcc, DimOneInUcc, DimOneIncludedLowerBoundUcc, DimOneLengthEqualUcc,
    DimOneLengthGreaterThanUcc, DimOneOverlapWithRangeUcc, DimOneOverlapsWithArrayUcc,
    DimOneRangeLengthUcc, DimOneRegularExpressionUcc, DimOneStrictlyToLeftOfRangeUcc,
    DimOneStrictlyToRightOfRangeUcc, DimThreeAllElementsEqualUcc,
    DimThreeAllElementsGreaterThanUcc, DimThreeAllElementsRegularExpressionUcc, DimThreeBetweenUcc,
    DimThreeContainsAllElementsOfArrayUcc, DimThreeContainsElGreaterThanUcc,
    DimThreeContainsElRegularExpressionUcc, DimThreeEqualUcc, DimThreeGreaterThanUcc,
    DimThreeInUcc, DimThreeLengthEqualUcc, DimThreeLengthGreaterThanUcc,
    DimThreeOverlapsWithArrayUcc, DimThreeRegularExpressionUcc, DimTwoAllElementsEqualUcc,
    DimTwoAllElementsGreaterThanUcc, DimTwoAllElementsRegularExpressionUcc, DimTwoBetweenUcc,
    DimTwoContainsAllElementsOfArrayUcc, DimTwoContainsElGreaterThanUcc,
    DimTwoContainsElRegularExpressionUcc, DimTwoEqualUcc, DimTwoGreaterThanUcc, DimTwoInUcc,
    DimTwoLengthEqualUcc, DimTwoLengthGreaterThanUcc, DimTwoOverlapsWithArrayUcc,
    DimTwoRegularExpressionUcc, DisplayPlusToTokens, EqualToEncodedStringRepresentationUcc,
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
    DimOneEqual { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    DimOneGreaterThan { ident: Ts2 },
    Between { ident: Ts2 },
    DimOneBetween { ident: Ts2 },
    In { ident: Ts2 },
    DimOneIn { ident: Ts2 },
    RegularExpression,
    DimOneRegularExpression,
    Before { ident: Ts2 },
    DimOneBefore { ident: Ts2 },
    CurrentDate,
    DimOneCurrentDate,
    GreaterThanCurrentDate,
    DimOneGreaterThanCurrentDate,
    CurrentTimestamp,
    DimOneCurrentTimestamp,
    GreaterThanCurrentTimestamp,
    DimOneGreaterThanCurrentTimestamp,
    CurrentTime,
    DimOneCurrentTime,
    GreaterThanCurrentTime,
    DimOneGreaterThanCurrentTime,
    DimOneLengthEqual,
    DimOneLengthGreaterThan,
    EqualToEncodedStringRepresentation,
    DimOneEqualToEncodedStringRepresentation,
    FindRangesWithinGivenRange { ident: Ts2 },
    DimOneFindRangesWithinGivenRange { ident: Ts2 },
    FindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    DimOneFindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    StrictlyToLeftOfRange { ident: Ts2 },
    DimOneStrictlyToLeftOfRange { ident: Ts2 },
    StrictlyToRightOfRange { ident: Ts2 },
    DimOneStrictlyToRightOfRange { ident: Ts2 },
    IncludedLowerBound { ident: Ts2 },
    DimOneIncludedLowerBound { ident: Ts2 },
    ExcludedUpperBound { ident: Ts2 },
    DimOneExcludedUpperBound { ident: Ts2 },
    GreaterThanIncludedLowerBound { ident: Ts2 },
    DimOneGreaterThanIncludedLowerBound { ident: Ts2 },
    GreaterThanExcludedUpperBound { ident: Ts2 },
    DimOneGreaterThanExcludedUpperBound { ident: Ts2 },
    OverlapWithRange { ident: Ts2 },
    DimOneOverlapWithRange { ident: Ts2 },
    AdjacentWithRange { ident: Ts2 },
    DimOneAdjacentWithRange { ident: Ts2 },
    RangeLength,
    DimOneRangeLength,
    //BitVecPositionEqual,//currently deactivated
}
impl PgFilter for PgTypeFilter {
    fn maybe_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Equal { ident }
            | Self::DimOneEqual { ident }
            | Self::GreaterThan { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::Between { ident }
            | Self::DimOneBetween { ident }
            | Self::In { ident }
            | Self::DimOneIn { ident }
            | Self::Before { ident }
            | Self::DimOneBefore { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::DimOneFindRangesWithinGivenRange { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::DimOneFindRangesThatFullyContainTheGivenRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::DimOneStrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident }
            | Self::DimOneStrictlyToRightOfRange { ident }
            | Self::IncludedLowerBound { ident }
            | Self::DimOneIncludedLowerBound { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::DimOneExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::DimOneGreaterThanIncludedLowerBound { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::DimOneGreaterThanExcludedUpperBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::DimOneOverlapWithRange { ident }
            | Self::AdjacentWithRange { ident }
            | Self::DimOneAdjacentWithRange { ident } => Some(ident.clone()),
            Self::RegularExpression
            | Self::DimOneRegularExpression
            | Self::CurrentDate
            | Self::DimOneCurrentDate
            | Self::GreaterThanCurrentDate
            | Self::DimOneGreaterThanCurrentDate
            | Self::CurrentTimestamp
            | Self::DimOneCurrentTimestamp
            | Self::GreaterThanCurrentTimestamp
            | Self::DimOneGreaterThanCurrentTimestamp
            | Self::CurrentTime
            | Self::DimOneCurrentTime
            | Self::GreaterThanCurrentTime
            | Self::DimOneGreaterThanCurrentTime
            | Self::DimOneLengthEqual
            | Self::DimOneLengthGreaterThan
            | Self::EqualToEncodedStringRepresentation
            | Self::DimOneEqualToEncodedStringRepresentation
            | Self::RangeLength
            | Self::DimOneRangeLength => None,
        }
    }
    fn prefix_where_self_ucc(&self) -> Ts2 {
        let v = PgTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Equal { .. } => &EqualUcc,
            Self::DimOneEqual { .. } => &DimOneEqualUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimOneGreaterThan { .. } => &DimOneGreaterThanUcc,
            Self::Between { .. } => &BetweenUcc,
            Self::DimOneBetween { .. } => &DimOneBetweenUcc,
            Self::In { .. } => &InUcc,
            Self::DimOneIn { .. } => &DimOneInUcc,
            Self::RegularExpression => &RegularExpressionUcc,
            Self::DimOneRegularExpression => &DimOneRegularExpressionUcc,
            Self::Before { .. } => &BeforeUcc,
            Self::DimOneBefore { .. } => &DimOneBeforeUcc,
            Self::CurrentDate => &CurrentDateUcc,
            Self::DimOneCurrentDate => &DimOneCurrentDateUcc,
            Self::GreaterThanCurrentDate => &GreaterThanCurrentDateUcc,
            Self::DimOneGreaterThanCurrentDate => &DimOneGreaterThanCurrentDateUcc,
            Self::CurrentTimestamp => &CurrentTimestampUcc,
            Self::DimOneCurrentTimestamp => &DimOneCurrentTimestampUcc,
            Self::GreaterThanCurrentTimestamp => &GreaterThanCurrentTimestampUcc,
            Self::DimOneGreaterThanCurrentTimestamp => &DimOneGreaterThanCurrentTimestampUcc,
            Self::CurrentTime => &CurrentTimeUcc,
            Self::DimOneCurrentTime => &DimOneCurrentTimeUcc,
            Self::GreaterThanCurrentTime => &GreaterThanCurrentTimeUcc,
            Self::DimOneGreaterThanCurrentTime => &DimOneGreaterThanCurrentTimeUcc,
            Self::DimOneLengthEqual => &DimOneLengthEqualUcc,
            Self::DimOneLengthGreaterThan => &DimOneLengthGreaterThanUcc,
            Self::EqualToEncodedStringRepresentation => &EqualToEncodedStringRepresentationUcc,
            Self::DimOneEqualToEncodedStringRepresentation => {
                &DimOneEqualToEncodedStringRepresentationUcc
            }
            Self::FindRangesWithinGivenRange { .. } => &FindRangesWithinGivenRangeUcc,
            Self::DimOneFindRangesWithinGivenRange { .. } => &DimOneFindRangesWithinGivenRangeUcc,
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &FindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                &DimOneFindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::StrictlyToLeftOfRange { .. } => &StrictlyToLeftOfRangeUcc,
            Self::DimOneStrictlyToLeftOfRange { .. } => &DimOneStrictlyToLeftOfRangeUcc,
            Self::StrictlyToRightOfRange { .. } => &StrictlyToRightOfRangeUcc,
            Self::DimOneStrictlyToRightOfRange { .. } => &DimOneStrictlyToRightOfRangeUcc,
            Self::IncludedLowerBound { .. } => &IncludedLowerBoundUcc,
            Self::DimOneIncludedLowerBound { .. } => &DimOneIncludedLowerBoundUcc,
            Self::ExcludedUpperBound { .. } => &ExcludedUpperBoundUcc,
            Self::DimOneExcludedUpperBound { .. } => &DimOneExcludedUpperBoundUcc,
            Self::GreaterThanIncludedLowerBound { .. } => &GreaterThanIncludedLowerBoundUcc,
            Self::DimOneGreaterThanIncludedLowerBound { .. } => {
                &DimOneGreaterThanIncludedLowerBoundUcc
            }
            Self::GreaterThanExcludedUpperBound { .. } => &GreaterThanExcludedUpperBoundUcc,
            Self::DimOneGreaterThanExcludedUpperBound { .. } => {
                &DimOneGreaterThanExcludedUpperBoundUcc
            }
            Self::OverlapWithRange { .. } => &OverlapWithRangeUcc,
            Self::DimOneOverlapWithRange { .. } => &DimOneOverlapWithRangeUcc,
            Self::AdjacentWithRange { .. } => &AdjacentWithRangeUcc,
            Self::DimOneAdjacentWithRange { .. } => &DimOneAdjacentWithRangeUcc,
            Self::RangeLength => &RangeLengthUcc,
            Self::DimOneRangeLength => &DimOneRangeLengthUcc,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, EnumExtension)]
pub enum PgJsonTypeFilter {
    Equal { ident: Ts2 },
    DimOneEqual { ident: Ts2 },
    DimTwoEqual { ident: Ts2 },
    DimThreeEqual { ident: Ts2 },
    DimFourEqual { ident: Ts2 },
    AllElementsEqual { ident: Ts2 },
    DimOneAllElementsEqual { ident: Ts2 },
    DimTwoAllElementsEqual { ident: Ts2 },
    DimThreeAllElementsEqual { ident: Ts2 },
    DimFourAllElementsEqual { ident: Ts2 },
    LengthEqual,
    DimOneLengthEqual,
    DimTwoLengthEqual,
    DimThreeLengthEqual,
    DimFourLengthEqual,
    LengthGreaterThan,
    DimOneLengthGreaterThan,
    DimTwoLengthGreaterThan,
    DimThreeLengthGreaterThan,
    DimFourLengthGreaterThan,
    GreaterThan { ident: Ts2 },
    DimOneGreaterThan { ident: Ts2 },
    DimTwoGreaterThan { ident: Ts2 },
    DimThreeGreaterThan { ident: Ts2 },
    DimFourGreaterThan { ident: Ts2 },
    ContainsElGreaterThan { ident: Ts2 },
    DimOneContainsElGreaterThan { ident: Ts2 },
    DimTwoContainsElGreaterThan { ident: Ts2 },
    DimThreeContainsElGreaterThan { ident: Ts2 },
    DimFourContainsElGreaterThan { ident: Ts2 },
    AllElementsGreaterThan { ident: Ts2 },
    DimOneAllElementsGreaterThan { ident: Ts2 },
    DimTwoAllElementsGreaterThan { ident: Ts2 },
    DimThreeAllElementsGreaterThan { ident: Ts2 },
    DimFourAllElementsGreaterThan { ident: Ts2 },
    Between { ident: Ts2 },
    DimOneBetween { ident: Ts2 },
    DimTwoBetween { ident: Ts2 },
    DimThreeBetween { ident: Ts2 },
    DimFourBetween { ident: Ts2 },
    In { ident: Ts2 },
    DimOneIn { ident: Ts2 },
    DimTwoIn { ident: Ts2 },
    DimThreeIn { ident: Ts2 },
    DimFourIn { ident: Ts2 },
    RegularExpression,
    DimOneRegularExpression,
    DimTwoRegularExpression,
    DimThreeRegularExpression,
    DimFourRegularExpression,
    ContainsElRegularExpression,
    DimOneContainsElRegularExpression,
    DimTwoContainsElRegularExpression,
    DimThreeContainsElRegularExpression,
    DimFourContainsElRegularExpression,
    AllElementsRegularExpression,
    DimOneAllElementsRegularExpression,
    DimTwoAllElementsRegularExpression,
    DimThreeAllElementsRegularExpression,
    DimFourAllElementsRegularExpression,
    ContainsAllElementsOfArray { ident: Ts2 },
    DimOneContainsAllElementsOfArray { ident: Ts2 },
    DimTwoContainsAllElementsOfArray { ident: Ts2 },
    DimThreeContainsAllElementsOfArray { ident: Ts2 },
    DimFourContainsAllElementsOfArray { ident: Ts2 },
    // ContainedInArray,
    OverlapsWithArray { ident: Ts2 },
    DimOneOverlapsWithArray { ident: Ts2 },
    DimTwoOverlapsWithArray { ident: Ts2 },
    DimThreeOverlapsWithArray { ident: Ts2 },
    DimFourOverlapsWithArray { ident: Ts2 },
}
impl PgFilter for PgJsonTypeFilter {
    fn maybe_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Equal { ident }
            | Self::DimOneEqual { ident }
            | Self::DimTwoEqual { ident }
            | Self::DimThreeEqual { ident }
            | Self::DimFourEqual { ident }
            | Self::AllElementsEqual { ident }
            | Self::DimOneAllElementsEqual { ident }
            | Self::DimTwoAllElementsEqual { ident }
            | Self::DimThreeAllElementsEqual { ident }
            | Self::DimFourAllElementsEqual { ident }
            | Self::GreaterThan { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::DimTwoGreaterThan { ident }
            | Self::DimThreeGreaterThan { ident }
            | Self::DimFourGreaterThan { ident }
            | Self::ContainsElGreaterThan { ident }
            | Self::DimOneContainsElGreaterThan { ident }
            | Self::DimTwoContainsElGreaterThan { ident }
            | Self::DimThreeContainsElGreaterThan { ident }
            | Self::DimFourContainsElGreaterThan { ident }
            | Self::AllElementsGreaterThan { ident }
            | Self::DimOneAllElementsGreaterThan { ident }
            | Self::DimTwoAllElementsGreaterThan { ident }
            | Self::DimThreeAllElementsGreaterThan { ident }
            | Self::DimFourAllElementsGreaterThan { ident }
            | Self::Between { ident }
            | Self::DimOneBetween { ident }
            | Self::DimTwoBetween { ident }
            | Self::DimThreeBetween { ident }
            | Self::DimFourBetween { ident }
            | Self::In { ident }
            | Self::DimOneIn { ident }
            | Self::DimTwoIn { ident }
            | Self::DimThreeIn { ident }
            | Self::DimFourIn { ident }
            | Self::ContainsAllElementsOfArray { ident }
            | Self::DimOneContainsAllElementsOfArray { ident }
            | Self::DimTwoContainsAllElementsOfArray { ident }
            | Self::DimThreeContainsAllElementsOfArray { ident }
            | Self::DimFourContainsAllElementsOfArray { ident }
            | Self::OverlapsWithArray { ident }
            | Self::DimOneOverlapsWithArray { ident }
            | Self::DimTwoOverlapsWithArray { ident }
            | Self::DimThreeOverlapsWithArray { ident }
            | Self::DimFourOverlapsWithArray { ident } => Some(ident.clone()),
            Self::LengthEqual
            | Self::DimOneLengthEqual
            | Self::DimTwoLengthEqual
            | Self::DimThreeLengthEqual
            | Self::DimFourLengthEqual
            | Self::RegularExpression
            | Self::DimOneRegularExpression
            | Self::DimTwoRegularExpression
            | Self::DimThreeRegularExpression
            | Self::DimFourRegularExpression
            | Self::ContainsElRegularExpression
            | Self::DimOneContainsElRegularExpression
            | Self::DimTwoContainsElRegularExpression
            | Self::DimThreeContainsElRegularExpression
            | Self::DimFourContainsElRegularExpression
            | Self::AllElementsRegularExpression
            | Self::DimOneAllElementsRegularExpression
            | Self::DimTwoAllElementsRegularExpression
            | Self::DimThreeAllElementsRegularExpression
            | Self::DimFourAllElementsRegularExpression
            | Self::LengthGreaterThan
            | Self::DimOneLengthGreaterThan
            | Self::DimTwoLengthGreaterThan
            | Self::DimThreeLengthGreaterThan
            | Self::DimFourLengthGreaterThan => None,
        }
    }
    fn prefix_where_self_ucc(&self) -> Ts2 {
        let v = PgJsonTypeWhereSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Equal { .. } => &EqualUcc,
            Self::DimOneEqual { .. } => &DimOneEqualUcc,
            Self::DimTwoEqual { .. } => &DimTwoEqualUcc,
            Self::DimThreeEqual { .. } => &DimThreeEqualUcc,
            Self::DimFourEqual { .. } => &DimFourEqualUcc,
            Self::AllElementsEqual { .. } => &AllElementsEqualUcc,
            Self::DimOneAllElementsEqual { .. } => &DimOneAllElementsEqualUcc,
            Self::DimTwoAllElementsEqual { .. } => &DimTwoAllElementsEqualUcc,
            Self::DimThreeAllElementsEqual { .. } => &DimThreeAllElementsEqualUcc,
            Self::DimFourAllElementsEqual { .. } => &DimFourAllElementsEqualUcc,
            Self::LengthEqual => &LengthEqualUcc,
            Self::DimOneLengthEqual => &DimOneLengthEqualUcc,
            Self::DimTwoLengthEqual => &DimTwoLengthEqualUcc,
            Self::DimThreeLengthEqual => &DimThreeLengthEqualUcc,
            Self::DimFourLengthEqual => &DimFourLengthEqualUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimOneGreaterThan { .. } => &DimOneGreaterThanUcc,
            Self::DimTwoGreaterThan { .. } => &DimTwoGreaterThanUcc,
            Self::DimThreeGreaterThan { .. } => &DimThreeGreaterThanUcc,
            Self::DimFourGreaterThan { .. } => &DimFourGreaterThanUcc,
            Self::ContainsElGreaterThan { .. } => &ContainsElGreaterThanUcc,
            Self::DimOneContainsElGreaterThan { .. } => &DimOneContainsElGreaterThanUcc,
            Self::DimTwoContainsElGreaterThan { .. } => &DimTwoContainsElGreaterThanUcc,
            Self::DimThreeContainsElGreaterThan { .. } => &DimThreeContainsElGreaterThanUcc,
            Self::DimFourContainsElGreaterThan { .. } => &DimFourContainsElGreaterThanUcc,
            Self::AllElementsGreaterThan { .. } => &AllElementsGreaterThanUcc,
            Self::DimOneAllElementsGreaterThan { .. } => &DimOneAllElementsGreaterThanUcc,
            Self::DimTwoAllElementsGreaterThan { .. } => &DimTwoAllElementsGreaterThanUcc,
            Self::DimThreeAllElementsGreaterThan { .. } => &DimThreeAllElementsGreaterThanUcc,
            Self::DimFourAllElementsGreaterThan { .. } => &DimFourAllElementsGreaterThanUcc,
            Self::Between { .. } => &BetweenUcc,
            Self::DimOneBetween { .. } => &DimOneBetweenUcc,
            Self::DimTwoBetween { .. } => &DimTwoBetweenUcc,
            Self::DimThreeBetween { .. } => &DimThreeBetweenUcc,
            Self::DimFourBetween { .. } => &DimFourBetweenUcc,
            Self::In { .. } => &InUcc,
            Self::DimOneIn { .. } => &DimOneInUcc,
            Self::DimTwoIn { .. } => &DimTwoInUcc,
            Self::DimThreeIn { .. } => &DimThreeInUcc,
            Self::DimFourIn { .. } => &DimFourInUcc,
            Self::RegularExpression => &RegularExpressionUcc,
            Self::DimOneRegularExpression => &DimOneRegularExpressionUcc,
            Self::DimTwoRegularExpression => &DimTwoRegularExpressionUcc,
            Self::DimThreeRegularExpression => &DimThreeRegularExpressionUcc,
            Self::DimFourRegularExpression => &DimFourRegularExpressionUcc,
            Self::ContainsElRegularExpression => &ContainsElRegularExpressionUcc,
            Self::DimOneContainsElRegularExpression => &DimOneContainsElRegularExpressionUcc,
            Self::DimTwoContainsElRegularExpression => &DimTwoContainsElRegularExpressionUcc,
            Self::DimThreeContainsElRegularExpression => &DimThreeContainsElRegularExpressionUcc,
            Self::DimFourContainsElRegularExpression => &DimFourContainsElRegularExpressionUcc,
            Self::AllElementsRegularExpression => &AllElementsRegularExpressionUcc,
            Self::DimOneAllElementsRegularExpression => &DimOneAllElementsRegularExpressionUcc,
            Self::DimTwoAllElementsRegularExpression => &DimTwoAllElementsRegularExpressionUcc,
            Self::DimThreeAllElementsRegularExpression => &DimThreeAllElementsRegularExpressionUcc,
            Self::DimFourAllElementsRegularExpression => &DimFourAllElementsRegularExpressionUcc,
            Self::LengthGreaterThan => &LengthGreaterThanUcc,
            Self::DimOneLengthGreaterThan => &DimOneLengthGreaterThanUcc,
            Self::DimTwoLengthGreaterThan => &DimTwoLengthGreaterThanUcc,
            Self::DimThreeLengthGreaterThan => &DimThreeLengthGreaterThanUcc,
            Self::DimFourLengthGreaterThan => &DimFourLengthGreaterThanUcc,
            Self::ContainsAllElementsOfArray { .. } => &ContainsAllElementsOfArrayUcc,
            Self::DimOneContainsAllElementsOfArray { .. } => &DimOneContainsAllElementsOfArrayUcc,
            Self::DimTwoContainsAllElementsOfArray { .. } => &DimTwoContainsAllElementsOfArrayUcc,
            Self::DimThreeContainsAllElementsOfArray { .. } => {
                &DimThreeContainsAllElementsOfArrayUcc
            }
            Self::DimFourContainsAllElementsOfArray { .. } => &DimFourContainsAllElementsOfArrayUcc,
            Self::OverlapsWithArray { .. } => &OverlapsWithArrayUcc,
            Self::DimOneOverlapsWithArray { .. } => &DimOneOverlapsWithArrayUcc,
            Self::DimTwoOverlapsWithArray { .. } => &DimTwoOverlapsWithArrayUcc,
            Self::DimThreeOverlapsWithArray { .. } => &DimThreeOverlapsWithArrayUcc,
            Self::DimFourOverlapsWithArray { .. } => &DimFourOverlapsWithArrayUcc,
        }
    }
}
pub trait PgFilter {
    fn maybe_generic(&self) -> Option<Ts2>;
    fn prefix_where_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
