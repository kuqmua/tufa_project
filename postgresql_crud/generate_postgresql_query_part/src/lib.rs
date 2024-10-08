#[derive(Debug, PartialEq, Eq)]
enum SupportedPredefinedType {
    JsonStdPrimitiveI8,
    JsonStdPrimitiveI16,
    JsonStdPrimitiveI32,
    JsonStdPrimitiveI64,
    JsonStdPrimitiveI128,
    JsonStdPrimitiveU8,
    JsonStdPrimitiveU16,
    JsonStdPrimitiveU32,
    JsonStdPrimitiveU64,
    JsonStdPrimitiveU128,
    JsonStdPrimitiveF32,
    JsonStdPrimitiveF64,
    JsonStdPrimitiveBool,
    JsonStdStringString,

    JsonStdOptionOptionStdPrimitiveI8,
    JsonStdOptionOptionStdPrimitiveI16,
    JsonStdOptionOptionStdPrimitiveI32,
    JsonStdOptionOptionStdPrimitiveI64,
    JsonStdOptionOptionStdPrimitiveI128,
    JsonStdOptionOptionStdPrimitiveU8,
    JsonStdOptionOptionStdPrimitiveU16,
    JsonStdOptionOptionStdPrimitiveU32,
    JsonStdOptionOptionStdPrimitiveU64,
    JsonStdOptionOptionStdPrimitiveU128,
    JsonStdOptionOptionStdPrimitiveF32,
    JsonStdOptionOptionStdPrimitiveF64,
    JsonStdOptionOptionStdPrimitiveBool,
    JsonStdOptionOptionStdStringString,

    JsonStdVecVecStdPrimitiveI8,
    JsonStdVecVecStdPrimitiveI16,
    JsonStdVecVecStdPrimitiveI32,
    JsonStdVecVecStdPrimitiveI64,
    JsonStdVecVecStdPrimitiveI128,
    JsonStdVecVecStdPrimitiveU8,
    JsonStdVecVecStdPrimitiveU16,
    JsonStdVecVecStdPrimitiveU32,
    JsonStdVecVecStdPrimitiveU64,
    JsonStdVecVecStdPrimitiveU128,
    JsonStdVecVecStdPrimitiveF32,
    JsonStdVecVecStdPrimitiveF64,
    JsonStdVecVecStdPrimitiveBool,
    JsonStdVecVecStdStringString,

    JsonStdOptionOptionStdVecVecStdPrimitiveI8,
    JsonStdOptionOptionStdVecVecStdPrimitiveI16,
    JsonStdOptionOptionStdVecVecStdPrimitiveI32,
    JsonStdOptionOptionStdVecVecStdPrimitiveI64,
    JsonStdOptionOptionStdVecVecStdPrimitiveI128,
    JsonStdOptionOptionStdVecVecStdPrimitiveU8,
    JsonStdOptionOptionStdVecVecStdPrimitiveU16,
    JsonStdOptionOptionStdVecVecStdPrimitiveU32,
    JsonStdOptionOptionStdVecVecStdPrimitiveU64,
    JsonStdOptionOptionStdVecVecStdPrimitiveU128,
    JsonStdOptionOptionStdVecVecStdPrimitiveF32,
    JsonStdOptionOptionStdVecVecStdPrimitiveF64,
    JsonStdOptionOptionStdVecVecStdPrimitiveBool,
    JsonStdOptionOptionStdVecVecStdStringString,

    JsonStdVecVecStdOptionOptionStdPrimitiveI8,
    JsonStdVecVecStdOptionOptionStdPrimitiveI16,
    JsonStdVecVecStdOptionOptionStdPrimitiveI32,
    JsonStdVecVecStdOptionOptionStdPrimitiveI64,
    JsonStdVecVecStdOptionOptionStdPrimitiveI128,
    JsonStdVecVecStdOptionOptionStdPrimitiveU8,
    JsonStdVecVecStdOptionOptionStdPrimitiveU16,
    JsonStdVecVecStdOptionOptionStdPrimitiveU32,
    JsonStdVecVecStdOptionOptionStdPrimitiveU64,
    JsonStdVecVecStdOptionOptionStdPrimitiveU128,
    JsonStdVecVecStdOptionOptionStdPrimitiveF32,
    JsonStdVecVecStdOptionOptionStdPrimitiveF64,
    JsonStdVecVecStdOptionOptionStdPrimitiveBool,
    JsonStdVecVecStdOptionOptionStdStringString,

    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString,

    JsonGeneric(syn::TypePath),
    JsonStdOptionOptionGeneric(syn::TypePath),

    JsonStdVecVecGenericWithId(syn::TypePath),
    JsonStdOptionOptionStdVecVecGenericWithId(syn::TypePath),

    //JsonStdVecVecStdOptionOptionGenericWithId and JsonStdOptionOptionStdVecVecStdOptionOptionGenericWithId not supported coz null does not have id, so its impossible to update and delete null-elements by id. only using index, but i dont like using index
    JsonUuid,
}
impl SupportedPredefinedType {
    fn to_original_type(&self) -> SupportedPredefinedOriginalType {
        match self {
            Self::JsonStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdVecVecStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdVecVecStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdVecVecStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdVecVecStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdVecVecStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdVecVecStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdVecVecStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdVecVecStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdVecVecStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdVecVecStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdVecVecStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdVecVecStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdVecVecStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdVecVecStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdOptionOptionStdVecVecStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdVecVecStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdOptionOptionGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),

            Self::JsonStdVecVecGenericWithId(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdOptionOptionStdVecVecGenericWithId(value) => SupportedPredefinedOriginalType::Generic(value.clone()),

            Self::JsonUuid => SupportedPredefinedOriginalType::Uuid,
        }
    }
    fn to_variant_name_stringified(&self) -> &str {
        match self {
            Self::JsonStdPrimitiveI8 => "JsonStdPrimitiveI8",
            Self::JsonStdPrimitiveI16 => "JsonStdPrimitiveI16",
            Self::JsonStdPrimitiveI32 => "JsonStdPrimitiveI32",
            Self::JsonStdPrimitiveI64 => "JsonStdPrimitiveI64",
            Self::JsonStdPrimitiveI128 => "JsonStdPrimitiveI128",
            Self::JsonStdPrimitiveU8 => "JsonStdPrimitiveU8",
            Self::JsonStdPrimitiveU16 => "JsonStdPrimitiveU16",
            Self::JsonStdPrimitiveU32 => "JsonStdPrimitiveU32",
            Self::JsonStdPrimitiveU64 => "JsonStdPrimitiveU64",
            Self::JsonStdPrimitiveU128 => "JsonStdPrimitiveU128",
            Self::JsonStdPrimitiveF32 => "JsonStdPrimitiveF32",
            Self::JsonStdPrimitiveF64 => "JsonStdPrimitiveF64",
            Self::JsonStdPrimitiveBool => "JsonStdPrimitiveBool",
            Self::JsonStdStringString => "JsonStdStringString",

            Self::JsonStdOptionOptionStdPrimitiveI8 => "JsonStdOptionOptionStdPrimitiveI8",
            Self::JsonStdOptionOptionStdPrimitiveI16 => "JsonStdOptionOptionStdPrimitiveI16",
            Self::JsonStdOptionOptionStdPrimitiveI32 => "JsonStdOptionOptionStdPrimitiveI32",
            Self::JsonStdOptionOptionStdPrimitiveI64 => "JsonStdOptionOptionStdPrimitiveI64",
            Self::JsonStdOptionOptionStdPrimitiveI128 => "JsonStdOptionOptionStdPrimitiveI128",
            Self::JsonStdOptionOptionStdPrimitiveU8 => "JsonStdOptionOptionStdPrimitiveU8",
            Self::JsonStdOptionOptionStdPrimitiveU16 => "JsonStdOptionOptionStdPrimitiveU16",
            Self::JsonStdOptionOptionStdPrimitiveU32 => "JsonStdOptionOptionStdPrimitiveU32",
            Self::JsonStdOptionOptionStdPrimitiveU64 => "JsonStdOptionOptionStdPrimitiveU64",
            Self::JsonStdOptionOptionStdPrimitiveU128 => "JsonStdOptionOptionStdPrimitiveU128",
            Self::JsonStdOptionOptionStdPrimitiveF32 => "JsonStdOptionOptionStdPrimitiveF32",
            Self::JsonStdOptionOptionStdPrimitiveF64 => "JsonStdOptionOptionStdPrimitiveF64",
            Self::JsonStdOptionOptionStdPrimitiveBool => "JsonStdOptionOptionStdPrimitiveBool",
            Self::JsonStdOptionOptionStdStringString => "JsonStdOptionOptionStdStringString",

            Self::JsonStdVecVecStdPrimitiveI8 => "JsonStdVecVecStdPrimitiveI8",
            Self::JsonStdVecVecStdPrimitiveI16 => "JsonStdVecVecStdPrimitiveI16",
            Self::JsonStdVecVecStdPrimitiveI32 => "JsonStdVecVecStdPrimitiveI32",
            Self::JsonStdVecVecStdPrimitiveI64 => "JsonStdVecVecStdPrimitiveI64",
            Self::JsonStdVecVecStdPrimitiveI128 => "JsonStdVecVecStdPrimitiveI128",
            Self::JsonStdVecVecStdPrimitiveU8 => "JsonStdVecVecStdPrimitiveU8",
            Self::JsonStdVecVecStdPrimitiveU16 => "JsonStdVecVecStdPrimitiveU16",
            Self::JsonStdVecVecStdPrimitiveU32 => "JsonStdVecVecStdPrimitiveU32",
            Self::JsonStdVecVecStdPrimitiveU64 => "JsonStdVecVecStdPrimitiveU64",
            Self::JsonStdVecVecStdPrimitiveU128 => "JsonStdVecVecStdPrimitiveU128",
            Self::JsonStdVecVecStdPrimitiveF32 => "JsonStdVecVecStdPrimitiveF32",
            Self::JsonStdVecVecStdPrimitiveF64 => "JsonStdVecVecStdPrimitiveF64",
            Self::JsonStdVecVecStdPrimitiveBool => "JsonStdVecVecStdPrimitiveBool",
            Self::JsonStdVecVecStdStringString => "JsonStdVecVecStdStringString",

            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI8 => "JsonStdOptionOptionStdVecVecStdPrimitiveI8",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI16 => "JsonStdOptionOptionStdVecVecStdPrimitiveI16",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI32 => "JsonStdOptionOptionStdVecVecStdPrimitiveI32",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI64 => "JsonStdOptionOptionStdVecVecStdPrimitiveI64",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveI128 => "JsonStdOptionOptionStdVecVecStdPrimitiveI128",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU8 => "JsonStdOptionOptionStdVecVecStdPrimitiveU8",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU16 => "JsonStdOptionOptionStdVecVecStdPrimitiveU16",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU32 => "JsonStdOptionOptionStdVecVecStdPrimitiveU32",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU64 => "JsonStdOptionOptionStdVecVecStdPrimitiveU64",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveU128 => "JsonStdOptionOptionStdVecVecStdPrimitiveU128",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveF32 => "JsonStdOptionOptionStdVecVecStdPrimitiveF32",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveF64 => "JsonStdOptionOptionStdVecVecStdPrimitiveF64",
            Self::JsonStdOptionOptionStdVecVecStdPrimitiveBool => "JsonStdOptionOptionStdVecVecStdPrimitiveBool",
            Self::JsonStdOptionOptionStdVecVecStdStringString => "JsonStdOptionOptionStdVecVecStdStringString",

            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI8 => "JsonStdVecVecStdOptionOptionStdPrimitiveI8",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI16 => "JsonStdVecVecStdOptionOptionStdPrimitiveI16",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI32 => "JsonStdVecVecStdOptionOptionStdPrimitiveI32",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI64 => "JsonStdVecVecStdOptionOptionStdPrimitiveI64",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveI128 => "JsonStdVecVecStdOptionOptionStdPrimitiveI128",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU8 => "JsonStdVecVecStdOptionOptionStdPrimitiveU8",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU16 => "JsonStdVecVecStdOptionOptionStdPrimitiveU16",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU32 => "JsonStdVecVecStdOptionOptionStdPrimitiveU32",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU64 => "JsonStdVecVecStdOptionOptionStdPrimitiveU64",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveU128 => "JsonStdVecVecStdOptionOptionStdPrimitiveU128",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveF32 => "JsonStdVecVecStdOptionOptionStdPrimitiveF32",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveF64 => "JsonStdVecVecStdOptionOptionStdPrimitiveF64",
            Self::JsonStdVecVecStdOptionOptionStdPrimitiveBool => "JsonStdVecVecStdOptionOptionStdPrimitiveBool",
            Self::JsonStdVecVecStdOptionOptionStdStringString => "JsonStdVecVecStdOptionOptionStdStringString",

            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool",
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => "JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString",

            Self::JsonGeneric(_) => "JsonGeneric",
            Self::JsonStdOptionOptionGeneric(_) => "JsonStdOptionOptionGeneric",

            Self::JsonStdVecVecGenericWithId(_) => "JsonStdVecVecGenericWithId",
            Self::JsonStdOptionOptionStdVecVecGenericWithId(_) => "JsonStdOptionOptionStdVecVecGenericWithId",

            Self::JsonUuid => "JsonUuid",
        }
    }
}
#[derive(Debug)]
enum SupportedPredefinedOriginalType {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    String,
    Generic(syn::TypePath),
    Uuid,
}

