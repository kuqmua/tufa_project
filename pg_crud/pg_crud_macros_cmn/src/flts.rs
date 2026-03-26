use naming::prm::{PgJsonWhSelfUcc, PgTypeWhSelfUcc};
use naming::{
    AdjacentWithRangeUcc, AllElsEqUcc, AllElsGreaterThanUcc, AllElsRgxUcc, BeforeUcc, BtwnUcc,
    ContainsAllElsOfArrUcc, ContainsElGreaterThanUcc, ContainsElRgxUcc, CrntDateUcc, CrntTimeUcc,
    CrntTimestampUcc, DimFourAllElsEqUcc, DimFourAllElsGreaterThanUcc, DimFourAllElsRgxUcc,
    DimFourBtwnUcc, DimFourContainsAllElsOfArrUcc, DimFourContainsElGreaterThanUcc,
    DimFourContainsElRgxUcc, DimFourEqUcc, DimFourGreaterThanUcc, DimFourInUcc, DimFourLenEqUcc,
    DimFourLenGreaterThanUcc, DimFourOverlapsWithArrUcc, DimFourRgxUcc, DimOneAdjacentWithRangeUcc,
    DimOneAllElsEqUcc, DimOneAllElsGreaterThanUcc, DimOneAllElsRgxUcc, DimOneBeforeUcc,
    DimOneBtwnUcc, DimOneContainsAllElsOfArrUcc, DimOneContainsElGreaterThanUcc,
    DimOneContainsElRgxUcc, DimOneCrntDateUcc, DimOneCrntTimeUcc, DimOneCrntTimestampUcc,
    DimOneEqToEncodedStringRepresentationUcc, DimOneEqUcc, DimOneExcludedUpperBoundUcc,
    DimOneFindRangesThatFullyContainTheGivenRangeUcc, DimOneFindRangesWithinGivenRangeUcc,
    DimOneGreaterThanCrntDateUcc, DimOneGreaterThanCrntTimeUcc, DimOneGreaterThanCrntTimestampUcc,
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
    GreaterThanCrntDateUcc, GreaterThanCrntTimeUcc, GreaterThanCrntTimestampUcc,
    GreaterThanExcludedUpperBoundUcc, GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc,
    IncludedLowerBoundUcc, LenEqUcc, LenGreaterThanUcc, OverlapWithRangeUcc, OverlapsWithArrUcc,
    RangeLenUcc, RgxUcc, StrictlyToLeftOfRangeUcc, StrictlyToRightOfRangeUcc,
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use strum_macros::{Display, EnumIter};
macro_rules! pg_json_flt_dim {
    ($fn_name:ident(dim: usize, ident: Ts2), [$b:ident, $d1:ident, $d2:ident, $d3:ident, $d4:ident], $uuid:literal) => {
        #[must_use]
        pub fn $fn_name(dim: usize, ident: Ts2) -> Self {
            match dim {
                0 => Self::$b { ident },
                1 => Self::$d1 { ident },
                2 => Self::$d2 { ident },
                3 => Self::$d3 { ident },
                4 => Self::$d4 { ident },
                _ => panic!(concat!($uuid, " unsupported dim")),
            }
        }
    };
    ($fn_name:ident(dim: usize), [$b:ident, $d1:ident, $d2:ident, $d3:ident, $d4:ident], $uuid:literal) => {
        #[must_use]
        pub fn $fn_name(dim: usize) -> Self {
            match dim {
                0 => Self::$b,
                1 => Self::$d1,
                2 => Self::$d2,
                3 => Self::$d3,
                4 => Self::$d4,
                _ => panic!(concat!($uuid, " unsupported dim")),
            }
        }
    };
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, Optml)]
pub enum PgTypeFlt {
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
    CrntDate,
    DimOneCrntDate,
    GreaterThanCrntDate,
    DimOneGreaterThanCrntDate,
    CrntTimestamp,
    DimOneCrntTimestamp,
    GreaterThanCrntTimestamp,
    DimOneGreaterThanCrntTimestamp,
    CrntTime,
    DimOneCrntTime,
    GreaterThanCrntTime,
    DimOneGreaterThanCrntTime,
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
impl PgFlt for PgTypeFlt {
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
            | Self::CrntDate
            | Self::DimOneCrntDate
            | Self::GreaterThanCrntDate
            | Self::DimOneGreaterThanCrntDate
            | Self::CrntTimestamp
            | Self::DimOneCrntTimestamp
            | Self::GreaterThanCrntTimestamp
            | Self::DimOneGreaterThanCrntTimestamp
            | Self::CrntTime
            | Self::DimOneCrntTime
            | Self::GreaterThanCrntTime
            | Self::DimOneGreaterThanCrntTime
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
            Self::CrntDate => &CrntDateUcc,
            Self::DimOneCrntDate => &DimOneCrntDateUcc,
            Self::GreaterThanCrntDate => &GreaterThanCrntDateUcc,
            Self::DimOneGreaterThanCrntDate => &DimOneGreaterThanCrntDateUcc,
            Self::CrntTimestamp => &CrntTimestampUcc,
            Self::DimOneCrntTimestamp => &DimOneCrntTimestampUcc,
            Self::GreaterThanCrntTimestamp => &GreaterThanCrntTimestampUcc,
            Self::DimOneGreaterThanCrntTimestamp => &DimOneGreaterThanCrntTimestampUcc,
            Self::CrntTime => &CrntTimeUcc,
            Self::DimOneCrntTime => &DimOneCrntTimeUcc,
            Self::GreaterThanCrntTime => &GreaterThanCrntTimeUcc,
            Self::DimOneGreaterThanCrntTime => &DimOneGreaterThanCrntTimeUcc,
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
#[derive(Debug, Clone, Display, EnumIter, Optml)]
pub enum PgJsonFlt {
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
impl PgJsonFlt {
    pg_json_flt_dim!(dim_all_els_eq(dim: usize, ident: Ts2), [AllElsEq, DimOneAllElsEq, DimTwoAllElsEq, DimThreeAllElsEq, DimFourAllElsEq], "a1b2c3d4");
    pg_json_flt_dim!(dim_all_els_greater_than(dim: usize, ident: Ts2), [AllElsGreaterThan, DimOneAllElsGreaterThan, DimTwoAllElsGreaterThan, DimThreeAllElsGreaterThan, DimFourAllElsGreaterThan], "e5f6a7b8");
    pg_json_flt_dim!(dim_all_els_rgx(dim: usize), [AllElsRgx, DimOneAllElsRgx, DimTwoAllElsRgx, DimThreeAllElsRgx, DimFourAllElsRgx], "c9d0e1f2");
    pg_json_flt_dim!(dim_btwn(dim: usize, ident: Ts2), [Btwn, DimOneBtwn, DimTwoBtwn, DimThreeBtwn, DimFourBtwn], "a3b4c5d6");
    pg_json_flt_dim!(dim_contains_all_els_of_arr(dim: usize, ident: Ts2), [ContainsAllElsOfArr, DimOneContainsAllElsOfArr, DimTwoContainsAllElsOfArr, DimThreeContainsAllElsOfArr, DimFourContainsAllElsOfArr], "e7f8a9b0");
    pg_json_flt_dim!(dim_contains_el_greater_than(dim: usize, ident: Ts2), [ContainsElGreaterThan, DimOneContainsElGreaterThan, DimTwoContainsElGreaterThan, DimThreeContainsElGreaterThan, DimFourContainsElGreaterThan], "c1d2e3f4");
    pg_json_flt_dim!(dim_contains_el_rgx(dim: usize), [ContainsElRgx, DimOneContainsElRgx, DimTwoContainsElRgx, DimThreeContainsElRgx, DimFourContainsElRgx], "a5b6c7d8");
    pg_json_flt_dim!(dim_eq(dim: usize, ident: Ts2), [Eq, DimOneEq, DimTwoEq, DimThreeEq, DimFourEq], "e9f0a1b2");
    pg_json_flt_dim!(dim_greater_than(dim: usize, ident: Ts2), [GreaterThan, DimOneGreaterThan, DimTwoGreaterThan, DimThreeGreaterThan, DimFourGreaterThan], "c3d4e5f6");
    pg_json_flt_dim!(dim_in(dim: usize, ident: Ts2), [In, DimOneIn, DimTwoIn, DimThreeIn, DimFourIn], "a7b8c9d0");
    pg_json_flt_dim!(dim_len_eq(dim: usize), [LenEq, DimOneLenEq, DimTwoLenEq, DimThreeLenEq, DimFourLenEq], "e1f2a3b4");
    pg_json_flt_dim!(dim_len_greater_than(dim: usize), [LenGreaterThan, DimOneLenGreaterThan, DimTwoLenGreaterThan, DimThreeLenGreaterThan, DimFourLenGreaterThan], "c5d6e7f8");
    pg_json_flt_dim!(dim_overlaps_with_arr(dim: usize, ident: Ts2), [OverlapsWithArr, DimOneOverlapsWithArr, DimTwoOverlapsWithArr, DimThreeOverlapsWithArr, DimFourOverlapsWithArr], "a9b0c1d2");
    pg_json_flt_dim!(dim_rgx(dim: usize), [Rgx, DimOneRgx, DimTwoRgx, DimThreeRgx, DimFourRgx], "e3f4a5b6");
}
impl PgFlt for PgJsonFlt {
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
pub trait PgFlt {
    fn mb_generic(&self) -> Option<Ts2>;
    fn prefix_wh_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
