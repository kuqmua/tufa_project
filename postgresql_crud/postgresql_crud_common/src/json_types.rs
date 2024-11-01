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
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType


    // postgresql_crud_types_macro_logic_reuse::GenerateIdentToCreate,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate,
    // postgresql_crud_types_macro_logic_reuse::GenerateFullTypePathFieldReader,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader,
    // postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionsToRead,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead,
    // postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber,
    // postgresql_crud_types_macro_logic_reuse::GenerateIdentOptionToUpdate,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate,
    // postgresql_crud_types_macro_logic_reuse::GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate,
)]
pub struct JsonStdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);