impl SupportedPredefinedOriginalType {
    fn to_read_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::I8 => quote::quote! {i8},
            Self::I16 => quote::quote! {i16},
            Self::I32 => quote::quote! {i32},
            Self::I64 => quote::quote! {i64},
            Self::I128 => quote::quote! {i128},
            Self::U8 => quote::quote! {u8},
            Self::U16 => quote::quote! {u16},
            Self::U32 => quote::quote! {u32},
            Self::U64 => quote::quote! {u64},
            Self::U128 => quote::quote! {u128},
            Self::F32 => quote::quote! {f32},
            Self::F64 => quote::quote! {f64},
            Self::Bool => quote::quote! {bool},
            Self::String => quote::quote! {String},
            Self::Generic(type_path) => naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&type_path),
            Self::Uuid => quote::quote! {Uuid},
        }
    }
    fn full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let wrap_into_std_primitive_token_stream = |value: proc_macro2::TokenStream| {
            quote::quote! {std::primitive::#value}
        };
        match self {
            Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 | Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128 | Self::F32 | Self::F64 | Self::Bool => wrap_into_std_primitive_token_stream(self.to_read_token_stream()),
            Self::String => {
                let value = self.to_read_token_stream();
                quote::quote! {std::string::#value}
            }
            Self::Generic(_) => self.to_read_token_stream(),
            Self::Uuid => quote::quote! {postgresql_crud::Uuid},
        }
    }
    fn std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::option::Option<#value>}
    }
    fn std_vec_vec_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::vec::Vec<#value>}
    }
    fn std_option_option_std_vec_vec_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::option::Option<std::vec::Vec<#value>>}
    }
    fn std_vec_vec_std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::vec::Vec<std::option::Option<#value>>}
    }
    fn std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::option::Option<std::vec::Vec<std::option::Option<#value>>>}
    }
    fn std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::vec::Vec<std::result::Result<#value,std::string::String>>}
    }
    fn std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::option::Option<std::vec::Vec<std::result::Result<#value,std::string::String>>>}
    }
    fn std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::vec::Vec<std::result::Result<std::option::Option<#value>,std::string::String>>}
    }
    fn std_option_option_std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote! {std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<#value>,std::string::String>>>}
    }
}
#[derive(Debug)]
enum SupportedPredefinedTypeTryFromSynField {
    TypePathPathSegmentsIsNotSynTypePath,
    TypePathPathSegmentsLastIsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsNone,
    UnsupportedPredefinedTypeWrapper,
    SynPathArgumentsAngleBracketedArgsLengthNotEqualOne,
    SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeSynTypePathSynPathArgumentsNone,
    SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeIsNotSynTypePath,
    SynPathArgumentsAngleBracketedFirstIsNotSynGenericArgumentType,
    SynPathArgumentsAngleBracketedFirstIsNone,
    SynPathArgumentsIsNotAngleBracketed,
}
impl std::convert::TryFrom<&syn::Field> for SupportedPredefinedType {
    type Error = SupportedPredefinedTypeTryFromSynField;
    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        match &value.ty {
            syn::Type::Path(type_path) => match type_path.path.segments.last() {
                Some(path_segment) => {
                    let try_generate_generic_ident_upper_camel_case_token_stream = |path_segment: &syn::PathSegment| match &path_segment.arguments {
                        syn::PathArguments::AngleBracketed(value) => {
                            if value.args.len() != 1 {
                                return Err(Self::Error::SynPathArgumentsAngleBracketedArgsLengthNotEqualOne);
                            }
                            match value.args.first() {
                                Some(value) => {
                                    if let syn::GenericArgument::Type(value) = value {
                                        match &value {
                                            syn::Type::Path(type_path) => {
                                                for element in &type_path.path.segments {
                                                    if let syn::PathArguments::None = element.arguments {
                                                    } else {
                                                        return Err(Self::Error::SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeSynTypePathSynPathArgumentsNone);
                                                    }
                                                }
                                                return Ok(type_path.clone());
                                            }
                                            _ => {
                                                return Err(Self::Error::SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeIsNotSynTypePath);
                                            }
                                        }
                                    } else {
                                        return Err(Self::Error::SynPathArgumentsAngleBracketedFirstIsNotSynGenericArgumentType);
                                    }
                                }
                                None => {
                                    return Err(Self::Error::SynPathArgumentsAngleBracketedFirstIsNone);
                                }
                            }
                        }
                        _ => {
                            return Err(Self::Error::SynPathArgumentsIsNotAngleBracketed);
                        }
                    };
                    let supported_predefined_type = match path_segment.ident.to_string().as_str() {
                        "JsonStdPrimitiveI8" => Self::JsonStdPrimitiveI8,
                        "JsonStdPrimitiveI16" => Self::JsonStdPrimitiveI16,
                        "JsonStdPrimitiveI32" => Self::JsonStdPrimitiveI32,
                        "JsonStdPrimitiveI64" => Self::JsonStdPrimitiveI64,
                        "JsonStdPrimitiveI128" => Self::JsonStdPrimitiveI128,
                        "JsonStdPrimitiveU8" => Self::JsonStdPrimitiveU8,
                        "JsonStdPrimitiveU16" => Self::JsonStdPrimitiveU16,
                        "JsonStdPrimitiveU32" => Self::JsonStdPrimitiveU32,
                        "JsonStdPrimitiveU64" => Self::JsonStdPrimitiveU64,
                        "JsonStdPrimitiveU128" => Self::JsonStdPrimitiveU128,
                        "JsonStdPrimitiveF32" => Self::JsonStdPrimitiveF32,
                        "JsonStdPrimitiveF64" => Self::JsonStdPrimitiveF64,
                        "JsonStdPrimitiveBool" => Self::JsonStdPrimitiveBool,
                        "JsonStdStringString" => Self::JsonStdStringString,

                        "JsonStdOptionOptionStdPrimitiveI8" => Self::JsonStdOptionOptionStdPrimitiveI8,
                        "JsonStdOptionOptionStdPrimitiveI16" => Self::JsonStdOptionOptionStdPrimitiveI16,
                        "JsonStdOptionOptionStdPrimitiveI32" => Self::JsonStdOptionOptionStdPrimitiveI32,
                        "JsonStdOptionOptionStdPrimitiveI64" => Self::JsonStdOptionOptionStdPrimitiveI64,
                        "JsonStdOptionOptionStdPrimitiveI128" => Self::JsonStdOptionOptionStdPrimitiveI128,
                        "JsonStdOptionOptionStdPrimitiveU8" => Self::JsonStdOptionOptionStdPrimitiveU8,
                        "JsonStdOptionOptionStdPrimitiveU16" => Self::JsonStdOptionOptionStdPrimitiveU16,
                        "JsonStdOptionOptionStdPrimitiveU32" => Self::JsonStdOptionOptionStdPrimitiveU32,
                        "JsonStdOptionOptionStdPrimitiveU64" => Self::JsonStdOptionOptionStdPrimitiveU64,
                        "JsonStdOptionOptionStdPrimitiveU128" => Self::JsonStdOptionOptionStdPrimitiveU128,
                        "JsonStdOptionOptionStdPrimitiveF32" => Self::JsonStdOptionOptionStdPrimitiveF32,
                        "JsonStdOptionOptionStdPrimitiveF64" => Self::JsonStdOptionOptionStdPrimitiveF64,
                        "JsonStdOptionOptionStdPrimitiveBool" => Self::JsonStdOptionOptionStdPrimitiveBool,
                        "JsonStdOptionOptionStdStringString" => Self::JsonStdOptionOptionStdStringString,

                        "JsonStdVecVecStdPrimitiveI8" => Self::JsonStdVecVecStdPrimitiveI8,
                        "JsonStdVecVecStdPrimitiveI16" => Self::JsonStdVecVecStdPrimitiveI16,
                        "JsonStdVecVecStdPrimitiveI32" => Self::JsonStdVecVecStdPrimitiveI32,
                        "JsonStdVecVecStdPrimitiveI64" => Self::JsonStdVecVecStdPrimitiveI64,
                        "JsonStdVecVecStdPrimitiveI128" => Self::JsonStdVecVecStdPrimitiveI128,
                        "JsonStdVecVecStdPrimitiveU8" => Self::JsonStdVecVecStdPrimitiveU8,
                        "JsonStdVecVecStdPrimitiveU16" => Self::JsonStdVecVecStdPrimitiveU16,
                        "JsonStdVecVecStdPrimitiveU32" => Self::JsonStdVecVecStdPrimitiveU32,
                        "JsonStdVecVecStdPrimitiveU64" => Self::JsonStdVecVecStdPrimitiveU64,
                        "JsonStdVecVecStdPrimitiveU128" => Self::JsonStdVecVecStdPrimitiveU128,
                        "JsonStdVecVecStdPrimitiveF32" => Self::JsonStdVecVecStdPrimitiveF32,
                        "JsonStdVecVecStdPrimitiveF64" => Self::JsonStdVecVecStdPrimitiveF64,
                        "JsonStdVecVecStdPrimitiveBool" => Self::JsonStdVecVecStdPrimitiveBool,
                        "JsonStdVecVecStdStringString" => Self::JsonStdVecVecStdStringString,

                        "JsonStdOptionOptionStdVecVecStdPrimitiveI8" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveI16" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI16,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveI32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI32,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveI64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI64,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveI128" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI128,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveU8" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU8,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveU16" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU16,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveU32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU32,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveU64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU64,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveU128" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU128,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveF32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveF32,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveF64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveF64,
                        "JsonStdOptionOptionStdVecVecStdPrimitiveBool" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveBool,
                        "JsonStdOptionOptionStdVecVecStdStringString" => Self::JsonStdOptionOptionStdVecVecStdStringString,

                        "JsonStdVecVecStdOptionOptionStdPrimitiveI8" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveI16" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI16,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveI32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI32,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveI64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI64,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveI128" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI128,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveU8" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU8,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveU16" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU16,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveU32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU32,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveU64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU64,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveU128" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU128,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveF32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveF32,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveF64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveF64,
                        "JsonStdVecVecStdOptionOptionStdPrimitiveBool" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveBool,
                        "JsonStdVecVecStdOptionOptionStdStringString" => Self::JsonStdVecVecStdOptionOptionStdStringString,

                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
                        "JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString,

                        "JsonGeneric" => Self::JsonGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdOptionOptionGeneric" => Self::JsonStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),

                        "JsonStdVecVecGenericWithId" => Self::JsonStdVecVecGenericWithId(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdOptionOptionStdVecVecGenericWithId" => Self::JsonStdOptionOptionStdVecVecGenericWithId(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),

                        "JsonUuid" => Self::JsonUuid,
                        _ => {
                            return Err(Self::Error::UnsupportedPredefinedTypeWrapper);
                        }
                    };
                    match &supported_predefined_type {
                        SupportedPredefinedType::JsonStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdStringString
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdOptionOptionStdStringString
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdVecVecStdStringString
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
                        | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                        | SupportedPredefinedType::JsonUuid => match path_segment.arguments {
                            syn::PathArguments::None => (),
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            }
                        },
                        _ => (),
                    }
                    Ok(supported_predefined_type)
                }
                None => Err(Self::Error::TypePathPathSegmentsLastIsNone),
            },
            _ => Err(Self::Error::TypePathPathSegmentsIsNotSynTypePath),
        }
    }
}
enum PrimitiveJsonType {
    Boolean,
    Number,
    String,
    Array,
    Object,
    Null,
}
impl std::fmt::Display for PrimitiveJsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
            Self::Null => write!(f, "null"),
        }
    }
}
//todo maybe generate example of valid json to create - maybe with serde_json::to_string() adn #[derive(Default)} then println or write into file
//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
#[proc_macro_derive(GeneratePostgresqlQueryPart)]
pub fn generate_postgresql_query_part(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlQueryPart";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().map(|element| element).collect::<std::vec::Vec<&syn::Field>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    // println!("{vec_syn_field:#?}");
    let id_snake_case = naming_conventions::IdSnakeCase;
    let is_id_field_exists = {
        let mut is_id_field_exists = false;
        for element in &vec_syn_field {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            if element_ident == &id_snake_case.to_string() {
                if let SupportedPredefinedType::JsonUuid = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
                    is_id_field_exists = true;
                    break;
                } else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field {id_snake_case} is not SupportedPredefinedType::JsonUuid");
                };
            }
        }
        is_id_field_exists
    };
    let vec_syn_field_filtered_id_iter = vec_syn_field
        .iter()
        .filter(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            element_ident != &id_snake_case.to_string()
        })
        .collect::<std::vec::Vec<&&syn::Field>>();
    let generate_ident_field_to_read_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = format!("{value}{}", naming_conventions::FieldToReadUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&ident.to_string());
    let ident_options_to_read_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::OptionsToReadUpperCamelCase);
    let ident_options_to_read_upper_camel_case_token_stream = {
        ident_options_to_read_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {ident_options_to_read_upper_camel_case_stringified} {}",
                proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
    };
    let ident_options_to_read_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&ident_options_to_read_upper_camel_case_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
    let ident_reader_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::ReaderUpperCamelCase);
    let ident_reader_upper_camel_case_token_stream = {
        ident_reader_upper_camel_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_reader_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_value_option_to_update_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = format!("{value}{}", naming_conventions::OptionToUpdateUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_option_to_update_upper_camel_case_token_stream = generate_value_option_to_update_upper_camel_case_token_stream(&ident.to_string());
    let generate_value_options_to_update_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = format!("{value}{}", naming_conventions::OptionsToUpdateUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_to_update_upper_camel_case_token_stream = generate_value_options_to_update_upper_camel_case_token_stream(&ident.to_string());
    let ident_field_to_update_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::FieldToUpdateUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::OptionsToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    let ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::GeneratePostgresqlQueryPartToReadErrorNamedUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_json_array_element_change_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = format!("{value}{}", naming_conventions::JsonArrayElementChangeUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = format!("{value}{}", naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsErrorNamedUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream = generate_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream(&ident.to_string());
    let add_postfix_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_stringified = |value: &std::primitive::str| format!("{value}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecErrorNamedUpperCamelCase);
    let ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = {
        let value = add_postfix_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_stringified(&ident.to_string());
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified = |value: &std::primitive::str| format!("{value}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecUpperCamelCase);
    let generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified(value);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = |value: &syn::Field| {
        let value = format!(
            "{}OffsetPlusLimitIsIntOverflow",
            proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(
                &value
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                    .to_string()
            ),
        );
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value));
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_field_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = |value: &std::primitive::str| {
        let value = add_postfix_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_stringified(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value));
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let update_snake_case = naming_conventions::UpdateSnakeCase;
    let postgresql_crud_uuid_token_stream = quote::quote! {postgresql_crud::Uuid};

    //remove this later
    let _unused = proc_macro_helpers::generate_serde_skip_serializing_if_value_attribute_token_stream::generate_serde_skip_serializing_if_value_attribute_token_stream();

    // let offset_plus_limit_is_int_overflow_variants_token_stream = vec_syn_field.iter().fold(vec![], |mut acc, element| {
    //     let ident_offset_plus_limit_is_int_overflow_token_stream = {
    //         let ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream(&element);
    //         quote::quote! {
    //             #ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream {
    //                 #[eo_to_std_string_string_serialize_deserialize]
    //                 limit: std::primitive::u64,
    //                 #[eo_to_std_string_string_serialize_deserialize]
    //                 offset: std::primitive::u64,
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //         }
    //     };
    //     let supported_predefined_type = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //     match supported_predefined_type {
    //         SupportedPredefinedType::JsonStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdStringString
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdOptionOptionStdStringString => (),

    //         SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdVecVecStdStringString
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //         | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //             acc.push(ident_offset_plus_limit_is_int_overflow_token_stream);
    //         }

    //         SupportedPredefinedType::JsonGeneric(_) | SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => (),

    //         SupportedPredefinedType::JsonStdVecVecGenericWithId(_) | SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => {
    //             acc.push(ident_offset_plus_limit_is_int_overflow_token_stream);
    //         }

    //         SupportedPredefinedType::JsonUuid => (),
    //     };
    //     acc
    // });
    // let field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream = {
    //     let unique_type_path = vec_syn_field.iter().fold(vec![], |mut acc, element| {
    //         let supported_predefined_type = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let option_type_path = match supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => None,

    //             SupportedPredefinedType::JsonGeneric(type_path) => Some(type_path),
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => Some(type_path),

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => Some(type_path),
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => Some(type_path),

    //             SupportedPredefinedType::JsonUuid => None,
    //         };
    //         if let Some(value) = option_type_path {
    //             if !acc.contains(&value) {
    //                 acc.push(value)
    //             }
    //         }
    //         acc
    //     });
    //     unique_type_path.iter().fold(vec![], |mut acc, element| {
    //         acc.push({
    //             let field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote! {#element}.to_string());
    //             let field_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = generate_field_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream(&quote::quote! {#element}.to_string());
    //             quote::quote! {
    //                 #field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
    //                     #[eo_error_occurence]
    //                     field: #field_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream,
    //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                 }
    //             }
    //         });
    //         acc
    //     })
    // };
    // let should_generate_ident_generate_postgresql_query_part_error_named_enum = match (offset_plus_limit_is_int_overflow_variants_token_stream.is_empty(), field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream.is_empty()) {
    //     (true, true) => false,
    //     (true, false) => true,
    //     (false, true) => true,
    //     (false, false) => true,
    // };
    // let impl_std_fmt_display_for_ident_token_stream = {
    //     quote::quote! {
    //         impl std::fmt::Display for #ident {
    //             fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //                 write!(formatter, "{:?}", &self)
    //             }
    //         }
    //     }
    // };
    // let pub_enum_ident_field_to_read_token_stream = {
    //     let variants_token_stream = vec_syn_field.iter().map(|element| {
    //         let field_ident_stringified = element
    //             .ident
    //             .as_ref()
    //             .unwrap_or_else(|| {
    //                 panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //             })
    //             .to_string();
    //         let serialize_deserialize_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
    //         let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
    //         let maybe_additional_token_stream = {
    //             let supported_predefined_type = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //             let generate_std_vec_vec_generic_ident_field_token_stream = |type_path: &syn::TypePath| {
    //                 let generic_ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&quote::quote! {#type_path}.to_string());
    //                 quote::quote! {
    //                     (std::vec::Vec<#generic_ident_field_to_read_upper_camel_case_token_stream>)
    //                 }
    //             };
    //             let generate_field_vec_limit_offset_token_stream = |type_path: &syn::TypePath| {
    //                 let generic_ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&quote::quote! {#type_path}.to_string());
    //                 quote::quote! {
    //                     {
    //                         field_vec: std::vec::Vec<#generic_ident_field_to_read_upper_camel_case_token_stream>,
    //                         limit: std::primitive::u64,
    //                         offset: std::primitive::u64,
    //                     }
    //                 }
    //             };
    //             match supported_predefined_type {
    //                 SupportedPredefinedType::JsonStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdStringString
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdStringString => proc_macro2::TokenStream::new(),

    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdVecVecStdStringString
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //                 | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote! {
    //                     {
    //                         limit: std::primitive::u64,
    //                         offset: std::primitive::u64,
    //                     }
    //                 },

    //                 SupportedPredefinedType::JsonGeneric(type_path) => generate_std_vec_vec_generic_ident_field_token_stream(&type_path),
    //                 SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => generate_std_vec_vec_generic_ident_field_token_stream(&type_path),

    //                 SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),

    //                 SupportedPredefinedType::JsonUuid => proc_macro2::TokenStream::new(),
    //             }
    //         };
    //         quote::quote! {
    //             #[serde(rename(
    //                serialize = #serialize_deserialize_field_ident_double_quotes_token_stream,
    //                deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream
    //             ))]
    //             #variant_ident_upper_camel_case_token_stream #maybe_additional_token_stream
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq,
    //         pub enum #ident_field_to_read_upper_camel_case_token_stream {
    //             #(#variants_token_stream),*
    //         }
    //     }
    // };
    // let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream = {
    //     quote::quote! {
    //         impl error_occurence_lib::ToStdStringString for #ident_field_to_read_upper_camel_case_token_stream {
    //             fn to_std_string_string(&self) -> std::string::String {
    //                 format!("{self:?}")
    //             }
    //         }
    //     }
    // };
    // let pub_enum_field_generate_postgresql_query_part_to_read_error_named_token_stream = {
    //     let ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
    //         let maybe_generate_postgresql_query_part_variant_token_stream = match should_generate_ident_generate_postgresql_query_part_error_named_enum {
    //             true => quote::quote! {
    //                 GeneratePostgresqlQueryPart {
    //                     #[eo_error_occurence]
    //                     error: #ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream,
    //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                 },
    //             },
    //             false => proc_macro2::TokenStream::new(),
    //         };
    //         quote::quote! {
    //             #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
    //             pub enum #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
    //                 FieldsFilterIsEmpty {
    //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                 },
    //                 NotUniqueFieldFilter {
    //                     #[eo_to_std_string_string_serialize_deserialize]
    //                     field: #ident_field_to_read_upper_camel_case_token_stream,
    //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                 },
    //                 #maybe_generate_postgresql_query_part_variant_token_stream
    //             }
    //         }
    //     };
    //     let impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
    //         quote::quote! {
    //             impl error_occurence_lib::ToStdStringString for #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
    //                 fn to_std_string_string(&self) -> std::string::String {
    //                     format!("{self:?}")
    //                 }
    //             }
    //         }
    //     };
    //     let maybe_ident_generate_postgresql_query_part_error_named_token_stream = {
    //         match should_generate_ident_generate_postgresql_query_part_error_named_enum {
    //             true => {
    //                 quote::quote! {
    //                     #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
    //                     pub enum #ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream {
    //                         #(#offset_plus_limit_is_int_overflow_variants_token_stream)*
    //                         #(#field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream),*
    //                     }
    //                 }
    //             }
    //             false => proc_macro2::TokenStream::new(),
    //         }
    //     };
    //     quote::quote! {
    //         #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
    //         #impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
    //         #maybe_ident_generate_postgresql_query_part_error_named_token_stream
    //     }
    // };
    // let impl_generate_postgresql_query_part_to_read_for_ident_field_to_read_token_stream = {
    //     let type_of_space_stringified = "type of ";
    //     let space_and_not_null_stringified = " and not null";
    //     let space_is_not_space_stringified = " is not ";
    //     let wrap_into_jsonb_object_build = |value: &std::primitive::str| format!("jsonb_build_object({value})");
    //     let wrap_into_jsonb_build_object_ok_stringified = |value: &std::primitive::str| wrap_into_jsonb_object_build(&format!("'Ok',{value}"));
    //     let wrap_into_jsonb_typeof_stringified = |value: &std::primitive::str| format!("jsonb_typeof({value})");
    //     let wrap_into_when_space_value_space_equals_space_null_stringified = |value: &std::primitive::str, primitive_json_type: &PrimitiveJsonType| format!("when {value} = '{primitive_json_type}'");
    //     let add_then_space_prefix_stringified = |value: &std::primitive::str| format!("then {value}");
    //     let generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = |value: &std::primitive::str| {
    //         let add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&wrap_into_jsonb_build_object_ok_stringified("null"));
    //         let wraped_into_when_space_value_space_equals_space_null_stringified = wrap_into_when_space_value_space_equals_space_null_stringified(&wrap_into_jsonb_typeof_stringified(value), &PrimitiveJsonType::Null);
    //         format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified}")
    //     };
    //     let generate_space_else_space_jsonb_build_object_err_stringified = |value: &std::primitive::str| {
    //         let wraped_into_jsonb_object_build = wrap_into_jsonb_object_build(&format!("'Err','{value}'"));
    //         format!(" else jsonb_build_object({wraped_into_jsonb_object_build})")
    //     };
    //     let generate_vec_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str| {
    //         format!(
    //             "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message}{space_is_not_space_stringified}array{}",
    //             match is_optional {
    //                 true => space_and_not_null_stringified,
    //                 false => "",
    //             }
    //         )
    //     };
    //     let wrap_into_case_end_stringified = |value: &std::primitive::str| format!("case {value} end");
    //     let wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified("value");
    //     let wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified = |json_type: &PrimitiveJsonType| wrap_into_when_space_value_space_equals_space_null_stringified(&wraped_into_jsonb_typeof_stringified, &json_type);
    //     let generate_postgresql_query_part_content = |match_value_token_stream: &proc_macro2::TokenStream, wrap_in_ok_token_stream: std::primitive::bool| {
    //         let generate_postgresql_query_part_match_variants_token_stream = vec_syn_field.iter().map(|element|{
    //             let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //                panic!(
    //                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
    //                    naming_conventions::FIELD_IDENT_IS_NONE
    //                );
    //             });
    //             let el_ident_str = element_ident.to_string();
    //             let element_ident_upper_camel_case_token_stream = {
    //                 let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&el_ident_str);
    //                 value.parse::<proc_macro2::TokenStream>()
    //                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //             };
    //             let gen_maybe_wrap_in_ok_token_stream = |value: &proc_macro2::TokenStream|{
    //                 match wrap_in_ok_token_stream {
    //                     true => quote::quote!{Ok(#value)},
    //                     false => quote::quote!{#value}
    //                 }
    //             };
    //             let gen_simple_types_token_stream = |query_string_token_stream: &proc_macro2::TokenStream|{
    //                 gen_maybe_wrap_in_ok_token_stream(&quote::quote!{format!(#query_string_token_stream)})
    //             };
    //             let gen_vec_simple_types_token_stream = |query_string_token_stream: &proc_macro2::TokenStream|{
    //                 let ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream(&element);
    //                 let handle_return_token_stream = gen_maybe_wrap_in_ok_token_stream(&quote::quote!{format!(#query_string_token_stream)});
    //                 quote::quote!{
    //                     {
    //                         let start = offset;
    //                         let end = match offset.checked_add(*limit) {
    //                             Some(value) => value,
    //                             None => {
    //                                 return Err(#ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream::#ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream {
    //                                     limit: *limit,
    //                                     offset: *offset,
    //                                     code_occurence: error_occurence_lib::code_occurence!(),
    //                                 });
    //                             }
    //                         };
    //                         #handle_return_token_stream
    //                     }
    //                 }
    //             };
    //             let column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified = format!("{{column_name_and_maybe_field_getter_for_error_message}}.{el_ident_str}");
    //             let generate_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, json_type: &PrimitiveJsonType|{
    //                 format!(
    //                     "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{space_is_not_space_stringified}{json_type}{}",
    //                     match is_optional {
    //                         true => " and not null",
    //                         false => ""
    //                     }
    //                 )
    //             };
    //             let array_element_stringified = "[array element]";
    //             let generate_vec_element_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, json_type: &PrimitiveJsonType|{
    //                 format!(
    //                     "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{array_element_stringified}{space_is_not_space_stringified}{json_type}{}",
    //                     match is_optional {
    //                         true => space_and_not_null_stringified,
    //                         false => ""
    //                     }
    //                 )
    //             };
    //             let el_ident_str_single_quotes_stringified = format!("'{el_ident_str}'");
    //             let column_name_and_maybe_field_getter_el_ident_str_stringified = format!("{{column_name_and_maybe_field_getter}}->{el_ident_str_single_quotes_stringified}");
    //             let column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
    //             let add_el_ident_str_comma_prefix_stringified = |value: &std::primitive::str|{
    //                 format!("{el_ident_str_single_quotes_stringified},{value}")
    //             };
    //             let wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified = |json_type: &PrimitiveJsonType|{
    //                 wrap_into_when_space_value_space_equals_space_null_stringified(
    //                     &column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified,
    //                     json_type
    //                 )
    //             };
    //             let add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified = add_then_space_prefix_stringified(&wrap_into_jsonb_build_object_ok_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified));
    //             let generate_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_wrong_type_error_message_stringified(false, &json_type));
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&json_type);
    //                         add_el_ident_str_comma_prefix_stringified(&format!(
    //                             "{} ",
    //                             wrap_into_case_end_stringified(&format!(
    //                                 "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified}{space_else_space_jsonb_build_object_err_stringified}"
    //                             ))
    //                         ))
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 gen_simple_types_token_stream(&query_string_token_stream)
    //             };
    //             let generate_optional_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_wrong_type_error_message_stringified(true, &json_type));
    //                         let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&json_type);
    //                         add_el_ident_str_comma_prefix_stringified(&format!(
    //                             "{} ",
    //                             wrap_into_case_end_stringified(&format!(
    //                                 "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{space_else_space_jsonb_build_object_err_stringified}"
    //                             ))
    //                         ))
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 gen_simple_types_token_stream(&query_string_token_stream)
    //             };
    //             let generate_jsonb_array_elements_stringified = |value: &std::primitive::str|{
    //                 format!("jsonb_array_elements({value})")
    //             };
    //             let jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified = generate_jsonb_array_elements_stringified(&format!("(select {column_name_and_maybe_field_getter_el_ident_str_stringified})"));
    //             let jsonb_build_object_ok_value_stringified = wrap_into_jsonb_build_object_ok_stringified("value");
    //             let space_with_ordinality_where_ordinality_between_start_and_end_stringified = " with ordinality where ordinality between {start} and {end}";
    //             let wrap_into_jsonb_agg_stringified = |value: &std::primitive::str|{
    //                 format!("jsonb_agg({value})")
    //             };
    //             let generate_select_from_stringified = |select_value: &std::primitive::str, from_value: &std::primitive::str|{
    //                 format!("select {select_value} from {from_value}")
    //             };
    //             let wrap_into_scopes_stringified = |value: &std::primitive::str|{
    //                 format!("({value})")
    //             };
    //             let jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified = format!(
    //                 "{jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified}{space_with_ordinality_where_ordinality_between_start_and_end_stringified}"
    //             );
    //             let add_then_space_prefix_jsonb_build_object_ok_value_stringified = add_then_space_prefix_stringified(&jsonb_build_object_ok_value_stringified);
    //             let generate_vec_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
    //                             let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(false, &json_type));
    //                             let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
    //                             let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(&
    //                                 wrap_into_case_end_stringified(&format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}"))
    //                             );
    //                             wrap_into_scopes_stringified(&generate_select_from_stringified(
    //                                 &wraped_into_jsonb_agg_stringified,
    //                                 &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
    //                             ))
    //                         });
    //                         let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(false, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
    //                         let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
    //                         add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
    //                             "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
    //                         )))
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ))
    //             };
    //             let generate_optional_vec_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
    //                             let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(false, &json_type));
    //                             let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
    //                             let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(&wrap_into_case_end_stringified(
    //                                 &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}")
    //                             ));
    //                             wrap_into_scopes_stringified(&generate_select_from_stringified(
    //                                 &wraped_into_jsonb_agg_stringified,
    //                                 &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
    //                             ))
    //                         });
    //                         let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(true, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
    //                         let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
    //                         let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
    //                         add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
    //                             "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
    //                         )))
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ))
    //             };
    //             let generate_vec_optional_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
    //                             let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("value");
    //                             let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(true, &json_type));
    //                             let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
    //                             let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(
    //                                 &wrap_into_case_end_stringified(
    //                                     &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}")
    //                                 )
    //                             );
    //                             wrap_into_scopes_stringified(&generate_select_from_stringified(
    //                                 &wraped_into_jsonb_agg_stringified,
    //                                 &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
    //                             ))
    //                         });
    //                         let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(false, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
    //                         let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
    //                         add_el_ident_str_comma_prefix_stringified(
    //                             &wrap_into_case_end_stringified(
    //                                 &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}")
    //                             )
    //                         )
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ))
    //             };
    //             let generate_optional_vec_optional_simple_json_type = |json_type: PrimitiveJsonType|{
    //                 gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
    //                             let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_element_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("value");
    //                             let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(true, &json_type));
    //                             let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
    //                             let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(
    //                                 &wrap_into_case_end_stringified(&format!(
    //                                     "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_element_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}"
    //                                 ))
    //                             );
    //                             wrap_into_scopes_stringified(&generate_select_from_stringified(
    //                                 &wraped_into_jsonb_agg_stringified,
    //                                 &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
    //                             ))
    //                         });
    //                         let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(true, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
    //                         let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
    //                         let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
    //                         add_el_ident_str_comma_prefix_stringified(
    //                             &wrap_into_case_end_stringified(&format!(
    //                                 "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
    //                             ))
    //                         )
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ))
    //             };
    //             let generic_and_std_option_option_generic_logic_token_stream = |type_path: &syn::TypePath, is_optional: std::primitive::bool|{
    //                 let column_name_and_maybe_field_getter_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &column_name_and_maybe_field_getter_el_ident_str_stringified,
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let column_name_and_maybe_field_getter_for_error_message_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified,
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let simple_types_token_stream = gen_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&add_el_ident_str_comma_prefix_stringified("{value}")),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ));
    //                 let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 let is_optional_token_stream = if is_optional {
    //                     quote::quote!{true}
    //                 }
    //                 else {
    //                     quote::quote!{false}
    //                 };
    //                 quote::quote!{
    //                     (fields_vec) => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(
    //                         fields_vec,
    //                         &format!(#column_name_and_maybe_field_getter_query_string_token_stream),
    //                         &format!(#column_name_and_maybe_field_getter_for_error_message_query_string_token_stream),
    //                         #is_optional_token_stream
    //                     ) {
    //                         Ok(value) => #simple_types_token_stream,
    //                         Err(error) => {
    //                             return Err(#ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream  {
    //                                 field: error,
    //                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                             });
    //                         }
    //                     }
    //                 }
    //             };
    //             let maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream = |
    //                 type_path: &syn::TypePath,
    //                 is_optional: std::primitive::bool,
    //                 is_std_vec_vec_optional: std::primitive::bool,
    //             |{
    //                 let column_name_and_maybe_field_getter_for_error_message_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{array_element_stringified}"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let vec_simple_types_token_stream = gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &wrap_into_jsonb_object_build(&{
    //                         let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
    //                             let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified("{value}");
    //                             wrap_into_scopes_stringified(&generate_select_from_stringified(
    //                                 &wraped_into_jsonb_agg_stringified,
    //                                 &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
    //                             ))
    //                         });
    //                         let maybe_check_on_null_stringified = if is_std_vec_vec_optional {
    //                             generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified)
    //                         }
    //                         else {
    //                             std::string::String::default()
    //                         };
    //                         let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(is_std_vec_vec_optional, "{{column_name_and_maybe_field_getter_for_error_message}}"));
    //                         let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
    //                         let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
    //                         add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
    //                             "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{maybe_check_on_null_stringified}{space_else_space_jsonb_build_object_err_stringified}"
    //                         )))
    //                     }),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ));
    //                 let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 let is_optional_token_stream = if is_optional {
    //                     quote::quote!{true}
    //                 }
    //                 else {
    //                     quote::quote!{false}
    //                 };
    //                 quote::quote!{
    //                     {
    //                         field_vec,
    //                         limit,
    //                         offset
    //                     } => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(
    //                         field_vec,
    //                         &format!("value"),
    //                         &format!(#column_name_and_maybe_field_getter_for_error_message_query_string_token_stream),
    //                         #is_optional_token_stream
    //                     ) {
    //                         Ok(value) => #vec_simple_types_token_stream,
    //                         Err(error) => {
    //                             return Err(#ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
    //                                 field: error,
    //                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                             });
    //                         }
    //                     }
    //                 }
    //             };
    //             let variant_logic_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
    //             {
    //                 SupportedPredefinedType::JsonStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },
    //                 SupportedPredefinedType::JsonStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },
    //                 SupportedPredefinedType::JsonStdStringString => {
    //                     let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },

    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdStringString => {
    //                     let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },

    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 // generate_vec_imple_json_types
    //                 SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdVecVecStdStringString => {
    //                     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },

    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => {
    //                     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },

    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => {
    //                     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },

    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => {
    //                     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::Number);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => {
    //                     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::Boolean);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{
    //                         {
    //                             limit,
    //                             offset
    //                         } => #query_part_token_stream
    //                     }
    //                 },

    //                 SupportedPredefinedType::JsonGeneric(type_path) => generic_and_std_option_option_generic_logic_token_stream(&type_path, false),
    //                 SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => generic_and_std_option_option_generic_logic_token_stream(&type_path, true),
    //                 SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
    //                     &type_path,
    //                     false,
    //                     false,
    //                 ),
    //                 SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
    //                     &type_path,
    //                     false,
    //                     true,
    //                 ),

    //                 SupportedPredefinedType::JsonUuid => {
    //                     let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::String);
    //                     quote::quote!{ => #query_part_token_stream}
    //                 },
    //             };
    //             quote::quote!{Self::#element_ident_upper_camel_case_token_stream #variant_logic_token_stream}
    //         });
    //         quote::quote! {
    //             match #match_value_token_stream {
    //                 #(#generate_postgresql_query_part_match_variants_token_stream),*
    //             }
    //         }
    //     };
    //     let acc_push_token_stream = {
    //         let jsonb_build_object_concatenation_stringified = "||";
    //         match should_generate_ident_generate_postgresql_query_part_error_named_enum {
    //             true => {
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("{{value}}{jsonb_build_object_concatenation_stringified}"), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     match element.generate_postgresql_query_part_to_read(
    //                         column_name_and_maybe_field_getter,
    //                         column_name_and_maybe_field_getter_for_error_message,
    //                     ) {
    //                         Ok(value) => {
    //                             acc.push_str(&format!(#format_handle_token_stream));
    //                         }
    //                         Err(error) => {
    //                             return Err(#ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream::GeneratePostgresqlQueryPart {
    //                                 error,
    //                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                             });
    //                         }
    //                     }
    //                 }
    //             }
    //             false => {
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("{{}}{jsonb_build_object_concatenation_stringified}"), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 let postgresql_query_part_content_token_stream = generate_postgresql_query_part_content(&quote::quote! {element}, false);
    //                 quote::quote! {
    //                     acc.push_str(&format!(#format_handle_token_stream, #postgresql_query_part_content_token_stream));
    //                 }
    //             }
    //         }
    //     };
    //     let second_generic_token_stream = match should_generate_ident_generate_postgresql_query_part_error_named_enum {
    //         true => &ident_generate_postgresql_query_part_to_read_error_named_upper_camel_case_token_stream,
    //         false => &quote::quote! {()},
    //     };
    //     let postgresql_query_part_content_token_stream = generate_postgresql_query_part_content(&quote::quote! {self}, true);
    //     let space_and_not_null_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&space_and_not_null_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
    //     let query_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //         &{
    //             let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&format!("{type_of_space_stringified}{{column_name_and_maybe_field_getter_for_error_message}}{space_is_not_space_stringified}object{{space_and_not_null}}"));
    //             let wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified("{acc}");
    //             let wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified("{column_name_and_maybe_field_getter}");
    //             let wraped_into_when_space_value_space_equals_space_null_stringified = wrap_into_when_space_value_space_equals_space_null_stringified(&wraped_into_jsonb_typeof_stringified, &PrimitiveJsonType::Object);
    //             let add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&wraped_into_jsonb_build_object_ok_stringified);
    //             wrap_into_case_end_stringified(&format!(
    //                 "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified}{{is_optional_query_part}}{space_else_space_jsonb_build_object_err_stringified}"
    //             ))
    //         },
    //         &proc_macro_name_upper_camel_case_ident_stringified,
    //     );
    //     let check_optional_query_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("{column_name_and_maybe_field_getter}"), &proc_macro_name_upper_camel_case_ident_stringified);
    //     quote::quote! {
    //         impl postgresql_crud::GeneratePostgresqlQueryPartToRead<
    //             #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream,
    //             #second_generic_token_stream
    //         > for #ident_field_to_read_upper_camel_case_token_stream {
    //             fn generate_postgresql_query_part_to_read_from_self_vec(
    //                 value: &std::vec::Vec<Self>,
    //                 column_name_and_maybe_field_getter: &std::primitive::str,
    //                 column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    //                 is_optional: std::primitive::bool,
    //             ) -> Result<std::string::String, #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream> {
    //                 if value.is_empty() {
    //                     return Err(#ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream::FieldsFilterIsEmpty {
    //                         code_occurence: error_occurence_lib::code_occurence!(),
    //                     });
    //                 }
    //                 let mut unique = vec![];
    //                 for element in value {
    //                     if unique.contains(&element) {
    //                         return Err(#ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream::NotUniqueFieldFilter {
    //                             field: element.clone(),
    //                             code_occurence: error_occurence_lib::code_occurence!(),
    //                         });
    //                     }
    //                     else {
    //                         unique.push(&element);
    //                     }
    //                 }
    //                 let mut acc = std::string::String::default();
    //                 for element in value {
    //                     #acc_push_token_stream
    //                 }
    //                 let _ = acc.pop();
    //                 let _ = acc.pop();
    //                 let is_optional_query_part = match is_optional {
    //                     true => format!(#check_optional_query_token_stream),
    //                     false => std::string::String::default()
    //                 };
    //                 Ok({
    //                     let space_and_not_null = if is_optional {
    //                         #space_and_not_null_quotes_token_stream
    //                     }
    //                     else {
    //                         ""
    //                     };
    //                     format!(
    //                         #query_token_stream
    //                     )
    //                 })
    //             }
    //             fn generate_postgresql_query_part_to_read(
    //                 &self,
    //                 column_name_and_maybe_field_getter: &std::primitive::str,
    //                 column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    //             ) -> Result<std::string::String, #second_generic_token_stream> {
    //                 #postgresql_query_part_content_token_stream
    //             }
    //         }
    //     }
    // };
    // let pub_struct_ident_options_to_read_token_stream = {
    //     let maybe_id_token_stream = if is_id_field_exists {
    //         quote::quote! {
    //             #id_snake_case: std::option::Option<#postgresql_crud_uuid_token_stream>,
    //         }
    //     } else {
    //         proc_macro2::TokenStream::new()
    //     };
    //     let fields_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let type_token_stream = match supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonGeneric(_) => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    //         };
    //         let serde_skip_serializing_if_value_attribute_token_stream = proc_macro_helpers::generate_serde_skip_serializing_if_value_attribute_token_stream::generate_serde_skip_serializing_if_value_attribute_token_stream();
    //         quote::quote! {
    //             #serde_skip_serializing_if_value_attribute_token_stream
    //             #element_ident: std::option::Option<postgresql_crud::Value<#type_token_stream>>
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait//Eq,
    //         pub struct #ident_options_to_read_upper_camel_case_token_stream {
    //             #maybe_id_token_stream
    //             #(#fields_token_stream),*

    //             // std_string_string: std::option::Option<postgresql_crud::Value<std::string::String>>,
    //             // std_vec_vec_std_primitive_bool: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::primitive::bool>>>,
    //             // generic: std::option::Option<postgresql_crud::Value<IdentOptions>>,
    //             // std_option_option_generic: std::option::Option<postgresql_crud::Value<std::option::Option<IdentOptions>>>,
    //             // std_vec_vec_generic: std::option::Option<postgresql_crud::Value<std::vec::Vec<IdentOptions>>>,
    //             // std_option_option_std_vec_vec_generic: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<IdentOptions>>>>,
    //             // std_vec_vec_std_option_option_generic: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::option::Option<IdentOptions>>>>,
    //             // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<std::option::Option<IdentOptions>>>>>,

    //             // std_string_string: std::option::Option<std::result::Result<std::string::String, std::string::String>>,
    //             // std_vec_vec_std_primitive_bool: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>, std::string::String>>,
    //             // generic: std::option::Option<std::result::Result<IdentOptions,std::string::String>>,
    //             // std_option_option_generic: std::option::Option<std::result::Result<std::option::Option<IdentOptions>,std::string::String>>,
    //             // std_vec_vec_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<IdentOptions,std::string::String>>,std::string::String>>,
    //             // std_option_option_std_vec_vec_generic: std::option::Option<std::result::Result<
    //             //     std::option::Option<std::vec::Vec<std::result::Result<IdentOptions,std::string::String>>>,
    //             //     std::string::String
    //             // >>,
    //             // std_vec_vec_std_option_option_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<IdentOptions>,std::string::String>>,std::string::String>>,
    //             // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<
    //             //     std::result::Result<
    //             //         std::option::Option<
    //             //             std::vec::Vec<
    //             //                 std::result::Result<
    //             //                     std::option::Option<IdentOptions>,
    //             //                     std::string::String
    //             //                 >
    //             //             >
    //             //         >,
    //             //         std::string::String
    //             //     >
    //             // >,
    //         }
    //     }
    // };
    // // let impl_std_convert_from_ident_for_ident_options_to_read_token_stream = {
    // //     let maybe_id_token_stream = if is_id_field_exists {
    // //         quote::quote! {
    // //             #id_snake_case: Some(value.#id_snake_case.0),
    // //         }
    // //     } else {
    // //         proc_macro2::TokenStream::new()
    // //     };
    // //     let fields_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    // //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    // //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    // //         });
    // //         let conversion_token_stream = match SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
    // //             SupportedPredefinedType::JsonStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdStringString => quote::quote! {value.#element_ident.0},

    // //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => quote::quote! {
    // //                 match value.#element_ident.0 {
    // //                     Some(value) => Some(value.0),
    // //                     None => None,
    // //                 }
    // //             },

    // //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdVecVecStdStringString => quote::quote! {value.#element_ident.0.into_iter().map(|element|element.0).collect()},

    // //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => quote::quote! {
    // //                 match value.#element_ident.0 {
    // //                     Some(value) => Some(value.into_iter().map(|element|element.0).collect()),
    // //                     None => None
    // //                 }
    // //             },

    // //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => quote::quote! {
    // //                 value.#element_ident.0.into_iter().map(|element|match element {
    // //                     Some(value) => Some(value.0),
    // //                     None => None
    // //                 }).collect()
    // //             },

    // //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    // //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote! {
    // //                 match value.#element_ident.0 {
    // //                     Some(value) => Some(value.into_iter().map(|element|match element {
    // //                         Some(value) => Some(value.0),
    // //                         None => None
    // //                     }).collect()),
    // //                     None => None
    // //                 }
    // //             },

    // //             SupportedPredefinedType::JsonGeneric(type_path) => {
    // //                 let generic_ident_options_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&type_path);
    // //                 quote::quote! {
    // //                     #generic_ident_options_upper_camel_case_token_stream::from(value.#element_ident.0)
    // //                 }
    // //             }
    // //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    // //                 let generic_ident_options_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&type_path);
    // //                 quote::quote! {
    // //                     match value.#element_ident.0 {
    // //                         Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
    // //                         None => None,
    // //                     }
    // //                 }
    // //             }
    // //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    // //                 let generic_ident_options_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&type_path);
    // //                 quote::quote! {
    // //                     value.#element_ident.0.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()
    // //                 }
    // //             }
    // //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    // //                 let generic_ident_options_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&type_path);
    // //                 quote::quote! {
    // //                     match value.#element_ident.0 {
    // //                         Some(value) => Some(value.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()),
    // //                         None => None
    // //                     }
    // //                 }
    // //             }

    // //             SupportedPredefinedType::JsonUuid => quote::quote! {value.#element_ident.0},
    // //         };
    // //         quote::quote! {
    // //             #element_ident: Some(postgresql_crud::Value{ value: #conversion_token_stream })
    // //         }
    // //     });
    // //     quote::quote! {
    // //         impl std::convert::From<#ident> for #ident_options_to_read_upper_camel_case_token_stream {
    // //             fn from(value: #ident) -> Self {
    // //                 Self {
    // //                     #maybe_id_token_stream
    // //                     #(#fields_token_stream),*

    // //                     //just for case if must return result impl
    // //                     // // std_string_string: Some(std::result::Result::Ok(value.std_string_string.0)),
    // //                     // // std_vec_vec_std_primitive_bool: Some(std::result::Result::Ok(
    // //                     // //     value.std_vec_vec_std_primitive_bool.0.into_iter().map(|element|
    // //                     // //         std::result::Result::Ok(element)
    // //                     // //     ).collect::<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>()
    // //                     // // )),
    // //                     // // generic: Some(std::result::Result::Ok(IdentOptions::from(value.generic.0))),
    // //                     // // std_option_option_generic: Some(std::result::Result::Ok(Some(match value.std_option_option_generic.0 {
    // //                     // //     Some(value) => IdentOptions {
    // //                     // //         std_string_string: Some(postgresql_crud::Value{ value: value.std_string_string.0 }),
    // //                     // //     },
    // //                     // //     None => IdentOptions {
    // //                     // //         std_string_string: None,
    // //                     // //     },
    // //                     // // }))),
    // //                     // // std_vec_vec_generic: Some(std::result::Result::Ok(value.std_vec_vec_generic.0.into_iter().map(|element|std::result::Result::Ok(IdentOptions::from(element))).collect::<std::vec::Vec<std::result::Result<IdentOptions,std::string::String>>>())),
    // //                     // // std_option_option_std_vec_vec_generic: Some(std::result::Result::Ok(match value.std_option_option_std_vec_vec_generic.0 {
    // //                     // //         Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(IdentOptions::from(element))).collect::<std::vec::Vec<std::result::Result<IdentOptions,std::string::String>>>()),
    // //                     // //         None => None
    // //                     // // })),
    // //                     // // std_vec_vec_std_option_option_generic: Some(std::result::Result::Ok(value.std_vec_vec_std_option_option_generic.0.into_iter().map(|element|std::result::Result::Ok(match element {
    // //                     // //     Some(value) => Some(IdentOptions::from(value)),
    // //                     // //     None => None
    // //                     // // })).collect::<std::vec::Vec<std::result::Result<std::option::Option<IdentOptions>,std::string::String>>>())),
    // //                     // // std_option_option_std_vec_vec_std_option_option_generic: Some(
    // //                     // //     std::result::Result::Ok(
    // //                     // //         match value.std_option_option_std_vec_vec_std_option_option_generic.0 {
    // //                     // //             Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(match element {
    // //                     // //                 Some(value) => Some(IdentOptions::from(value)),
    // //                     // //                 None => None
    // //                     // //             })).collect::<std::vec::Vec<std::result::Result<std::option::Option<IdentOptions>,std::string::String>>>()),
    // //                     // //             None => None
    // //                     // //         }
    // //                     // //     )
    // //                     // // ),
    // //                 }
    // //             }
    // //         }
    // //     }
    // // };
    // let impl_serde_deserialize_for_ident_options_to_read_token_stream = {
    //     let field_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)| {
    //         let value = format!("__{}{index}", naming_conventions::FieldSnakeCase,);
    //         value
    //             .parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //     });
    //     let generate_field_index_token_stream = |index: std::primitive::usize| {
    //         let value = format!("__field{index}");
    //         value
    //             .parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //     };
    //     let visit_u64_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)| {
    //         let index_u64_token_stream = {
    //             let value = format!("{index}u64");
    //             value
    //                 .parse::<proc_macro2::TokenStream>()
    //                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         quote::quote! {
    //             #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
    //         }
    //     });
    //     let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_name_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //             &{
    //                 element
    //                     .ident
    //                     .as_ref()
    //                     .unwrap_or_else(|| {
    //                         panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //                     })
    //                     .to_string()
    //             },
    //             &proc_macro_name_upper_camel_case_ident_stringified,
    //         );
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         quote::quote! {
    //             #field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
    //         }
    //     });
    //     let visit_bytes_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let b_field_name_double_quotes_token_stream = {
    //             let element_ident_double_quotes_stringified = proc_macro_common::generate_quotes::double_quotes_stringified(
    //                 &element
    //                     .ident
    //                     .as_ref()
    //                     .unwrap_or_else(|| {
    //                         panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //                     })
    //                     .to_string(),
    //             );
    //             let value = format!("b{element_ident_double_quotes_stringified}");
    //             value
    //                 .parse::<proc_macro2::TokenStream>()
    //                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         quote::quote! {
    //             #b_field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
    //         }
    //     });
    //     let struct_ident_options_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case_stringified}"), &proc_macro_name_upper_camel_case_ident_stringified);
    //     let struct_ident_options_with_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case_stringified} with {} elements", vec_syn_field.len()), &proc_macro_name_upper_camel_case_ident_stringified);
    //     let generate_type_token_stream = |value: &syn::Field| {
    //         let supported_predefined_type = SupportedPredefinedType::try_from(value).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),

    //             SupportedPredefinedType::JsonGeneric(_) => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),

    //             SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    //         }
    //     };
    //     let visit_seq_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         let index_usize_token_stream = {
    //             let value = format!("{index}usize");
    //             value
    //                 .parse::<proc_macro2::TokenStream>()
    //                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let type_token_stream = generate_type_token_stream(element);
    //         quote::quote! {
    //             let #field_index_token_stream = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<#type_token_stream, std::string::String>>>(&mut __seq)? {
    //                 serde::__private::Some(__value) => __value,
    //                 serde::__private::None => {
    //                     return serde::__private::Err(
    //                         serde::de::Error::invalid_length(
    //                             #index_usize_token_stream,
    //                             &#struct_ident_options_with_double_quotes_token_stream,
    //                         ),
    //                     );
    //                 }
    //             };
    //         }
    //     });
    //     let visit_seq_fields_assignment_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         let conversion_logic_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString => quote::quote! {value},

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => quote::quote! {value},

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString => quote::quote! {
    //                 {
    //                     let mut acc = vec![];
    //                     for element in value {
    //                         match element {
    //                             Ok(value) => {
    //                                 acc.push(value);
    //                             }
    //                             Err(error) => {
    //                                 return Err(serde::de::Error::custom(error));
    //                             }
    //                         }
    //                     }
    //                     acc
    //                 }
    //             },

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => quote::quote! {
    //                 match value {
    //                     Some(value) => {
    //                         let mut acc = vec![];
    //                         for element in value {
    //                             match element {
    //                                 Ok(value) => {
    //                                     acc.push(value);
    //                                 }
    //                                 Err(error) => {
    //                                     return Err(serde::de::Error::custom(error));
    //                                 }
    //                             }
    //                         }
    //                         Some(acc)
    //                     }
    //                     None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => quote::quote! {
    //                 {
    //                     let mut acc = vec![];
    //                     for element in value {
    //                         match element {
    //                             Ok(value) => {
    //                                 acc.push(value);
    //                             }
    //                             Err(error) => {
    //                                 return Err(serde::de::Error::custom(error));
    //                             }
    //                         }
    //                     }
    //                     acc
    //                 }
    //             },

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote! {
    //                 match value {
    //                     Some(value) => {
    //                         let mut acc = vec![];
    //                         for element in value {
    //                             match element {
    //                                 Ok(value) => {
    //                                     acc.push(value);
    //                                 }
    //                                 Err(error) => {
    //                                     return Err(serde::de::Error::custom(error));
    //                                 }
    //                             }
    //                         }
    //                         Some(acc)
    //                     }
    //                     None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonGeneric(_) => quote::quote! {value},
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => quote::quote! {value},

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => quote::quote! {
    //                 {
    //                     let mut acc = vec![];
    //                     for element in value {
    //                         match element {
    //                             Ok(value) => {
    //                                 acc.push(value);
    //                             }
    //                             Err(error) => {
    //                                 return Err(serde::de::Error::custom(error));
    //                             }
    //                         }
    //                     }
    //                     acc
    //                 }
    //             },
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => quote::quote! {
    //                 match value {
    //                     Some(value) => {
    //                         let mut acc = vec![];
    //                         for element in value {
    //                             match element {
    //                                 Ok(value) => {
    //                                     acc.push(value);
    //                                 }
    //                                 Err(error) => {
    //                                     return Err(serde::de::Error::custom(error));
    //                                 }
    //                             }
    //                         }
    //                         Some(acc)
    //                     }
    //                     None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonUuid => quote::quote! {value},
    //         };
    //         let ok_some_content_token_stream = if field_ident == &id_snake_case.to_string() {
    //             conversion_logic_token_stream
    //         } else {
    //             quote::quote! {
    //                 postgresql_crud::Value{ value: #conversion_logic_token_stream }
    //             }
    //         };
    //         quote::quote! {
    //             #field_ident: match #field_index_token_stream {
    //                 Some(value) => match value {
    //                     Ok(value) => Some(#ok_some_content_token_stream),
    //                     Err(error) => {
    //                         return Err(serde::de::Error::custom(error));
    //                     }
    //                 },
    //                 None => None
    //             }
    //         }
    //     });
    //     //its just #(#visit_seq_fields_assignment_token_stream),* reusage making move error
    //     let visit_seq_fields_assignment_handle_token_stream = quote::quote! {#(#visit_seq_fields_assignment_token_stream),*};
    //     let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         let type_token_stream = generate_type_token_stream(element);
    //         quote::quote! {
    //             let mut #field_index_token_stream: serde::__private::Option<
    //                 std::option::Option<
    //                     std::result::Result<#type_token_stream, std::string::String>,
    //                 >,
    //             > = serde::__private::None;
    //         }
    //     });
    //     let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
    //         proc_macro_common::generate_quotes::double_quotes_token_stream(
    //             &value
    //                 .ident
    //                 .as_ref()
    //                 .unwrap_or_else(|| {
    //                     panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //                 })
    //                 .to_string(),
    //             &proc_macro_name_upper_camel_case_ident_stringified,
    //         )
    //     };
    //     let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
    //         let type_token_stream = generate_type_token_stream(element);
    //         quote::quote! {
    //             __Field::#field_index_token_stream => {
    //                 if serde::__private::Option::is_some(&#field_index_token_stream) {
    //                     return serde::__private::Err(
    //                         <__A::Error as serde::de::Error>::duplicate_field(
    //                             #field_ident_double_quotes_token_stream,
    //                         ),
    //                     );
    //                 }
    //                 #field_index_token_stream = serde::__private::Some(
    //                     serde::de::MapAccess::next_value::<
    //                         std::option::Option<
    //                             std::result::Result<
    //                                 #type_token_stream,
    //                                 std::string::String,
    //                             >,
    //                         >,
    //                     >(&mut __map)?,
    //                 );
    //             }
    //         }
    //     });
    //     let visit_map_missing_fields_check_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
    //         let field_index_token_stream = generate_field_index_token_stream(index);
    //         let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
    //         quote::quote! {
    //             let #field_index_token_stream = match #field_index_token_stream {
    //                 serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
    //                 serde::__private::None => {
    //                     serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
    //                 }
    //             };
    //         }
    //     });
    //     let fields_array_elements_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(&element));
    //     quote::quote! {
    //         impl<'de> serde::Deserialize<'de> for #ident_options_to_read_upper_camel_case_token_stream {
    //             fn deserialize<__D>(
    //                 __deserializer: __D,
    //             ) -> serde::__private::Result<Self, __D::Error>
    //             where
    //                 __D: serde::Deserializer<'de>,
    //             {
    //                 #[allow(non_camel_case_types)]
    //                 #[doc(hidden)]
    //                 enum __Field {
    //                     #(#field_enum_variants_token_stream),*,
    //                     __ignore,
    //                 }
    //                 #[doc(hidden)]
    //                 struct __FieldVisitor;
    //                 impl serde::de::Visitor<'_> for __FieldVisitor {
    //                     type Value = __Field;
    //                     fn expecting(
    //                         &self,
    //                         __formatter: &mut serde::__private::Formatter<'_>,
    //                     ) -> serde::__private::fmt::Result {
    //                         serde::__private::Formatter::write_str(
    //                             __formatter,
    //                             "field identifier",
    //                         )
    //                     }
    //                     fn visit_u64<__E>(
    //                         self,
    //                         __value: u64,
    //                     ) -> serde::__private::Result<Self::Value, __E>
    //                     where
    //                         __E: serde::de::Error,
    //                     {
    //                         match __value {
    //                             #(#visit_u64_value_enum_variants_token_stream),*,
    //                             _ => serde::__private::Ok(__Field::__ignore),
    //                         }
    //                     }
    //                     fn visit_str<__E>(
    //                         self,
    //                         __value: &str,
    //                     ) -> serde::__private::Result<Self::Value, __E>
    //                     where
    //                         __E: serde::de::Error,
    //                     {
    //                         match __value {
    //                             #(#visit_str_value_enum_variants_token_stream),*,
    //                             _ => serde::__private::Ok(__Field::__ignore),
    //                         }
    //                     }
    //                     fn visit_bytes<__E>(
    //                         self,
    //                         __value: &[u8],
    //                     ) -> serde::__private::Result<Self::Value, __E>
    //                     where
    //                         __E: serde::de::Error,
    //                     {
    //                         match __value {
    //                             #(#visit_bytes_value_enum_variants_token_stream),*,
    //                             _ => serde::__private::Ok(__Field::__ignore),
    //                         }
    //                     }
    //                 }
    //                 impl<'de> serde::Deserialize<'de> for __Field {
    //                     #[inline]
    //                     fn deserialize<__D>(
    //                         __deserializer: __D,
    //                     ) -> serde::__private::Result<Self, __D::Error>
    //                     where
    //                         __D: serde::Deserializer<'de>,
    //                     {
    //                         serde::Deserializer::deserialize_identifier(
    //                             __deserializer,
    //                             __FieldVisitor,
    //                         )
    //                     }
    //                 }
    //                 #[doc(hidden)]
    //                 struct __Visitor<'de> {
    //                     marker: serde::__private::PhantomData<#ident_options_to_read_upper_camel_case_token_stream>,
    //                     lifetime: serde::__private::PhantomData<&'de ()>,
    //                 }
    //                 impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
    //                     type Value = #ident_options_to_read_upper_camel_case_token_stream;
    //                     fn expecting(
    //                         &self,
    //                         __formatter: &mut serde::__private::Formatter<'_>,
    //                     ) -> serde::__private::fmt::Result {
    //                         serde::__private::Formatter::write_str(
    //                             __formatter,
    //                             #struct_ident_options_double_quotes_token_stream,
    //                         )
    //                     }
    //                     #[inline]
    //                     fn visit_seq<__A>(
    //                         self,
    //                         mut __seq: __A,
    //                     ) -> serde::__private::Result<Self::Value, __A::Error>
    //                     where
    //                         __A: serde::de::SeqAccess<'de>,
    //                     {
    //                         #(#visit_seq_fields_initialization_token_stream)*
    //                         serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream {
    //                             #visit_seq_fields_assignment_handle_token_stream
    //                         })
    //                     }
    //                     #[inline]
    //                     fn visit_map<__A>(
    //                         self,
    //                         mut __map: __A,
    //                     ) -> serde::__private::Result<Self::Value, __A::Error>
    //                     where
    //                         __A: serde::de::MapAccess<'de>,
    //                     {
    //                         #(#visit_map_fields_initialization_token_stream)*
    //                         while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
    //                             __Field,
    //                         >(&mut __map)? {
    //                             match __key {
    //                                 #(#visit_map_match_variants_token_stream),*
    //                                 _ => {
    //                                     let _ = serde::de::MapAccess::next_value::<
    //                                         serde::de::IgnoredAny,
    //                                     >(&mut __map)?;
    //                                 }
    //                             }
    //                         }
    //                         #(#visit_map_missing_fields_check_token_stream)*
    //                         serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream {
    //                             #visit_seq_fields_assignment_handle_token_stream
    //                         })
    //                     }
    //                 }
    //                 #[doc(hidden)]
    //                 const FIELDS: &'static [&'static str] = &[
    //                     #(#fields_array_elements_token_stream),*
    //                 ];
    //                 serde::Deserializer::deserialize_struct(
    //                     __deserializer,
    //                     #ident_options_to_read_double_quotes_token_stream,
    //                     FIELDS,
    //                     __Visitor {
    //                         marker: serde::__private::PhantomData::<#ident_options_to_read_upper_camel_case_token_stream>,
    //                         lifetime: serde::__private::PhantomData,
    //                     },
    //                 )
    //             }
    //         }
    //     }
    // };
    // let ident_reader_token_stream = {
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait //, serde::Deserialize//Eq,
    //         pub struct #ident_reader_upper_camel_case_token_stream(pub #ident_options_to_read_upper_camel_case_token_stream);
    //     }
    // };
    // let impl_serde_deserialize_for_ident_reader_token_stream = {
    //     let tuple_struct_space_stringified = "tuple struct ";
    //     let tuple_struct_ident_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("{tuple_struct_space_stringified}{ident_reader_upper_camel_case_stringified}"), &proc_macro_name_upper_camel_case_ident_stringified);
    //     let tuple_struct_ident_reader_with_1_element_double_quotes_token_stream =
    //         proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("{tuple_struct_space_stringified}{ident_reader_upper_camel_case_stringified} with 1 element"), &proc_macro_name_upper_camel_case_ident_stringified);
    //     let ident_wrapper_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&ident_reader_upper_camel_case_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
    //     quote::quote! {
    //         impl<'de> serde::Deserialize<'de> for #ident_reader_upper_camel_case_token_stream {
    //             fn deserialize<__D>(
    //                 __deserializer: __D,
    //             ) -> serde::__private::Result<Self, __D::Error>
    //             where
    //                 __D: serde::Deserializer<'de>,
    //             {
    //                 #[doc(hidden)]
    //                 struct __Visitor<'de> {
    //                     marker: serde::__private::PhantomData<#ident_reader_upper_camel_case_token_stream>,
    //                     lifetime: serde::__private::PhantomData<&'de ()>,
    //                 }
    //                 impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
    //                     type Value = #ident_reader_upper_camel_case_token_stream;
    //                     fn expecting(
    //                         &self,
    //                         __formatter: &mut serde::__private::Formatter<'_>,
    //                     ) -> serde::__private::fmt::Result {
    //                         serde::__private::Formatter::write_str(
    //                             __formatter,
    //                             #tuple_struct_ident_reader_double_quotes_token_stream,
    //                         )
    //                     }
    //                     #[inline]
    //                     fn visit_newtype_struct<__E>(
    //                         self,
    //                         __e: __E,
    //                     ) -> serde::__private::Result<Self::Value, __E::Error>
    //                     where
    //                         __E: serde::Deserializer<'de>,
    //                     {
    //                         let __field0: Result<#ident_options_to_read_upper_camel_case_token_stream, std::string::String> = <Result<
    //                             #ident_options_to_read_upper_camel_case_token_stream,
    //                             std::string::String,
    //                         > as serde::Deserialize>::deserialize(__e)?;
    //                         serde::__private::Ok(#ident_reader_upper_camel_case_token_stream(match __field0 {
    //                             Ok(value) => value,
    //                             Err(error) => {
    //                                 return Err(serde::de::Error::custom(error));
    //                             }
    //                         }))
    //                     }
    //                     #[inline]
    //                     fn visit_seq<__A>(
    //                         self,
    //                         mut __seq: __A,
    //                     ) -> serde::__private::Result<Self::Value, __A::Error>
    //                     where
    //                         __A: serde::de::SeqAccess<'de>,
    //                     {
    //                         let __field0 = match serde::de::SeqAccess::next_element::<
    //                             Result<#ident_options_to_read_upper_camel_case_token_stream, std::string::String>,
    //                         >(&mut __seq)? {
    //                             serde::__private::Some(__value) => __value,
    //                             serde::__private::None => {
    //                                 return serde::__private::Err(
    //                                     serde::de::Error::invalid_length(
    //                                         0usize,
    //                                         &#tuple_struct_ident_reader_with_1_element_double_quotes_token_stream,
    //                                     ),
    //                                 );
    //                             }
    //                         };
    //                         serde::__private::Ok(#ident_reader_upper_camel_case_token_stream(match __field0 {
    //                             Ok(value) => value,
    //                             Err(error) => {
    //                                 return Err(serde::de::Error::custom(error));
    //                             }
    //                         }))
    //                     }
    //                 }
    //                 serde::Deserializer::deserialize_newtype_struct(
    //                     __deserializer,
    //                     #ident_wrapper_double_quotes_token_stream,
    //                     __Visitor {
    //                         marker: serde::__private::PhantomData::<#ident_reader_upper_camel_case_token_stream>,
    //                         lifetime: serde::__private::PhantomData,
    //                     },
    //                 )
    //             }
    //         }
    //     }
    // };
    // let impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_token_stream = {
    //     let fields_initialization_token_stream = vec_syn_field.iter().map(|element| {
    //         let field_ident = element.ident.as_ref().unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE));
    //         quote::quote! {
    //             #field_ident: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //         }
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
    //             #[inline]
    //             fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
    //                 Self {
    //                     #(#fields_initialization_token_stream),*
    //                 }
    //             }
    //         }
    //     }
    // };
    // let pub_enum_ident_option_to_update_token_stream = {
    //     let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let type_token_stream = match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 let value = format!("{}{}", quote::quote! {#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase);
    //                 value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let type_path_options_to_update_upper_camel_case = {
    //                     let value = format!("{}{}", quote::quote! {#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {std::option::Option<#type_path_options_to_update_upper_camel_case>}
    //             }

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 let type_path_options_to_update_upper_camel_case_token_stream = {
    //                     let value = format!("{}{}", quote::quote! {#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase,);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     postgresql_crud::JsonArrayChange<#type_path_to_create_upper_camel_case_token_stream, #type_path_options_to_update_upper_camel_case_token_stream>
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 let type_path_options_to_update_upper_camel_case_token_stream = {
    //                     let value = format!("{}{}", quote::quote! {#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase,);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     std::option::Option<postgresql_crud::JsonArrayChange<#type_path_to_create_upper_camel_case_token_stream, #type_path_options_to_update_upper_camel_case_token_stream>>
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    //         };
    //         quote::quote! {
    //             #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
    //             #element_ident_upper_camel_case_token_stream(postgresql_crud::Value<#type_token_stream>)
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    //         pub enum #ident_option_to_update_upper_camel_case_token_stream {
    //             #(#variants_token_stream),*
    //         }
    //     }
    // };
    // let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream = {
    //     let elements_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let supported_predefined_type_variant_name_token_stream = {
    //             let value = supported_predefined_type.to_variant_name_stringified();
    //             value
    //                 .parse::<proc_macro2::TokenStream>()
    //                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let value_content_token_stream = match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString => quote::quote! {
    //                 <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0
    //             },

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString => quote::quote! {
    //                 match <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
    //                     Some(value) => Some(value.0),
    //                     None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString => quote::quote! {
    //                 <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0
    //                 .into_iter()
    //                 .map(|element|element.0).collect()
    //             },

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => quote::quote! {
    //                 match <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
    //                     Some(value) => Some(value.into_iter().map(|element|element.0).collect()),
    //                     None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => quote::quote! {
    //                 <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0
    //                 .into_iter().map(|element|match element {
    //                     Some(value) => Some(value.0),
    //                     None => None
    //                 }).collect()
    //             },

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote! {
    //                 match <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
    //                    Some(value) => Some(
    //                        value.into_iter().map(|element|match element {
    //                            Some(value) => Some(value.0),
    //                            None => None
    //                        }).collect()
    //                    ),
    //                    None => None
    //                 }
    //             },

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 quote::quote! {
    //                     postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 quote::quote! {
    //                     Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    //                 }
    //             }

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 quote::quote! {
    //                     postgresql_crud::JsonArrayChange {
    //                         create: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ],
    //                         update: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ],
    //                         delete: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ]
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 quote::quote! {
    //                     Some(postgresql_crud::JsonArrayChange {
    //                         create: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ],
    //                         update: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ],
    //                         delete: vec![
    //                             postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //                         ]
    //                     })
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => quote::quote! {
    //                 <postgresql_crud::#supported_predefined_type_variant_name_token_stream as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0
    //             },
    //         };
    //         quote::quote! {
    //             #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(postgresql_crud::Value{
    //                 value: #value_content_token_stream
    //             })
    //         }
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_option_to_update_upper_camel_case_token_stream {
    //             fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<#ident_option_to_update_upper_camel_case_token_stream> {
    //                 vec![
    //                     #(#elements_token_stream),*
    //                 ]
    //             }
    //         }
    //     }
    // };
    // let pub_struct_ident_options_to_update_token_stream = {
    //     let content_token_stream = if is_id_field_exists {
    //         let fields_snake_case = naming_conventions::FieldsSnakeCase;
    //         quote::quote! {
    //             {
    //                 pub #id_snake_case: postgresql_crud::JsonUuid,
    //                 pub #fields_snake_case: std::vec::Vec<#ident_option_to_update_upper_camel_case_token_stream>,
    //             }
    //         }
    //     } else {
    //         quote::quote! {
    //             (pub std::vec::Vec<#ident_option_to_update_upper_camel_case_token_stream>);
    //         }
    //     };
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    //         pub struct #ident_options_to_update_upper_camel_case_token_stream #content_token_stream
    //     }
    // };
    // let maybe_impl_postgresql_crud_get_json_id_for_ident_options_to_update_token_stream = if is_id_field_exists {
    //     quote::quote! {
    //         impl postgresql_crud::GetJsonId for #ident_options_to_update_upper_camel_case_token_stream {
    //             fn get_json_id(&self) -> &postgresql_crud::JsonUuid {
    //                 &self.id
    //             }
    //         }
    //     }
    // } else {
    //     proc_macro2::TokenStream::new()
    // };
    // let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream = {
    //     let self_initialization_content_token_stream = if is_id_field_exists {
    //         quote::quote! {
    //             {
    //                 id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    //                 fields: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //             }
    //         }
    //     } else {
    //         quote::quote! {
    //             (postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    //         }
    //     };
    //     quote::quote! {
    //         impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_options_to_update_upper_camel_case_token_stream {
    //             #[inline]
    //             fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
    //                 Self #self_initialization_content_token_stream
    //             }
    //         }
    //     }
    // };
    // let pub_enum_ident_field_to_update_token_stream = {
    //     let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         quote::quote! {
    //             #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
    //             #element_ident_upper_camel_case_token_stream
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(
    //             Debug,
    //             Clone,
    //             PartialEq,
    //             serde::Serialize,
    //             serde::Deserialize,
    //             utoipa::ToSchema,
    //             schemars::JsonSchema,
    //         )]
    //         pub enum #ident_field_to_update_upper_camel_case_token_stream {
    //             #(#variants_token_stream),*
    //         }
    //     }
    // };
    // let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream = {
    //     let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         quote::quote! {
    //             Self::#element_ident_upper_camel_case_token_stream => #element_ident_double_quotes_token_stream.to_owned()
    //         }
    //     });
    //     quote::quote! {
    //         impl error_occurence_lib::ToStdStringString for #ident_field_to_update_upper_camel_case_token_stream {
    //             fn to_std_string_string(&self) -> std::string::String {
    //                 match &self {
    //                     #(#variants_token_stream),*
    //                 }
    //             }
    //         }
    //     }
    // };
    // let pub_enum_ident_options_to_update_try_generate_bind_increments_error_named_token_stream = {
    //     let mut acc = vec![];
    //     vec_syn_field_filtered_id_iter.iter().for_each(|element| {
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => (),

    //             SupportedPredefinedType::JsonGeneric(_) => {
    //                 if !acc.contains(&element) {
    //                     // acc.push(type_path.clone());
    //                     acc.push(element);
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => {
    //                 if !acc.contains(&element) {
    //                     // acc.push(type_path.clone());
    //                     acc.push(element);
    //                 }
    //             }

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => {
    //                 if !acc.contains(&element) {
    //                     // acc.push(type_path.clone());
    //                     acc.push(element);
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => {
    //                 if !acc.contains(&element) {
    //                     // acc.push(type_path.clone());
    //                     acc.push(element);
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => (),
    //         }
    //     });
    //     let additional_generic_variants = acc.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let supported_predefined_type = SupportedPredefinedType::try_from(***element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => panic!("{proc_macro_name_upper_camel_case_ident_stringified} unreachable variant"),

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 let value = quote::quote! {#type_path}.to_string();
    //                 let generic_ident_token_stream = value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 let generic_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&value);
    //                 let generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream = {
    //                     let value = format!("{value}{}", naming_conventions::OptionsToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     #generic_ident_token_stream {
    //                         #[eo_error_occurence]
    //                         #generic_ident_snake_case_token_stream: #generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let value = quote::quote! {#type_path}.to_string();
    //                 let generic_ident_token_stream = value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 let generic_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&value);
    //                 let generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream = {
    //                     let value = format!("{value}{}", naming_conventions::OptionsToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     #generic_ident_token_stream {
    //                         #[eo_error_occurence]
    //                         #generic_ident_snake_case_token_stream: #generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 let generic_ident_stringified = quote::quote! {#type_path}.to_string();
    //                 let generic_ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&generic_ident_stringified);
    //                 let generic_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&generic_ident_stringified);
    //                 let element_ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element_ident.to_string());
    //                 let element_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element_ident.to_string());
    //                 let element_ident_generic_ident_not_unique_id_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::NotUniqueIdUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_not_unique_id_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::NotUniqueIdSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };

    //                 let element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let generic_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream = generate_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream(&generic_ident_upper_camel_case_stringified);

    //                 let element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let try_generate_json_array_element_delete_bind_increments_error_named_upper_camel_case_token_stream = {
    //                     let value = naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamedUpperCamelCase.to_string();
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };

    //                 let element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let try_generate_json_array_element_create_bind_increments_error_named_upper_camel_case_token_stream = {
    //                     let value = naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamedUpperCamelCase.to_string();
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     #element_ident_generic_ident_not_unique_id_upper_camel_case_token_stream {
    //                         #[eo_to_std_string_string_serialize_deserialize]
    //                         #element_ident_generic_ident_not_unique_id_snake_case_token_stream: postgresql_crud::JsonUuid,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_snake_case_token_stream: #generic_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream: postgresql_crud::#try_generate_json_array_element_delete_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_snake_case_token_stream: postgresql_crud::#try_generate_json_array_element_create_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let generic_ident_stringified = quote::quote! {#type_path}.to_string();
    //                 let generic_ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&generic_ident_stringified);
    //                 let generic_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&generic_ident_stringified);
    //                 let element_ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element_ident.to_string());
    //                 let element_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element_ident.to_string());
    //                 let element_ident_generic_ident_not_unique_id_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::NotUniqueIdUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_not_unique_id_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::NotUniqueIdSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };

    //                 let element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let generic_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream = generate_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream(&generic_ident_upper_camel_case_stringified);

    //                 let element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let try_generate_json_array_element_delete_bind_increments_error_named_upper_camel_case_token_stream = {
    //                     let value = naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamedUpperCamelCase.to_string();
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };

    //                 let element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream = {
    //                     let value = format!("{element_ident_upper_camel_case_stringified}{generic_ident_upper_camel_case_stringified}{}", naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsUpperCamelCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_snake_case_token_stream = {
    //                     let value = format!("{element_ident_snake_case_stringified}_{generic_ident_snake_case_stringified}_{}", naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsSnakeCase);
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 let try_generate_json_array_element_create_bind_increments_error_named_upper_camel_case_token_stream = {
    //                     let value = naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamedUpperCamelCase.to_string();
    //                     value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //                 };
    //                 quote::quote! {
    //                     #element_ident_generic_ident_not_unique_id_upper_camel_case_token_stream {
    //                         #[eo_to_std_string_string_serialize_deserialize]
    //                         #element_ident_generic_ident_not_unique_id_snake_case_token_stream: postgresql_crud::JsonUuid,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_update_bind_increments_snake_case_token_stream: #generic_ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream: postgresql_crud::#try_generate_json_array_element_delete_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     },
    //                     #element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream {
    //                         #[eo_error_occurence]
    //                         #element_ident_generic_ident_try_generate_json_array_element_create_bind_increments_snake_case_token_stream: postgresql_crud::#try_generate_json_array_element_create_bind_increments_error_named_upper_camel_case_token_stream,
    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} unreachable variant"),
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
    //         pub enum #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream {
    //             FieldsIsEmpty {
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             NotUniqueField {
    //                 #[eo_to_std_string_string_serialize_deserialize]
    //                 field: #ident_field_to_update_upper_camel_case_token_stream,
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             CheckedAdd {
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             #(#additional_generic_variants),*
    //         }
    //     }
    // };
    // let maybe_impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_options_to_update_try_generate_bind_increments_error_named_for_ident_options_to_update_token_stream = if is_id_field_exists {
    //     proc_macro2::TokenStream::new()
    // } else {
    //     let check_not_unique_field_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         quote::quote! {
    //             #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
    //                 let value = #ident_field_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream;
    //                 if acc.contains(&value) {
    //                     return Err(
    //                         #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::NotUniqueField {
    //                             field: value,
    //                             code_occurence: error_occurence_lib::code_occurence!(),
    //                         },
    //                     );
    //                 }
    //                 else {
    //                     acc.push(value);
    //                 }
    //             }
    //         }
    //     });
    //     let query_part_generation_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!(
    //                 "{proc_macro_name_upper_camel_case_ident_stringified} {}",
    //                 naming_conventions::FIELD_IDENT_IS_NONE
    //             );
    //         });
    //         let element_ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element_ident.to_string());
    //         let element_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element_ident.to_string());
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let generate_element_ident_type_path_not_unique_id_upper_camel_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_upper_camel_case_stringified}{}{}",
    //                 proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::NotUniqueIdUpperCamelCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_not_unique_id_snake_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_snake_case_stringified}_{}_{}",
    //                 proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::NotUniqueIdSnakeCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_upper_camel_case_stringified}{}{}",
    //                 proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsUpperCamelCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_snake_case_stringified}_{}_{}",
    //                 proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementUpdateBindIncrementsSnakeCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_upper_camel_case_stringified}{}{}",
    //                 proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsUpperCamelCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_snake_case_stringified}_{}_{}",
    //                 proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementDeleteBindIncrementsSnakeCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_upper_camel_case_stringified}{}{}",
    //                 proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsUpperCamelCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream = |type_path: &syn::TypePath|{
    //             let value = format!(
    //                 "{element_ident_snake_case_stringified}_{}_{}",
    //                 proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&quote::quote!{#type_path}.to_string()),
    //                 naming_conventions::TryGenerateJsonArrayElementCreateBindIncrementsSnakeCase
    //             );
    //             value.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //         };
    //         let current_jsonb_set_target_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //             &format!("{{jsonb_set_target}}->'{element_ident}'"),
    //             &proc_macro_name_upper_camel_case_ident_stringified
    //         );
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let content_token_stream = match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdStringString |

    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdOptionOptionStdStringString |

    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdVecVecStdStringString |

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("jsonb_set({{acc}},'{{{{{{previous_jsonb_set_path}}{element_ident}}}}}',${{increment}})"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 quote::quote!{
    //                     (_) => match increment.checked_add(1) {
    //                         Some(value) => {
    //                             *increment = value;
    //                             acc = format!(#format_handle_token_stream);
    //                         }
    //                         None => {
    //                             return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                 code_occurence: error_occurence_lib::code_occurence!()
    //                             });
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &element_ident.to_string(),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let element_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 quote::quote!{
    //                     (value) => {
    //                         match value.value.try_generate_bind_increments(
    //                             &jsonb_set_accumulator,
    //                             &jsonb_set_target,
    //                             &jsonb_set_path,
    //                             increment,
    //                             is_array_object_element.clone(),
    //                         ) {
    //                             Ok(value) => {
    //                                 acc = value;
    //                             }
    //                             Err(error) => {
    //                                 return Err(
    //                                     #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#type_path {
    //                                         #element_ident_snake_case_token_stream: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             },
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let jsonb_set_path_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("{{previous_jsonb_set_path}}{element_ident}"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let type_path_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 let checked_add_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("jsonb_set({{acc}},'{{previous_jsonb_set_path}}{element_ident}',${{increment}})"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 quote::quote!{
    //                     (value) => {
    //                         match &value.value {
    //                             Some(value) => match value.try_generate_bind_increments(
    //                                 &jsonb_set_accumulator,
    //                                 &jsonb_set_target,
    //                                 &format!(#jsonb_set_path_format_handle_token_stream),
    //                                 increment,
    //                                 is_array_object_element.clone(),
    //                             ) {
    //                                 Ok(value) => {
    //                                     acc = value;
    //                                 }
    //                                 Err(error) => {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#type_path {
    //                                         #type_path_snake_case_token_stream: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             },
    //                             None => {
    //                                 match increment.checked_add(1) {
    //                                     Some(value) => {
    //                                         *increment = value;
    //                                         acc = format!(#checked_add_format_handle_token_stream);
    //                                     }
    //                                     None => {
    //                                         return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                         });
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             },
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 let element_ident_type_path_not_unique_id_upper_camel_case_token_stream = generate_element_ident_type_path_not_unique_id_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element_ident.to_string());
    //                 let element_ident_type_path_not_unique_id_snake_case_token_stream = generate_element_ident_type_path_not_unique_id_snake_case_token_stream(&type_path);
    //                 let type_path_options_to_update_upper_camel_case_token_stream = generate_value_options_to_update_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream(&type_path);
    //                 let acc_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("jsonb_set({{acc}},'{{{{{{previous_jsonb_set_path}}{element_ident}}}}}',(select jsonb_agg({{maybe_jsonb_agg_case}}) from jsonb_array_elements({{current_jsonb_set_target}}) as elem {{maybe_where}}){{maybe_jsonb_build_array}})"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 quote::quote!{
    //                     (value) => {
    //                         {
    //                             let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
    //                             for element in &value.value.update {
    //                                 if ids.contains(&&element.id) {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_not_unique_id_upper_camel_case_token_stream {
    //                                         #element_ident_type_path_not_unique_id_snake_case_token_stream: element.id,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 } else {
    //                                     ids.push(&element.id);
    //                                 }
    //                             }
    //                             for element in &value.value.delete {
    //                                 if ids.contains(&element) {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_not_unique_id_upper_camel_case_token_stream {
    //                                         #element_ident_type_path_not_unique_id_snake_case_token_stream: *element,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 } else {
    //                                     ids.push(&element);
    //                                 }
    //                             }
    //                         }
    //                         let current_jsonb_set_target = format!(#current_jsonb_set_target_format_handle_token_stream);
    //                         let mut update_query_part_acc = std::string::String::default();
    //                         for (index, element) in &value
    //                             .value
    //                             .update
    //                             .iter()
    //                             .enumerate()
    //                             .collect::<std::vec::Vec<(usize, &#type_path_options_to_update_upper_camel_case_token_stream)>>()
    //                         {
    //                             match postgresql_crud::JsonArrayElementUpdateBindQuery::try_generate_update_bind_increments(
    //                                 *element,
    //                                 &jsonb_set_accumulator,
    //                                 &jsonb_set_target,
    //                                 &jsonb_set_path,
    //                                 increment,
    //                                 is_array_object_element.clone()
    //                             )
    //                             {
    //                                 Ok(value) => {
    //                                     if let Some(value) = value {
    //                                         update_query_part_acc.push_str(&value);
    //                                     }
    //                                 },
    //                                 Err(error) => {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream {
    //                                         #element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         }
    //                         let delete_query_part_acc = {
    //                             if value.value.delete.is_empty() {
    //                                 std::string::String::default()
    //                             }
    //                             else {
    //                                 let mut delete_query_part_acc = std::string::String::default();
    //                                 for (index, element) in &value
    //                                     .value
    //                                     .delete
    //                                     .iter()
    //                                     .enumerate()
    //                                     .collect::<std::vec::Vec<(usize, &postgresql_crud::JsonUuid)>>()
    //                                 {
    //                                     match increment.checked_add(1) {
    //                                         Some(value) => {
    //                                             *increment = value;
    //                                             let maybe_space_and_space = if delete_query_part_acc.is_empty() {
    //                                                 ""
    //                                             }
    //                                             else {
    //                                                 " and "
    //                                             };
    //                                             delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
    //                                         }
    //                                         None => {
    //                                             return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream {
    //                                                 #element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed::CheckedAdd {
    //                                                     code_occurence: error_occurence_lib::code_occurence!(),
    //                                                 },
    //                                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                                             });
    //                                         }
    //                                     }
    //                                 }
    //                                 delete_query_part_acc
    //                             }
    //                         };
    //                         let mut create_query_part_acc = std::string::String::default();
    //                         for (index, element) in &value
    //                             .value
    //                             .create
    //                             .iter()
    //                             .enumerate()
    //                             .collect::<std::vec::Vec<(usize, &#type_path_to_create_upper_camel_case_token_stream)>>()
    //                         {
    //                             match postgresql_crud::JsonArrayElementCreateBindQuery::try_generate_create_bind_increments(*element, increment) {
    //                                 Ok(value) => {
    //                                     if let Some(value) = value {
    //                                         create_query_part_acc.push_str(&format!("{value},"));
    //                                     }
    //                                 },
    //                                 Err(error) => {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream {
    //                                         #element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         }
    //                         let _ = create_query_part_acc.pop();
    //                         let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
    //                             std::string::String::from("elem")
    //                         } else {
    //                             format!("case {update_query_part_acc} else elem end")
    //                         };
    //                         let maybe_where = if delete_query_part_acc.is_empty() {
    //                             std::string::String::default()
    //                         } else {
    //                             format!(" where {delete_query_part_acc}")
    //                         };
    //                         let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
    //                             std::string::String::default()
    //                         } else {
    //                             format!(" || jsonb_build_array({create_query_part_acc})")
    //                         };
    //                         acc = format!(#acc_format_handle_token_stream);
    //                     }
    //                 }
    //             },
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let type_path_stringified = quote::quote!{#type_path}.to_string();
    //                 let type_path_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&type_path_stringified);
    //                 let type_path_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&type_path_stringified);
    //                 let element_ident_type_path_not_unique_id_upper_camel_case_token_stream = generate_element_ident_type_path_not_unique_id_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element_ident.to_string());
    //                 let element_ident_type_path_not_unique_id_snake_case_token_stream = generate_element_ident_type_path_not_unique_id_snake_case_token_stream(&type_path);
    //                 let type_path_options_to_update_upper_camel_case_token_stream = generate_value_options_to_update_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream(&type_path);
    //                 let element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream = generate_element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream(&type_path);
    //                 let acc_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("jsonb_set({{acc}},'{{{{{{previous_jsonb_set_path}}{element_ident}}}}}',case when {{jsonb_set_target}}->'{element_ident}' = 'null' then {{maybe_jsonb_build_array_in_case_of_null}} else (select jsonb_agg({{maybe_jsonb_agg_case}}) from jsonb_array_elements({{current_jsonb_set_target}}) as elem {{maybe_where}}) {{maybe_jsonb_build_array}} end)"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 let value_is_none_format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                     &format!("jsonb_set({{acc}},'{{{{{{previous_jsonb_set_path}}{element_ident}}}}}',${{increment}})"),
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 );
    //                 quote::quote!{
    //                     (value) => {
    //                         {
    //                             let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
    //                             if let Some(value) = &value.value {
    //                                 for element in &value.update {
    //                                     if ids.contains(&&element.id) {
    //                                         return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_not_unique_id_upper_camel_case_token_stream {
    //                                             #element_ident_type_path_not_unique_id_snake_case_token_stream: element.id,
    //                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                         });
    //                                     } else {
    //                                         ids.push(&element.id);
    //                                     }
    //                                 }
    //                                 for element in &value.delete {
    //                                     if ids.contains(&element) {
    //                                         return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_not_unique_id_upper_camel_case_token_stream {
    //                                             #element_ident_type_path_not_unique_id_snake_case_token_stream: *element,
    //                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                         });
    //                                     } else {
    //                                         ids.push(&element);
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                         let current_jsonb_set_target = format!(#current_jsonb_set_target_format_handle_token_stream);
    //                         match &value.value {
    //                             Some(value) => {
    //                                 let mut update_query_part_acc = std::string::String::default();
    //                                 for (index, element) in &value
    //                                     .update
    //                                     .iter()
    //                                     .enumerate()
    //                                     .collect::<std::vec::Vec<(usize, &#type_path_options_to_update_upper_camel_case_token_stream)>>()
    //                                 {
    //                                     match postgresql_crud::JsonArrayElementUpdateBindQuery::try_generate_update_bind_increments(
    //                                         *element,
    //                                         &jsonb_set_accumulator,
    //                                         &jsonb_set_target,
    //                                         &jsonb_set_path,
    //                                         increment,
    //                                         is_array_object_element.clone()
    //                                     ) {
    //                                         Ok(value) => {
    //                                             if let Some(value) = value {
    //                                                 update_query_part_acc.push_str(& value);
    //                                             }
    //                                         },
    //                                         Err(error) => {
    //                                             return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_update_bind_increments_upper_camel_case_token_stream {
    //                                                 #element_ident_type_path_try_generate_json_array_element_update_bind_increments_snake_case_token_stream: error,
    //                                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                                             });
    //                                         }
    //                                     }
    //                                 }
    //                                 let delete_query_part_acc = {
    //                                     if value.delete.is_empty() {
    //                                         std::string::String::default()
    //                                     }
    //                                     else {
    //                                         let mut delete_query_part_acc = std::string::String::default();
    //                                         for (index, element) in &value
    //                                             .delete
    //                                             .iter()
    //                                             .enumerate()
    //                                             .collect::<std::vec::Vec<(usize, &postgresql_crud::JsonUuid)>>()
    //                                         {
    //                                             match increment.checked_add(1) {
    //                                                 Some(value) => {
    //                                                     *increment = value;
    //                                                     let maybe_space_and_space = if delete_query_part_acc.is_empty() {
    //                                                         ""
    //                                                     } else {
    //                                                         " and "
    //                                                     };
    //                                                     delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
    //                                                 }
    //                                                 None => {
    //                                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_delete_bind_increments_upper_camel_case_token_stream {
    //                                                         #element_ident_type_path_try_generate_json_array_element_delete_bind_increments_snake_case_token_stream: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed::CheckedAdd {
    //                                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                                         },
    //                                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                                     });
    //                                                 }
    //                                             }
    //                                         }
    //                                         delete_query_part_acc
    //                                     }
    //                                 };
    //                                 let mut create_query_part_acc = std::string::String::default();
    //                                 for (index, element) in &value
    //                                     .create
    //                                     .iter()
    //                                     .enumerate()
    //                                     .collect::<std::vec::Vec<(usize, &#type_path_to_create_upper_camel_case_token_stream)>>()
    //                                 {
    //                                     match postgresql_crud::JsonArrayElementCreateBindQuery::try_generate_create_bind_increments(*element, increment) {
    //                                         Ok(value) => {
    //                                             if let Some(value) = value {
    //                                                 create_query_part_acc.push_str(&format!("{value},"));
    //                                             }
    //                                         },
    //                                         Err(error) => {
    //                                             return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#element_ident_type_path_try_generate_json_array_element_create_bind_increments_upper_camel_case_token_stream {
    //                                                 #element_ident_type_path_try_generate_json_array_element_create_bind_increments_snake_case_token_stream: error,
    //                                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                                             });
    //                                         }
    //                                     }
    //                                 }
    //                                 //
    //                                 // let _ = create_query_part_acc.pop();
    //                                 // let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
    //                                 //     std::string::String::from("elem")
    //                                 // } else {
    //                                 //     format!("case {update_query_part_acc} else elem end")
    //                                 // };
    //                                 // let maybe_where = if delete_query_part_acc.is_empty() {
    //                                 //     std::string::String::default()
    //                                 // } else {
    //                                 //     format!(" where {delete_query_part_acc}")
    //                                 // };
    //                                 // let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
    //                                 //     std::string::String::default()
    //                                 // } else {
    //                                 //     format!(" || jsonb_build_array({create_query_part_acc})")
    //                                 // };
    //                                 // acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
    //                                 let _ = create_query_part_acc.pop();
    //                                 let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
    //                                     std::string::String::from("elem")
    //                                 } else {
    //                                     format!("case {update_query_part_acc} else elem end")
    //                                 };
    //                                 let maybe_where = if delete_query_part_acc.is_empty() {
    //                                     std::string::String::default()
    //                                 } else {
    //                                     format!(" where {delete_query_part_acc}")
    //                                 };
    //                                 let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
    //                                     std::string::String::default()
    //                                 } else {
    //                                     format!(" || jsonb_build_array({create_query_part_acc})")
    //                                 };
    //                                 let maybe_jsonb_build_array_in_case_of_null = if create_query_part_acc.is_empty() {
    //                                     current_jsonb_set_target.clone()
    //                                 } else {
    //                                     format!("jsonb_build_array({create_query_part_acc})")
    //                                 };
    //                                 acc = format!(#acc_format_handle_token_stream);
    //                             }
    //                             None => match increment.checked_add(1) {
    //                                 Some(value) => {
    //                                     *increment = value;
    //                                     acc = format!(#value_is_none_format_handle_token_stream);
    //                                 }
    //                                 None => {
    //                                     return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             },
    //                         }
    //                     }
    //                 }
    //             },

    //             SupportedPredefinedType::JsonUuid => {
    //                 // let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //                 //     &format!("jsonb_set({{acc}},'{{{{{{previous_path}}{element_ident}}}}}',${{increment}})"),//element_ident.to_string(),
    //                 //     &proc_macro_name_upper_camel_case_ident_stringified
    //                 // );
    //                 // quote::quote!{
    //                 //     (_) => {
    //                 //         match increment.checked_add(1) {
    //                 //             Some(value) => {
    //                 //                 *increment = value;
    //                 //                 acc = format!(#format_handle_token_stream);
    //                 //             }
    //                 //             None => {
    //                 //                 return Err(
    //                 //                     #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                 //                         code_occurence: error_occurence_lib::code_occurence!(),
    //                 //                     },
    //                 //                 );
    //                 //             }
    //                 //         }
    //                 //     }
    //                 // }
    //                 todo!()
    //             }
    //         };
    //         quote::quote!{
    //             #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream #content_token_stream
    //         }
    //     });
    //     let bind_query_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         let content_token_stream = match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 quote::quote! {
    //                     query = query.bind(sqlx::types::Json(value.value));
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(_) => {
    //                 quote::quote! {
    //                     query = value.value.bind_value_to_query(query);
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     match value.value {
    //                         Some(value) => {
    //                             query = value.bind_value_to_query(query);
    //                         }
    //                         None => {
    //                             query = query.bind(sqlx::types::Json(
    //                                 None::<std::option::Option<#type_path_to_create_upper_camel_case_token_stream>>,
    //                             ));
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => {
    //                 quote::quote! {
    //                     // query = value.value.bind_value_to_query(query);
    //                     for element in &value.value.update {
    //                         query = postgresql_crud::JsonArrayElementUpdateBindQuery::bind_update_value_to_query(
    //                             element.clone(),
    //                             query,
    //                         );
    //                     }
    //                     for element in &value.value.delete {
    //                         query = query.bind(element.0.to_string());
    //                     }
    //                     for element in &value.value.create {
    //                         query = postgresql_crud::JsonArrayElementCreateBindQuery::bind_create_value_to_query(
    //                             element.clone(),
    //                             query,
    //                         );
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 let type_path_options_to_update_upper_camel_case_token_stream = generate_value_options_to_update_upper_camel_case_token_stream(&quote::quote! {#type_path}.to_string());
    //                 quote::quote! {
    //                     match &value.value {
    //                         Some(value) => {
    //                             for element in &value.update {
    //                                 query = postgresql_crud::JsonArrayElementUpdateBindQuery::bind_update_value_to_query(
    //                                     element.clone(),
    //                                     query,
    //                                 );
    //                             }
    //                             for element in &value.delete {
    //                                 query = query.bind(element.0.to_string());
    //                             }
    //                             for element in &value.create {
    //                                 query = postgresql_crud::JsonArrayElementCreateBindQuery::bind_create_value_to_query(
    //                                     element.clone(),
    //                                     query,
    //                                 );
    //                             }
    //                         },
    //                         None => {
    //                             query = query.bind(sqlx::types::Json(None::<std::option::Option<postgresql_crud::JsonArrayChange<#type_path_to_create_upper_camel_case_token_stream, #type_path_options_to_update_upper_camel_case_token_stream>>>));
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => {
    //                 // quote::quote!{
    //                 //     query = query.bind(sqlx::types::Json(value.value));
    //                 // }
    //                 todo!()
    //             }
    //         };
    //         quote::quote! {
    //             #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                 #content_token_stream
    //             }
    //         }
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream> for #ident_options_to_update_upper_camel_case_token_stream {
    //             fn try_generate_bind_increments(
    //                 &self,
    //                 jsonb_set_accumulator: &std::primitive::str,
    //                 jsonb_set_target: &std::primitive::str,
    //                 jsonb_set_path: &std::primitive::str,
    //                 increment: &mut std::primitive::u64,
    //                 is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    //             ) -> Result<std::string::String, #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream>
    //             {
    //                 if self.0.is_empty() {
    //                     return Err(
    //                         #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::FieldsIsEmpty {
    //                             code_occurence: error_occurence_lib::code_occurence!(),
    //                         },
    //                     );
    //                 }
    //                 {
    //                     let mut acc = vec![];
    //                     for element in &self.0 {
    //                         match element {
    //                             #(#check_not_unique_field_token_stream),*
    //                         }
    //                     }
    //                 }
    //                 let mut acc = std::string::String::from(jsonb_set_accumulator);
    //                 let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
    //                     true => std::string::String::default(),
    //                     false => format!("{jsonb_set_path},"),
    //                 };
    //                 for element in &self.0 {
    //                     match &element {
    //                         #(#query_part_generation_token_stream),*
    //                     }
    //                 }
    //                 Ok(acc)
    //             }
    //             fn bind_value_to_query<'a>(
    //                 self,
    //                 mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    //             ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
    //                 for element in self.0 {
    //                     match element {
    //                         #(#bind_query_token_stream),*
    //                     }
    //                 }
    //                 query
    //             }
    //         }
    //     }
    // };
    // let maybe_pub_enum_ident_try_generate_json_array_element_update_bind_increments_error_named_token_stream = if is_id_field_exists {
    //     quote::quote! {
    //         #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
    //         pub enum #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream {
    //             CheckedAdd {
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             FieldsIsEmpty {
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             NotUniqueField {
    //                 #[eo_to_std_string_string_serialize_deserialize]
    //                 field: #ident_field_to_update_upper_camel_case_token_stream,
    //                 code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //             },
    //             //here will be additional variants
    //         }
    //     }
    // } else {
    //     proc_macro2::TokenStream::new()
    // };
    // let maybe_impl_postgresql_crud_json_array_element_update_bind_query_ident_try_generate_json_array_element_update_bind_increments_error_named_for_ident_options_to_update_token_stream = if is_id_field_exists {
    //     let ident_json_array_element_change_upper_camel_case_token_stream = generate_ident_json_array_element_change_upper_camel_case_token_stream(&ident.to_string());
    //     let try_generate_update_bind_increments_check_not_unique_field_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         quote::quote! {
    //             #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
    //                 let value = #ident_field_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream;
    //                 if acc.contains(&value) {
    //                     return Err(#ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::NotUniqueField {
    //                         field: value,
    //                         code_occurence: error_occurence_lib::code_occurence!(),
    //                     });
    //                 }
    //                 else{
    //                     acc.push(value);
    //                 }
    //             }
    //         }
    //     });
    //     let try_generate_update_bind_increments_try_generate_bind_increments_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{{{{{}}}}}',${{increment}}", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
    //                         match increment.checked_add(1) {
    //                             Some(value) => {
    //                                 *increment = value;
    //                                 acc.push_str(&format!(#format_handle_token_stream));
    //                             }
    //                             None => {
    //                                 return Err(
    //                                     #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 //here just copypaste
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{{{{{}}}}}',${{increment}}", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
    //                         match increment.checked_add(1) {
    //                             Some(value) => {
    //                                 *increment = value;
    //                                 acc.push_str(&format!(#format_handle_token_stream));
    //                             }
    //                             None => {
    //                                 return Err(
    //                                     #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 //here just copypaste
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{{{{{}}}}}',${{increment}}", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
    //                         match increment.checked_add(1) {
    //                             Some(value) => {
    //                                 *increment = value;
    //                                 acc.push_str(&format!(#format_handle_token_stream));
    //                             }
    //                             None => {
    //                                 return Err(
    //                                     #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 //here just copypaste
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{{{{{}}}}}',${{increment}}", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         match increment.checked_add(1) {
    //                             Some(value) => {
    //                                 *increment = value;
    //                                 acc.push_str(&format!(#format_handle_token_stream));
    //                             }
    //                             None => {
    //                                 return Err(
    //                                     #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 //here just copypaste
    //                 let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{{{{{}}}}}',${{increment}}", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         match increment.checked_add(1) {
    //                             Some(value) => {
    //                                 *increment = value;
    //                                 acc.push_str(&format!(#format_handle_token_stream));
    //                             }
    //                             None => {
    //                                 return Err(
    //                                     #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     },
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => {
    //                 todo!()
    //             }
    //         }
    //     });
    //     let bind_update_value_to_query_match_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         query = query.bind(sqlx::types::Json(value.value));
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 //here just copypaste
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         query = query.bind(sqlx::types::Json(value.value));
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 //here just copypaste
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         query = query.bind(sqlx::types::Json(value.value));
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 //here just copypaste
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         query = query.bind(sqlx::types::Json(value.value));
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 //here just copypaste
    //                 quote::quote! {
    //                     #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
    //                         query = query.bind(sqlx::types::Json(value.value));
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => {
    //                 todo!()
    //             }
    //         }
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::JsonArrayElementUpdateBindQuery<#ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream> for #ident_options_to_update_upper_camel_case_token_stream {
    //             fn try_generate_update_bind_increments(
    //                 &self,
    //                 jsonb_set_accumulator: &std::primitive::str,
    //                 jsonb_set_target: &std::primitive::str,
    //                 jsonb_set_path: &std::primitive::str,
    //                 increment: &mut std::primitive::u64,
    //                 is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    //             ) -> Result<std::option::Option<std::string::String>, #ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream> {
    //                 match increment.checked_add(1) {
    //                     Some(new_increment_value) => {
    //                         *increment = new_increment_value;
    //                         if self.fields.is_empty() {
    //                             return Err(#ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::FieldsIsEmpty {
    //                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                             });
    //                         }
    //                         {
    //                             let mut acc = vec![];
    //                             for element in &self.fields {
    //                                 match element {
    //                                     #(#try_generate_update_bind_increments_check_not_unique_field_token_stream),*
    //                                 }
    //                             }
    //                         }
    //                         let id_increment = format!("${increment}");
    //                         let mut acc = std::string::String::default();
    //                         for element in &self.fields {
    //                             match & element {
    //                                 #(#try_generate_update_bind_increments_try_generate_bind_increments_token_stream),*
    //                             }
    //                             acc.push_str(",");
    //                         }
    //                         let _ = acc.pop();
    //                         Ok(Some(format!("when elem->>'id' = {id_increment} then jsonb_set(elem,{acc})")))
    //                     }
    //                     None => Err(#ident_try_generate_json_array_element_update_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
    //                         code_occurence: error_occurence_lib::code_occurence!(),
    //                     })
    //                 }
    //             }
    //             fn bind_update_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
    //                 query = query.bind(self.id.0.to_string());
    //                 for element in self.fields {
    //                     match element {
    //                         #(#bind_update_value_to_query_match_token_stream),*
    //                     }
    //                 }
    //                 query
    //             }
    //         }
    //     }
    // } else {
    //     proc_macro2::TokenStream::new()
    // };
    // let maybe_impl_postgresql_crud_json_array_element_create_bind_query_for_ident_to_create_token_stream = if is_id_field_exists {
    //     quote::quote! {
    //         impl postgresql_crud::JsonArrayElementCreateBindQuery for #ident_to_create_upper_camel_case_token_stream {
    //             fn try_generate_create_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed> {
    //                 match postgresql_crud::BindQuery::try_generate_bind_increments(self, increment) {
    //                     Ok(value) => Ok(Some(value)),
    //                     Err(error) => Err(postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed::TryGenerateBindIncrements {
    //                         error: error,
    //                         code_occurence: error_occurence_lib::code_occurence!(),
    //                     }),
    //                 }
    //             }
    //             fn bind_create_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
    //                 query = postgresql_crud::BindQuery::bind_value_to_query(self, query);
    //                 query
    //             }
    //         }
    //     }
    // } else {
    //     proc_macro2::TokenStream::new()
    // };

    // let pub_struct_ident_to_create_token_stream = {
    //     let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let element_type = &element.ty;
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 quote::quote! {
    //                     pub #element_ident: #element_type
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     pub #element_ident: postgresql_crud::JsonGeneric<#type_path_to_create_upper_camel_case_token_stream>
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     pub #element_ident: postgresql_crud::JsonStdOptionOptionGeneric<#type_path_to_create_upper_camel_case_token_stream>
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     pub #element_ident: postgresql_crud::JsonStdVecVecGenericWithId<#type_path_to_create_upper_camel_case_token_stream>
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     pub #element_ident: postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<#type_path_to_create_upper_camel_case_token_stream>
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
    //         }
    //     });
    //     quote::quote! {
    //         #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
    //        pub struct #ident_to_create_upper_camel_case_token_stream {
    //            #(#variants_token_stream),*
    //        }
    //     }
    // };
    // let impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_to_create_token_stream = {
    //     let fields_initialization_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let field_ident = element.ident.as_ref().unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE));
    //         quote::quote! {
    //             #field_ident: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    //         }
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_to_create_upper_camel_case_token_stream {
    //             #[inline]
    //             fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
    //                 Self {
    //                     #(#fields_initialization_token_stream),*
    //                 }
    //             }
    //         }
    //     }
    // };
    // let impl_postgresql_crud_bind_query_for_ident_to_create_token_stream = {
    //     let maybe_id_initialization_token_stream = if is_id_field_exists {
    //         quote::quote! {"'id', to_jsonb(gen_random_uuid()),"}
    //     } else {
    //         quote::quote! {""}
    //     };
    //     let try_generate_bind_increments_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 let element_ident_double_quotes_increment_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{}',${{increment}},", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     match increment.checked_add(1) {
    //                         Some(incr) => {
    //                             *increment = incr;
    //                             increments.push_str(&format!(#element_ident_double_quotes_increment_token_stream));
    //                         }
    //                         None => {
    //                             return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
    //                                 code_occurence: error_occurence_lib::code_occurence!(),
    //                             });
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 let element_ident_double_quotes_value_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{}',{{value}},", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     match self.#element_ident.0.try_generate_bind_increments(increment) {
    //                         Ok(value) => {
    //                             increments.push_str(&format!(#element_ident_double_quotes_value_token_stream));
    //                         }
    //                         Err(error) => {
    //                             return Err(error);
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let element_ident_value_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{element_ident}',{{value}},"), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 let element_ident_increment_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{element_ident}',${{increment}},"), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     match &self.#element_ident.0 {
    //                         Some(value) => match value.try_generate_bind_increments(increment) {
    //                             Ok(value) => {
    //                                 increments.push_str(&format!(#element_ident_value_double_quotes_token_stream));
    //                             }
    //                             Err(error) => {
    //                                 return Err(error);
    //                             }
    //                         },
    //                         None => {
    //                             match increment.checked_add(1) {
    //                                 Some(value) => {
    //                                     *increment = value;
    //                                     increments.push_str(&format!(#element_ident_increment_double_quotes_token_stream));
    //                                 }
    //                                 None => {
    //                                     return Err(
    //                                         postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
    //                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                         },
    //                                     );
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 let element_ident_double_quotes_jsonb_build_array_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{}',jsonb_build_array({{acc}}),", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     {
    //                         let mut acc = std::string::String::default();
    //                         for element in &self.#element_ident.0 {
    //                             match element.try_generate_bind_increments(increment) {
    //                                 Ok(value) => {
    //                                     acc.push_str(&format!("{value},"));
    //                                 }
    //                                 Err(error) => {
    //                                     return Err(error);
    //                                 }
    //                             }
    //                         }
    //                         let _ = acc.pop();
    //                         increments.push_str(&format!(#element_ident_double_quotes_jsonb_build_array_token_stream));
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let element_ident_double_quotes_jsonb_build_array_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("'{}',{{maybe_jsonb_build_array_stringified}},", element_ident), &proc_macro_name_upper_camel_case_ident_stringified);
    //                 quote::quote! {
    //                     {
    //                         let maybe_jsonb_build_array_stringified: std::string::String;
    //                         match &self.#element_ident.0 {
    //                             Some(value) => {
    //                                 let mut acc = std::string::String::default();
    //                                 for element in value {
    //                                     match element.try_generate_bind_increments(increment) {
    //                                         Ok(value) => {
    //                                             acc.push_str(&format!("{value},"));
    //                                         }
    //                                         Err(error) => {
    //                                             return Err(error);
    //                                         }
    //                                     }
    //                                 }
    //                                 let _ = acc.pop();
    //                                 maybe_jsonb_build_array_stringified = format!("jsonb_build_array({acc})");
    //                             }
    //                             None => {
    //                                 match increment.checked_add(1) {
    //                                     Some(value) => {
    //                                         *increment = value;
    //                                         maybe_jsonb_build_array_stringified = format!("${increment}");
    //                                     }
    //                                     None => {
    //                                         return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
    //                                             code_occurence: error_occurence_lib::code_occurence!(),
    //                                         });
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                         increments.push_str(&format!(#element_ident_double_quotes_jsonb_build_array_token_stream));
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
    //         }
    //     });
    //     let bind_value_to_query_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
    //                 quote::quote! {
    //                     query = query.bind(sqlx::types::Json(self.#element_ident.0));
    //                 }
    //             }

    //             SupportedPredefinedType::JsonGeneric(type_path) => {
    //                 quote::quote! {
    //                     query = self.#element_ident.0.bind_value_to_query(query);
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
    //                 let type_path_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     match self.#element_ident.0 {
    //                         Some(value) => {
    //                             query = value.bind_value_to_query(query);
    //                         },
    //                         None => {
    //                             query = query.bind(sqlx::types::Json(None::<std::option::Option<#type_path_to_create_upper_camel_case_token_stream>>));
    //                         }
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(type_path) => {
    //                 quote::quote! {
    //                     for element in self.#element_ident.0 {
    //                         query = element.bind_value_to_query(query);
    //                     }
    //                 }
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(type_path) => {
    //                 let type_path_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&type_path);
    //                 quote::quote! {
    //                     match self.#element_ident.0 {
    //                         Some(value) => {
    //                             for element in value {
    //                                 query = element.bind_value_to_query(query);
    //                             }
    //                         }
    //                         None => {
    //                             query = query.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<#type_path_upper_camel_case_token_stream>>>));
    //                         }
    //                     }
    //                 }
    //             }

    //             SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
    //         }
    //     });
    //     quote::quote! {
    //         impl<'a> postgresql_crud::BindQuery<'a> for #ident_to_create_upper_camel_case_token_stream {
    //             fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
    //                 todo!()//not usefull here - refactor later
    //             }
    //             fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
    //                 let mut increments = std::string::String::from(#maybe_id_initialization_token_stream);
    //                 #(#try_generate_bind_increments_token_stream)*
    //                 let _ = increments.pop();
    //                 Ok(format!("jsonb_build_object({increments})"))
    //             }
    //             fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
    //                 #(#bind_value_to_query_token_stream)*
    //                 query
    //             }
    //         }
    //     }
    // };
    // let maybe_impl_postgresql_crud_get_json_id_for_ident_token_stream = if is_id_field_exists {
    //     quote::quote! {
    //         impl postgresql_crud::GetJsonId for #ident {
    //             fn get_json_id(&self) -> &postgresql_crud::JsonUuid {
    //                 &self.id
    //             }
    //         }
    //     }
    // } else {
    //     proc_macro2::TokenStream::new()
    // };
    // let impl_postgresql_crud_check_id_exists_in_json_generic_fields_for_ident_token_stream = {
    //     let check_id_exists_in_json_std_vec_vec_generic_token_stream = vec_syn_field_filtered_id_iter.iter().fold(vec![], |mut acc, element| {
    //         let element_ident = element.ident.as_ref().unwrap_or_else(|| {
    //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
    //         });
    //         let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
    //         match &supported_predefined_type {
    //             SupportedPredefinedType::JsonStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool
    //             | SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => (),

    //             SupportedPredefinedType::JsonGeneric(_) => (),
    //             SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => (),
    //             SupportedPredefinedType::JsonStdVecVecGenericWithId(_) => {
    //                 acc.push(quote::quote! {
    //                     let _: () = postgresql_crud::CheckIdExistsInJsonStdVecVecGenericWithId::check_id_exists_in_json_std_vec_vec_generic_with_id(&self.#element_ident);
    //                 });
    //             }
    //             SupportedPredefinedType::JsonStdOptionOptionStdVecVecGenericWithId(_) => {
    //                 acc.push(quote::quote! {
    //                     let _: () = postgresql_crud::CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId::check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self.#element_ident);
    //                 });
    //             }

    //             SupportedPredefinedType::JsonUuid => (),
    //         }
    //         acc
    //     });
    //     quote::quote! {
    //         impl postgresql_crud::CheckIdExistsInJsonGenericFields for #ident {
    //             fn check_id_exists_in_json_generic_fields(&self) {
    //                 #(#check_id_exists_in_json_std_vec_vec_generic_token_stream)*
    //             }
    //         }
    //     }
    // };
    // //
    // let update = quote::quote!{
    //     #pub_enum_ident_field_to_update_token_stream
    //     #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream
    //     #pub_enum_ident_option_to_update_token_stream
    //     #pub_struct_ident_options_to_update_token_stream
    //     #pub_enum_ident_options_to_update_try_generate_bind_increments_error_named_token_stream
    // };
    //
    let generic_ident_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_upper_camel_case_token_stream(&ident);
    fn generate_impl_std_fmt_display_for_tokens_token_stream(value_token_stream: &impl quote::ToTokens) -> proc_macro2::TokenStream {
        quote::quote!{
            impl std::fmt::Display for #value_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", &self)
                }
            }
        }
    }
    
    let generate_template_field_to_read_struct_token_stream = |
        struct_ident_token_stream: &proc_macro2::TokenStream,
        additional_content_token_stream: &proc_macro2::TokenStream,
    |{
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident_stringified = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                })
                .to_string();
            let serialize_deserialize_field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let type_path_field_reader_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::FieldReaderUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #[serde(rename(serialize = #serialize_deserialize_field_ident_double_quotes_token_stream, deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream))]
                #variant_ident_upper_camel_case_token_stream(#type_path_field_reader_token_stream)
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub enum #struct_ident_token_stream {
                #additional_content_token_stream
                #(#variants_token_stream),*
            }
        }
    };

    let ident_field_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfFieldToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_field_to_read_upper_camel_case_token_stream(&ident);
    let ident_with_id_field_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfWithIdFieldToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_with_id_field_to_read_upper_camel_case_token_stream(&ident);

    let generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream = |value: &proc_macro2::TokenStream|{
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #value {
                fn to_std_string_string(&self) -> std::string::String {
                    format!("{self:?}")
                }
            }
        }
    };

    let postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()};

    let (
        fields_some_value_self_options_to_read_initialization_content_token_stream,
        fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
    ) = {
        let generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = |
            field_ident: &syn::Ident,
        |{
            quote::quote!{
                #field_ident: Some(postgresql_crud::Value { 
                    value: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                })
            }
        };
        let fields_some_value_self_options_to_read_initialization_content_token_stream = {
            let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                &element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                })
            ));
            quote::quote!{{#(#fields_token_stream),*}}
        };
        let fields_with_id_some_value_self_options_to_read_initialization_content_token_stream = {
            let fields_token_stream = {
                let fields_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                    &element
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                ));
                let id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = generate_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream(
                    &syn::Ident::new(&naming_conventions::IdSnakeCase.to_string(), ident.span()),
                );
                quote::quote!{
                    #id_field_ident_some_value_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    #(#fields_token_stream),*
                }
            };
            quote::quote!{{ #fields_token_stream }}
        };
        (
            fields_some_value_self_options_to_read_initialization_content_token_stream,
            fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
        )
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream = |
        struct_ident_token_stream: &proc_macro2::TokenStream,
        self_initialization_content_token_stream: &proc_macro2::TokenStream,
    |{
        quote::quote!{
            impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #struct_ident_token_stream {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self #self_initialization_content_token_stream
                }
            }
        }
    };

    let ident_field_to_read_token_stream = generate_template_field_to_read_struct_token_stream(
        &ident_field_to_read_upper_camel_case_token_stream,
        &proc_macro2::TokenStream::new(),
    );
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_field_to_read_upper_camel_case_token_stream);
    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream = {
        let elements_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident = &element.ident.as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let field_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
            quote::quote!{
                #ident_field_to_read_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
            }
        });
        quote::quote!{
            impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_field_to_read_upper_camel_case_token_stream {
                fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                    vec![
                        #(#elements_token_stream),*
                    ]
                }
            }
        }
    };

    let ident_with_id_field_to_read_token_stream = generate_template_field_to_read_struct_token_stream(
        &ident_with_id_field_to_read_upper_camel_case_token_stream,
        &quote::quote!{
            #[serde(rename(serialize = "id", deserialize = "id"))]
             Id(postgresql_crud::JsonUuidFieldReader),
        },
    );
    let impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_value_token_stream(&ident_with_id_field_to_read_upper_camel_case_token_stream);
    let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream = {
        let elements_token_stream = {
            let elements_token_stream = vec_syn_field.iter().map(|element|{
                let field_ident = &element.ident.as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    });
                let field_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
                quote::quote!{
                    #ident_with_id_field_to_read_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                }
            });
            quote::quote!{
                #ident_with_id_field_to_read_upper_camel_case_token_stream::Id(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream),
                #(#elements_token_stream),*
            }
        };
        quote::quote!{
            impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_with_id_field_to_read_upper_camel_case_token_stream {
                fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                    vec![#elements_token_stream]
                }
            }
        }
    };

    let ident_field_to_update_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            quote::quote! {
                #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
                #element_ident_upper_camel_case_token_stream
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub enum #ident_field_to_update_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&element_ident.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            quote::quote! {
                Self::#element_ident_upper_camel_case_token_stream => #element_ident_double_quotes_token_stream.to_owned()
            }
        });
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #ident_field_to_update_upper_camel_case_token_stream {
                fn to_std_string_string(&self) -> std::string::String {
                    match &self {
                        #(#variants_token_stream),*
                    }
                }
            }
        }
    };

    fn generate_supported_generics_template_struct_token_stream(struct_ident_token_stream: &impl quote::ToTokens, content_token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #struct_ident_token_stream #content_token_stream
        }
    }

    let fields_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
            });
            let element_type = &element.ty;
            quote::quote!{pub #element_ident: #element_type}
        });
        quote::quote!{#(#fields_token_stream),*}
    };

    let tuple_struct_space_stringified = "tuple struct ";
    let struct_space_stringified = "struct ";
    let postgersql_crud_pagination_token_stream = quote::quote!{postgresql_crud::Pagination};

    enum FieldReaderContent {
        GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent,
        GenericIdentAndStdOptionOptionGenericIdent,
        StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
    }
    
    let generate_tokens_field_reader_token_stream = |
        struct_prefix_stringified: &std::primitive::str, 
        field_reader_content: &FieldReaderContent
    |{
        let struct_ident_token_stream = naming_conventions::SelfFieldReaderUpperCamelCaseTokenStream::self_field_reader_upper_camel_case_token_stream(&struct_prefix_stringified);
        let std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream = quote::quote!{std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>};
        let std_vec_vec_ident_field_to_read_upper_camel_case_token_stream_token_stream = quote::quote!{std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream>};
        let field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream = quote::quote!{field_vec: std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>, pagination: #postgersql_crud_pagination_token_stream};
        let content_token_stream = match &field_reader_content {
            FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent => quote::quote!{(#std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream);},
            FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent => quote::quote!{(#std_vec_vec_ident_field_to_read_upper_camel_case_token_stream_token_stream);},
            FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent => quote::quote!{
                {
                    #field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream
                }
            },
        };
        let struct_prefix_try_new_error_named_upper_camel_case_token_stream = naming_conventions::SelfTryNewErrorNamedUpperCamelCaseTokenStream::self_try_new_error_named_upper_camel_case_token_stream(&struct_prefix_stringified);
        let fields_filter_is_empty_upper_camel_case = naming_conventions::FieldsFilterIsEmptyUpperCamelCase;
        let not_unique_field_filter_upper_camel_case = naming_conventions::NotUniqueFieldFilterUpperCamelCase;
        let value_snake_case = naming_conventions::ValueSnakeCase;
        let generate_impl_try_new_token_stream = |
            contains_id: std::primitive::bool,
            input_parameters_token_stream: &proc_macro2::TokenStream,
            is_vec: std::primitive::bool,
        |{
            let field_type_token_stream = if contains_id {
                &ident_with_id_field_to_read_upper_camel_case_token_stream
            }
            else {
                &ident_field_to_read_upper_camel_case_token_stream
            };
            
            let field_vec_snake_case = naming_conventions::FieldVecSnakeCase;
            let pagination_snake_case = naming_conventions::PaginationSnakeCase;
            let check_handle_token_stream = if is_vec {
                quote::quote!{#field_vec_snake_case}
            }
            else {
                quote::quote!{#value_snake_case}
            };
            let self_initialization_token_stream = if is_vec {
                quote::quote!{{ #field_vec_snake_case, #pagination_snake_case }}
            }
            else {
                quote::quote!{(#value_snake_case)}
            };
            quote::quote!{
                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #struct_prefix_try_new_error_named_upper_camel_case_token_stream {
                    #fields_filter_is_empty_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #not_unique_field_filter_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #field_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
                impl #struct_ident_token_stream {
                    pub fn try_new(#input_parameters_token_stream) -> Result<Self, #struct_prefix_try_new_error_named_upper_camel_case_token_stream> {
                        if #check_handle_token_stream.is_empty() {
                            return Err(#struct_prefix_try_new_error_named_upper_camel_case_token_stream::#fields_filter_is_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            });
                        }
                        let mut unique = vec![];
                        for element in &#check_handle_token_stream {
                            if unique.contains(&element) {
                                return Err(#struct_prefix_try_new_error_named_upper_camel_case_token_stream::#not_unique_field_filter_upper_camel_case {
                                    field: element.clone(),
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            } else {
                                unique.push(&element);
                            }
                        }
                        Ok(Self #self_initialization_token_stream)
                    }
                }
            }
        };
        let impl_try_new_token_stream = {
            let generate_value_input_parameter_type_token_stream = |value_token_stream: &proc_macro2::TokenStream|{
                quote::quote!{#value_snake_case: #value_token_stream}
            };
            match &field_reader_content {
                FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent => {
                    generate_impl_try_new_token_stream(
                        true,
                        &generate_value_input_parameter_type_token_stream(&std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_token_stream),
                        false,
                    )
                },
                FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent => {
                    generate_impl_try_new_token_stream(
                        false,
                        &generate_value_input_parameter_type_token_stream(&std_vec_vec_ident_field_to_read_upper_camel_case_token_stream_token_stream),
                        false,
                    )
                },
                FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent => {
                    generate_impl_try_new_token_stream(
                        true,
                        &field_vec_std_vec_vec_ident_with_id_field_to_read_upper_camel_case_token_stream_pagination_postgersql_crud_pagination_token_stream_token_stream,
                        true,
                    )
                },
            }
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub struct #struct_ident_token_stream #content_token_stream
            #impl_try_new_token_stream
        }
    };
    let generate_match_try_new_in_deserialize_token_stream = |ident_token_stream: &proc_macro2::TokenStream, initialization_token_stream: &proc_macro2::TokenStream|{
        quote::quote!{
            match #ident_token_stream::try_new(#initialization_token_stream) {
                Ok(value) => serde::__private::Ok(value),
                Err(error) => {
                    return Err(serde::de::Error::custom(format!("{error:?}")));
                }
            }
        }
    };

    let generate_tokens_options_to_read_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream, contains_id: std::primitive::bool|{
        let maybe_id_token_stream = if contains_id {
            quote::quote!{
                #[serde(skip_serializing_if = "Option::is_none")]
                id: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonUuidOptionsToRead>>,
            }
        }
        else {
            proc_macro2::TokenStream::new()
        };
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let type_path_options_to_read_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::OptionsToReadUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #[serde(skip_serializing_if = "Option::is_none")]
                #field_ident: std::option::Option<postgresql_crud::Value<#type_path_options_to_read_token_stream>>
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
            pub struct #struct_ident_token_stream {
                #maybe_id_token_stream
                #(#fields_token_stream),*
            }
        }
    };
    let generate_tokens_to_create_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream|{
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let type_path_to_create_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::ToCreateUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #field_ident: #type_path_to_create_token_stream
            }
        });
        generate_supported_generics_template_struct_token_stream(
            struct_ident_token_stream,
            &quote::quote!{{ #(#fields_token_stream),*}}
        )
    };

    let postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream|{
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &struct_ident_token_stream,
            &quote::quote!{(#postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
        )
    };
    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream|{
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &struct_ident_token_stream,
            &quote::quote!{
                {
                    field_vec: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    pagination: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                }
            },
        )
    };

    let generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream|{
        let fields_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            quote::quote!{
                #field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            }
        });
        generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &struct_ident_token_stream,
            &quote::quote!{{#(#fields_token_stream),*}}
        )
    };
    let generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream = |struct_ident_token_stream: &proc_macro2::TokenStream, contains_id: std::primitive::bool|{
        let increment_initialization_string_content_token_stream = if contains_id {
            quote::quote!{"'id', to_jsonb(gen_random_uuid()),"}
        }
        else {
            quote::quote!{""}
        };
        let json_create_try_generate_bind_increments_fields_token_stream = vec_syn_field.iter().map(|element| {
            let element_field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let element_field_ident_value_comma_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("'{element_field_ident}',{{value}},"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            //todo maybe wrap into own generic error type
            quote::quote!{
                match self.#element_field_ident.json_create_try_generate_bind_increments(increment) {
                    Ok(value) => {
                        increments.push_str(&format!(#element_field_ident_value_comma_double_quotes_token_stream));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        });
        let json_create_bind_value_to_query_fields_token_stream = vec_syn_field.iter().map(|element| {
            let element_field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            quote::quote!{
                query = self.#element_field_ident.json_create_bind_value_to_query(query);
            }
        });
        quote::quote!{
            impl<'a> postgresql_crud::JsonCreateBindQuery<'a> for #struct_ident_token_stream {
                fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed> {
                    let mut increments = std::string::String::from(#increment_initialization_string_content_token_stream);
                    #(#json_create_try_generate_bind_increments_fields_token_stream)*
                    let _ = increments.pop();
                    Ok(format!("jsonb_build_object({increments})"))
                }
                fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #(#json_create_bind_value_to_query_fields_token_stream)*
                    query
                }
            }
        }
    };
    let generate_tokens_reader_token_stream = |struct_reader_token_stream: &proc_macro2::TokenStream, struct_options_to_read_token_stream: &proc_macro2::TokenStream|{
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #struct_reader_token_stream(pub #struct_options_to_read_token_stream);
        }
    };


    let generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream = |
        tokens_field_reader_token_stream: &proc_macro2::TokenStream,
        contains_id: std::primitive::bool,
        format_handle_double_quotes_token_stream: &proc_macro2::TokenStream,
    |{
        let tokens_field_to_read_upper_camel_case_token_stream = if contains_id {
            &ident_with_id_field_to_read_upper_camel_case_token_stream
        }
        else {
            &ident_field_to_read_upper_camel_case_token_stream
        };
        let generate_acc_push_str_variant_logic_token_stream = |
            variant_name_token_stream: &proc_macro2::TokenStream,
            field_ident_double_quotes_token_stream: &proc_macro2::TokenStream,
            column_name_and_maybe_field_getter_token_stream: &proc_macro2::TokenStream,
        |{
            quote::quote!{
                #tokens_field_to_read_upper_camel_case_token_stream::#variant_name_token_stream(value) => {
                    acc.push_str(&format!(
                        "{}||",
                        postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
                            value,
                            #field_ident_double_quotes_token_stream,
                            #column_name_and_maybe_field_getter_token_stream,
                            &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
                        )
                    ));
                }
            }
        };
        let value_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&naming_conventions::ValueSnakeCase.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let field_ident_stringified = field_ident.to_string();
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let field_ident_double_quotes_token_stream = if contains_id {
                &value_snake_case_double_quotes_token_stream
            }
            else {
                &proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified)
            };
            generate_acc_push_str_variant_logic_token_stream(
                &variant_ident_upper_camel_case_token_stream,
                &field_ident_double_quotes_token_stream,
                &quote::quote!{&format!("{column_name_and_maybe_field_getter}->'{field_ident}'")},
            )
        });
        let self_field_vec_token_stream = if contains_id {
            quote::quote!{field_vec}
        }
        else {
            quote::quote!{0}
        };
        let maybe_id_variant_token_stream = if contains_id {
            let id_upper_camel_case = naming_conventions::IdUpperCamelCase;
            let id_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&naming_conventions::IdSnakeCase.to_string(), &proc_macro_name_upper_camel_case_ident_stringified);
            generate_acc_push_str_variant_logic_token_stream(
                &quote::quote!{#id_upper_camel_case},
                &id_snake_case_double_quotes_token_stream,
                &value_snake_case_double_quotes_token_stream,
            )
        }
        else {
            proc_macro2::TokenStream::new()
        };
        let maybe_pagination_start_end_initialization_token_stream = if contains_id {
            proc_macro_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream()
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for #tokens_field_reader_token_stream {
                fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
                    let mut acc = std::string::String::default();
                    for element in &self.#self_field_vec_token_stream {
                        match element {
                            #maybe_id_variant_token_stream
                            #(#variants_token_stream),*
                        }
                    }
                    let _ = acc.pop();
                    let _ = acc.pop();
                    #maybe_pagination_start_end_initialization_token_stream
                    format!(#format_handle_double_quotes_token_stream)
                }
            }
        }
    };

    let generate_impl_serde_deserialize_for_options_to_read_origin_token_stream = |
        struct_ident_stringified: &std::primitive::str,
        struct_ident_token_stream: &proc_macro2::TokenStream,
        contains_id: std::primitive::bool,
    |{
        let range_end = {
            let vec_syn_field_len = vec_syn_field.len();
            if contains_id {
                vec_syn_field_len.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} vec_syn_field_len + 1 is None(int overflow)"))
            }
            else {
                vec_syn_field_len
            }
        };
        let field_enum_variants_token_stream = {
            let mut vec = vec![];
            for element in 0..range_end {
                let value = format!("__{}{element}", naming_conventions::FieldSnakeCase);
                vec.push(
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                );
            }
            vec
        };
        let generate_field_index_token_stream = |index: std::primitive::usize| {
            let value = format!("__field{index}");
            value
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let visit_u64_value_enum_variants_token_stream = {
            let mut acc = vec![];
            for index in 0..range_end {
                let index_u64_token_stream = {
                    let value = format!("{index}u64");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let field_index_token_stream = generate_field_index_token_stream(index);
                acc.push(quote::quote! {
                    #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
                });
            }
            acc
        };
        let generate_field_ident_double_quotes_serde_private_ok_field_token_stream = |
            field_name_double_quotes_token_stream: &proc_macro2::TokenStream,
            index: std::primitive::usize,
        |{
            let field_index_token_stream = generate_field_index_token_stream(index);
            quote::quote!{#field_name_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
        };
        let generate_index = |index: std::primitive::usize|{
            if contains_id {
                index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} vec_syn_field_len + 1 is None(int overflow)"))
            }
            else {
                index
            }
        };
        let visit_str_value_enum_variants_token_stream = {
            let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_name_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &{
                        element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string()
                    },
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let field_index_token_stream = generate_field_index_token_stream(index);
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &field_name_double_quotes_token_stream,
                    index,
                )
            });
            let maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &quote::quote!{"id"},
                    0,
                );
                quote::quote!{#value_token_stream,}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_field_ident_double_quotes_serde_private_ok_field_token_stream
                #(#visit_str_value_enum_variants_token_stream),*,
            }
        };
        let visit_bytes_value_enum_variants_token_stream = {
            let visit_bytes_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let b_field_name_double_quotes_token_stream = {
                    let element_ident_double_quotes_stringified = proc_macro_common::generate_quotes::double_quotes_stringified(
                        &element
                            .ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                            })
                            .to_string(),
                    );
                    let value = format!("b{element_ident_double_quotes_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &b_field_name_double_quotes_token_stream,
                    index,
                )
            });
            let maybe_b_field_ident_double_quotes_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &quote::quote!{b"id"},
                    0,
                );
                quote::quote!{#value_token_stream,}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_b_field_ident_double_quotes_token_stream
                #(#visit_bytes_value_enum_variants_token_stream),*,
            }
        };
        let struct_ident_options_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&format!("struct {ident_options_to_read_upper_camel_case_stringified}"), &proc_macro_name_upper_camel_case_ident_stringified);
        let struct_ident_options_with_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("struct {ident_options_to_read_upper_camel_case_stringified} with {range_end} elements"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let visit_seq_fields_initialization_token_stream = {
            let generate_serde_de_seq_access_next_element_token_stream = |
                index: std::primitive::usize,
                type_options_to_read_token_stream: &proc_macro2::TokenStream,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    let #field_index_token_stream = match serde::de::SeqAccess::next_element::<
                        std::option::Option<postgresql_crud::Value<#type_options_to_read_token_stream>>,
                    >(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(
                                serde::de::Error::invalid_length(
                                    0usize,
                                    &#struct_ident_options_with_double_quotes_token_stream,
                                ),
                            );
                        }
                    };
                }
            };
            let visit_seq_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_serde_de_seq_access_next_element_token_stream(
                    index,
                    &type_options_to_read_token_stream,
                )
            });
            let maybe_id_serde_de_seq_access_next_element_token_stream = if contains_id {
                generate_serde_de_seq_access_next_element_token_stream(
                    0,
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
                )
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_serde_de_seq_access_next_element_token_stream
                #(#visit_seq_fields_initialization_token_stream)*
            }
        };
        let if_let_nones_fields_serde_custom_error_token_stream = {
            let nones_token_stream = {
                let mut acc = vec![];
                for element in 0..range_end {
                    acc.push(quote::quote!{None});
                }
                acc
            };
            let fields_token_stream = {
                let mut acc = vec![];
                for element in 0..range_end {
                    let field_index_token_stream = generate_field_index_token_stream(element);
                    acc.push(quote::quote!{&#field_index_token_stream});
                }
                acc
            };
            let format_handle_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("custom serde error deserializing {struct_ident_token_stream}: all fields are None"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            quote::quote!{
                if let (#(#nones_token_stream),*) = (#(#fields_token_stream),*) {
                    return Err(serde::de::Error::custom(format!(#format_handle_double_quotes_token_stream)));
                }
            }
        };
        let visit_seq_fields_assignment_token_stream = {
            let generate_field_ident_field_index_token_stream = |
                field_ident: &proc_macro2::TokenStream,
                index: std::primitive::usize,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    #field_ident: #field_index_token_stream
                }
            };
            let visit_seq_fields_assignment_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
                let field_index_token_stream = generate_field_index_token_stream(index);
                generate_field_ident_field_index_token_stream(
                    &quote::quote!{#field_ident},
                    index,
                )
            });
            let maybe_id_field_ident_field_index_token_stream = if contains_id {
                let value_token_stream = generate_field_ident_field_index_token_stream(
                    &quote::quote!{id},
                    0,
                );
                quote::quote!{#value_token_stream,}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_field_ident_field_index_token_stream
                #(#visit_seq_fields_assignment_token_stream),*
            }
        };
        let visit_map_fields_initialization_token_stream = {
            let generate_mut_field_index_serde_private_option_token_stream = |
                index: std::primitive::usize,
                type_token_stream: &proc_macro2::TokenStream,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    let mut #field_index_token_stream: serde::__private::Option<
                        std::option::Option<postgresql_crud::Value<#type_token_stream>>,
                    > = serde::__private::None;
                }
            };
            let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_mut_field_index_serde_private_option_token_stream(
                    index,
                    &type_options_to_read_token_stream,
                )
            });
            let maybe_id_mut_field_index_serde_private_option_token_stream = if contains_id {
                generate_mut_field_index_serde_private_option_token_stream(
                    0,
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
                )
            }
            else {
                quote::quote!{}
            };
            quote::quote! {
                #maybe_id_mut_field_index_serde_private_option_token_stream
                #(#visit_map_fields_initialization_token_stream)*
            }
        };
        let generate_field_ident_double_quotes_token_stream = |value: &syn::Field| {
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &value
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    })
                    .to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            )
        };
        let visit_map_match_variants_token_stream = {
            let generate_field_initialization_token_stream = |
                index: std::primitive::usize,
                field_ident_double_quotes_token_stream: &proc_macro2::TokenStream,
                type_token_stream: &proc_macro2::TokenStream,
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    __Field::#field_index_token_stream => {
                        if serde::__private::Option::is_some(&#field_index_token_stream) {
                            return serde::__private::Err(
                                <__A::Error as serde::de::Error>::duplicate_field(
                                    #field_ident_double_quotes_token_stream,
                                ),
                            );
                        }
                        #field_index_token_stream = serde::__private::Some(
                            serde::de::MapAccess::next_value::<
                                std::option::Option<postgresql_crud::Value<#type_token_stream>>,
                            >(&mut __map)?,
                        );
                    }
                }
            };
            let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
                let type_options_to_read_token_stream = {
                    let type_path = &element.ty;
                    let value = format!("{}{}", quote::quote!{#type_path}, naming_conventions::OptionsToReadUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                generate_field_initialization_token_stream(
                    index,
                    &field_ident_double_quotes_token_stream,
                    &type_options_to_read_token_stream,
                )
            });
            let id_field_initialization_token_stream = if contains_id {
                generate_field_initialization_token_stream(
                    0,
                    &quote::quote!{"id"},
                    &quote::quote!{postgresql_crud::JsonUuidOptionsToRead},
                )
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #id_field_initialization_token_stream
                #(#visit_map_match_variants_token_stream)*
            }
        };
        let visit_map_missing_fields_check_token_stream = {
            let generate_missing_field_token_stream = |
                index: std::primitive::usize,
                field_ident_double_quotes_token_stream: &proc_macro2::TokenStream
            |{
                let field_index_token_stream = generate_field_index_token_stream(index);
                quote::quote! {
                    let #field_index_token_stream = match #field_index_token_stream {
                        serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                        serde::__private::None => {
                            serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
                        }
                    };
                }
            };
            let visit_map_missing_fields_check_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)| {
                let index = generate_index(index);
                let field_index_token_stream = generate_field_index_token_stream(index);
                let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
                generate_missing_field_token_stream(
                    index,
                    &field_ident_double_quotes_token_stream
                )
            });
            let maybe_id_missing_field_token_stream = if contains_id {
                generate_missing_field_token_stream(
                    0,
                    &quote::quote!{"id"}
                )
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_missing_field_token_stream
                #(#visit_map_missing_fields_check_token_stream)*
            }
        };
        let fields_array_elements_token_stream = {
            let fields_array_elements_token_stream = vec_syn_field.iter().map(|element| generate_field_ident_double_quotes_token_stream(&element));
            let maybe_id_double_quotes_comma_token_stream = if contains_id {
                quote::quote! {"id",}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #maybe_id_double_quotes_comma_token_stream
                #(#fields_array_elements_token_stream),*
            }
        };
        let std_vec_vec_generic_with_id_ident_options_to_read_origin_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &struct_ident_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #struct_ident_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #(#field_enum_variants_token_stream),*,
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
                                #(#visit_u64_value_enum_variants_token_stream),*,
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
                                #visit_str_value_enum_variants_token_stream
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
                                #visit_bytes_value_enum_variants_token_stream
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
                        marker: serde::__private::PhantomData<
                            #struct_ident_token_stream,
                        >,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #struct_ident_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_ident_options_double_quotes_token_stream,
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
                            #visit_seq_fields_initialization_token_stream
                            #if_let_nones_fields_serde_custom_error_token_stream
                            serde::__private::Ok(#struct_ident_token_stream {
                                #visit_seq_fields_assignment_token_stream
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
                            #visit_map_fields_initialization_token_stream
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    #visit_map_match_variants_token_stream
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<
                                            serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            #visit_map_missing_fields_check_token_stream
                            #if_let_nones_fields_serde_custom_error_token_stream
                            serde::__private::Ok(#struct_ident_token_stream {
                                #visit_seq_fields_assignment_token_stream
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[#fields_array_elements_token_stream];
                    serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #std_vec_vec_generic_with_id_ident_options_to_read_origin_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: serde::__private::PhantomData::<
                                #struct_ident_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };

    let generate_tokens_option_to_update_origin_token_stream = |
        struct_ident_token_stream: &proc_macro2::TokenStream,
        contains_id: std::primitive::bool,
    |{
        let variants_token_stream = vec_syn_field.iter().map(|element| {
            let field_ident = element
                .ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
            let type_path_option_to_update_token_stream = {
                let value = format!(
                    "{}{}",
                    {
                        let type_path = &element.ty;
                        quote::quote!{#type_path}.to_string()
                    },
                    naming_conventions::OptionToUpdateUpperCamelCase
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident.to_string());
            let field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            quote::quote!{
                #[serde(rename(serialize = #field_ident_double_quotes_token_stream, deserialize = #field_ident_double_quotes_token_stream))]
                #variant_ident_upper_camel_case_token_stream(postgresql_crud::Value<#type_path_option_to_update_token_stream>)
            }
        });
        let maybe_id_variant_token_stream = if contains_id {
            quote::quote!{Id(postgresql_crud::Value<postgresql_crud::JsonUuidOptionToUpdate>),}
        }
        else {
            proc_macro2::TokenStream::new()
        };
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub enum #struct_ident_token_stream {
                #maybe_id_variant_token_stream
                #(#variants_token_stream),*
            }
        }
    };

    let field0_token_stream = quote::quote!{__field0};
    let field0_field1_token_stream = quote::quote!{__field0, __field1};


    //its for GeneratePostgresqlCrud
    let ident_token_stream = {
        let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&ident);


        let ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_self_reader_upper_camel_case_token_stream(&ident);
        let ident_reader_token_stream = generate_tokens_reader_token_stream(&ident_reader_upper_camel_case_token_stream, &ident_options_to_read_upper_camel_case_token_stream);

        let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
        let ident_to_create_token_stream = generate_tokens_to_create_token_stream(&ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&ident_to_create_upper_camel_case_token_stream, false);
        let impl_postgresql_crud_bind_query_for_ident_to_create_token_stream = {
            let try_generate_bind_increments_token_stream = vec_syn_field.iter().map(|element| {
                let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
                let element_ident_value_comma_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &format!("'{element_ident}',{{value}},"),
                    &proc_macro_name_upper_camel_case_ident_stringified
                );
                quote::quote!{
                    match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(&self.#element_ident, increment) {
                        Ok(value) => {
                            increments.push_str(&format!(#element_ident_value_comma_double_quotes_token_stream));
                        }
                        Err(error) => {
                            return Err(error.into());
                        }
                    }
                }
            });
            let bind_value_to_query_token_stream = vec_syn_field.iter().map(|element| {
                let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                });
                quote::quote!{
                    query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(self.#element_ident, query);
                }
            });
            quote::quote!{
                impl<'a> postgresql_crud::BindQuery<'a> for #ident_to_create_upper_camel_case_token_stream {
                    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                        todo!()
                    }
                    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                        let mut increments = std::string::String::from("");
                        #(#try_generate_bind_increments_token_stream)*
                        let _ = increments.pop();
                        Ok(format!("jsonb_build_object({increments})"))
                    }
                    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #(#bind_value_to_query_token_stream)*
                        query
                    }
                }
            }
        };


        let ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_self_options_to_read_upper_camel_case_stringified(&ident);
        let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&ident);
        let ident_options_to_read_token_stream = generate_tokens_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, false);
        let impl_serde_deserialize_for_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &ident_options_to_read_upper_camel_case_stringified,
            &ident_options_to_read_upper_camel_case_token_stream,
            false,
        );
        //todo maybe no need to impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &ident_options_to_read_upper_camel_case_token_stream,
            &fields_some_value_self_options_to_read_initialization_content_token_stream,
        );

        let ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_self_field_reader_upper_camel_case_stringified(&ident);
        let ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_self_field_reader_upper_camel_case_token_stream(&ident);
        let ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &ident.to_string(),
            &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
        );
        let impl_serde_deserialize_for_ident_field_reader_token_stream = {
            let tuple_struct_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_ident_field_reader_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{ident_field_reader_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &ident_field_reader_upper_camel_case_token_stream,
                &field0_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #ident_field_reader_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#ident_field_reader_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_ident_field_reader_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream> = <std::vec::Vec<
                                    #ident_field_to_read_upper_camel_case_token_stream,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_ident_field_reader_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<#ident_field_reader_upper_camel_case_token_stream>,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };


        //this is temporary, todo remove and refactor later
        let impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
            let ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream = {
                let value = format!("{ident}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecErrorNamedUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let pub_enum_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
                quote::quote!{
                    #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                    pub enum #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
                        Todo {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                    }
                }
            };
            let impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream = {
                quote::quote!{
                    impl error_occurence_lib::ToStdStringString for #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream {
                        fn to_std_string_string(&self) -> std::string::String {
                            format!("{self:?}")
                        }
                    }
                }
            };
            let impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream = {
                let variants_token_stream = vec_syn_field.iter().map(|element| {
                    let field_ident_stringified = element
                        .ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                        })
                        .to_string();
                    let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
                    let field_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&field_ident_stringified, &proc_macro_name_upper_camel_case_ident_stringified);
                    quote::quote!{
                        #ident_field_to_read_upper_camel_case_token_stream::#variant_ident_upper_camel_case_token_stream(value) => {
                            acc.push_str(&format!(
                                "{}||",
                                postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
                                    value,
                                    #field_ident_double_quotes_token_stream,
                                    column_name_and_maybe_field_getter,
                                    column_name_and_maybe_field_getter_for_error_message
                                )
                            ));
                        }
                    }
                });
                quote::quote!{
                    impl postgresql_crud::GeneratePostgresqlQueryPartToRead<#ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream, ()> for #ident_field_to_read_upper_camel_case_token_stream {
                        fn generate_postgresql_query_part_to_read_from_self_vec(
                            value: &std::vec::Vec<Self>,
                            column_name_and_maybe_field_getter: &std::primitive::str,
                            column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
                            is_optional: std::primitive::bool
                        ) -> Result<std::string::String, #ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_upper_camel_case_token_stream> {
                            let mut acc = std::string::String::default();
                            for element in value {
                                match element {
                                    #(#variants_token_stream),*
                                }
                            }
                            let _ = acc.pop();
                            let _ = acc.pop();
                            Ok(format!("{acc}"))
                        }
                        fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, ()> {
                            todo!()
                        }
                    }
                }
            };
            quote::quote!{
                #pub_enum_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_token_stream
                #impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream
            }
        };
        quote::quote!{
            #impl_std_fmt_display_for_ident_token_stream


            #ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_ident_to_create_token_stream
            // //todo impl postgresql_crud::CheckIdExistsInJsonGenericFields for Something
            #impl_postgresql_crud_bind_query_for_ident_to_create_token_stream


            #ident_options_to_read_token_stream
            #impl_serde_deserialize_for_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_token_stream

            #ident_field_reader_token_stream
            #impl_serde_deserialize_for_ident_field_reader_token_stream
            //// #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_upper_camel_case_token_stream_token_stream

            #ident_reader_token_stream
            //todo impl_serde_deserialize_for_generic_ident_reader_token_stream
            //todo impl postgresql_crud_StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement_for_generic_ident_token_stream //todo maybe rename trait

            //todo generic_ident_field_to_update_token_stream
            //todo impl_error_occurence_lib::ToStdStringString for SomethingFieldToUpdate
            //todo generic_ident_option_to_update_token_stream
            //todo impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionToUpdate
            //todo SomethingOptionsToUpdate
            //todo impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionsToUpdate
            //todo SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
            //todo impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed> for SomethingOptionsToUpdate

            #impl_postgresql_crud_generate_postgresql_query_part_to_read_ident_generate_postgresql_query_part_to_read_from_self_vec_error_named_for_ident_field_to_read_token_stream

        }
    };

    let generic_with_id_ident_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericWithIdSelfUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_with_id_self_upper_camel_case_token_stream(&ident);
    
    //its for GeneratePostgresqlQueryPart (json logic)
    let generic_with_id_ident_token_stream = {
        let generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &generic_with_id_ident_upper_camel_case_token_stream,
            &{
                quote::quote!{{
                    pub id: postgresql_crud::JsonUuid,
                    #fields_token_stream
                }}
            }
        );
        let impl_std_fmt_display_for_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&generic_with_id_ident_upper_camel_case_token_stream);

        let generic_with_id_ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericWithIdSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_with_id_self_to_create_upper_camel_case_token_stream(&ident);
        let generic_with_id_ident_to_create_token_stream = generate_tokens_to_create_token_stream(&generic_with_id_ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&generic_with_id_ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&generic_with_id_ident_to_create_upper_camel_case_token_stream, true);


        let generic_with_id_ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensGenericWithIdSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_generic_with_id_self_options_to_read_upper_camel_case_stringified(&ident);
        let generic_with_id_ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericWithIdSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_with_id_self_options_to_read_upper_camel_case_token_stream(&ident);
        let generic_with_id_ident_options_to_read_token_stream = generate_tokens_options_to_read_token_stream(&generic_with_id_ident_options_to_read_upper_camel_case_token_stream, true);
        let impl_serde_deserialize_for_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &generic_with_id_ident_options_to_read_upper_camel_case_stringified,
            &generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
            true,
        );
        //todo maybe no need to impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_options_to_read_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
            &fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
        );
        //

        let generic_with_id_ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericWithIdSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_with_id_self_field_reader_upper_camel_case_token_stream(&ident);
        let generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &naming_conventions::ImplQuoteToTokensGenericWithIdSelfUpperCamelCaseStringified::impl_quote_to_tokens_generic_with_id_self_upper_camel_case_stringified(&ident),
            &FieldReaderContent::GenericWithIdIdentAndStdOptionOptionGenericWithIdIdent
        );
        let impl_serde_deserialize_for_generic_with_id_ident_field_reader_token_stream = {
            let generic_with_id_ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensGenericWithIdSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_generic_with_id_self_field_reader_upper_camel_case_stringified(&ident);
            let tuple_struct_generic_with_id_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{generic_with_id_ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_generic_with_id_ident_field_reader_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{generic_with_id_ident_field_reader_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &generic_with_id_ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                &field0_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #generic_with_id_ident_field_reader_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#generic_with_id_ident_field_reader_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #generic_with_id_ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_generic_with_id_ident_field_reader_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream> = <std::vec::Vec<
                                    #ident_with_id_field_to_read_upper_camel_case_token_stream,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_generic_with_id_ident_field_reader_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        //
        let generic_with_id_ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericWithIdSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_with_id_self_reader_upper_camel_case_token_stream(&ident);
        let generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(&generic_with_id_ident_reader_upper_camel_case_token_stream, &generic_with_id_ident_options_to_read_upper_camel_case_token_stream);
        //
        quote::quote!{
            #generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_generic_with_id_ident_token_stream


            #generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_generic_with_id_ident_to_create_token_stream


            #generic_with_id_ident_options_to_read_token_stream
            #impl_serde_deserialize_for_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_with_id_ident_options_to_read_token_stream

            #generic_with_id_ident_field_reader_token_stream
            // #impl_try_new_token_stream
            #impl_serde_deserialize_for_generic_with_id_ident_field_reader_token_stream

            //
            #generic_with_id_ident_reader_token_stream
        }
    };

    let when_jsonb_typeof_column_name_and_maybe_field_getter_null_then_jsonb_build_object_ok_null_stringified = "when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)";
    let space_and_not_null_stringified = " and not null";

    //its for GeneratePostgresqlQueryPart (json logic)
    let generic_ident_token_stream = {
        let generic_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &generic_ident_upper_camel_case_token_stream,
            &quote::quote!{{#fields_token_stream}}
        );
        let impl_std_fmt_display_for_generic_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&generic_ident_upper_camel_case_token_stream);


        let generic_ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_to_create_upper_camel_case_token_stream(&ident);
        let generic_ident_to_create_token_stream = generate_tokens_to_create_token_stream(&generic_ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&generic_ident_to_create_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&generic_ident_to_create_upper_camel_case_token_stream, false);


        let generic_ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensGenericSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_generic_self_options_to_read_upper_camel_case_stringified(&ident);
        let generic_ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_options_to_read_upper_camel_case_token_stream(&ident);
        let generic_ident_options_to_read_token_stream = generate_tokens_options_to_read_token_stream(&generic_ident_options_to_read_upper_camel_case_token_stream, false);
        let impl_serde_deserialize_for_generic_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &generic_ident_options_to_read_upper_camel_case_stringified,
            &generic_ident_options_to_read_upper_camel_case_token_stream,
            false,
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_options_to_read_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &generic_ident_options_to_read_upper_camel_case_token_stream,
            &fields_some_value_self_options_to_read_initialization_content_token_stream,
        );

        let generic_ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&ident);
        let generic_ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&ident);
        let generic_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &naming_conventions::ImplQuoteToTokensGenericSelfUpperCamelCaseStringified::impl_quote_to_tokens_generic_self_upper_camel_case_stringified(&ident),
            &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
        );
        let impl_serde_deserialize_for_generic_ident_field_reader_token_stream = {
            let tuple_struct_generic_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{generic_ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_generic_ident_field_reader_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{generic_ident_field_reader_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &generic_ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &generic_ident_field_reader_upper_camel_case_token_stream,
                &field0_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #generic_ident_field_reader_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#generic_ident_field_reader_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #generic_ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_generic_ident_field_reader_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream> = <std::vec::Vec<
                                    #ident_field_to_read_upper_camel_case_token_stream,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_generic_ident_field_reader_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<#generic_ident_field_reader_upper_camel_case_token_stream>,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream(&generic_ident_field_reader_upper_camel_case_token_stream);
        
        /////////////
        //todo
        let generic_ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_reader_upper_camel_case_token_stream(&ident);
        let generic_ident_reader_token_stream = generate_tokens_reader_token_stream(&generic_ident_reader_upper_camel_case_token_stream, &generic_ident_options_to_read_upper_camel_case_token_stream);
        //
        
        let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
            &generic_ident_field_reader_upper_camel_case_token_stream,
            false,
            &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))"}
        );

        let generic_ident_option_to_update_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensGenericSelfOptionToUpdateUpperCamelCaseTokenStream::impl_quote_to_tokens_generic_self_option_to_update_upper_camel_case_token_stream(&ident);
        let generic_ident_option_to_update_token_stream = generate_tokens_option_to_update_origin_token_stream(
            &generic_ident_option_to_update_upper_camel_case_token_stream,
            false,
        );
        quote::quote!{
            #generic_ident_token_stream
            #impl_std_fmt_display_for_generic_ident_token_stream


            #generic_ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_generic_ident_to_create_token_stream


            #generic_ident_options_to_read_token_stream
            #impl_serde_deserialize_for_generic_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_options_to_read_token_stream

            #generic_ident_field_reader_token_stream
            #impl_serde_deserialize_for_generic_ident_field_reader_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_generic_ident_field_reader_token_stream
            // #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_upper_camel_case_token_stream_token_stream

            #generic_ident_reader_token_stream
            //todo impl_serde_deserialize_for_generic_ident_reader_token_stream
            //todo impl postgresql_crud_StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement_for_generic_ident_token_stream //todo maybe rename trait

            //todo generic_ident_field_to_update_token_stream
            //todo impl_error_occurence_lib::ToStdStringString for SomethingFieldToUpdate
            //todo generic_ident_option_to_update_token_stream
            //todo impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionToUpdate
            //todo SomethingOptionsToUpdate
            //todo impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionsToUpdate
            //todo SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
            //todo impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed> for SomethingOptionsToUpdate

            //todo impl postgresql_crud::CheckIdExistsInJsonGenericFields for Something
            //todo 

            #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_generic_ident_field_reader_token_stream

            #generic_ident_option_to_update_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_option_option_generic_ident_token_stream = {
        let std_option_option_generic_ident_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_option_option_generic_ident_upper_camel_case_token_stream,
            &quote::quote!{(pub std::option::Option<#generic_ident_upper_camel_case_token_stream>);}
        );
        let impl_std_fmt_display_for_std_option_option_generic_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_option_option_generic_ident_upper_camel_case_token_stream);


        //todo maybe later reuse it as generic_ident_to_create
        let std_option_option_generic_ident_to_create_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfToCreateOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_to_create_origin_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_to_create_origin_token_stream = generate_tokens_to_create_token_stream(&std_option_option_generic_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&std_option_option_generic_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&std_option_option_generic_ident_to_create_origin_upper_camel_case_token_stream, false);
        let std_option_option_generic_ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_to_create_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_option_option_generic_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(pub std::option::Option<#std_option_option_generic_ident_to_create_origin_upper_camel_case_token_stream>);}
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_generic_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))}
        );
        let impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_token_stream = {
            quote::quote!{
                impl<'a> postgresql_crud::JsonCreateBindQuery<'a> for #std_option_option_generic_ident_to_create_upper_camel_case_token_stream {
                    fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed> {
                        match &self.0 {
                            Some(value) => match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(value, increment) {
                                Ok(value) => Ok(value),
                                //todo additional error variant
                                Err(error) => Err(error)
                            },
                            //maybe not use null here and use increment logic
                            None => Ok(std::string::String::from("null"))
                        }
                    }
                    fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        if let Some(value) = self.0 {
                            query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(value, query);
                        }
                        query
                    }
                }
            }
        };


        let std_option_option_generic_ident_options_to_read_origin_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfOptionsToReadOriginUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_generic_self_options_to_read_origin_upper_camel_case_stringified(&ident);
        let std_option_option_generic_ident_options_to_read_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfOptionsToReadOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_options_to_read_origin_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_options_to_read_origin_token_stream = generate_tokens_options_to_read_token_stream(&std_option_option_generic_ident_options_to_read_origin_upper_camel_case_token_stream, false);
        let impl_serde_deserialize_for_std_option_option_generic_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &std_option_option_generic_ident_options_to_read_origin_upper_camel_case_stringified,
            &std_option_option_generic_ident_options_to_read_origin_upper_camel_case_token_stream,
            false,
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_options_to_read_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_generic_ident_options_to_read_origin_upper_camel_case_token_stream,
            &fields_some_value_self_options_to_read_initialization_content_token_stream,
        );
        let std_option_option_generic_ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_options_to_read_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_options_to_read_token_stream = quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #std_option_option_generic_ident_options_to_read_upper_camel_case_token_stream(std::option::Option<#std_option_option_generic_ident_options_to_read_origin_upper_camel_case_token_stream>);
        };
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_options_to_read_token_stream =             generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_generic_ident_options_to_read_upper_camel_case_token_stream,
            &quote::quote!{(Some(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream))},
        );

        let std_option_option_generic_ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_generic_self_field_reader_upper_camel_case_stringified(&ident);
        let std_option_option_generic_ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_field_reader_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_generic_self_upper_camel_case_stringified(&ident),
            &FieldReaderContent::GenericIdentAndStdOptionOptionGenericIdent
        );
        let impl_serde_deserialize_for_std_option_option_generic_ident_field_reader_token_stream = {
            let tuple_struct_std_option_option_generic_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_option_option_generic_ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_std_option_option_generic_ident_field_reader_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_option_option_generic_ident_field_reader_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &std_option_option_generic_ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &std_option_option_generic_ident_field_reader_upper_camel_case_token_stream,
                &field0_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #std_option_option_generic_ident_field_reader_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<
                                #std_option_option_generic_ident_field_reader_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #std_option_option_generic_ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_std_option_option_generic_ident_field_reader_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream> = <std::vec::Vec<
                                    #ident_field_to_read_upper_camel_case_token_stream,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_std_option_option_generic_ident_field_reader_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #std_option_option_generic_ident_field_reader_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_non_vec_field_reader_token_stream(&std_option_option_generic_ident_field_reader_upper_camel_case_token_stream);
        //

        let std_option_option_generic_ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_reader_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_reader_token_stream = generate_tokens_reader_token_stream(&std_option_option_generic_ident_reader_upper_camel_case_token_stream, &std_option_option_generic_ident_options_to_read_upper_camel_case_token_stream);


        let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
            &std_option_option_generic_ident_field_reader_upper_camel_case_token_stream,
            false,
            &quote::quote!{"jsonb_build_object('{field_ident}', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('value', null) else jsonb_build_object('value',{acc}) end)"}
        );

        let std_option_option_generic_ident_option_to_update_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfOptionToUpdateOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_option_to_update_origin_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_option_to_update_origin_token_stream = generate_tokens_option_to_update_origin_token_stream(
            &std_option_option_generic_ident_option_to_update_origin_upper_camel_case_token_stream,
            false,
        );
        let std_option_option_generic_ident_option_to_update_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionGenericSelfOptionToUpdateUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_generic_self_option_to_update_upper_camel_case_token_stream(&ident);
        let std_option_option_generic_ident_option_to_update_token_stream = quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub enum #std_option_option_generic_ident_option_to_update_upper_camel_case_token_stream(pub std::option::Option<#std_option_option_generic_ident_option_to_update_origin_upper_camel_case_token_stream>);
        };
        quote::quote!{
            #std_option_option_generic_ident_token_stream
            #impl_std_fmt_display_for_std_option_option_generic_ident_token_stream


            #std_option_option_generic_ident_to_create_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_origin_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_origin_token_stream

            #std_option_option_generic_ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_option_option_generic_ident_to_create_token_stream

            #std_option_option_generic_ident_options_to_read_origin_token_stream
            #impl_serde_deserialize_for_std_option_option_generic_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_options_to_read_origin_token_stream
            #std_option_option_generic_ident_options_to_read_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_options_to_read_token_stream

            #std_option_option_generic_ident_field_reader_token_stream
            #impl_serde_deserialize_for_std_option_option_generic_ident_field_reader_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_generic_ident_field_reader_token_stream
            // #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_upper_camel_case_token_stream_token_stream
            //
            #std_option_option_generic_ident_reader_token_stream
            //

            #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_generic_ident_field_reader_token_stream

            #std_option_option_generic_ident_option_to_update_origin_token_stream
            #std_option_option_generic_ident_option_to_update_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_vec_vec_generic_with_id_ident_token_stream = {
        let std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream,
            &quote::quote!{(pub std::vec::Vec<#generic_with_id_ident_upper_camel_case_token_stream>);}
        );
        let impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream);


        let std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfToCreateOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_to_create_origin_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_tokens_to_create_token_stream(&std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream, true);
        let std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_to_create_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(pub std::vec::Vec<#std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream>);}
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])}
        );
        let impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_token_stream = {
            quote::quote!{
                impl<'a> postgresql_crud::JsonCreateBindQuery<'a> for #std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream {
                    fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed> {
                        let mut acc = std::string::String::default();
                        for element in &self.0 {
                            match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(element, increment) {
                                Ok(value) => {
                                    acc.push_str(&format!("{value},"));
                                },
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                        let _ = acc.pop();
                        Ok(format!("jsonb_build_array({acc})"))
                    }
                    fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        for element in self.0 {
                            query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(element, query);
                        }
                        query
                    }
                }
            }
        };


        let std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfOptionsToReadOriginUpperCamelCaseStringified::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_options_to_read_origin_upper_camel_case_stringified(&ident);
        let std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfOptionsToReadOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_options_to_read_origin_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_tokens_options_to_read_token_stream(&std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream, true);
        let impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_stringified,
            &std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
            true,
        );
        //
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
            &fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
        );
        //
        let std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_options_to_read_upper_camel_case_stringified(&ident);
        let std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_options_to_read_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_options_to_read_token_stream = quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
            pub struct #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(pub std::vec::Vec<#std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream>);
        };
        //
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream =             generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
            &quote::quote!{(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
        );
        //
        let impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = {
            let tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let check_not_unique_id_token_stream = {
                quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in &__field0 {
                            if let Some(value) = &element.id {
                                if acc.contains(&&value.value) {
                                    return Err(serde::de::Error::custom(format!("not unique id {}", value.value.0)));
                                }
                                else {
                                    acc.push(&value.value);
                                }
                            }
                        }
                    }
                }
            };
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<
                                #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter<'_>,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::vec::Vec<
                                    #std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                > = <std::vec::Vec<
                                    #std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #check_not_unique_id_token_stream
                                serde::__private::Ok(
                                    #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(__field0),
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
                                    std::vec::Vec<
                                        #std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                    >,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #check_not_unique_id_token_stream
                                serde::__private::Ok(
                                    #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(__field0),
                                )
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #std_option_option_generic_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };

        let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_field_reader_upper_camel_case_stringified(&ident);
        let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_field_reader_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfUpperCamelCaseStringified::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_upper_camel_case_stringified(&ident),
            &FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
        );
        let impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = {
            let struct_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{struct_space_stringified}{std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{struct_space_stringified}{std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified} with 2 elements"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                &field0_field1_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream {
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
                        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
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
                                    "field_vec" => serde::__private::Ok(__Field::__field0),
                                    "pagination" => serde::__private::Ok(__Field::__field1),
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
                                    b"field_vec" => serde::__private::Ok(__Field::__field0),
                                    b"pagination" => serde::__private::Ok(__Field::__field1),
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
                            marker: serde::__private::PhantomData<
                                #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream,
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
                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match serde::de::SeqAccess::next_element::<
                                    #postgersql_crud_pagination_token_stream,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                > = serde::__private::None;
                                let mut __field1: serde::__private::Option<#postgersql_crud_pagination_token_stream> = serde::__private::None;
                                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if serde::__private::Option::is_some(&__field0) {
                                                return serde::__private::Err(
                                                    <__A::Error as serde::de::Error>::duplicate_field(
                                                        "field_vec",
                                                    ),
                                                );
                                            }
                                            __field0 = serde::__private::Some(
                                                serde::de::MapAccess::next_value::<
                                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if serde::__private::Option::is_some(&__field1) {
                                                return serde::__private::Err(
                                                    <__A::Error as serde::de::Error>::duplicate_field(
                                                        "pagination",
                                                    ),
                                                );
                                            }
                                            __field1 = serde::__private::Some(
                                                serde::de::MapAccess::next_value::<#postgersql_crud_pagination_token_stream>(&mut __map)?,
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
                                        serde::__private::de::missing_field("field_vec")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    serde::__private::Some(__field1) => __field1,
                                    serde::__private::None => {
                                        serde::__private::de::missing_field("pagination")?
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
                        serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream(&std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream);

        //
        let std_vec_vec_generic_with_id_ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdVecVecGenericWithIdSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_vec_vec_generic_with_id_self_reader_upper_camel_case_token_stream(&ident);
        let std_vec_vec_generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(&std_vec_vec_generic_with_id_ident_reader_upper_camel_case_token_stream, &std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream);
        //


        let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
            &std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
            true,
            &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value',(select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))"}
        );
        quote::quote!{
            #std_vec_vec_generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_std_vec_vec_generic_with_id_ident_token_stream


            #std_vec_vec_generic_with_id_ident_to_create_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream

            #std_vec_vec_generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_vec_vec_generic_with_id_ident_to_create_token_stream


            #std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #std_vec_vec_generic_with_id_ident_options_to_read_token_stream
            #impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
            
            #std_vec_vec_generic_with_id_ident_field_reader_token_stream
            #impl_serde_deserialize_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            // #impl_postgersql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_ident_field_reader_upper_camel_case_token_stream_token_stream

            //
            #std_vec_vec_generic_with_id_ident_reader_token_stream
            //

            #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_vec_vec_generic_with_id_ident_field_reader_token_stream
        }
    };
    //its for GeneratePostgresqlQueryPart (json logic)
    let std_option_option_std_vec_vec_generic_with_id_ident_token_stream = {
        let std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream,
            &quote::quote!{(std::option::Option<std::vec::Vec<#generic_with_id_ident_upper_camel_case_token_stream>>);}
        );
        let impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_upper_camel_case_token_stream);
        
        
        let std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfToCreateOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_to_create_origin_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_tokens_to_create_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_to_create_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream);
        let impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream = generate_impl_postgresql_crud_json_create_bind_query_for_tokens_to_create_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream, true);
        let std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_to_create_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_supported_generics_template_struct_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(pub std::option::Option<std::vec::Vec<#std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_upper_camel_case_token_stream>>);}
        );
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream,
            &quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))}
        );
        let impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream = {
            quote::quote!{
                impl<'a> postgresql_crud::JsonCreateBindQuery<'a> for #std_option_option_std_vec_vec_generic_with_id_ident_to_create_upper_camel_case_token_stream {
                    fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::JsonCreateTryGenerateBindIncrementsErrorNamed> {
                        match &self.0 {
                            Some(value) => {
                                let mut acc = std::string::String::default();
                                for element in value {
                                    match postgresql_crud::JsonCreateBindQuery::json_create_try_generate_bind_increments(element, increment) {
                                        Ok(value) => {
                                            acc.push_str(&format!("{value},"));
                                        },
                                        Err(error) => {
                                            return Err(error);
                                        }
                                    }
                                }
                                let _ = acc.pop();
                                Ok(format!("jsonb_build_array({acc})"))
                            },
                            None => Ok(std::string::String::from("null"))
                        }
                    }
                    fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        if let Some(value) = self.0 {
                            for element in value {
                                query = postgresql_crud::JsonCreateBindQuery::json_create_bind_value_to_query(element, query);
                            }
                        }
                        query
                    }
                }
            }
        };


        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfOptionsToReadOriginUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_options_to_read_origin_upper_camel_case_stringified(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfOptionsToReadOriginUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_options_to_read_origin_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_tokens_options_to_read_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream, true);
        let impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_serde_deserialize_for_options_to_read_origin_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_stringified,
            &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
            true,
        );
        //
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
            &fields_with_id_some_value_self_options_to_read_initialization_content_token_stream,
        );
        //
        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_options_to_read_upper_camel_case_stringified(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_options_to_read_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
            pub struct #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(pub std::option::Option<std::vec::Vec<#std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream>>);
        };
        //
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_with_content_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
            &quote::quote!{(Some(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]))},
        );
        //
        let impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream = {
            let tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{tuple_struct_space_stringified}{std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified} with 1 element"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let check_not_unique_id_token_stream = {
                quote::quote!{
                    if let Some(value) = &__field0 {
                        let mut acc = vec![];
                        for element in value {
                            if let Some(value) = &element.id {
                                if acc.contains(&&value.value) {
                                    return Err(serde::de::Error::custom(format!("not unique id {}", value.value.0)));
                                }
                                else {
                                    acc.push(&value.value);
                                }
                            }
                        }
                    }
                }
            };
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<
                                #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter<'_>,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_double_quotes_token_stream,
                                )
                            }
                            #[inline]
                            fn visit_newtype_struct<__E>(
                                self,
                                __e: __E,
                            ) -> serde::__private::Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let __field0: std::option::Option<
                                    std::vec::Vec<
                                        #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                    >,
                                > = <std::option::Option<
                                    std::vec::Vec<
                                        #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                    >,
                                > as serde::Deserialize>::deserialize(__e)?;
                                #check_not_unique_id_token_stream
                                serde::__private::Ok(
                                    #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(
                                        __field0,
                                    ),
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
                                    std::option::Option<
                                        std::vec::Vec<
                                            #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_upper_camel_case_token_stream,
                                        >,
                                    >,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#tuple_struct_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #check_not_unique_id_token_stream
                                serde::__private::Ok(
                                    #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream(
                                        __field0,
                                    ),
                                )
                            }
                        }
                        serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        

        let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfFieldReaderUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_field_reader_upper_camel_case_stringified(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_field_reader_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_tokens_field_reader_token_stream(
            &naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfUpperCamelCaseStringified::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_upper_camel_case_stringified(&ident),
            &FieldReaderContent::StdVecVecGenericWithIdIdentAndStdOptionOptionStdVecVecGenericWithIdIdent
        );
        let impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = {
            let struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{struct_space_stringified}{std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified}"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &format!("{struct_space_stringified}{std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified} with 2 elements"),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let match_try_new_in_deserialize_token_stream = generate_match_try_new_in_deserialize_token_stream(
                &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                &field0_field1_token_stream,
            );
            quote::quote!{
                impl<'de> serde::Deserialize<'de> for #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream {
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
                        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
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
                                    "field_vec" => serde::__private::Ok(__Field::__field0),
                                    "pagination" => serde::__private::Ok(__Field::__field1),
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
                                    b"field_vec" => serde::__private::Ok(__Field::__field0),
                                    b"pagination" => serde::__private::Ok(__Field::__field1),
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
                            marker: serde::__private::PhantomData<
                                #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream;
                            fn expecting(
                                &self,
                                __formatter: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_double_quotes_token_stream,
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
                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                0usize,
                                                &#struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match serde::de::SeqAccess::next_element::<
                                    #postgersql_crud_pagination_token_stream,
                                >(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(
                                            serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
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
                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                > = serde::__private::None;
                                let mut __field1: serde::__private::Option<#postgersql_crud_pagination_token_stream> = serde::__private::None;
                                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if serde::__private::Option::is_some(&__field0) {
                                                return serde::__private::Err(
                                                    <__A::Error as serde::de::Error>::duplicate_field(
                                                        "field_vec",
                                                    ),
                                                );
                                            }
                                            __field0 = serde::__private::Some(
                                                serde::de::MapAccess::next_value::<
                                                    std::vec::Vec<#ident_with_id_field_to_read_upper_camel_case_token_stream>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if serde::__private::Option::is_some(&__field1) {
                                                return serde::__private::Err(
                                                    <__A::Error as serde::de::Error>::duplicate_field(
                                                        "pagination",
                                                    ),
                                                );
                                            }
                                            __field1 = serde::__private::Some(
                                                serde::de::MapAccess::next_value::<#postgersql_crud_pagination_token_stream>(&mut __map)?,
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
                                        serde::__private::de::missing_field("field_vec")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    serde::__private::Some(__field1) => __field1,
                                    serde::__private::None => {
                                        serde::__private::de::missing_field("pagination")?
                                    }
                                };
                                #match_try_new_in_deserialize_token_stream
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
                        serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: serde::__private::PhantomData::<
                                    #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
                                >,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        };
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_vec_field_reader_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream);

        //
        let std_option_option_std_vec_vec_generic_with_id_ident_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensStdOptionOptionStdVecVecGenericWithIdSelfReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_std_option_option_std_vec_vec_generic_with_id_self_reader_upper_camel_case_token_stream(&ident);
        let std_option_option_std_vec_vec_generic_with_id_ident_reader_token_stream = generate_tokens_reader_token_stream(&std_option_option_std_vec_vec_generic_with_id_ident_reader_upper_camel_case_token_stream, &std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_upper_camel_case_token_stream);
        //


        let impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream = generate_impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_tokens_token_stream(
            &std_option_option_std_vec_vec_generic_with_id_ident_field_reader_upper_camel_case_token_stream,
            true,
            &quote::quote!{"jsonb_build_object('{field_ident}', jsonb_build_object('value', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then null else (select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end}) end))"}
        );
        quote::quote!{
            #std_option_option_std_vec_vec_generic_with_id_ident_token_stream
            #impl_std_fmt_display_for_std_option_option_std_vec_vec_generic_with_id_ident_token_stream


            #std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_origin_token_stream

            #std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream
            #impl_postgresql_crud_json_create_bind_query_for_std_option_option_std_vec_vec_generic_with_id_ident_to_create_token_stream


            #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_origin_token_stream
            #std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
            #impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_options_to_read_token_stream

            #std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            #impl_serde_deserialize_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream

            //
            #std_option_option_std_vec_vec_generic_with_id_ident_reader_token_stream 
            //

            #impl_postgresql_crud_generate_postgresql_query_part_field_to_read_for_std_option_option_std_vec_vec_generic_with_id_ident_field_reader_token_stream
        }
    };
    let generated = quote::quote! {
        // #impl_std_fmt_display_for_ident_token_stream
        // #pub_enum_ident_field_to_read_token_stream
        // #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream
        // #pub_enum_field_generate_postgresql_query_part_to_read_error_named_token_stream
        // #impl_generate_postgresql_query_part_to_read_for_ident_field_to_read_token_stream
        // #pub_struct_ident_options_to_read_token_stream
        // // #impl_std_convert_from_ident_for_ident_options_to_read_token_stream
        // #impl_serde_deserialize_for_ident_options_to_read_token_stream
        // #ident_reader_token_stream
        // #impl_serde_deserialize_for_ident_reader_token_stream
        // #impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_token_stream

        // #pub_enum_ident_field_to_update_token_stream
        // #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream
        // #pub_enum_ident_option_to_update_token_stream //todo write manual serde::Deserialize and check if vec is empty
        // #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream
        // #pub_struct_ident_options_to_update_token_stream
        // #maybe_impl_postgresql_crud_get_json_id_for_ident_options_to_update_token_stream
        // #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_update_token_stream
        // #pub_enum_ident_options_to_update_try_generate_bind_increments_error_named_token_stream
        // #maybe_impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_options_to_update_try_generate_bind_increments_error_named_for_ident_options_to_update_token_stream
        // #maybe_pub_enum_ident_try_generate_json_array_element_update_bind_increments_error_named_token_stream

        // #maybe_impl_postgresql_crud_json_array_element_update_bind_query_ident_try_generate_json_array_element_update_bind_increments_error_named_for_ident_options_to_update_token_stream
        // #maybe_impl_postgresql_crud_json_array_element_create_bind_query_for_ident_to_create_token_stream

        // #pub_struct_ident_to_create_token_stream
        // #impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_to_create_token_stream
        // #impl_postgresql_crud_bind_query_for_ident_to_create_token_stream
        // #maybe_impl_postgresql_crud_get_json_id_for_ident_token_stream
        // #impl_postgresql_crud_check_id_exists_in_json_generic_fields_for_ident_token_stream

        //todo same struct as ident but with id field


        #ident_field_to_read_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_read_token_stream
        #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_to_read_token_stream
        #ident_with_id_field_to_read_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_with_id_field_to_read_token_stream
        #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_with_id_field_to_read_token_stream

        #ident_field_to_update_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream




        #ident_token_stream

        #generic_with_id_ident_token_stream

        #generic_ident_token_stream
        #std_option_option_generic_ident_token_stream
        #std_vec_vec_generic_with_id_ident_token_stream
        #std_option_option_std_vec_vec_generic_with_id_ident_token_stream
    };
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "GeneratePostgresqlQueryPart",
    //         &generated,
    //         "GeneratePostgresqlQueryPart",
    //     );
    // }
    generated.into()
}

// i have a jsonb in postgresql

// {
//     "std_primitive_i8": 8,
//     "std_vec_vec_generic": [
//         {
//             "id": "8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4",
//             "std_primitive_i16": 5
//         },
//         {
//             "id": "951240e0-990e-4cb8-909d-5183ff7725a4",
//             "std_primitive_i16": 6
//         },
//         {
//             "id": "5d628632-13f0-409f-8288-42b356cc033c",
//             "std_primitive_i16": 2
//         },
//         {
//             "id": "63b83936-24c8-429b-ab67-ee2c76856f18",
//             "std_primitive_i16": 3
//         }
//     ]
// }

// i want to set key "std_primitive_i8" to 7.

// and i want to add element { "id": "1ff4db66-1395-4d58-bcf5-8bf69f1b90d3", "std_primitive_i16": 10 } to array in key "std_vec_vec_generic".

// and i want to add element { "id": "847e5f32-d1a5-4d6a-9c55-040cbf60f229", "std_primitive_i16": 20 } to array in key "std_vec_vec_generic".

// and i want to update key "std_primitive_i16" to value 44 where key "id" is "5d628632-13f0-409f-8288-42b356cc033c".

// and i want to update key "std_primitive_i16" to value 55 where key "id" is "63b83936-24c8-429b-ab67-ee2c76856f18".

// and i want to delete element in value of key "std_vec_vec_generic" where key "id" = "8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4".

// and i want to delete element in value of key "std_vec_vec_generic" where key "id" = "951240e0-990e-4cb8-909d-5183ff7725a4".

// please write a query for it

// UPDATE jsongeneric
// SET sqlx_types_json_t_as_postgresql_json_b_not_null =
//     jsonb_set(
//         jsonb_set(
//             sqlx_types_json_t_as_postgresql_json_b_not_null,
//             '{std_primitive_i8}',
//             '27'::jsonb
//         ),
//         '{std_vec_vec_generic}',
// 		(
//                 SELECT jsonb_agg(
//                     CASE
//                         WHEN elem->>'id' = '5d628632-13f0-409f-8288-42b356cc033c'
//                         THEN jsonb_set(elem, '{std_primitive_i16}', '44'::jsonb)

//                         WHEN elem->>'id' = '63b83936-24c8-429b-ab67-ee2c76856f18'
//                         THEN jsonb_set(elem, '{std_primitive_i16}', '55'::jsonb)

//                         ELSE elem
//                     END
//                 )
//                 FROM jsonb_array_elements(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic') AS elem
//                 WHERE elem->>'id' <> '8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4' and elem->>'id' <> '951240e0-990e-4cb8-909d-5183ff7725a4'
//          )
// 		||
// 		jsonb_build_array(
//     		jsonb_build_object('id', '1ff4db66-1395-4d58-bcf5-8bf69f1b90d3', 'std_primitive_i16', 10),
//     		jsonb_build_object('id', '847e5f32-d1a5-4d6a-9c55-040cbf60f229', 'std_primitive_i16', 20)
// 		)
//     )
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
