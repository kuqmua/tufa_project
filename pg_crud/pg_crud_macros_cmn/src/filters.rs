use enum_extension_lib::EnumExtension;
use naming::{
    AdjacentWithRangeUcc, AllElsEqUcc, AllElsGreaterThanUcc, AllElsRgxUcc, BeforeUcc, BtwnUcc,
    ContainsAllElsOfArrUcc, ContainsElGreaterThanUcc, ContainsElRgxUcc, CurrentDateUcc,
    CurrentTimeUcc, CurrentTimestampUcc, DimFourAllElsEqUcc, DimFourAllElsGreaterThanUcc,
    DimFourAllElsRgxUcc, DimFourBtwnUcc, DimFourContainsAllElsOfArrUcc,
    DimFourContainsElGreaterThanUcc, DimFourContainsElRgxUcc, DimFourEqUcc, DimFourGreaterThanUcc,
    DimFourInUcc, DimFourLenEqUcc, DimFourLenGreaterThanUcc, DimFourOverlapsWithArrUcc,
    DimFourRgxUcc, DimOneAdjacentWithRangeUcc, DimOneAllElsEqUcc, DimOneAllElsGreaterThanUcc,
    DimOneAllElsRgxUcc, DimOneBeforeUcc, DimOneBtwnUcc, DimOneContainsAllElsOfArrUcc,
    DimOneContainsElGreaterThanUcc, DimOneContainsElRgxUcc, DimOneCurrentDateUcc,
    DimOneCurrentTimeUcc, DimOneCurrentTimestampUcc, DimOneEqToEncodedStringRepresentationUcc,
    DimOneEqUcc, DimOneExcludedUpperBoundUcc, DimOneFindRangesThatFullyContainTheGivenRangeUcc,
    DimOneFindRangesWithinGivenRangeUcc, DimOneGreaterThanCurrentDateUcc,
    DimOneGreaterThanCurrentTimeUcc, DimOneGreaterThanCurrentTimestampUcc,
    DimOneGreaterThanExcludedUpperBoundUcc, DimOneGreaterThanIncludedLowerBoundUcc,
    DimOneGreaterThanUcc, DimOneInUcc, DimOneIncludedLowerBoundUcc, DimOneLenEqUcc,
    DimOneLenGreaterThanUcc, DimOneOverlapWithRangeUcc, DimOneOverlapsWithArrUcc,
    DimOneRangeLenUcc, DimOneRgxUcc, DimOneStrictlyToLeftOfRangeUcc,
    DimOneStrictlyToRightOfRangeUcc, DimThreeAllElsEqUcc, DimThreeAllElsGreaterThanUcc,
    DimThreeAllElsRgxUcc, DimThreeBtwnUcc, DimThreeContainsAllElsOfArrUcc,
    DimThreeContainsElGreaterThanUcc, DimThreeContainsElRgxUcc, DimThreeEqUcc,
    DimThreeGreaterThanUcc, DimThreeInUcc, DimThreeLenEqUcc, DimThreeLenGreaterThanUcc,
    DimThreeOverlapsWithArrUcc, DimThreeRgxUcc, DimTwoAllElsEqUcc, DimTwoAllElsGreaterThanUcc,
    DimTwoAllElsRgxUcc, DimTwoBtwnUcc, DimTwoContainsAllElsOfArrUcc,
    DimTwoContainsElGreaterThanUcc, DimTwoContainsElRgxUcc, DimTwoEqUcc, DimTwoGreaterThanUcc,
    DimTwoInUcc, DimTwoLenEqUcc, DimTwoLenGreaterThanUcc, DimTwoOverlapsWithArrUcc, DimTwoRgxUcc,
    DisplayPlusToTokens, EqToEncodedStringRepresentationUcc, EqUcc, ExcludedUpperBoundUcc,
    FindRangesThatFullyContainTheGivenRangeUcc, FindRangesWithinGivenRangeUcc,
    GreaterThanCurrentDateUcc, GreaterThanCurrentTimeUcc, GreaterThanCurrentTimestampUcc,
    GreaterThanExcludedUpperBoundUcc, GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc,
    IncludedLowerBoundUcc, LenEqUcc, LenGreaterThanUcc, OverlapWithRangeUcc, OverlapsWithArrUcc,
    RangeLenUcc, RgxUcc, StrictlyToLeftOfRangeUcc, StrictlyToRightOfRangeUcc,
    prm::{PgJsonWhSelfUcc, PgTypeWhSelfUcc},
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use strum_macros::{Display, EnumIter};
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, EnumExtension, Optml)]
pub enum PgTypeFilter {
    Eq { ident: Ts2 },
    DimOneEq { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    DimOneGreaterThan { ident: Ts2 },
    Btwn { ident: Ts2 },
    DimOneBtwn { ident: Ts2 },
    In { ident: Ts2 },
    DimOneIn { ident: Ts2 },
    Rgx,
    DimOneRgx,
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
    DimOneLenEq,
    DimOneLenGreaterThan,
    EqToEncodedStringRepresentation,
    DimOneEqToEncodedStringRepresentation,
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
    RangeLen,
    DimOneRangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFilter for PgTypeFilter {
    fn mb_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Eq { ident }
            | Self::DimOneEq { ident }
            | Self::GreaterThan { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::Btwn { ident }
            | Self::DimOneBtwn { ident }
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
            Self::Rgx
            | Self::DimOneRgx
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
            | Self::DimOneLenEq
            | Self::DimOneLenGreaterThan
            | Self::EqToEncodedStringRepresentation
            | Self::DimOneEqToEncodedStringRepresentation
            | Self::RangeLen
            | Self::DimOneRangeLen => None,
        }
    }
    fn prefix_wh_self_ucc(&self) -> Ts2 {
        let v = PgTypeWhSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &EqUcc,
            Self::DimOneEq { .. } => &DimOneEqUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimOneGreaterThan { .. } => &DimOneGreaterThanUcc,
            Self::Btwn { .. } => &BtwnUcc,
            Self::DimOneBtwn { .. } => &DimOneBtwnUcc,
            Self::In { .. } => &InUcc,
            Self::DimOneIn { .. } => &DimOneInUcc,
            Self::Rgx => &RgxUcc,
            Self::DimOneRgx => &DimOneRgxUcc,
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
            Self::DimOneLenEq => &DimOneLenEqUcc,
            Self::DimOneLenGreaterThan => &DimOneLenGreaterThanUcc,
            Self::EqToEncodedStringRepresentation => &EqToEncodedStringRepresentationUcc,
            Self::DimOneEqToEncodedStringRepresentation => {
                &DimOneEqToEncodedStringRepresentationUcc
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
            Self::RangeLen => &RangeLenUcc,
            Self::DimOneRangeLen => &DimOneRangeLenUcc,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, EnumExtension, Optml)]
pub enum PgJsonFilter {
    Eq { ident: Ts2 },
    DimOneEq { ident: Ts2 },
    DimTwoEq { ident: Ts2 },
    DimThreeEq { ident: Ts2 },
    DimFourEq { ident: Ts2 },
    AllElsEq { ident: Ts2 },
    DimOneAllElsEq { ident: Ts2 },
    DimTwoAllElsEq { ident: Ts2 },
    DimThreeAllElsEq { ident: Ts2 },
    DimFourAllElsEq { ident: Ts2 },
    LenEq,
    DimOneLenEq,
    DimTwoLenEq,
    DimThreeLenEq,
    DimFourLenEq,
    LenGreaterThan,
    DimOneLenGreaterThan,
    DimTwoLenGreaterThan,
    DimThreeLenGreaterThan,
    DimFourLenGreaterThan,
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
    Btwn { ident: Ts2 },
    DimOneBtwn { ident: Ts2 },
    DimTwoBtwn { ident: Ts2 },
    DimThreeBtwn { ident: Ts2 },
    DimFourBtwn { ident: Ts2 },
    In { ident: Ts2 },
    DimOneIn { ident: Ts2 },
    DimTwoIn { ident: Ts2 },
    DimThreeIn { ident: Ts2 },
    DimFourIn { ident: Ts2 },
    Rgx,
    DimOneRgx,
    DimTwoRgx,
    DimThreeRgx,
    DimFourRgx,
    ContainsElRgx,
    DimOneContainsElRgx,
    DimTwoContainsElRgx,
    DimThreeContainsElRgx,
    DimFourContainsElRgx,
    AllElsRgx,
    DimOneAllElsRgx,
    DimTwoAllElsRgx,
    DimThreeAllElsRgx,
    DimFourAllElsRgx,
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
impl PgFilter for PgJsonFilter {
    fn mb_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Eq { ident }
            | Self::DimOneEq { ident }
            | Self::DimTwoEq { ident }
            | Self::DimThreeEq { ident }
            | Self::DimFourEq { ident }
            | Self::AllElsEq { ident }
            | Self::DimOneAllElsEq { ident }
            | Self::DimTwoAllElsEq { ident }
            | Self::DimThreeAllElsEq { ident }
            | Self::DimFourAllElsEq { ident }
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
            | Self::Btwn { ident }
            | Self::DimOneBtwn { ident }
            | Self::DimTwoBtwn { ident }
            | Self::DimThreeBtwn { ident }
            | Self::DimFourBtwn { ident }
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
            Self::LenEq
            | Self::DimOneLenEq
            | Self::DimTwoLenEq
            | Self::DimThreeLenEq
            | Self::DimFourLenEq
            | Self::Rgx
            | Self::DimOneRgx
            | Self::DimTwoRgx
            | Self::DimThreeRgx
            | Self::DimFourRgx
            | Self::ContainsElRgx
            | Self::DimOneContainsElRgx
            | Self::DimTwoContainsElRgx
            | Self::DimThreeContainsElRgx
            | Self::DimFourContainsElRgx
            | Self::AllElsRgx
            | Self::DimOneAllElsRgx
            | Self::DimTwoAllElsRgx
            | Self::DimThreeAllElsRgx
            | Self::DimFourAllElsRgx
            | Self::LenGreaterThan
            | Self::DimOneLenGreaterThan
            | Self::DimTwoLenGreaterThan
            | Self::DimThreeLenGreaterThan
            | Self::DimFourLenGreaterThan => None,
        }
    }
    fn prefix_wh_self_ucc(&self) -> Ts2 {
        let v = PgJsonWhSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &EqUcc,
            Self::DimOneEq { .. } => &DimOneEqUcc,
            Self::DimTwoEq { .. } => &DimTwoEqUcc,
            Self::DimThreeEq { .. } => &DimThreeEqUcc,
            Self::DimFourEq { .. } => &DimFourEqUcc,
            Self::AllElsEq { .. } => &AllElsEqUcc,
            Self::DimOneAllElsEq { .. } => &DimOneAllElsEqUcc,
            Self::DimTwoAllElsEq { .. } => &DimTwoAllElsEqUcc,
            Self::DimThreeAllElsEq { .. } => &DimThreeAllElsEqUcc,
            Self::DimFourAllElsEq { .. } => &DimFourAllElsEqUcc,
            Self::LenEq => &LenEqUcc,
            Self::DimOneLenEq => &DimOneLenEqUcc,
            Self::DimTwoLenEq => &DimTwoLenEqUcc,
            Self::DimThreeLenEq => &DimThreeLenEqUcc,
            Self::DimFourLenEq => &DimFourLenEqUcc,
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
            Self::Btwn { .. } => &BtwnUcc,
            Self::DimOneBtwn { .. } => &DimOneBtwnUcc,
            Self::DimTwoBtwn { .. } => &DimTwoBtwnUcc,
            Self::DimThreeBtwn { .. } => &DimThreeBtwnUcc,
            Self::DimFourBtwn { .. } => &DimFourBtwnUcc,
            Self::In { .. } => &InUcc,
            Self::DimOneIn { .. } => &DimOneInUcc,
            Self::DimTwoIn { .. } => &DimTwoInUcc,
            Self::DimThreeIn { .. } => &DimThreeInUcc,
            Self::DimFourIn { .. } => &DimFourInUcc,
            Self::Rgx => &RgxUcc,
            Self::DimOneRgx => &DimOneRgxUcc,
            Self::DimTwoRgx => &DimTwoRgxUcc,
            Self::DimThreeRgx => &DimThreeRgxUcc,
            Self::DimFourRgx => &DimFourRgxUcc,
            Self::ContainsElRgx => &ContainsElRgxUcc,
            Self::DimOneContainsElRgx => &DimOneContainsElRgxUcc,
            Self::DimTwoContainsElRgx => &DimTwoContainsElRgxUcc,
            Self::DimThreeContainsElRgx => &DimThreeContainsElRgxUcc,
            Self::DimFourContainsElRgx => &DimFourContainsElRgxUcc,
            Self::AllElsRgx => &AllElsRgxUcc,
            Self::DimOneAllElsRgx => &DimOneAllElsRgxUcc,
            Self::DimTwoAllElsRgx => &DimTwoAllElsRgxUcc,
            Self::DimThreeAllElsRgx => &DimThreeAllElsRgxUcc,
            Self::DimFourAllElsRgx => &DimFourAllElsRgxUcc,
            Self::LenGreaterThan => &LenGreaterThanUcc,
            Self::DimOneLenGreaterThan => &DimOneLenGreaterThanUcc,
            Self::DimTwoLenGreaterThan => &DimTwoLenGreaterThanUcc,
            Self::DimThreeLenGreaterThan => &DimThreeLenGreaterThanUcc,
            Self::DimFourLenGreaterThan => &DimFourLenGreaterThanUcc,
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
    fn mb_generic(&self) -> Option<Ts2>;
    fn prefix_wh_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
