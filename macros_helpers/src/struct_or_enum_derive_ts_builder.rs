//todo
use proc_macro2::TokenStream as Ts2;
gen_struct_or_enum_derive_ts_builder::gen_struct_or_enum_derive_ts_builder!([
    "Debug",
    "Default",
    "Clone",
    "Copy",
    "PartialEq",
    "Eq",
    "PartialOrd",
    "Ord",
    "serde::Serialize",
    "serde::Deserialize",
    "utoipa::ToSchema",
    "schemars::JsonSchema",
    "thiserror::Error",
    "er_occurence_lib::ErOccurence"
]);
