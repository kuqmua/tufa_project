#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI8(pub std::primitive::i8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI128(pub std::primitive::i128);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU8(pub std::primitive::u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU16(pub std::primitive::u16);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU32(pub std::primitive::u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU64(pub std::primitive::u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU128(pub std::primitive::u128);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveBool(pub std::primitive::bool);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdStringString(pub std::string::String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI8(pub std::option::Option<std::primitive::i8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI16(pub std::option::Option<std::primitive::i16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI32(pub std::option::Option<std::primitive::i32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI64(pub std::option::Option<std::primitive::i64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI128(pub std::option::Option<std::primitive::i128>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU8(pub std::option::Option<std::primitive::u8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU16(pub std::option::Option<std::primitive::u16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU32(pub std::option::Option<std::primitive::u32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU64(pub std::option::Option<std::primitive::u64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU128(pub std::option::Option<std::primitive::u128>);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveF32(pub std::option::Option<std::primitive::f32>);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveF64(pub std::option::Option<std::primitive::f64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveBool(pub std::option::Option<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdStringString(pub std::option::Option<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI8(pub std::vec::Vec<std::primitive::i8>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI16(pub std::vec::Vec<std::primitive::i16>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI32(pub std::vec::Vec<std::primitive::i32>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI64(pub std::vec::Vec<std::primitive::i64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI128(pub std::vec::Vec<std::primitive::i128>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU16(pub std::vec::Vec<std::primitive::u16>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU32(pub std::vec::Vec<std::primitive::u32>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU64(pub std::vec::Vec<std::primitive::u64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU128(pub std::vec::Vec<std::primitive::u128>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveF32(pub std::vec::Vec<std::primitive::f32>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveF64(pub std::vec::Vec<std::primitive::f64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveBool(pub std::vec::Vec<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdStringString(pub std::vec::Vec<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::primitive::i64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdStringString(pub std::option::Option<std::vec::Vec<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI8(pub std::vec::Vec<std::option::Option<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI16(pub std::vec::Vec<std::option::Option<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI32(pub std::vec::Vec<std::option::Option<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI64(pub std::vec::Vec<std::option::Option<std::primitive::i64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI128(pub std::vec::Vec<std::option::Option<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU8(pub std::vec::Vec<std::option::Option<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU16(pub std::vec::Vec<std::option::Option<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU32(pub std::vec::Vec<std::option::Option<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU64(pub std::vec::Vec<std::option::Option<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU128(pub std::vec::Vec<std::option::Option<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF32(pub std::vec::Vec<std::option::Option<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF64(pub std::vec::Vec<std::option::Option<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveBool(pub std::vec::Vec<std::option::Option<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdStringString(pub std::vec::Vec<std::option::Option<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i128>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u128>>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool(pub std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdStringString(pub std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Generic<T>(pub T);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionGeneric<T>(pub std::option::Option<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecGeneric<T>(pub std::vec::Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecGeneric<T>(pub std::option::Option<std::vec::Vec<T>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionGeneric<T>(pub std::vec::Vec<std::option::Option<T>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionGeneric<T>(pub std::option::Option<std::vec::Vec<std::option::Option<T>>>);


