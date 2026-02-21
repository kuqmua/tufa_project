use enum_extension_lib::EnumExtension;
use naming::{
    AdjacentWithRangeUcc, AllElementsEqualUcc, AllElementsGreaterThanUcc, AllElementsRegexUcc,
    BeforeUcc, BetweenUcc, ContainsAllElementsOfArrUcc, ContainsElGreaterThanUcc,
    ContainsElRegexUcc, CurrentDateUcc, CurrentTimeUcc, CurrentTimestampUcc,
    DimFourAllElementsEqualUcc, DimFourAllElementsGreaterThanUcc, DimFourAllElementsRegexUcc,
    DimFourBetweenUcc, DimFourContainsAllElementsOfArrUcc, DimFourContainsElGreaterThanUcc,
    DimFourContainsElRegexUcc, DimFourEqualUcc, DimFourGreaterThanUcc, DimFourInUcc,
    DimFourLengthEqualUcc, DimFourLengthGreaterThanUcc, DimFourOverlapsWithArrUcc, DimFourRegexUcc,
    DimOneAdjacentWithRangeUcc, DimOneAllElementsEqualUcc, DimOneAllElementsGreaterThanUcc,
    DimOneAllElementsRegexUcc, DimOneBeforeUcc, DimOneBetweenUcc,
    DimOneContainsAllElementsOfArrUcc, DimOneContainsElGreaterThanUcc, DimOneContainsElRegexUcc,
    DimOneCurrentDateUcc, DimOneCurrentTimeUcc, DimOneCurrentTimestampUcc,
    DimOneEqualToEncodedStringRepresentationUcc, DimOneEqualUcc, DimOneExcludedUpperBoundUcc,
    DimOneFindRangesThatFullyContainTheGivenRangeUcc, DimOneFindRangesWithinGivenRangeUcc,
    DimOneGreaterThanCurrentDateUcc, DimOneGreaterThanCurrentTimeUcc,
    DimOneGreaterThanCurrentTimestampUcc, DimOneGreaterThanExcludedUpperBoundUcc,
    DimOneGreaterThanIncludedLowerBoundUcc, DimOneGreaterThanUcc, DimOneInUcc,
    DimOneIncludedLowerBoundUcc, DimOneLengthEqualUcc, DimOneLengthGreaterThanUcc,
    DimOneOverlapWithRangeUcc, DimOneOverlapsWithArrUcc, DimOneRangeLengthUcc, DimOneRegexUcc,
    DimOneStrictlyToLeftOfRangeUcc, DimOneStrictlyToRightOfRangeUcc, DimThreeAllElementsEqualUcc,
    DimThreeAllElementsGreaterThanUcc, DimThreeAllElementsRegexUcc, DimThreeBetweenUcc,
    DimThreeContainsAllElementsOfArrUcc, DimThreeContainsElGreaterThanUcc,
    DimThreeContainsElRegexUcc, DimThreeEqualUcc, DimThreeGreaterThanUcc, DimThreeInUcc,
    DimThreeLengthEqualUcc, DimThreeLengthGreaterThanUcc, DimThreeOverlapsWithArrUcc,
    DimThreeRegexUcc, DimTwoAllElementsEqualUcc, DimTwoAllElementsGreaterThanUcc,
    DimTwoAllElementsRegexUcc, DimTwoBetweenUcc, DimTwoContainsAllElementsOfArrUcc,
    DimTwoContainsElGreaterThanUcc, DimTwoContainsElRegexUcc, DimTwoEqualUcc, DimTwoGreaterThanUcc,
    DimTwoInUcc, DimTwoLengthEqualUcc, DimTwoLengthGreaterThanUcc, DimTwoOverlapsWithArrUcc,
    DimTwoRegexUcc, DisplayPlusToTokens, EqualToEncodedStringRepresentationUcc, EqualUcc,
    ExcludedUpperBoundUcc, FindRangesThatFullyContainTheGivenRangeUcc,
    FindRangesWithinGivenRangeUcc, GreaterThanCurrentDateUcc, GreaterThanCurrentTimeUcc,
    GreaterThanCurrentTimestampUcc, GreaterThanExcludedUpperBoundUcc,
    GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc, IncludedLowerBoundUcc, LengthEqualUcc,
    LengthGreaterThanUcc, OverlapWithRangeUcc, OverlapsWithArrUcc, RangeLengthUcc, RegexUcc,
    StrictlyToLeftOfRangeUcc, StrictlyToRightOfRangeUcc,
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
    Regex,
    DimOneRegex,
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
            Self::Regex
            | Self::DimOneRegex
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
            Self::Regex => &RegexUcc,
            Self::DimOneRegex => &DimOneRegexUcc,
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
    Regex,
    DimOneRegex,
    DimTwoRegex,
    DimThreeRegex,
    DimFourRegex,
    ContainsElRegex,
    DimOneContainsElRegex,
    DimTwoContainsElRegex,
    DimThreeContainsElRegex,
    DimFourContainsElRegex,
    AllElementsRegex,
    DimOneAllElementsRegex,
    DimTwoAllElementsRegex,
    DimThreeAllElementsRegex,
    DimFourAllElementsRegex,
    ContainsAllElementsOfArr { ident: Ts2 },
    DimOneContainsAllElementsOfArr { ident: Ts2 },
    DimTwoContainsAllElementsOfArr { ident: Ts2 },
    DimThreeContainsAllElementsOfArr { ident: Ts2 },
    DimFourContainsAllElementsOfArr { ident: Ts2 },
    // ContainedInArr,
    OverlapsWithArr { ident: Ts2 },
    DimOneOverlapsWithArr { ident: Ts2 },
    DimTwoOverlapsWithArr { ident: Ts2 },
    DimThreeOverlapsWithArr { ident: Ts2 },
    DimFourOverlapsWithArr { ident: Ts2 },
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
            | Self::ContainsAllElementsOfArr { ident }
            | Self::DimOneContainsAllElementsOfArr { ident }
            | Self::DimTwoContainsAllElementsOfArr { ident }
            | Self::DimThreeContainsAllElementsOfArr { ident }
            | Self::DimFourContainsAllElementsOfArr { ident }
            | Self::OverlapsWithArr { ident }
            | Self::DimOneOverlapsWithArr { ident }
            | Self::DimTwoOverlapsWithArr { ident }
            | Self::DimThreeOverlapsWithArr { ident }
            | Self::DimFourOverlapsWithArr { ident } => Some(ident.clone()),
            Self::LengthEqual
            | Self::DimOneLengthEqual
            | Self::DimTwoLengthEqual
            | Self::DimThreeLengthEqual
            | Self::DimFourLengthEqual
            | Self::Regex
            | Self::DimOneRegex
            | Self::DimTwoRegex
            | Self::DimThreeRegex
            | Self::DimFourRegex
            | Self::ContainsElRegex
            | Self::DimOneContainsElRegex
            | Self::DimTwoContainsElRegex
            | Self::DimThreeContainsElRegex
            | Self::DimFourContainsElRegex
            | Self::AllElementsRegex
            | Self::DimOneAllElementsRegex
            | Self::DimTwoAllElementsRegex
            | Self::DimThreeAllElementsRegex
            | Self::DimFourAllElementsRegex
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
            Self::Regex => &RegexUcc,
            Self::DimOneRegex => &DimOneRegexUcc,
            Self::DimTwoRegex => &DimTwoRegexUcc,
            Self::DimThreeRegex => &DimThreeRegexUcc,
            Self::DimFourRegex => &DimFourRegexUcc,
            Self::ContainsElRegex => &ContainsElRegexUcc,
            Self::DimOneContainsElRegex => &DimOneContainsElRegexUcc,
            Self::DimTwoContainsElRegex => &DimTwoContainsElRegexUcc,
            Self::DimThreeContainsElRegex => &DimThreeContainsElRegexUcc,
            Self::DimFourContainsElRegex => &DimFourContainsElRegexUcc,
            Self::AllElementsRegex => &AllElementsRegexUcc,
            Self::DimOneAllElementsRegex => &DimOneAllElementsRegexUcc,
            Self::DimTwoAllElementsRegex => &DimTwoAllElementsRegexUcc,
            Self::DimThreeAllElementsRegex => &DimThreeAllElementsRegexUcc,
            Self::DimFourAllElementsRegex => &DimFourAllElementsRegexUcc,
            Self::LengthGreaterThan => &LengthGreaterThanUcc,
            Self::DimOneLengthGreaterThan => &DimOneLengthGreaterThanUcc,
            Self::DimTwoLengthGreaterThan => &DimTwoLengthGreaterThanUcc,
            Self::DimThreeLengthGreaterThan => &DimThreeLengthGreaterThanUcc,
            Self::DimFourLengthGreaterThan => &DimFourLengthGreaterThanUcc,
            Self::ContainsAllElementsOfArr { .. } => &ContainsAllElementsOfArrUcc,
            Self::DimOneContainsAllElementsOfArr { .. } => &DimOneContainsAllElementsOfArrUcc,
            Self::DimTwoContainsAllElementsOfArr { .. } => &DimTwoContainsAllElementsOfArrUcc,
            Self::DimThreeContainsAllElementsOfArr { .. } => &DimThreeContainsAllElementsOfArrUcc,
            Self::DimFourContainsAllElementsOfArr { .. } => &DimFourContainsAllElementsOfArrUcc,
            Self::OverlapsWithArr { .. } => &OverlapsWithArrUcc,
            Self::DimOneOverlapsWithArr { .. } => &DimOneOverlapsWithArrUcc,
            Self::DimTwoOverlapsWithArr { .. } => &DimTwoOverlapsWithArrUcc,
            Self::DimThreeOverlapsWithArr { .. } => &DimThreeOverlapsWithArrUcc,
            Self::DimFourOverlapsWithArr { .. } => &DimFourOverlapsWithArrUcc,
        }
    }
}
pub trait PgFilter {
    fn maybe_generic(&self) -> Option<Ts2>;
    fn prefix_where_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
