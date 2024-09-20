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
    #[validate(range(min = 0u16, max = 65_535u16))]
    pub std::primitive::u16
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU32(
    #[validate(range(min = 0u32, max = 4_294_967_295u32))]
    pub std::primitive::u32
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveU64(
    #[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))]
    pub std::primitive::u64
);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
// pub struct JsonStdPrimitiveU128(
//     #[validate(range(min = 0u128, max = 340_282_366_920_938_463_463_374_607_431_768_211_455u128))]
//     pub std::primitive::u128
// );
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveF32(
    #[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))]//todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
    pub std::primitive::f32
);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveF64(
    #[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))]//todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
    pub std::primitive::f64
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdPrimitiveBool(pub std::primitive::bool);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonStdStringString(pub std::string::String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI8(pub std::option::Option<JsonStdPrimitiveI8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI16(pub std::option::Option<JsonStdPrimitiveI16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI32(pub std::option::Option<JsonStdPrimitiveI32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveI64(pub std::option::Option<JsonStdPrimitiveI64>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdPrimitiveI128(pub std::option::Option<std::primitive::i128>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU8(pub std::option::Option<JsonStdPrimitiveU8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU16(pub std::option::Option<JsonStdPrimitiveU16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU32(pub std::option::Option<JsonStdPrimitiveU32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveU64(pub std::option::Option<JsonStdPrimitiveU64>);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdPrimitiveU128(pub std::option::Option<std::primitive::u128>);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveF32(pub std::option::Option<JsonStdPrimitiveF32>);
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveF64(pub std::option::Option<JsonStdPrimitiveF64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdPrimitiveBool(pub std::option::Option<JsonStdPrimitiveBool>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdStringString(pub std::option::Option<JsonStdStringString>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI8(pub std::vec::Vec<JsonStdPrimitiveI8>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI16(pub std::vec::Vec<JsonStdPrimitiveI16>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI32(pub std::vec::Vec<JsonStdPrimitiveI32>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveI64(pub std::vec::Vec<JsonStdPrimitiveI64>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
// pub struct JsonStdVecVecStdPrimitiveI128(pub std::vec::Vec<std::primitive::i128>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU8(pub std::vec::Vec<JsonStdPrimitiveU8>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU16(pub std::vec::Vec<JsonStdPrimitiveU16>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU32(pub std::vec::Vec<JsonStdPrimitiveU32>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveU64(pub std::vec::Vec<JsonStdPrimitiveU64>);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
// pub struct JsonStdVecVecStdPrimitiveU128(pub std::vec::Vec<std::primitive::u128>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveF32(pub std::vec::Vec<JsonStdPrimitiveF32>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveF64(pub std::vec::Vec<JsonStdPrimitiveF64>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdPrimitiveBool(pub std::vec::Vec<JsonStdPrimitiveBool>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub struct JsonStdVecVecStdStringString(pub std::vec::Vec<JsonStdStringString>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveI8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveI16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveI32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveI64>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveU8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveU16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveU32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveU64>>);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveF32>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveF64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<JsonStdPrimitiveBool>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<JsonStdStringString>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveI8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveI16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveI32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveI64>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI128(pub std::vec::Vec<std::option::Option<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveU8>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveU16>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveU32>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveU64>>);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU128(pub std::vec::Vec<std::option::Option<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveF32>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveF64>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<JsonStdPrimitiveBool>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<JsonStdStringString>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveI8>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveI16>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveI32>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveI64>>>);
//the trait `From<i128>` is not implemented for `JsonValue`, which is required by `i128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i128>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveU8>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveU16>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveU32>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveU64>>>);
// the trait `From<u128>` is not implemented for `JsonValue`, which is required by `u128: Into<JsonValue>` - schemars::JsonSchema validate range error
// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
// pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u128>>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveF32>>>);
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveF64>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdPrimitiveBool>>>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<JsonStdStringString>>>);

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementGenericFullTypePath)]
pub struct JsonGeneric<T>(pub T);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionGenericFullTypePath)]
pub struct JsonStdOptionOptionGeneric<T>(pub std::option::Option<T>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecGenericFullTypePath)]
pub struct JsonStdVecVecGenericWithId<T>(pub std::vec::Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecGenericFullTypePath)]
pub struct JsonStdOptionOptionStdVecVecGenericWithId<T>(pub std::option::Option<std::vec::Vec<T>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct JsonUuid(pub uuid::Uuid);
impl schemars::JsonSchema for JsonUuid {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("JsonUuid")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed(
            "postgresql_crud_common::f::JsonUuid",
        )
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(
                &mut schema,
                "string",
                "minLength",
                36,
            );
            schemars::_private::insert_validation_property(
                &mut schema,
                "string",
                "maxLength",
                36,
            );
            schemars::_private::insert_validation_property(
                &mut schema,
                "array",
                "minItems",
                36,
            );
            schemars::_private::insert_validation_property(
                &mut schema,
                "array",
                "maxItems",
                36,
            );
            schema
        }
    }
}
impl error_occurence_lib::ToStdStringString for JsonUuid {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}

/////////////////////
//todo rename it coz it applied to read_one and read_many
pub trait GeneratePostgresqlQueryPartToRead<T1, T2> {
    fn generate_postgresql_query_part_to_read_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, T1> where Self: Sized;
    fn generate_postgresql_query_part_to_read(
        &self,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> Result<std::string::String, T2>;
}

pub trait StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self;
}

#[derive(Debug, Clone)]
pub enum ArrayObjectElementOrSimple {
    ArrayObjectElement {
        jsonb_set_path: std::string::String,
        index: usize,
    },
    Simple
}
pub trait GeneratePostgresqlQueryPartToUpdate<T1> {
    fn try_generate_bind_increments(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: ArrayObjectElementOrSimple,
    ) -> Result<std::string::String, T1>;
    fn bind_value_to_query<'a>(
        self,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait GetJsonId {
    fn get_json_id(&self) -> &JsonUuid;
}

pub trait CheckIdExistsInJsonGenericFields {
    fn check_id_exists_in_json_generic_fields(&self);
}

pub trait CheckIdExistsInJsonStdVecVecGenericWithId {
    fn check_id_exists_in_json_std_vec_vec_generic_with_id(&self);// -> &std::vec::Vec<JsonUuid>;
}
impl<T: GetJsonId> CheckIdExistsInJsonStdVecVecGenericWithId for JsonStdVecVecGenericWithId<T> {
    fn check_id_exists_in_json_std_vec_vec_generic_with_id(&self) {}
}

pub trait CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId  {
    fn check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self);// -> &std::option::Option<std::vec::Vec<JsonUuid>>;
}
impl<T: GetJsonId> CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId for JsonStdOptionOptionStdVecVecGenericWithId<T> {
    fn check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, 
    // serde::Deserialize, 
    utoipa::ToSchema, schemars::JsonSchema)]
pub struct JsonArrayChange<CreateGeneric, UpdateGeneric> {
    pub create: std::vec::Vec<CreateGeneric>,
    pub update: std::vec::Vec<UpdateGeneric>,
    pub delete: std::vec::Vec<JsonUuid>
}
//

impl<'de, CreateGeneric, UpdateGeneric> serde::Deserialize<'de> for JsonArrayChange<CreateGeneric, UpdateGeneric>
where
    CreateGeneric: serde::Deserialize<'de>,
    UpdateGeneric: serde::Deserialize<'de> + GetJsonId,
{
    fn deserialize<__D>(
        __deserializer: __D,
    ) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "field identifier",
                )
            }
            fn visit_u64<__E>(
                self,
                __value: u64,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(
                self,
                __value: &str,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "create" => serde::__private::Ok(__Field::__field0),
                    "update" => serde::__private::Ok(__Field::__field1),
                    "delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(
                self,
                __value: &[u8],
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"create" => serde::__private::Ok(__Field::__field0),
                    b"update" => serde::__private::Ok(__Field::__field1),
                    b"delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(
                    __deserializer,
                    __FieldVisitor,
                )
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de, CreateGeneric, UpdateGeneric>
        where
            CreateGeneric: serde::Deserialize<'de>,
            UpdateGeneric: serde::Deserialize<'de>,
        {
            marker: serde::__private::PhantomData<
                JsonArrayChange<CreateGeneric, UpdateGeneric>,
            >,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de, CreateGeneric, UpdateGeneric> serde::de::Visitor<'de>
        for __Visitor<'de, CreateGeneric, UpdateGeneric>
        where
            CreateGeneric: serde::Deserialize<'de>,
            UpdateGeneric: serde::Deserialize<'de>,
        {
            type Value = JsonArrayChange<CreateGeneric, UpdateGeneric>;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct JsonArrayChange",
                )
            }
            #[inline]
            fn visit_seq<__A>(
                self,
                mut __seq: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<
                    std::vec::Vec<CreateGeneric>,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"struct JsonArrayChange with 3 elements",
                            ),
                        );
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::vec::Vec<UpdateGeneric>,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                1usize,
                                &"struct JsonArrayChange with 3 elements",
                            ),
                        );
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<
                    std::vec::Vec<JsonUuid>,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                2usize,
                                &"struct JsonArrayChange with 3 elements",
                            ),
                        );
                    }
                };
                serde::__private::Ok(JsonArrayChange {
                    create: __field0,
                    update: __field1,
                    delete: __field2,
                })
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<
                    std::vec::Vec<CreateGeneric>,
                > = serde::__private::None;
                let mut __field1: serde::__private::Option<
                    std::vec::Vec<UpdateGeneric>,
                > = serde::__private::None;
                let mut __field2: serde::__private::Option<
                    std::vec::Vec<JsonUuid>,
                > = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                    __Field,
                >(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("create"),
                                );
                            }
                            __field0 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::vec::Vec<CreateGeneric>,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("update"),
                                );
                            }
                            __field1 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::vec::Vec<UpdateGeneric>,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("delete"),
                                );
                            }
                            __field2 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::vec::Vec<JsonUuid>,
                                >(&mut __map)?,
                            );
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<
                                serde::de::IgnoredAny,
                            >(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        serde::__private::de::missing_field("create")?
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        serde::__private::de::missing_field("update")?
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        serde::__private::de::missing_field("delete")?
                    }
                };
                serde::__private::Ok(JsonArrayChange {
                    create: __field0,
                    update: __field1,
                    delete: __field2,
                })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "JsonArrayChange",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<
                    JsonArrayChange<CreateGeneric, UpdateGeneric>,
                >,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
