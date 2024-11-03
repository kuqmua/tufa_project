#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    //todo do i need utoipa::ToSchema and schemars::JsonSchema for original type? maybe just for create update delete types
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveF32(
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveF64(
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdPrimitiveBool(pub std::primitive::bool);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdStringString(pub std::string::String);

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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveI8(pub std::option::Option<std::primitive::i8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveI16(pub std::option::Option<std::primitive::i16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveI32(pub std::option::Option<std::primitive::i32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveI64(pub std::option::Option<std::primitive::i64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveU8(pub std::option::Option<std::primitive::u8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveU16(pub std::option::Option<std::primitive::u16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveU32(pub std::option::Option<std::primitive::u32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveU64(pub std::option::Option<std::primitive::u64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveF32(pub std::option::Option<std::primitive::f32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveF64(pub std::option::Option<std::primitive::f64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdPrimitiveBool(pub std::option::Option<std::primitive::bool>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdStringString(pub std::option::Option<std::string::String>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdStringString(pub std::vec::Vec<std::string::String>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::primitive::i8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::primitive::i16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::primitive::i32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::primitive::i64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::primitive::u8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::primitive::u16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::primitive::u32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::primitive::u64>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::primitive::f32>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::primitive::f64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::primitive::bool>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<std::string::String>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<std::primitive::i8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<std::primitive::i16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<std::primitive::i32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<std::primitive::i64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<std::primitive::u8>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<std::primitive::u16>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<std::primitive::u32>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<std::primitive::u64>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<std::primitive::f32>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<std::primitive::f64>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<std::primitive::bool>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<std::string::String>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
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
    postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArray,
    postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);

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
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReader,
    postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
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
pub struct UuidToCreate(pub uuid::Uuid);
impl schemars::JsonSchema for UuidToCreate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("UuidToCreate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::UuidToCreate")
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
pub struct UuidOptionsToRead(pub uuid::Uuid);
impl schemars::JsonSchema for UuidOptionsToRead {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("UuidOptionsToRead")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::UuidOptionsToRead")
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
pub struct UuidOptionToUpdate(pub uuid::Uuid);
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UuidOptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        //maybe compile time uuid gen? not need to gen on runtime. if all different on compile time than its enough 
        Self(uuid::Uuid::new_v4())
    }
}
impl schemars::JsonSchema for UuidOptionToUpdate {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("UuidOptionToUpdate")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::UuidOptionToUpdate")
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
////////////////////
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct FJsonStdPrimitiveI8ToCreate(pub std::primitive::i8);
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for FJsonStdPrimitiveI8ToCreate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct FJsonStdPrimitiveI8FieldReader {}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for FJsonStdPrimitiveI8FieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct FJsonStdPrimitiveI8OptionsToRead(pub std::primitive::i8);
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for FJsonStdPrimitiveI8OptionsToRead {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct FJsonStdPrimitiveI8OptionToUpdate(pub std::primitive::i8);
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for FJsonStdPrimitiveI8OptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum FJsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}

impl crate::generate_postgresql_query_part::PostgresqlJsonType for StdPrimitiveI8 {
    type ToCreate<'a> = FJsonStdPrimitiveI8ToCreate;
    fn json_create_try_generate_bind_increments(
        self_to_create: &Self::ToCreate<'_>,
        increment: &mut std::primitive::u64
    ) -> Result<std::string::String, crate::generate_postgresql_query_part::JsonCreateTryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            }
            None => Err(crate::generate_postgresql_query_part::JsonCreateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn json_create_bind_value_to_query<'a>(
        self_to_create: Self::ToCreate<'a>,
        mut query: sqlx::query::Query<'a,
        sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self_to_create.0));
        query
    }
    type FieldReader<'a> = FJsonStdPrimitiveI8FieldReader;
    type OptionsToRead<'a> = FJsonStdPrimitiveI8OptionsToRead;
    fn generate_postgresql_query_part_field_to_read(
        read_reader: &Self::FieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
    ) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type OptionToUpdate<'a>: = FJsonStdPrimitiveI8OptionToUpdate;
    type OptionToUpdateTryGenerateBindIncrementsErrorNamed = FJsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed;
    fn try_generate_bind_increments(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_query<'a>(
        option_to_update: Self::OptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(option_to_update.0));
        query
    }
}