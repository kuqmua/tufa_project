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
    // postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
)]
pub struct StdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveF32(
//     #[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))] //todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
//     pub std::primitive::f32,
// );
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveF64(
//     #[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))] //todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
//     pub std::primitive::f64,
// );
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdPrimitiveBool(pub std::primitive::bool);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeFullTypePath,
// )]
// pub struct StdStringString(pub std::string::String);


// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]//todo add validate range
// pub struct StdOptionOptionStdPrimitiveI8(pub std::option::Option<std::primitive::i8>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveI16(pub std::option::Option<std::primitive::i16>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveI32(pub std::option::Option<std::primitive::i32>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveI64(pub std::option::Option<std::primitive::i64>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveU8(pub std::option::Option<std::primitive::u8>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveU16(pub std::option::Option<std::primitive::u16>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveU32(pub std::option::Option<std::primitive::u32>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveU64(pub std::option::Option<std::primitive::u64>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveF32(pub std::option::Option<std::primitive::f32>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveF64(pub std::option::Option<std::primitive::f64>);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdPrimitiveBool(pub std::option::Option<std::primitive::bool>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdStringString(pub std::option::Option<std::string::String>);

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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecFullTypePath,
// )]
// pub struct StdVecVecStdStringString(pub std::vec::Vec<std::string::String>);

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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::primitive::i8>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::primitive::i16>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::primitive::i32>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::primitive::i64>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::primitive::u8>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::primitive::u16>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::primitive::u32>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::primitive::u64>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::primitive::f32>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::primitive::f64>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::primitive::bool>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<std::string::String>>);

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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<std::primitive::i8>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<std::primitive::i16>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<std::primitive::i32>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<std::primitive::i64>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<std::primitive::u8>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<std::primitive::u16>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<std::primitive::u32>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<std::primitive::u64>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<std::primitive::f32>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<std::primitive::f64>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<std::primitive::bool>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<std::string::String>>);

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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
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
//     postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
// )]
// pub struct StdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);
///////////////////
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
)]
pub struct Uuid(pub uuid::Uuid);
impl schemars::JsonSchema for Uuid {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("Uuid")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::Uuid")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
impl error_occurence_lib::ToStdStringString for Uuid {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct PostgresqlJsonTypeUuidToCreate(pub uuid::Uuid);
impl schemars::JsonSchema for PostgresqlJsonTypeUuidToCreate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("PostgresqlJsonTypeUuidToCreate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::PostgresqlJsonTypeUuidToCreate")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct PostgresqlJsonTypeUuidOptionsToRead(pub uuid::Uuid);
impl schemars::JsonSchema for PostgresqlJsonTypeUuidOptionsToRead {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("PostgresqlJsonTypeUuidOptionsToRead")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::PostgresqlJsonTypeUuidOptionsToRead")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
/////////////////////
// PostgresqlJsonTypeSelfWhereElement
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlJsonTypeUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: uuid::Uuid,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlJsonTypeUuidWhereElement {
    Equal(PostgresqlJsonTypeUuidWhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlJsonTypeUuidWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlJsonTypeUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlJsonTypeUuidWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlJsonTypeUuidWhereElement>,
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhere {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
            match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
//todo impl try_new + custom serde::Deserialize
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhere {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}






////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct PostgresqlJsonTypeUuidOptionToUpdate(pub uuid::Uuid);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidOptionToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        //maybe compile time uuid gen? not need to gen on runtime. if all different on compile time than its enough 
        Self(uuid::Uuid::new_v4())
    }
}
impl schemars::JsonSchema for PostgresqlJsonTypeUuidOptionToUpdate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("PostgresqlJsonTypeUuidOptionToUpdate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::PostgresqlJsonTypeUuidOptionToUpdate")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
//
impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for Uuid {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeUuidToCreate;
    fn try_generate_postgresql_json_type_to_create(_: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_to_create.0));
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeUuidFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeUuidOptionsToRead;
    fn generate_postgresql_json_type_to_read(
        _: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        _: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String {
        //todo refactor is_empty
        let column_name_and_maybe_field_getter_handle = if column_name_and_maybe_field_getter.is_empty() {
            column_name_and_maybe_field_getter.to_string()
        }
        else {
            format!("{column_name_and_maybe_field_getter}->")
        };
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter_handle}'{field_ident}'))")
    }
    type PostgresqlJsonTypeSelfWhereElement<'a> = PostgresqlJsonTypeUuidWhereElement;
    type PostgresqlJsonTypeSelfWhere = PostgresqlJsonTypeUuidWhere;
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeUuidOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed = PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        _: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        _: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_option_to_update.0));
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Uuid {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidToCreate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeUuidFieldReader {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidOptionsToRead {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
// impl UuidFieldReader {
//     fn generate_postgresql_json_type_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
//         format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
//     }
// }
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
// impl PostgresqlJsonTypeUuidOptionToUpdate {
//     fn try_generate_postgresql_json_type_to_update(&self, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
//             }
//             None => Err(PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.0));
//         query
//     }
// }

/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
/////////////////////////
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI8 {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
pub type PostgresqlJsonTypeStdPrimitiveI8ToCreate = StdPrimitiveI8;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdPrimitiveI8FieldReader {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdPrimitiveI8FieldReader {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type PostgresqlJsonTypeStdPrimitiveI8OptionsToRead = StdPrimitiveI8;
/////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdPrimitiveI8WhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::primitive::i8,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdPrimitiveI8WhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdPrimitiveI8WhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        println!("self.value {}", &self.value);
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum PostgresqlJsonTypeStdPrimitiveI8WhereElement {
    Equal(PostgresqlJsonTypeStdPrimitiveI8WhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdPrimitiveI8WhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlJsonTypeStdPrimitiveI8WhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlJsonTypeStdPrimitiveI8WhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdPrimitiveI8WhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}

////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdPrimitiveI8Where {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlJsonTypeStdPrimitiveI8WhereElement>,
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdPrimitiveI8Where {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
            match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
//todo impl try_new + custom serde::Deserialize
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdPrimitiveI8Where {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}







/////////////
pub type PostgresqlJsonTypeStdPrimitiveI8OptionToUpdate = StdPrimitiveI8;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for StdPrimitiveI8 {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeStdPrimitiveI8ToCreate;
    fn try_generate_postgresql_json_type_to_create(
        postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_to_create.0));
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeStdPrimitiveI8FieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeStdPrimitiveI8OptionsToRead;
    fn generate_postgresql_json_type_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type PostgresqlJsonTypeSelfWhereElement<'a> = PostgresqlJsonTypeStdPrimitiveI8WhereElement;
    type PostgresqlJsonTypeSelfWhere = PostgresqlJsonTypeStdPrimitiveI8Where;
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeStdPrimitiveI8OptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed = PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_option_to_update.0));
        query
    }
}
impl crate::BindQuery<'_> for StdPrimitiveI8 {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.0));
        query
    }
}