//

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed {
    TryGenerateBindIncrements {
        #[eo_error_occurence]
        error: crate::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub trait JsonArrayElementUpdateBindQuery<UpdateErrorGeneric> {
    fn try_generate_update_bind_increments(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: ArrayObjectElementOrSimple,
    ) -> Result<std::option::Option<std::string::String>, UpdateErrorGeneric>;
    fn bind_update_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
// pub trait JsonArrayElementDeleteBindQuery {
//     fn try_generate_delete_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed>;
//     fn bind_delete_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
// }
pub trait JsonArrayElementCreateBindQuery {
    fn try_generate_create_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed>;
    fn bind_create_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

// pub trait JsonArrayElementBindQuery<UpdateErrorGeneric> {
//     fn try_generate_update_bind_increments(
//         &self,
//         jsonb_set_accumulator: &std::primitive::str,
//         jsonb_set_target: &std::primitive::str,
//         jsonb_set_path: &std::primitive::str,
//         increment: &mut std::primitive::u64,
//         is_array_object_element: ArrayObjectElementOrSimple,
//     ) -> Result<std::option::Option<std::string::String>, UpdateErrorGeneric>;
//     fn bind_update_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
//     fn try_generate_delete_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed>;
//     fn bind_delete_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
//     fn try_generate_create_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed>;
//     fn bind_create_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
// }

//
// pub enum JsonArrayUpdate<T> {
//     AddElements,
//     RemoveFromStart
//     RemoveFromEnd
//     //Where 
// }
/////////////////

//update by index (dont want to support)

// UPDATE test
// SET jsoncolumn = jsonb_set(
//     jsonb_set(
//         jsoncolumn,
//         '{one,0}',
//         '{"two": "twovalue1updated", "three": "threevalue1updated"}',
//         true
//     ),
//     '{one,1,three}',
//     '"threevalue2updated"',
//     true
// )
// WHERE id = 1 returning id;

//update adding one element

// UPDATE test
// SET jsoncolumn = jsonb_set(
//         jsoncolumn,
//         '{one}',
// 		(jsoncolumn->'one') || '{"two": "20", "three": "20"}'
//     )
// WHERE id = 1 returning id;





// remove first two elements

// UPDATE test
// SET jsoncolumn = jsonb_set(
//     jsoncolumn,
//     '{one}',
//     COALESCE(
//         (
//             SELECT jsonb_agg(elem)
//             FROM (
//                 SELECT elem
//                 FROM jsonb_array_elements(jsoncolumn->'one') WITH ORDINALITY arr(elem, ord)
//                 WHERE ord > 2  -- Exclude the first element (ordinal 1)
//             ) sub
//         ),
//         '[]'::jsonb  -- Default to an empty array if the result is NULL
//     )
// )
// WHERE id = 1 returning id;

// remove two last elements

// UPDATE test
// SET jsoncolumn = jsonb_set(
//     jsoncolumn,
//     '{one}',
//     COALESCE(
//         (
//             SELECT jsonb_agg(elem)
//             FROM (
//                 SELECT elem
//                 FROM jsonb_array_elements(jsoncolumn->'one') WITH ORDINALITY arr(elem, ord)
//                 WHERE ord <= (SELECT count(*) FROM jsonb_array_elements(jsoncolumn->'one')) - 2
//             ) sub
//         ),
//         '[]'::jsonb  -- Default to an empty array if no elements are left
//     )
// )
// WHERE id = 1 returning id;