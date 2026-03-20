use enum_extension_lib::EnumExtension;
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, EnumExtension, Optml)]
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
#[derive(Debug, Clone, Display, EnumIter, EnumExtension, Optml)]
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
    #[must_use]
    pub fn dim_all_els_eq(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::AllElsEq { ident },
            1 => Self::DimOneAllElsEq { ident },
            2 => Self::DimTwoAllElsEq { ident },
            3 => Self::DimThreeAllElsEq { ident },
            4 => Self::DimFourAllElsEq { ident },
            _ => panic!("a1b2c3d4 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_all_els_greater_than(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::AllElsGreaterThan { ident },
            1 => Self::DimOneAllElsGreaterThan { ident },
            2 => Self::DimTwoAllElsGreaterThan { ident },
            3 => Self::DimThreeAllElsGreaterThan { ident },
            4 => Self::DimFourAllElsGreaterThan { ident },
            _ => panic!("e5f6a7b8 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_all_els_rgx(dim: usize) -> Self {
        match dim {
            0 => Self::AllElsRgx,
            1 => Self::DimOneAllElsRgx,
            2 => Self::DimTwoAllElsRgx,
            3 => Self::DimThreeAllElsRgx,
            4 => Self::DimFourAllElsRgx,
            _ => panic!("c9d0e1f2 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_btwn(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::Btwn { ident },
            1 => Self::DimOneBtwn { ident },
            2 => Self::DimTwoBtwn { ident },
            3 => Self::DimThreeBtwn { ident },
            4 => Self::DimFourBtwn { ident },
            _ => panic!("a3b4c5d6 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_contains_all_els_of_arr(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::ContainsAllElsOfArr { ident },
            1 => Self::DimOneContainsAllElsOfArr { ident },
            2 => Self::DimTwoContainsAllElsOfArr { ident },
            3 => Self::DimThreeContainsAllElsOfArr { ident },
            4 => Self::DimFourContainsAllElsOfArr { ident },
            _ => panic!("e7f8a9b0 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_contains_el_greater_than(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::ContainsElGreaterThan { ident },
            1 => Self::DimOneContainsElGreaterThan { ident },
            2 => Self::DimTwoContainsElGreaterThan { ident },
            3 => Self::DimThreeContainsElGreaterThan { ident },
            4 => Self::DimFourContainsElGreaterThan { ident },
            _ => panic!("c1d2e3f4 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_contains_el_rgx(dim: usize) -> Self {
        match dim {
            0 => Self::ContainsElRgx,
            1 => Self::DimOneContainsElRgx,
            2 => Self::DimTwoContainsElRgx,
            3 => Self::DimThreeContainsElRgx,
            4 => Self::DimFourContainsElRgx,
            _ => panic!("a5b6c7d8 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_eq(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::Eq { ident },
            1 => Self::DimOneEq { ident },
            2 => Self::DimTwoEq { ident },
            3 => Self::DimThreeEq { ident },
            4 => Self::DimFourEq { ident },
            _ => panic!("e9f0a1b2 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_greater_than(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::GreaterThan { ident },
            1 => Self::DimOneGreaterThan { ident },
            2 => Self::DimTwoGreaterThan { ident },
            3 => Self::DimThreeGreaterThan { ident },
            4 => Self::DimFourGreaterThan { ident },
            _ => panic!("c3d4e5f6 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_in(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::In { ident },
            1 => Self::DimOneIn { ident },
            2 => Self::DimTwoIn { ident },
            3 => Self::DimThreeIn { ident },
            4 => Self::DimFourIn { ident },
            _ => panic!("a7b8c9d0 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_len_eq(dim: usize) -> Self {
        match dim {
            0 => Self::LenEq,
            1 => Self::DimOneLenEq,
            2 => Self::DimTwoLenEq,
            3 => Self::DimThreeLenEq,
            4 => Self::DimFourLenEq,
            _ => panic!("e1f2a3b4 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_len_greater_than(dim: usize) -> Self {
        match dim {
            0 => Self::LenGreaterThan,
            1 => Self::DimOneLenGreaterThan,
            2 => Self::DimTwoLenGreaterThan,
            3 => Self::DimThreeLenGreaterThan,
            4 => Self::DimFourLenGreaterThan,
            _ => panic!("c5d6e7f8 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_overlaps_with_arr(dim: usize, ident: Ts2) -> Self {
        match dim {
            0 => Self::OverlapsWithArr { ident },
            1 => Self::DimOneOverlapsWithArr { ident },
            2 => Self::DimTwoOverlapsWithArr { ident },
            3 => Self::DimThreeOverlapsWithArr { ident },
            4 => Self::DimFourOverlapsWithArr { ident },
            _ => panic!("a9b0c1d2 unsupported dim"),
        }
    }
    #[must_use]
    pub fn dim_rgx(dim: usize) -> Self {
        match dim {
            0 => Self::Rgx,
            1 => Self::DimOneRgx,
            2 => Self::DimTwoRgx,
            3 => Self::DimThreeRgx,
            4 => Self::DimFourRgx,
            _ => panic!("e3f4a5b6 unsupported dim"),
        }
    }
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
