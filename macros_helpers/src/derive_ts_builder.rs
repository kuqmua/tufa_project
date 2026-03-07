//todo
use proc_macro2::TokenStream as Ts2;
gen_derive_ts_builder::gen_derive_ts_builder!([
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
    "location_lib::Location"
]);
