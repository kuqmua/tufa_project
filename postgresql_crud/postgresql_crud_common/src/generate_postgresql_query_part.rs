#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveI8(
    #[validate(range(min = -128i8, max = 127i8))]
    pub std::primitive::i8
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveI16(
    #[validate(range(min = -32_768i16, max = 32_767i16))]
    pub std::primitive::i16
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveI32(
    #[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))]
    pub std::primitive::i32
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveI64(
    #[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))]
    pub std::primitive::i64
);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
// pub struct JsonStdPrimitiveI128(
//     #[validate(range(min = -170_141_183_460_469_231_731_687_303_715_884_105_728i128, max = 170_141_183_460_469_231_731_687_303_715_884_105_727i128))]
//     pub std::primitive::i128
// );
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU8(
    #[validate(range(min = 0u8, max = 255u8))]
    pub std::primitive::u8
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU16(
    // #[validate(range(min = , max = ))]
    pub std::primitive::u16
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU32(
    // #[validate(range(min = , max = ))]
    pub std::primitive::u32
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU64(
    // #[validate(range(min = , max = ))]
    pub std::primitive::u64
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU128(
    // #[validate(range(min = , max = ))]
    pub std::primitive::u128
);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveF32(
    // #[validate(range(min = , max = ))]
    pub std::primitive::f32
);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveF64(
    // #[validate(range(min = , max = ))]
    pub std::primitive::f64
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveBool(pub std::primitive::bool);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdStringString(pub std::string::String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::primitive::i8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::primitive::i16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::primitive::i32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::primitive::i64>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdPrimitiveI128(pub std::option::Option<std::primitive::i128>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::primitive::u8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::primitive::u16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::primitive::u32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::primitive::u64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU128(pub std::option::Option<std::primitive::u128>);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::primitive::f32>);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::primitive::f64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdStringString(pub std::option::Option<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
// pub struct JsonStdVecVecStdPrimitiveI128(pub std::vec::Vec<std::primitive::i128>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU128(pub std::vec::Vec<std::primitive::u128>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdStringString(pub std::vec::Vec<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::primitive::i64>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<std::primitive::i64>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI128(pub std::vec::Vec<std::option::Option<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU128(pub std::vec::Vec<std::option::Option<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i128>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u128>>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementGenericFullTypePath)]
pub struct JsonGeneric<T>(pub T);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionGenericFullTypePath)]
pub struct JsonStdOptionOptionGeneric<T>(pub std::option::Option<T>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecGenericFullTypePath)]
pub struct JsonStdVecVecGeneric<T>(pub std::vec::Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecGenericFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecGeneric<T>(pub std::option::Option<std::vec::Vec<T>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionGenericFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionGeneric<T>(pub std::vec::Vec<std::option::Option<T>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionGenericFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionGeneric<T>(pub std::option::Option<std::vec::Vec<std::option::Option<T>>>);
/////////////////////
//todo rename it coz it applied to read_one and read_many
pub trait GeneratePostgresqlQueryPart<T1, T2> {
    fn generate_postgresql_query_part_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, T1> where Self: Sized;
    fn generate_postgresql_query_part(
        &self,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> Result<std::string::String, T2>;
}

pub trait StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self;
}

pub trait GeneratePostgresqlQueryPartToUpdate<T1> {
    fn try_generate_bind_increments(
        &self,
        jsonb_set_acc: &std::primitive::str,
        option_path: std::option::Option<&std::primitive::str>,
        increment: &mut std::primitive::u64,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, T1>;
    fn bind_value_to_query<'a>(
        self,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        is_optional: std::primitive::bool,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}