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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdStringString(pub std::vec::Vec<std::string::String>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::primitive::i8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::primitive::i16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::primitive::i32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::primitive::i64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::primitive::u8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::primitive::u16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::primitive::u32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::primitive::u64>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::primitive::f32>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::primitive::f64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::primitive::bool>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<std::string::String>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<std::primitive::i8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<std::primitive::i16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<std::primitive::i32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<std::primitive::i64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<std::primitive::u8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<std::primitive::u16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<std::primitive::u32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<std::primitive::u64>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<std::primitive::f32>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<std::primitive::f64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<std::primitive::bool>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<std::string::String>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableBoolean,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableString,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToUpdate,
)]
pub struct JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderString,
)]
pub struct JsonUuid(pub uuid::Uuid);
impl schemars::JsonSchema for JsonUuid {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("JsonUuid")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::JsonUuid")
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
impl error_occurence_lib::ToStdStringString for JsonUuid {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct JsonUuidToCreate(pub uuid::Uuid);
impl schemars::JsonSchema for JsonUuidToCreate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("JsonUuidToCreate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::JsonUuidToCreate")
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
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct JsonUuidOptionsToRead(pub uuid::Uuid);
impl schemars::JsonSchema for JsonUuidOptionsToRead {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("JsonUuidOptionsToRead")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::JsonUuidOptionsToRead")
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
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct JsonUuidOptionsToUpdate(pub uuid::Uuid);
impl schemars::JsonSchema for JsonUuidOptionsToUpdate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("JsonUuidOptionsToUpdate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::JsonUuidOptionsToUpdate")
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


//todo rename it coz it applied to read_one and read_many
pub trait GeneratePostgresqlQueryPartToRead<T1, T2> {
    fn generate_postgresql_query_part_to_read_from_self_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_optional: std::primitive::bool) -> Result<std::string::String, T1>
    where
        Self: Sized;
    fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, T2>;
}

pub trait StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self;
}

impl<T> StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for std::option::Option<crate::value::Value<crate::SqlxTypesJson<T>>>
where
    T: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement,
{
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Some(crate::value::Value {
            value: crate::SqlxTypesJson(sqlx::types::Json(
                <T as StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            )),
        })
    }
}

pub trait AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self>;
}

#[derive(Debug, Clone)]
pub enum ArrayObjectElementOrSimple {
    ArrayObjectElement { jsonb_set_path: std::string::String, index: usize },
    Simple,
}
pub trait GeneratePostgresqlQueryPartToUpdate<T1> {
    fn try_generate_bind_increments(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64, is_array_object_element: ArrayObjectElementOrSimple) -> Result<std::string::String, T1>;
    fn bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum JsonCreateTryGenerateBindIncrementsErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
pub trait JsonCreateBindQuery<'a> {
    fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, JsonCreateTryGenerateBindIncrementsErrorNamed>;
    fn json_create_bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait GetJsonId {
    fn get_json_id(&self) -> &JsonUuid;
}

pub trait CheckIdExistsInJsonGenericFields {
    fn check_id_exists_in_json_generic_fields(&self);
}

// pub trait CheckIdExistsInJsonStdVecVecGenericWithId {
//     fn check_id_exists_in_json_std_vec_vec_generic_with_id(&self); // -> &std::vec::Vec<JsonUuid>;
// }
// impl<T: GetJsonId> CheckIdExistsInJsonStdVecVecGenericWithId for JsonStdVecVecGenericWithId<T> {
//     fn check_id_exists_in_json_std_vec_vec_generic_with_id(&self) {}
// }

// pub trait CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId {
//     fn check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self); // -> &std::option::Option<std::vec::Vec<JsonUuid>>;
// }
// impl<T: GetJsonId> CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId for JsonStdOptionOptionStdVecVecGenericWithId<T> {
//     fn check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self) {}
// }

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct JsonArrayChange<CreateGeneric, UpdateGeneric> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub create: std::vec::Vec<CreateGeneric>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub update: std::vec::Vec<UpdateGeneric>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub delete: std::vec::Vec<JsonUuid>,
}