/////////////////////
pub trait GeneratePostgresqlQueryPart<T1, T2> {
    fn generate_postgresql_query_part_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, T1> where Self: Sized;
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, T2>;
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    pub std_string_string: StdStringString,
    pub std_vec_vec_std_primitive_bool: StdVecVecStdPrimitiveBool,
    pub generic: Generic<Doggie>,
    pub std_option_option_generic: StdOptionOptionGeneric<Doggie>,
    pub std_vec_vec_generic: StdVecVecGeneric<Doggie>,
    pub std_option_option_std_vec_vec_generic: StdOptionOptionStdVecVecGeneric<Doggie>,
    pub std_vec_vec_std_option_option_generic: StdVecVecStdOptionOptionGeneric<Doggie>,
    pub std_option_option_std_vec_vec_std_option_option_generic: StdOptionOptionStdVecVecStdOptionOptionGeneric<Doggie>,
}
impl std::fmt::Display for Something {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
impl std::convert::From<Something> for SomethingOptions {
    fn from(value: Something) -> Self {
        Self {
            std_string_string: Some(crate::value::Value{ value: value.std_string_string.0 }),
            std_vec_vec_std_primitive_bool: Some(crate::value::Value{ value: value.std_vec_vec_std_primitive_bool.0 }),
            generic: Some(crate::value::Value{ value: DoggieOptions::from(value.generic.0) }),
            //todo rewrite to from or try from impl
            std_option_option_generic: Some(crate::value::Value{ value: Some(match value.std_option_option_generic.0 {
                Some(value) => DoggieOptions {
                    std_string_string: Some(crate::value::Value{ value: value.std_string_string.0 }),
                },
                None => DoggieOptions {
                    std_string_string: None,
                },
            })}),
            std_vec_vec_generic: Some(crate::value::Value{ value: value.std_vec_vec_generic.0.into_iter().map(|element|DoggieOptions::from(element)).collect::<std::vec::Vec<DoggieOptions>>()}),
            std_option_option_std_vec_vec_generic: Some(crate::value::Value{ value: match value.std_option_option_std_vec_vec_generic.0 {
                    Some(value) => Some(value.into_iter().map(|element|DoggieOptions::from(element)).collect::<std::vec::Vec<DoggieOptions>>()),
                    None => None
            }}),
            std_vec_vec_std_option_option_generic: Some(crate::value::Value{ value: value.std_vec_vec_std_option_option_generic.0.into_iter().map(|element|match element {
                Some(value) => Some(DoggieOptions::from(value)),
                None => None
            }).collect::<std::vec::Vec<std::option::Option<DoggieOptions>>>()}),
            std_option_option_std_vec_vec_std_option_option_generic: Some(crate::value::Value{ value: match value.std_option_option_std_vec_vec_std_option_option_generic.0 {
                    Some(value) => Some(value.into_iter().map(|element|match element {
                        Some(value) => Some(DoggieOptions::from(value)),
                        None => None
                    }).collect::<std::vec::Vec<std::option::Option<DoggieOptions>>>()),
                    None => None
                }
            }),


            // std_string_string: Some(std::result::Result::Ok(value.std_string_string.0)),
            // std_vec_vec_std_primitive_bool: Some(std::result::Result::Ok(
            //     value.std_vec_vec_std_primitive_bool.0.into_iter().map(|element|
            //         std::result::Result::Ok(element)
            //     ).collect::<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>()
            // )),
            // generic: Some(std::result::Result::Ok(DoggieOptions::from(value.generic.0))),
            // //todo rewrite to from or try from impl
            // std_option_option_generic: Some(std::result::Result::Ok(Some(match value.std_option_option_generic.0 {
            //     Some(value) => DoggieOptions {
            //         std_string_string: Some(crate::value::Value{ value: value.std_string_string.0 }),
            //     },
            //     None => DoggieOptions {
            //         std_string_string: None,
            //     },
            // }))),
            // std_vec_vec_generic: Some(std::result::Result::Ok(value.std_vec_vec_generic.0.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>())),
            // std_option_option_std_vec_vec_generic: Some(std::result::Result::Ok(match value.std_option_option_std_vec_vec_generic.0 {
            //         Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>()),
            //         None => None
            // })),
            // std_vec_vec_std_option_option_generic: Some(std::result::Result::Ok(value.std_vec_vec_std_option_option_generic.0.into_iter().map(|element|std::result::Result::Ok(match element {
            //     Some(value) => Some(DoggieOptions::from(value)),
            //     None => None
            // })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>())),
            // std_option_option_std_vec_vec_std_option_option_generic: Some(
            //     std::result::Result::Ok(
            //         match value.std_option_option_std_vec_vec_std_option_option_generic.0 {
            //             Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(match element {
            //                 Some(value) => Some(DoggieOptions::from(value)),
            //                 None => None
            //             })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>()),
            //             None => None
            //         }
            //     )
            // ),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum SomethingField {
    #[serde(rename(
        serialize = "std_string_string",
        deserialize = "std_string_string"
    ))]
    StdStringString,
    #[serde(rename(
        serialize = "std_vec_vec_std_primitive_bool",
        deserialize = "std_vec_vec_std_primitive_bool"
    ))]
    StdVecVecStdPrimitiveBool {
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "generic",
        deserialize = "generic"
    ))]
    Generic(std::vec::Vec<DoggieField>),
    #[serde(rename(
        serialize = "std_option_option_generic",
        deserialize = "std_option_option_generic"
    ))]
    StdOptionOptionGeneric(std::vec::Vec<DoggieField>),
    #[serde(rename(
        serialize = "std_vec_vec_generic",
        deserialize = "std_vec_vec_generic"
    ))]
    StdVecVecGeneric {
        field_vec: std::vec::Vec<DoggieField>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "std_option_option_std_vec_vec_generic",
        deserialize = "std_option_option_std_vec_vec_generic"
    ))]
    StdOptionOptionStdVecVecGeneric {
        field_vec: std::vec::Vec<DoggieField>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "std_vec_vec_std_option_option_generic",
        deserialize = "std_vec_vec_std_option_option_generic"
    ))]
    StdVecVecStdOptionOptionGeneric {
        field_vec: std::vec::Vec<DoggieField>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "std_option_option_std_vec_vec_std_option_option_generic",
        deserialize = "std_option_option_std_vec_vec_std_option_option_generic"
    ))]
    StdOptionOptionStdVecVecStdOptionOptionGeneric {
        field_vec: std::vec::Vec<DoggieField>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
}
impl error_occurence_lib::ToStdStringString for SomethingField {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: SomethingField,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    GeneratePostgresqlQueryPart {
        #[eo_error_occurence]
        error: SomethingGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SomethingGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::u64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdOptionOptionGenericFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieField,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DoggieGeneratePostgresqlQueryPartFromSelfVec {
        #[eo_error_occurence]
        field: DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl GeneratePostgresqlQueryPart<SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed, SomethingGeneratePostgresqlQueryPartErrorNamed> for SomethingField {
    fn generate_postgresql_query_part_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed> {
        if value.is_empty() {
            return Err(SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed::FieldsFilterIsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed::NotUniqueFieldFilter {
                    field: element.clone(),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            else {
                unique.push(&element);
            }
        }
        let mut acc = std::string::String::default();
        for element in value {
            match element.generate_postgresql_query_part(column_name_and_maybe_field_getter) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed::GeneratePostgresqlQueryPart {
                        error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        let _ = acc.pop();
        let is_optional_query_part = match is_optional {
            true => format!(r#"
                when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then
                    jsonb_build_object(
                        'Ok',
                        null
                    )
            "#),
            false => std::string::String::default()
        };
        Ok(format!(r#"
            case 
                when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then 
                    jsonb_build_object(
                        'Ok',
                        jsonb_build_object({acc})
                    )
                {is_optional_query_part}
                else 
                    jsonb_build_object(
                        'Err',
                        'todo error message'
                    ) 
            end
        "#))
    }
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            //
            // select jsonb_build_object('std_string_string',sqlx_types_json_t_as_postgresql_json_b_not_null->'std_string_string') as sqlx_types_json_t_as_postgresql_json_b_not_null from jsongeneric where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1

            // SELECT
            // CASE 
            // WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object'
            //        THEN
            //         CASE 
            //             WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_string_string') = 'string' THEN
            //                 jsonb_build_object(
            //                     'std_string_string',
            //                     jsonb_build_object(
            //                         'value',
            //                         sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_string_string'
            //                     )
            //                 )
            //             ELSE 
            //                 jsonb_build_object(
            //                     'std_string_string', 
            //                     NULL
            //                 )
            //         END
            //     ELSE 
            //         NULL
            // END
            
            //  as sqlx_types_json_t_as_postgresql_json_b_not_null
            // FROM jsongeneric
            // where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14;



        // SELECT
        //     CASE 
        //         WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' THEN
        //             jsonb_build_object(
        //                 'std_string_string',
        //                 CASE 
        //                     WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_string_string') = 'string' THEN
        //                          jsonb_build_object(
        //                                 'value',
        //                             sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_string_string'
        //                          )
        //                     ELSE 
        //                            NULL
        //                     END
        //                 )
        //         ELSE 
        //             NULL
        //     END
        // as sqlx_types_json_t_as_postgresql_json_b_not_null
        // FROM jsongeneric
        // where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14;
            //
            Self::StdStringString => Ok(format!(r#"
                'std_string_string',
                case 
                    when jsonb_typeof({column_name_and_maybe_field_getter}->'std_string_string') = 'string' then
                        jsonb_build_object(
                            'Ok',
                            {column_name_and_maybe_field_getter}->'std_string_string'
                        )
                    else 
                        jsonb_build_object('Err','todo this must be error message')
                end 
            "#)),
        //     select 
        //     jsonb_build_object(
        //             'std_string_string',
        //             case 
        //                 when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_string_string') = 'string' then 
        //                     jsonb_build_object(
        //                         'value',
        //                         sqlx_types_json_t_as_postgresql_json_b_not_null->'std_string_string'
        //                     ) 
        //                 else 
        //                     jsonb_build_object(
        //                         'error',
        //                         'sdfsdfsdfsdfsdfsdfs'
        //                     ) 
        //             end
        //         ) 
        // as sqlx_types_json_t_as_postgresql_json_b_not_null 
        // from jsongeneric 
        // where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14


            Self::StdVecVecStdPrimitiveBool {
                limit,
                offset
            } => {
                let start = offset;
                let end = match offset.checked_add(*limit) {
                    Some(value) => value,
                    None => {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                            limit: *limit,
                            offset: *offset,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                };
                //todo maybe check all types are boolean

// WITH elements AS (
//     SELECT 
//         id,
//         jsonb_array_elements(my_jsonb_col) AS element
//     FROM my_table
// ),
// types AS (
//     SELECT
//         id,
//         jsonb_typeof(element) AS type
//     FROM elements
// ),
// checks AS (
//     SELECT
//         id,
//         bool_and(type = 'boolean') AS all_are_boolean
//     FROM types
//     GROUP BY id
// )
// SELECT jsonb_agg(
//     CASE
//         WHEN all_are_boolean THEN
//             jsonb_build_object('Ok', TRUE)
//         ELSE
//             jsonb_build_object('Error', 'Array contains non-boolean elements')
//     END
// ) AS result
// FROM checks;

                Ok(format!(r#"
                    'std_vec_vec_std_primitive_bool',
                    case 
                        when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_bool') = 'array' then 
                            jsonb_build_object(
                                'Ok',
                                (
                                    select jsonb_agg(
                                        case 
                                            when jsonb_typeof(value) = 'boolean' then 
                                                jsonb_build_object(
                                                  'Ok', 
                                                  value
                                                ) 
                                          else 
                                            jsonb_build_object(
                                                'Err', 
                                                'todo error message'
                                            ) 
                                        end
                                    ) 
                                    from jsonb_array_elements(
                                        (select {column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_bool')
                                    )
                                    with ordinality 
                                    where ordinality between {start} and {end}
                                )
                            )
                        else 
                            jsonb_build_object(
                                'Err', 
                                'todo this must be error message'
                            ) 
                    end
                "#))
// select 
// jsonb_build_object(
// 	'std_vec_vec_std_primitive_bool',
// 	case 
// 		when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_std_primitive_bool') = 'array' then
// 			jsonb_build_object(
// 				'Ok',
// 				(
// 					select jsonb_agg(value) 
// 					from jsonb_array_elements(
// 						(select sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_std_primitive_bool')
// 					) 
// 					with ordinality where ordinality between 0 and 5
// 				)
// 			)
// 		else 
// 			jsonb_build_object(
// 				'Err',
// 				'todo this must be error message'
// 			)
		
// 	end
// )
// as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
            },
            Self::Generic(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                fields_vec,
                &format!("{column_name_and_maybe_field_getter}->'generic'"),
                false
            ) {
                Ok(value) => Ok(format!("'generic',{value}")),
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },

        // select 
        //     case 
        //     when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' then 
        //         jsonb_build_object(
        //             'Ok',
        //             jsonb_build_object(
        //                 'generic',
        //                 case
        //                      when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'generic') = 'object' then 
        //                         jsonb_build_object(
        //                             'Ok',
        //                             jsonb_build_object(
        //                                 'std_string_string',
        //                                 case 
        //                                     when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'generic'->'std_string_string') = 'string' then 
        //                                         jsonb_build_object(
        //                                             'Ok',
        //                                             sqlx_types_json_t_as_postgresql_json_b_not_null->'generic'->'std_string_string'
        //                                         )
        //                                     else 
        //                                         jsonb_build_object(
        //                                             'Err',
        //                                             'todo error message'
        //                                         )
        //                                 end
        //                             )
        //                         )
        //                     else 
        //                         jsonb_build_object(
        //                             'Err',
        //                             'todo error message'
        //                         )
        //                 end
        //             )
        //         ) 
        //     else 
        //         jsonb_build_object(
        //             'Err',
        //             'todo error message'
        //         ) 
        // end  
        // as sqlx_types_json_t_as_postgresql_json_b_not_null 
        // from jsongeneric 
        // where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14

            Self::StdOptionOptionGeneric(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                fields_vec,
                &format!("{column_name_and_maybe_field_getter}->'std_option_option_generic'"),
                true
            ) {
                Ok(value) => Ok(format!("'std_option_option_generic',{value}")),
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },

// select 
// 	case 
// 		when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' then 
// 			jsonb_build_object(
// 				'Ok',
// 				jsonb_build_object(
// 					'std_option_option_generic',
// 					case
// 						when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_generic') = 'object' then
// 							case
// 								when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_generic'->'std_string_string') = 'string' then
// 									jsonb_build_object(
// 										'Ok',
// 										sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_generic'->'std_string_string'
// 									)
// 								else 
// 									jsonb_build_object(
// 										'Err',
// 										'todo error message'
// 									)				
// 							end
// 						when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_generic') = 'null' then
// 							jsonb_build_object(
// 								'Ok',
// 								null
// 							)
// 						else 
// 							jsonb_build_object(
// 								'Err',
// 								'todo error message'
// 							)
// 					end
// 				)
// 			) 
// 		else 
// 			jsonb_build_object(
// 				'Err',
// 				'todo error message'
// 			) 
// 	end  
// as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
            Self::StdVecVecGeneric {
                field_vec,
                limit,
                offset
            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                field_vec,
                &format!("value"),
                false
            ) {
                Ok(value) => {
                    let start = offset;
                    let end = match offset.checked_add(*limit) {
                        Some(value) => value,
                        None => {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                                limit: *limit,
                                offset: *offset,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    };
                    Ok(format!(r#"
                        'std_vec_vec_generic',
                        case 
                            when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_generic') = 'array' then
                                jsonb_build_object(
                                    'Ok',
                                    (
                                        select jsonb_agg({value}) 
                                        from jsonb_array_elements(
                                            (select {column_name_and_maybe_field_getter}->'std_vec_vec_generic')
                                        ) 
                                        with ordinality 
                                        where ordinality between {start} and {end}
                                    )
                                )
                            else 
                                jsonb_build_object(
                                    'Err',
                                    'todo error message'
                                )
                        end
                    "#))
                },
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },
// select 
//    case 
//        when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' then 
//            jsonb_build_object(
//                'Ok',
//                jsonb_build_object(
//                    'std_vec_vec_generic',
//                    case 
//                        when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic') = 'array' then
//                            jsonb_build_object(
//                                'Ok',
//                                (
//                                    select jsonb_agg(
// 									   case
// 									   		when jsonb_typeof(value) = 'object' then 
//                                             	jsonb_build_object(
//                                                 	'Ok',
//                                        				jsonb_build_object(
//                                            				'std_string_string',
//                                            				case 
//                                                				when jsonb_typeof(value->'std_string_string') = 'string' then 
//                                                    				jsonb_build_object(
//                                                        				'Ok',
//                                                        				value->'std_string_string'
//                                                    				) 
//                                                				else 
//                                                    				jsonb_build_object(
//                                                        				'Err',
//                                                        				'todo error message'
//                                                    				)
//                                            				end
//                                        				)
//                                                  )
// 									   		else
//                                             	jsonb_build_object(
//                                                 	'Err',
//                                                     'todo error message'
//                                                 )
// 									   end
//                                    ) 
//                                    from jsonb_array_elements(
//                                        (select sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic')
//                                    ) 
//                                    with ordinality 
//                                    where ordinality between 0 AND 3
//                                )
//                            )
//                        else 
//                            jsonb_build_object(
//                                'Err',
//                                'todo error message'
//                            )
//                    end
//                )
//            )
//        else 
//            jsonb_build_object(
//                'Err',
//                'todo error message'
//            ) 
//    end
// as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
            Self::StdOptionOptionStdVecVecGeneric {
                field_vec,
                limit,
                offset
            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                field_vec,
                &format!("value"),
                false
            ) {
                Ok(value) => {
                    let start = offset;
                    let end = match offset.checked_add(*limit) {
                        Some(value) => value,
                        None => {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                                limit: *limit,
                                offset: *offset,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    };
                    Ok(format!(r#"
                        'std_option_option_std_vec_vec_generic',
                        case 
                            when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic') = 'array' then
                                jsonb_build_object(
                                    'Ok',
                                    (
                                        select jsonb_agg({value}) 
                                        from jsonb_array_elements(
                                            (select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic')
                                        ) 
                                        with ordinality 
                                        where ordinality between {start} and {end}
                                    )
                                )
                            when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic') = 'null' then
                                jsonb_build_object(
                                    'Ok',
                                    null
                                )
                            else 
                                jsonb_build_object(
                                    'Err',
                                    'todo error message'
                                )
                        end
                    "#))
                },
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },
            Self::StdVecVecStdOptionOptionGeneric {
                field_vec,
                limit,
                offset
            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                field_vec,
                &format!("value"),
                true
            ) {
                Ok(value) => {
                    let start = offset;
                    let end = match offset.checked_add(*limit) {
                        Some(value) => value,
                        None => {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                                limit: *limit,
                                offset: *offset,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    };
                    Ok(format!(r#"
                        'std_vec_vec_std_option_option_generic',
                        case 
                        	when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_generic') = 'array' then 
                        		jsonb_build_object(
                        			'Ok',
                        			(
                        				select jsonb_agg({value}) 
                        				from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_generic')) 
                        				with ordinality 
                        				where ordinality between {start} and {end}
                        			)
                        		)
                        	else 
                        		jsonb_build_object(
                        			'Err',
                        			'todo error message'
                        		) 
                        end
                    "#))
                },
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },

// select 
// 	case 
// 		when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' then 
//              jsonb_build_object(
//              	'Ok',
//                 jsonb_build_object(
// 					'std_vec_vec_std_option_option_generic',
// 					case 
// 						when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_std_option_option_generic') = 'array' then 
// 							jsonb_build_object(
// 								'Ok',
// 								(
// 									select jsonb_agg(
// 										case 
// 											when jsonb_typeof(value) = 'object' then
// 												jsonb_build_object(
// 													'Ok',
// 													jsonb_build_object(
// 														'std_string_string',
// 														case 
// 															when jsonb_typeof(value->'std_string_string') = 'string' then 
// 																jsonb_build_object(
// 																	'Ok',
// 																	value->'std_string_string'
// 																) 
// 															else 
// 																jsonb_build_object(
// 																	'Err',
// 																	'todo error message'
// 																) 
// 														end
// 													)	
// 												)
// 											when jsonb_typeof(value) = 'null' then
// 												jsonb_build_object(
// 													'Ok',
// 													null
// 												) 
// 											else 
// 												jsonb_build_object(
// 													'Err',
// 													'todo error message'
// 												)
// 										end
// 									) 
// 									from jsonb_array_elements((select sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_std_option_option_generic')) 
// 									with ordinality 
// 									where ordinality between 0 and 3
// 								)
// 							)
// 						else 
// 							jsonb_build_object(
// 								'Err',
// 								'todo error message'
// 							) 
// 					end
// 				)
//             )
//         else 
//         	jsonb_build_object(
//             	'Err',
//                 'todo error message'
//             ) 
// end
// as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14

            Self::StdOptionOptionStdVecVecStdOptionOptionGeneric {
                field_vec,
                limit,
                offset
            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                field_vec,
                &format!("value"),
                true
            ) {
                Ok(value) => {
                    let start = offset;
                    let end = match offset.checked_add(*limit) {
                        Some(value) => value,
                        None => {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                                limit: *limit,
                                offset: *offset,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    };
                    Ok(format!(r#"
                        'std_option_option_std_vec_vec_std_option_option_generic',
                        case 
                        	when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic') = 'array' then 
                        		jsonb_build_object(
                        			'Ok',
                        			(
                        				select jsonb_agg({value}) 
                        				from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic')) 
                        				with ordinality 
                        				where ordinality between {start} and {end}
                        			)
                        		)
                            when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic') = 'null' then
                            	jsonb_build_object(
                            		'Ok',
                            		null
                            	)
                        	else 
                        		jsonb_build_object(
                        			'Err',
                        			'todo error message'
                        		) 
                        end
                    "#))
                },
                Err(error) => {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        field: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },
// select 
// 	case 
// 		when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object' then 
//              jsonb_build_object(
//              	'Ok',
//                 jsonb_build_object(
// 					'std_option_option_std_vec_vec_std_option_option_generic',
// 					case 
// 						when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_std_vec_vec_std_option_option_generic') = 'array' then 
// 							jsonb_build_object(
// 								'Ok',
// 								(
// 									select jsonb_agg(
// 										case 
// 											when jsonb_typeof(value) = 'object' then
// 												jsonb_build_object(
// 													'Ok',
// 													jsonb_build_object(
// 														'std_string_string',
// 														case 
// 															when jsonb_typeof(value->'std_string_string') = 'string' then 
// 																jsonb_build_object(
// 																	'Ok',
// 																	value->'std_string_string'
// 																) 
// 															else 
// 																jsonb_build_object(
// 																	'Err',
// 																	'todo error message'
// 																) 
// 														end
// 													)	
// 												)
// 											when jsonb_typeof(value) = 'null' then
// 												jsonb_build_object(
// 													'Ok',
// 													null
// 												) 
// 											else 
// 												jsonb_build_object(
// 													'Err',
// 													'todo error message'
// 												)
// 										end
// 									) 
// 									from jsonb_array_elements((select sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_std_vec_vec_std_option_option_generic')) 
// 									with ordinality 
// 									where ordinality between 0 and 3
// 								)
// 							)
// 						when jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_std_vec_vec_std_option_option_generic') = 'null' then
// 							jsonb_build_object(
// 								'Ok',
// 								null
// 							)
// 						else 
// 							jsonb_build_object(
// 								'Err',
// 								'todo error message'
// 							) 
// 					end
// 				)
//             )
//         else 
//         	jsonb_build_object(
//             	'Err',
//                 'todo error message'
//             ) 
// end
// as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
        }
    }
}

// SELECT 
//     jsonb_build_object(
//         'cats',
//         (SELECT json_agg(jsonb_build_object('meow', value->>'meow')) 
//          FROM json_array_elements(
//                  (SELECT sqlx_types_json_t_as_postgresql_json_b_not_null->'cats')
//              ) WITH ORDINALITY 
//              WHERE ordinality BETWEEN 0 AND 4)
//     ) AS sqlx_types_json_t_as_postgresql_json_b_not_null 
// FROM jsongeneric 
// WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait//serde::Deserialize,
pub struct SomethingOptions {
    std_string_string: std::option::Option<crate::value::Value<std::string::String>>,
    std_vec_vec_std_primitive_bool: std::option::Option<crate::value::Value<std::vec::Vec<std::primitive::bool>>>,
    generic: std::option::Option<crate::value::Value<DoggieOptions>>,
    std_option_option_generic: std::option::Option<crate::value::Value<std::option::Option<DoggieOptions>>>,
    std_vec_vec_generic: std::option::Option<crate::value::Value<std::vec::Vec<DoggieOptions>>>,
    std_option_option_std_vec_vec_generic: std::option::Option<crate::value::Value<std::option::Option<std::vec::Vec<DoggieOptions>>>>,
    std_vec_vec_std_option_option_generic: std::option::Option<crate::value::Value<std::vec::Vec<std::option::Option<DoggieOptions>>>>,
    std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<crate::value::Value<std::option::Option<std::vec::Vec<std::option::Option<DoggieOptions>>>>>,

    // std_string_string: std::option::Option<std::result::Result<std::string::String, std::string::String>>,
    // std_vec_vec_std_primitive_bool: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>, std::string::String>>,
    // generic: std::option::Option<std::result::Result<DoggieOptions,std::string::String>>,
    // std_option_option_generic: std::option::Option<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,
    // std_vec_vec_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>,std::string::String>>,
    // std_option_option_std_vec_vec_generic: std::option::Option<std::result::Result<
    //     std::option::Option<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>,
    //     std::string::String
    // >>,
    // std_vec_vec_std_option_option_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,std::string::String>>,
    // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<
    //     std::result::Result<
    //         std::option::Option<
    //             std::vec::Vec<
    //                 std::result::Result<
    //                     std::option::Option<DoggieOptions>,
    //                     std::string::String
    //                 >
    //             >
    //         >,
    //         std::string::String
    //     >
    // >,
}

impl<'de> serde::Deserialize<'de> for SomethingOptions {
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
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
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
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    6u64 => serde::__private::Ok(__Field::__field6),
                    7u64 => serde::__private::Ok(__Field::__field7),
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
                    "std_string_string" => serde::__private::Ok(__Field::__field0),
                    "std_vec_vec_std_primitive_bool" => {
                        serde::__private::Ok(__Field::__field1)
                    }
                    "generic" => serde::__private::Ok(__Field::__field2),
                    "std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field3)
                    }
                    "std_vec_vec_generic" => serde::__private::Ok(__Field::__field4),
                    "std_option_option_std_vec_vec_generic" => {
                        serde::__private::Ok(__Field::__field5)
                    }
                    "std_vec_vec_std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field6)
                    }
                    "std_option_option_std_vec_vec_std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field7)
                    }
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
                    b"std_string_string" => serde::__private::Ok(__Field::__field0),
                    b"std_vec_vec_std_primitive_bool" => {
                        serde::__private::Ok(__Field::__field1)
                    }
                    b"generic" => serde::__private::Ok(__Field::__field2),
                    b"std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field3)
                    }
                    b"std_vec_vec_generic" => {
                        serde::__private::Ok(__Field::__field4)
                    }
                    b"std_option_option_std_vec_vec_generic" => {
                        serde::__private::Ok(__Field::__field5)
                    }
                    b"std_vec_vec_std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field6)
                    }
                    b"std_option_option_std_vec_vec_std_option_option_generic" => {
                        serde::__private::Ok(__Field::__field7)
                    }
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
            marker: serde::__private::PhantomData<SomethingOptions>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SomethingOptions;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct SomethingOptions",
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
                        std::result::Result<std::string::String, std::string::String>,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<
                                    std::primitive::bool,
                                    std::string::String,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                1usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<DoggieOptions, std::string::String>,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                2usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field3 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<DoggieOptions>,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                3usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field4 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<DoggieOptions, std::string::String>,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                4usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field5 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<DoggieOptions, std::string::String>,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                5usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field6 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<
                                    std::option::Option<DoggieOptions>,
                                    std::string::String,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                6usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                let __field7 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<
                                        std::option::Option<DoggieOptions>,
                                        std::string::String,
                                    >,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                7usize,
                                &"struct SomethingOptions with 8 elements",
                            ),
                        );
                    }
                };
                serde::__private::Ok(SomethingOptions {
                    std_string_string: match __field0 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_std_primitive_bool: match __field1 {
                        Some(value) => 
                        match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    generic: match __field2 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_generic: match __field3 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_generic: match __field4 {
                        Some(value) => match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_std_vec_vec_generic: match __field5 {
                        Some(value) => match value {
                            Ok(value) => match value {
                                Some(value) => {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    Some(crate::value::Value{ value: Some(acc) })
                                }
                                None => Some(crate::value::Value{ value: None })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_std_option_option_generic: match __field6 {
                        Some(value) => match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            }
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_std_vec_vec_std_option_option_generic: match __field7 {
                        Some(value) => match value {
                            Ok(value) => match value {
                                Some(value) => {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    Some(crate::value::Value{ value: Some(acc) })
                                }
                                None => Some(crate::value::Value{ value: None })
                            }
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        }
                        None => None
                    },
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
                    std::option::Option<
                        std::result::Result<std::string::String, std::string::String>,
                    >,
                > = serde::__private::None;
                let mut __field1: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<
                                    std::primitive::bool,
                                    std::string::String,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field2: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<DoggieOptions, std::string::String>,
                    >,
                > = serde::__private::None;
                let mut __field3: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<DoggieOptions>,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field4: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<DoggieOptions, std::string::String>,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field5: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<DoggieOptions, std::string::String>,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field6: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<
                                    std::option::Option<DoggieOptions>,
                                    std::string::String,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field7: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<
                                        std::option::Option<DoggieOptions>,
                                        std::string::String,
                                    >,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                    __Field,
                >(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_string_string",
                                    ),
                                );
                            }
                            __field0 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::string::String,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_vec_vec_std_primitive_bool",
                                    ),
                                );
                            }
                            __field1 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::vec::Vec<
                                                std::result::Result<
                                                    std::primitive::bool,
                                                    std::string::String,
                                                >,
                                            >,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "generic",
                                    ),
                                );
                            }
                            __field2 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<DoggieOptions, std::string::String>,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_option_option_generic",
                                    ),
                                );
                            }
                            __field3 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::option::Option<DoggieOptions>,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_vec_vec_generic",
                                    ),
                                );
                            }
                            __field4 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::vec::Vec<
                                                std::result::Result<DoggieOptions, std::string::String>,
                                            >,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_option_option_std_vec_vec_generic",
                                    ),
                                );
                            }
                            __field5 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::option::Option<
                                                std::vec::Vec<
                                                    std::result::Result<DoggieOptions, std::string::String>,
                                                >,
                                            >,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field6 => {
                            if serde::__private::Option::is_some(&__field6) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_vec_vec_std_option_option_generic",
                                    ),
                                );
                            }
                            __field6 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::vec::Vec<
                                                std::result::Result<
                                                    std::option::Option<DoggieOptions>,
                                                    std::string::String,
                                                >,
                                            >,
                                            std::string::String,
                                        >,
                                    >,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field7 => {
                            if serde::__private::Option::is_some(&__field7) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_option_option_std_vec_vec_std_option_option_generic",
                                    ),
                                );
                            }
                            __field7 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::option::Option<
                                                std::vec::Vec<
                                                    std::result::Result<
                                                        std::option::Option<DoggieOptions>,
                                                        std::string::String,
                                                    >,
                                                >,
                                            >,
                                            std::string::String,
                                        >,
                                    >,
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
                        serde::__private::de::missing_field("std_string_string")?
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        serde::__private::de::missing_field(
                            "std_vec_vec_std_primitive_bool",
                        )?
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        serde::__private::de::missing_field("generic")?
                    }
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => {
                        serde::__private::de::missing_field(
                            "std_option_option_generic",
                        )?
                    }
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => {
                        serde::__private::de::missing_field("std_vec_vec_generic")?
                    }
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => {
                        serde::__private::de::missing_field(
                            "std_option_option_std_vec_vec_generic",
                        )?
                    }
                };
                let __field6 = match __field6 {
                    serde::__private::Some(__field6) => __field6,
                    serde::__private::None => {
                        serde::__private::de::missing_field(
                            "std_vec_vec_std_option_option_generic",
                        )?
                    }
                };
                let __field7 = match __field7 {
                    serde::__private::Some(__field7) => __field7,
                    serde::__private::None => {
                        serde::__private::de::missing_field(
                            "std_option_option_std_vec_vec_std_option_option_generic",
                        )?
                    }
                };
                serde::__private::Ok(SomethingOptions {
                    std_string_string: match __field0 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_std_primitive_bool: match __field1 {
                        Some(value) => 
                        match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    generic: match __field2 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_generic: match __field3 {
                        Some(value) => 
                        match value {
                            Ok(value) => Some(crate::value::Value{ value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_generic: match __field4 {
                        Some(value) => match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_std_vec_vec_generic: match __field5 {
                        Some(value) => match value {
                            Ok(value) => match value {
                                Some(value) => {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    Some(crate::value::Value{ value: Some(acc) })
                                }
                                None => Some(crate::value::Value{ value: None })
                            },
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_vec_vec_std_option_option_generic: match __field6 {
                        Some(value) => match value {
                            Ok(value) => {
                                let mut acc = vec![];
                                for element in value {
                                    match element {
                                        Ok(value) => {
                                            acc.push(value);
                                        }
                                        Err(error) => {
                                            return Err(serde::de::Error::custom(error));
                                        }
                                    }
                                }
                                Some(crate::value::Value{ value: acc })
                            }
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None
                    },
                    std_option_option_std_vec_vec_std_option_option_generic: match __field7 {
                        Some(value) => match value {
                            Ok(value) => match value {
                                Some(value) => {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    Some(crate::value::Value{ value: Some(acc) })
                                }
                                None => Some(crate::value::Value{ value: None })
                            }
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        }
                        None => None
                    },
                })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "std_string_string",
            "std_vec_vec_std_primitive_bool",
            "generic",
            "std_option_option_generic",
            "std_vec_vec_generic",
            "std_option_option_std_vec_vec_generic",
            "std_vec_vec_std_option_option_generic",
            "std_option_option_std_vec_vec_std_option_option_generic",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SomethingOptions",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SomethingOptions>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct SomethingWrapper(pub Result<SomethingOptions,std::string::String>);

// #[test]
// fn test() {
//     // #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] 
//     // struct VValue<T> {
//     //     value: T,
//     // }
//     #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
//     struct DOptions {
//         std_string_string: std::option::Option<crate::Value<StdStringString>>,
//     }
//     // #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
//     // struct SStdOptionOptionGeneric<T>(std::option::Option<T>);
//     #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
//     struct SOptions {
//         std_option_option_generic: std::option::Option<crate::Value<std::option::Option<DOptions>>>,//todo value between two options
//     }

//     // let h = SOptions {
//     //     std_option_option_generic: Some(crate::Value {
//     //         value: Some(DOptions {
//     //             std_string_string: Some(crate::Value {
//     //                 value: StdStringString(std::string::String::from(""))
//     //             }),
//     //         }),
//     //     }),//todo value between two options
//     // };
//     // println!("*****{h:#?}");
//     // let u = serde_json::to_string(&h).unwrap();
//     // println!("$$$${u:#?}");
//     // {
//     //     "std_option_option_generic": {
//     //         "value": {
//     //             "std_string_string": {
//     //                 "value": ""
//     //             }
//     //         }
//     //     }
//     // }
//     let h = SOptions {
//         std_option_option_generic: Some(crate::Value {
//             value: Some(DOptions {
//                 std_string_string: Some(crate::Value {
//                     value: StdStringString(std::string::String::from(""))
//                 }),
//             }),
//         }),//todo value between two options
//     };
//     println!("*****{h:#?}");
//     let u = serde_json::to_string(&h).unwrap();
//     println!("$$$${u:#?}");
//     let f = r#"
//         {
//             "std_option_option_generic": {
//                 "value": {
//                     "std_string_string": {
//                         "value": null
//                     }
//                 }
//             }
//         }
//     "#;
//     let g: SomethingOptions = serde_json::from_str(f).unwrap();
//     println!("{g:#?}");
//     // let mut s = SomethingOptions {
//     //     std_string_string: None,
//     //     std_vec_vec_std_primitive_bool: None,
//     //     generic: None,
//     //     std_option_option_generic: None,//todo value between two options
//     //     std_vec_vec_generic: None,
//     //     std_option_option_std_vec_vec_generic: std::option::Option<StdOptionOptionStdVecVecGeneric<DoggieOptions>>,
//     //     std_vec_vec_std_option_option_generic: std::option::Option<StdVecVecStdOptionOptionGeneric<DoggieOptions>>,
//     //     std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<StdOptionOptionStdVecVecStdOptionOptionGeneric<DoggieOptions>>,
//     // };
// }

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub std_string_string: StdStringString,
}
impl std::convert::From<Doggie> for DoggieOptions {
    fn from(value: Doggie) -> Self {
        Self {
            std_string_string: Some(crate::value::Value{ value: value.std_string_string.0 })
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum DoggieField {
    #[serde(rename(
        serialize = "std_string_string",
        deserialize = "std_string_string"
    ))]
    StdStringString
}
impl error_occurence_lib::ToStdStringString for DoggieField {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, Clone, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieField,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    GeneratePostgresqlQueryPart {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl GeneratePostgresqlQueryPart<DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed, ()> for DoggieField {
    fn generate_postgresql_query_part_from_self_vec(
        value: &std::vec::Vec<Self>, 
        column_name_and_maybe_field_getter: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed> {
        if value.is_empty() {
            return Err(DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed::FieldsFilterIsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed::NotUniqueFieldFilter {
                    field: element.clone(),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            else {
                unique.push(&element);
            }
        }
        let mut acc = std::string::String::default();
        for element in value {
            match element.generate_postgresql_query_part(column_name_and_maybe_field_getter) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(_) => {
                    return Err(DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed::GeneratePostgresqlQueryPart {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        let _ = acc.pop();
        let is_optional_query_part = match is_optional {
            true => format!(r#"
                when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then
                    jsonb_build_object(
                        'Ok',
                        null
                    )
            "#),
            false => std::string::String::default()
        };
        Ok(format!(r#"
            case 
                when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then 
                    jsonb_build_object(
                        'Ok',
                        jsonb_build_object(
                            {acc}
                        )
                    )
                {is_optional_query_part}
                else 
                    jsonb_build_object(
                        'Err',
                        'todo error message'
                    ) 
            end 
        "#))
    }
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, ()> {
        match self {
            Self::StdStringString => Ok(format!(
                "'std_string_string',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_string_string') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_string_string') else jsonb_build_object('Err','todo error message') end"
            )),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct DoggieOptions {
    std_string_string: std::option::Option<crate::value::Value<std::string::String>>,
    // std_string_string: std::option::Option<std::result::Result<std::string::String, std::string::String>>,
}

impl<'de> serde::Deserialize<'de> for DoggieOptions {
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
                    "std_string_string" => serde::__private::Ok(__Field::__field0),
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
                    b"std_string_string" => serde::__private::Ok(__Field::__field0),
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
            marker: serde::__private::PhantomData<DoggieOptions>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = DoggieOptions;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct DoggieOptions",
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
                        std::result::Result<std::string::String, std::string::String>,
                    >,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"struct DoggieOptions with 1 element",
                            ),
                        );
                    }
                };
                serde::__private::Ok(DoggieOptions {
                    std_string_string: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(crate::value::Value { value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        }
                        None => None
                    },
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
                    std::option::Option<
                        std::result::Result<std::string::String, std::string::String>,
                    >,
                > = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                    __Field,
                >(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_string_string",
                                    ),
                                );
                            }
                            __field0 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::option::Option<
                                        std::result::Result<
                                            std::string::String,
                                            std::string::String,
                                        >,
                                    >,
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
                        serde::__private::de::missing_field("std_string_string")?
                    }
                };
                serde::__private::Ok(DoggieOptions {
                    std_string_string: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(crate::value::Value { value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        }
                        None => None
                    },
                })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["std_string_string"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "DoggieOptions",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<DoggieOptions>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}





// [
//     Field {
//         attrs: [],
//         vis: Visibility::Public(
//             Pub,
//         ),
//         mutability: FieldMutability::None,
//         ident: Some(
//             Ident {
//                 ident: "something",
//                 span: #0 bytes(566260..566269),
//             },
//         ),
//         colon_token: Some(
//             Colon,
//         ),
//         ty: Type::Path {
//             qself: None,
//             path: Path {
//                 leading_colon: None,
//                 segments: [
//                     PathSegment {
//                         ident: Ident {
//                             ident: "StdStringString",
//                             span: #0 bytes(566271..566290),
//                         },
//                         arguments: PathArguments::None,
//                     },
//                 ],
//             },
//         },
//     },
//     Field {
//         attrs: [],
//         vis: Visibility::Public(
//             Pub,
//         ),
//         mutability: FieldMutability::None,
//         ident: Some(
//             Ident {
//                 ident: "omega",
//                 span: #0 bytes(566300..566305),
//             },
//         ),
//         colon_token: Some(
//             Colon,
//         ),
//         ty: Type::Path {
//             qself: None,
//             path: Path {
//                 leading_colon: None,
//                 segments: [
//                     PathSegment {
//                         ident: Ident {
//                             ident: "StdVecVecStdPrimitiveBool",
//                             span: #0 bytes(566307..566336),
//                         },
//                         arguments: PathArguments::None,
//                     },
//                 ],
//             },
//         },
//     },
//     Field {
//         attrs: [],
//         vis: Visibility::Public(
//             Pub,
//         ),
//         mutability: FieldMutability::None,
//         ident: Some(
//             Ident {
//                 ident: "doggie",
//                 span: #0 bytes(566346..566352),
//             },
//         ),
//         colon_token: Some(
//             Colon,
//         ),
//         ty: Type::Path {
//             qself: None,
//             path: Path {
//                 leading_colon: None,
//                 segments: [
//                     PathSegment {
//                         ident: Ident {
//                             ident: "Generic",
//                             span: #0 bytes(566354..566365),
//                         },
//                         arguments: PathArguments::AngleBracketed {
//                             colon2_token: None,
//                             lt_token: Lt,
//                             args: [
//                                 GenericArgument::Type(
//                                     Type::Path {
//                                         qself: None,
//                                         path: Path {
//                                             leading_colon: None,
//                                             segments: [
//                                                 PathSegment {
//                                                     ident: Ident {
//                                                         ident: "Doggie",
//                                                         span: #0 bytes(566366..566372),
//                                                     },
//                                                     arguments: PathArguments::None,
//                                                 },
//                                             ],
//                                         },
//                                     },
//                                 ),
//                             ],
//                             gt_token: Gt,
//                         },
//                     },
//                 ],
//             },
//         },
//     },
//     Field {
//         attrs: [],
//         vis: Visibility::Public(
//             Pub,
//         ),
//         mutability: FieldMutability::None,
//         ident: Some(
//             Ident {
//                 ident: "cats",
//                 span: #0 bytes(566383..566387),
//             },
//         ),
//         colon_token: Some(
//             Colon,
//         ),
//         ty: Type::Path {
//             qself: None,
//             path: Path {
//                 leading_colon: None,
//                 segments: [
//                     PathSegment {
//                         ident: Ident {
//                             ident: "StdVecVecGeneric",
//                             span: #0 bytes(566389..566409),
//                         },
//                         arguments: PathArguments::AngleBracketed {
//                             colon2_token: None,
//                             lt_token: Lt,
//                             args: [
//                                 GenericArgument::Type(
//                                     Type::Path {
//                                         qself: None,
//                                         path: Path {
//                                             leading_colon: None,
//                                             segments: [
//                                                 PathSegment {
//                                                     ident: Ident {
//                                                         ident: "Cat",
//                                                         span: #0 bytes(566410..566413),
//                                                     },
//                                                     arguments: PathArguments::None,
//                                                 },
//                                             ],
//                                         },
//                                     },
//                                 ),
//                             ],
//                             gt_token: Gt,
//                         },
//                     },
//                 ],
//             },
//         },
//     },
// ]


// select 
// sqlx_types_json_t_as_postgresql_json_b_not_null
// -- jsonb_build_object(
// -- 	'std_option_option_generic',
	
// -- 	-- jsonb_build_object(
// -- 	-- 	'value',
// -- 	-- 	jsonb_build_object(
// -- 	-- 		'std_string_string',
// -- 	-- 		jsonb_build_object(
// -- 	-- 			'value',
// -- 	-- 			sqlx_types_json_t_as_postgresql_json_b_not_null->'std_option_option_generic'->'std_string_string'
// -- 	-- 		)
// -- 	-- 	)
// -- 	-- )
// -- ) 
// -- as sqlx_types_json_t_as_postgresql_json_b_not_null 
// from jsongeneric 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
// AND json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object'
// AND
// (
// 	(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' IS NULL)
// 	OR
// 	(
// 		json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic') = 'object'
// 		AND 
// 		(
// 			json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string') = 'string'
// 			OR
// 			sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string' IS NULL
// 		)
// 	)
// );

////////////
// SELECT
// CASE 
//            WHEN json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object'
// 		   		THEN 
// 	            jsonb_build_object(
//                     'std_option_option_generic', 
//                     CASE 
//                         WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' IS NULL THEN 
//                             NULL
//                         WHEN json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic') = 'object' THEN 
//                             jsonb_build_object(
// 								'std_string_string',
// 								CASE 
// 									WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'  IS NULL THEN
// 										NULL
// 									WHEN json_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string') = 'string' THEN 
// 										sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'
// 									ELSE 
// 										NULL
									
// 								END
// 							) 
//                         ELSE 
//                             NULL
//                     END
//                 )
//            ELSE NULL
//        END as sqlx_types_json_t_as_postgresql_json_b_not_null
// FROM jsongeneric
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14;



// SELECT
// CASE 
// WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object'
//    	THEN 
//         jsonb_build_object(
//             'std_option_option_generic', 
//             CASE 
//                 WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' IS NULL THEN 
//                     NULL
//                 WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic') = 'object' THEN 
//                     jsonb_build_object(
// 						'std_string_string',
// 						CASE 
// 							WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'  IS NULL THEN
// 								NULL
// 							WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string') = 'string' THEN 
// 								sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'
// 							ELSE 
// 								NULL
							
// 						END
// 					) 
//                 ELSE 
//                     NULL
//             END
//         )
//     ELSE NULL
// END as sqlx_types_json_t_as_postgresql_json_b_not_null
// FROM jsongeneric
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14;


// SELECT 
//        jsonb_build_object(
//            'a', 
//            (SELECT jsonb_agg(jsonb_build_object('b', elem -> 'b'))
//             FROM jsonb_array_elements(data -> 'a') AS elem
//             WHERE jsonb_typeof(elem) = 'object' 
//               AND jsonb_typeof(elem -> 'b') = 'number')
//        ) AS transformed_data
// FROM my_table
// WHERE 
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
// AND jsonb_typeof(data) = 'object'
//   AND data IS NOT NULL
//   AND jsonb_typeof(data -> 'a') = 'array'
//   AND (
//         SELECT bool_and(
//                  jsonb_typeof(elem) = 'object'
//                  AND jsonb_typeof(elem -> 'b') = 'number'
//                )
//         FROM jsonb_array_elements(data -> 'a') AS elem
//      );


// SELECT
// CASE 
// WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null) = 'object'
//    	THEN
//             CASE 
//                 WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' IS NULL THEN 
// 				    jsonb_build_object(
//             			'std_option_option_generic', 
// 						NULL
//         			)
//                 WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic') = 'object' THEN
// 					CASE 
// 						WHEN sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'  IS NULL THEN
							
// 							jsonb_build_object(
//             					'std_option_option_generic', 
//                     			jsonb_build_object(
// 									'std_string_string',
// 									NULL
// 								)
//         					)
// 						WHEN jsonb_typeof(sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string') = 'string' THEN 
// 							jsonb_build_object(
//             					'std_option_option_generic', 
//                     			jsonb_build_object(
// 									'std_string_string',
// 									sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_option_option_generic' -> 'std_string_string'
// 								)
//         					)
// 						ELSE 
// 							NULL	
// 					END
//                 ELSE 
//                     NULL
//             END
//     ELSE NULL
// END as sqlx_types_json_t_as_postgresql_json_b_not_null
// FROM jsongeneric
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14;