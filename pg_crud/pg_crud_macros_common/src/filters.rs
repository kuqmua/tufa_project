use enum_extension_lib::EnumExtension;
use naming::{
    AdjacentWithRangeUcc, AllElsEqualUcc, AllElsGreaterThanUcc, AllElsRegexUcc, BeforeUcc,
    BetweenUcc, ContainsAllElsOfArrUcc, ContainsElGreaterThanUcc, ContainsElRegexUcc,
    CurrentDateUcc, CurrentTimeUcc, CurrentTimestampUcc, DimFourAllElsEqualUcc,
    DimFourAllElsGreaterThanUcc, DimFourAllElsRegexUcc, DimFourBetweenUcc,
    DimFourContainsAllElsOfArrUcc, DimFourContainsElGreaterThanUcc, DimFourContainsElRegexUcc,
    DimFourEqualUcc, DimFourGreaterThanUcc, DimFourInUcc, DimFourLengthEqualUcc,
    DimFourLengthGreaterThanUcc, DimFourOverlapsWithArrUcc, DimFourRegexUcc,
    DimOneAdjacentWithRangeUcc, DimOneAllElsEqualUcc, DimOneAllElsGreaterThanUcc,
    DimOneAllElsRegexUcc, DimOneBeforeUcc, DimOneBetweenUcc, DimOneContainsAllElsOfArrUcc,
    DimOneContainsElGreaterThanUcc, DimOneContainsElRegexUcc, DimOneCurrentDateUcc,
    DimOneCurrentTimeUcc, DimOneCurrentTimestampUcc, DimOneEqualToEncodedStringRepresentationUcc,
    DimOneEqualUcc, DimOneExcludedUpperBoundUcc, DimOneFindRangesThatFullyContainTheGivenRangeUcc,
    DimOneFindRangesWithinGivenRangeUcc, DimOneGreaterThanCurrentDateUcc,
    DimOneGreaterThanCurrentTimeUcc, DimOneGreaterThanCurrentTimestampUcc,
    DimOneGreaterThanExcludedUpperBoundUcc, DimOneGreaterThanIncludedLowerBoundUcc,
    DimOneGreaterThanUcc, DimOneInUcc, DimOneIncludedLowerBoundUcc, DimOneLengthEqualUcc,
    DimOneLengthGreaterThanUcc, DimOneOverlapWithRangeUcc, DimOneOverlapsWithArrUcc,
    DimOneRangeLengthUcc, DimOneRegexUcc, DimOneStrictlyToLeftOfRangeUcc,
    DimOneStrictlyToRightOfRangeUcc, DimThreeAllElsEqualUcc, DimThreeAllElsGreaterThanUcc,
    DimThreeAllElsRegexUcc, DimThreeBetweenUcc, DimThreeContainsAllElsOfArrUcc,
    DimThreeContainsElGreaterThanUcc, DimThreeContainsElRegexUcc, DimThreeEqualUcc,
    DimThreeGreaterThanUcc, DimThreeInUcc, DimThreeLengthEqualUcc, DimThreeLengthGreaterThanUcc,
    DimThreeOverlapsWithArrUcc, DimThreeRegexUcc, DimTwoAllElsEqualUcc, DimTwoAllElsGreaterThanUcc,
    DimTwoAllElsRegexUcc, DimTwoBetweenUcc, DimTwoContainsAllElsOfArrUcc,
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
    AllElsEqual { ident: Ts2 },
    DimOneAllElsEqual { ident: Ts2 },
    DimTwoAllElsEqual { ident: Ts2 },
    DimThreeAllElsEqual { ident: Ts2 },
    DimFourAllElsEqual { ident: Ts2 },
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
    AllElsGreaterThan { ident: Ts2 },
    DimOneAllElsGreaterThan { ident: Ts2 },
    DimTwoAllElsGreaterThan { ident: Ts2 },
    DimThreeAllElsGreaterThan { ident: Ts2 },
    DimFourAllElsGreaterThan { ident: Ts2 },
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
    AllElsRegex,
    DimOneAllElsRegex,
    DimTwoAllElsRegex,
    DimThreeAllElsRegex,
    DimFourAllElsRegex,
    ContainsAllElsOfArr { ident: Ts2 },
    DimOneContainsAllElsOfArr { ident: Ts2 },
    DimTwoContainsAllElsOfArr { ident: Ts2 },
    DimThreeContainsAllElsOfArr { ident: Ts2 },
    DimFourContainsAllElsOfArr { ident: Ts2 },
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
            | Self::AllElsEqual { ident }
            | Self::DimOneAllElsEqual { ident }
            | Self::DimTwoAllElsEqual { ident }
            | Self::DimThreeAllElsEqual { ident }
            | Self::DimFourAllElsEqual { ident }
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
            | Self::AllElsGreaterThan { ident }
            | Self::DimOneAllElsGreaterThan { ident }
            | Self::DimTwoAllElsGreaterThan { ident }
            | Self::DimThreeAllElsGreaterThan { ident }
            | Self::DimFourAllElsGreaterThan { ident }
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
            | Self::ContainsAllElsOfArr { ident }
            | Self::DimOneContainsAllElsOfArr { ident }
            | Self::DimTwoContainsAllElsOfArr { ident }
            | Self::DimThreeContainsAllElsOfArr { ident }
            | Self::DimFourContainsAllElsOfArr { ident }
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
            | Self::AllElsRegex
            | Self::DimOneAllElsRegex
            | Self::DimTwoAllElsRegex
            | Self::DimThreeAllElsRegex
            | Self::DimFourAllElsRegex
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
            Self::AllElsEqual { .. } => &AllElsEqualUcc,
            Self::DimOneAllElsEqual { .. } => &DimOneAllElsEqualUcc,
            Self::DimTwoAllElsEqual { .. } => &DimTwoAllElsEqualUcc,
            Self::DimThreeAllElsEqual { .. } => &DimThreeAllElsEqualUcc,
            Self::DimFourAllElsEqual { .. } => &DimFourAllElsEqualUcc,
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
            Self::AllElsGreaterThan { .. } => &AllElsGreaterThanUcc,
            Self::DimOneAllElsGreaterThan { .. } => &DimOneAllElsGreaterThanUcc,
            Self::DimTwoAllElsGreaterThan { .. } => &DimTwoAllElsGreaterThanUcc,
            Self::DimThreeAllElsGreaterThan { .. } => &DimThreeAllElsGreaterThanUcc,
            Self::DimFourAllElsGreaterThan { .. } => &DimFourAllElsGreaterThanUcc,
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
            Self::AllElsRegex => &AllElsRegexUcc,
            Self::DimOneAllElsRegex => &DimOneAllElsRegexUcc,
            Self::DimTwoAllElsRegex => &DimTwoAllElsRegexUcc,
            Self::DimThreeAllElsRegex => &DimThreeAllElsRegexUcc,
            Self::DimFourAllElsRegex => &DimFourAllElsRegexUcc,
            Self::LengthGreaterThan => &LengthGreaterThanUcc,
            Self::DimOneLengthGreaterThan => &DimOneLengthGreaterThanUcc,
            Self::DimTwoLengthGreaterThan => &DimTwoLengthGreaterThanUcc,
            Self::DimThreeLengthGreaterThan => &DimThreeLengthGreaterThanUcc,
            Self::DimFourLengthGreaterThan => &DimFourLengthGreaterThanUcc,
            Self::ContainsAllElsOfArr { .. } => &ContainsAllElsOfArrUcc,
            Self::DimOneContainsAllElsOfArr { .. } => &DimOneContainsAllElsOfArrUcc,
            Self::DimTwoContainsAllElsOfArr { .. } => &DimTwoContainsAllElsOfArrUcc,
            Self::DimThreeContainsAllElsOfArr { .. } => &DimThreeContainsAllElsOfArrUcc,
            Self::DimFourContainsAllElsOfArr { .. } => &DimFourContainsAllElsOfArrUcc,
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
