use macros_helpers::ShouldWriteTsIntoFile;
use optml::Optml;
use pg_crud_macros_cmn::IsNl;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Optml)]
pub enum TraitGen {
    PgJson,
    PgTypeAndPgJson,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumIter, Optml)]
pub enum Pattern {
    Stdrt,
    Arr,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Optml)]
pub struct PgJsonObjRecord {
    pub is_nl: IsNl,
    pub pattern: Pattern,
    pub trait_gen: TraitGen,
}
#[derive(Debug, Deserialize, Optml)]
pub struct GenPgJsonsConfig {
    pub pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs: ShouldWriteTsIntoFile,
    pub vrt: PgJsonObjRecord,
    pub whole_write_into_gen_pg_json_obj: ShouldWriteTsIntoFile,
}