impl<'de, CreateGeneric, UpdateGeneric> serde::Deserialize<'de> for JsonArrayChange<CreateGeneric, UpdateGeneric>
where
    CreateGeneric: serde::Deserialize<'de>,
    UpdateGeneric: serde::Deserialize<'de> + GetJsonId,
{
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
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
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
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
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
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
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de, CreateGeneric, UpdateGeneric>
        where
            CreateGeneric: serde::Deserialize<'de>,
            UpdateGeneric: serde::Deserialize<'de> + GetJsonId,
        {
            marker: serde::__private::PhantomData<JsonArrayChange<CreateGeneric, UpdateGeneric>>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        const FIELDS_ARE_EMPTY_ERROR_MESSAGE: &str = "create, update, delete fields are empty";
        const NOT_UNIQUE_ID_IN_JSON_UPDATE_ARRAY: &str = "not unique id in json update array: ";
        const NOT_UNIQUE_ID_IN_JSON_DELETE_ARRAY: &str = "not unique id in json delete array: ";
        const NOT_UNIQUE_ID_IN_JSON_UPDATE_AND_DELETE_ARRAYS: &str = "not unique id in json update and delete arrays: ";
        impl<'de, CreateGeneric, UpdateGeneric> serde::de::Visitor<'de> for __Visitor<'de, CreateGeneric, UpdateGeneric>
        where
            CreateGeneric: serde::Deserialize<'de>,
            UpdateGeneric: serde::Deserialize<'de> + GetJsonId,
        {
            type Value = JsonArrayChange<CreateGeneric, UpdateGeneric>;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct JsonArrayChange")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<CreateGeneric>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<UpdateGeneric>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<JsonUuid>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                if __field0.is_empty() && __field1.is_empty() && __field2.is_empty() {
                    return Err(serde::de::Error::custom(&FIELDS_ARE_EMPTY_ERROR_MESSAGE));
                }
                {
                    let mut update_acc = vec![];
                    for element in &__field1 {
                        let id = element.get_json_id();
                        if update_acc.contains(id) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_UPDATE_ARRAY}{}", id.0)));
                        } else {
                            update_acc.push(*id);
                        }
                    }
                    let mut delete_acc = vec![];
                    for element in &__field2 {
                        if delete_acc.contains(&element) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_DELETE_ARRAY}{}", element.0)));
                        } else {
                            delete_acc.push(element);
                        }
                    }
                    for element in update_acc {
                        if delete_acc.contains(&&element) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_UPDATE_AND_DELETE_ARRAYS}{}", element.0)));
                        }
                    }
                }
                serde::__private::Ok(JsonArrayChange { create: __field0, update: __field1, delete: __field2 })
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<CreateGeneric>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::vec::Vec<UpdateGeneric>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::vec::Vec<JsonUuid>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<CreateGeneric>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<UpdateGeneric>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<JsonUuid>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                if __field0.is_empty() && __field1.is_empty() && __field2.is_empty() {
                    return Err(serde::de::Error::custom(&FIELDS_ARE_EMPTY_ERROR_MESSAGE));
                }
                {
                    let mut update_acc = vec![];
                    for element in &__field1 {
                        let id = element.get_json_id();
                        if update_acc.contains(id) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_UPDATE_ARRAY}{}", id.0)));
                        } else {
                            update_acc.push(*id);
                        }
                    }
                    let mut delete_acc = vec![];
                    for element in &__field2 {
                        if delete_acc.contains(&element) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_DELETE_ARRAY}{}", element.0)));
                        } else {
                            delete_acc.push(element);
                        }
                    }
                    for element in update_acc {
                        if delete_acc.contains(&&element) {
                            return Err(serde::de::Error::custom(&format!("{NOT_UNIQUE_ID_IN_JSON_UPDATE_AND_DELETE_ARRAYS}{}", element.0)));
                        }
                    }
                }
                serde::__private::Ok(JsonArrayChange { create: __field0, update: __field1, delete: __field2 })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "JsonArrayChange",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<JsonArrayChange<CreateGeneric, UpdateGeneric>>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
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

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed {
    TryGenerateBindIncrements {
        #[eo_error_occurence]
        error: crate::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub trait JsonArrayElementCreateBindQuery {
    fn try_generate_create_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed>;
    fn bind_create_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait GeneratePostgresqlQueryPartFieldToRead {
    fn generate_postgresql_query_part_field_to_read(
        &self,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str //todo remove this coz does not have 
    ) -> std::string::String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Pagination {
    limit: std::primitive::u64,
    offset: std::primitive::u64,
}
impl<'de> serde::Deserialize<'de> for Pagination {
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
                    "limit" => serde::__private::Ok(__Field::__field0),
                    "offset" => serde::__private::Ok(__Field::__field1),
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
                    b"limit" => serde::__private::Ok(__Field::__field0),
                    b"offset" => serde::__private::Ok(__Field::__field1),
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
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<Pagination>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = Pagination;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct Pagination",
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
                    std::primitive::u64,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"struct Pagination with 2 elements",
                            ),
                        );
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::primitive::u64,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                1usize,
                                &"struct Pagination with 2 elements",
                            ),
                        );
                    }
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                }
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::u64> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::u64> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                    __Field,
                >(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("limit"),
                                );
                            }
                            __field0 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::primitive::u64,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("offset"),
                                );
                            }
                            __field1 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::primitive::u64,
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
                        serde::__private::de::missing_field("limit")?
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        serde::__private::de::missing_field("offset")?
                    }
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["limit", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "Pagination",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Pagination>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum PaginationTryNewErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::u64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    LimitIsZero {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl Pagination {
    pub fn try_new(limit: std::primitive::u64, offset: std::primitive::u64) -> Result<Self, PaginationTryNewErrorNamed> {
        match offset.checked_add(limit) {
            Some(value) => match limit == 0 {
                true => Err(PaginationTryNewErrorNamed::LimitIsZero {
                    code_occurence: error_occurence_lib::code_occurence!()
                }),
                false => Ok(Self{ limit, offset })
            },
            None => Err(PaginationTryNewErrorNamed::OffsetPlusLimitIsIntOverflow {
                limit,
                offset,
                code_occurence: error_occurence_lib::code_occurence!()
            })
        }
    }
    pub fn start(&self) -> std::primitive::u64 {
        self.offset
    }
    pub fn end(&self) -> std::primitive::u64 {
        self.offset + self.limit
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Pagination {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            limit: 1,
            offset: std::default::Default::default(),
        }
    }
}