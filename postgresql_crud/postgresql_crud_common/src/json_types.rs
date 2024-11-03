//todo maybe some derive impl does not need?
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveF32(
    #[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))] //todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
    pub std::primitive::f32,
);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveF64(
    #[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))] //todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
    pub std::primitive::f64,
);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdPrimitiveBool(pub std::primitive::bool);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeFullTypePath,
)]
pub struct JsonStdStringString(pub std::string::String);


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]//todo add validate range
pub struct JsonStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::primitive::i8>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::primitive::i16>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::primitive::i32>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::primitive::i64>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::primitive::u8>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::primitive::u16>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::primitive::u32>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::primitive::u64>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::primitive::f32>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::primitive::f64>);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::primitive::bool>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath,
)]
pub struct JsonStdOptionOptionStdStringString(pub std::option::Option<std::string::String>);

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
)]
pub struct JsonStdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);




// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct JsonStdVecVecStdStringString(pub std::vec::Vec<std::string::String>);
