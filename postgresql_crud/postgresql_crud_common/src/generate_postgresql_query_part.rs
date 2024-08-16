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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    pub std_primitive_i8: StdPrimitiveI8,
    pub std_primitive_i16: StdPrimitiveI16, 
    pub std_primitive_i32: StdPrimitiveI32, 
    pub std_primitive_i64: StdPrimitiveI64, 
    pub std_primitive_i128: StdPrimitiveI128,
    pub std_primitive_u8: StdPrimitiveU8, 
    pub std_primitive_u16: StdPrimitiveU16, 
    pub std_primitive_u32: StdPrimitiveU32, 
    pub std_primitive_u64: StdPrimitiveU64, 
    pub std_primitive_u128: StdPrimitiveU128, 
    pub std_primitive_f32: StdPrimitiveF32, 
    pub std_primitive_f64: StdPrimitiveF64, 
    pub std_primitive_bool: StdPrimitiveBool,
    pub std_string_string: StdStringString, 

    pub std_option_option_std_primitive_i8: StdOptionOptionStdPrimitiveI8, 
    pub std_option_option_std_primitive_i16: StdOptionOptionStdPrimitiveI16,  
    pub std_option_option_std_primitive_i32: StdOptionOptionStdPrimitiveI32,  
    pub std_option_option_std_primitive_i64: StdOptionOptionStdPrimitiveI64,  
    pub std_option_option_std_primitive_i128: StdOptionOptionStdPrimitiveI128,  
    pub std_option_option_std_primitive_u8: StdOptionOptionStdPrimitiveU8,  
    pub std_option_option_std_primitive_u16: StdOptionOptionStdPrimitiveU16,  
    pub std_option_option_std_primitive_u32: StdOptionOptionStdPrimitiveU32,  
    pub std_option_option_std_primitive_u64: StdOptionOptionStdPrimitiveU64,  
    pub std_option_option_std_primitive_u128: StdOptionOptionStdPrimitiveU128,  
    pub std_option_option_std_primitive_f32: StdOptionOptionStdPrimitiveF32,  
    pub std_option_option_std_primitive_f64: StdOptionOptionStdPrimitiveF64,  
    pub std_option_option_std_primitive_bool: StdOptionOptionStdPrimitiveBool,  
    pub std_option_option_std_string_string: StdOptionOptionStdStringString, 

    pub std_vec_vec_std_primitive_i8: StdVecVecStdPrimitiveI8, 
    pub std_vec_vec_std_primitive_i16: StdVecVecStdPrimitiveI16,  
    pub std_vec_vec_std_primitive_i32: StdVecVecStdPrimitiveI32,  
    pub std_vec_vec_std_primitive_i64: StdVecVecStdPrimitiveI64,  
    pub std_vec_vec_std_primitive_i128: StdVecVecStdPrimitiveI128,  
    pub std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8,  
    pub std_vec_vec_std_primitive_u16: StdVecVecStdPrimitiveU16,  
    pub std_vec_vec_std_primitive_u32: StdVecVecStdPrimitiveU32,  
    pub std_vec_vec_std_primitive_u64: StdVecVecStdPrimitiveU64,  
    pub std_vec_vec_std_primitive_u128: StdVecVecStdPrimitiveU128,  
    pub std_vec_vec_std_primitive_f32: StdVecVecStdPrimitiveF32,  
    pub std_vec_vec_std_primitive_f64: StdVecVecStdPrimitiveF64,  
    pub std_vec_vec_std_primitive_bool: StdVecVecStdPrimitiveBool,  
    pub std_vec_vec_std_string_string: StdVecVecStdStringString, 

    pub std_option_option_std_vec_vec_std_primitive_i8: StdOptionOptionStdVecVecStdPrimitiveI8, 
    pub std_option_option_std_vec_vec_std_primitive_i16: StdOptionOptionStdVecVecStdPrimitiveI16, 
    pub std_option_option_std_vec_vec_std_primitive_i32: StdOptionOptionStdVecVecStdPrimitiveI32, 
    pub std_option_option_std_vec_vec_std_primitive_i64: StdOptionOptionStdVecVecStdPrimitiveI64, 
    pub std_option_option_std_vec_vec_std_primitive_i128: StdOptionOptionStdVecVecStdPrimitiveI128, 
    pub std_option_option_std_vec_vec_std_primitive_u8: StdOptionOptionStdVecVecStdPrimitiveU8, 
    pub std_option_option_std_vec_vec_std_primitive_u16: StdOptionOptionStdVecVecStdPrimitiveU16, 
    pub std_option_option_std_vec_vec_std_primitive_u32: StdOptionOptionStdVecVecStdPrimitiveU32, 
    pub std_option_option_std_vec_vec_std_primitive_u64: StdOptionOptionStdVecVecStdPrimitiveU64, 
    pub std_option_option_std_vec_vec_std_primitive_u128: StdOptionOptionStdVecVecStdPrimitiveU128, 
    pub std_option_option_std_vec_vec_std_primitive_f32: StdOptionOptionStdVecVecStdPrimitiveF32, 
    pub std_option_option_std_vec_vec_std_primitive_f64: StdOptionOptionStdVecVecStdPrimitiveF64, 
    pub std_option_option_std_vec_vec_std_primitive_bool: StdOptionOptionStdVecVecStdPrimitiveBool, 
    pub std_option_option_std_vec_vec_std_string_string: StdOptionOptionStdVecVecStdStringString, 

    pub std_vec_vec_std_option_option_std_primitive_i8: StdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i16: StdVecVecStdOptionOptionStdPrimitiveI16, 
    pub std_vec_vec_std_option_option_std_primitive_i32: StdVecVecStdOptionOptionStdPrimitiveI32, 
    pub std_vec_vec_std_option_option_std_primitive_i64: StdVecVecStdOptionOptionStdPrimitiveI64, 
    pub std_vec_vec_std_option_option_std_primitive_i128: StdVecVecStdOptionOptionStdPrimitiveI128, 
    pub std_vec_vec_std_option_option_std_primitive_u8: StdVecVecStdOptionOptionStdPrimitiveU8, 
    pub std_vec_vec_std_option_option_std_primitive_u16: StdVecVecStdOptionOptionStdPrimitiveU16, 
    pub std_vec_vec_std_option_option_std_primitive_u32: StdVecVecStdOptionOptionStdPrimitiveU32, 
    pub std_vec_vec_std_option_option_std_primitive_u64: StdVecVecStdOptionOptionStdPrimitiveU64, 
    pub std_vec_vec_std_option_option_std_primitive_u128: StdVecVecStdOptionOptionStdPrimitiveU128, 
    pub std_vec_vec_std_option_option_std_primitive_f32: StdVecVecStdOptionOptionStdPrimitiveF32, 
    pub std_vec_vec_std_option_option_std_primitive_f64: StdVecVecStdOptionOptionStdPrimitiveF64, 
    pub std_vec_vec_std_option_option_std_primitive_bool: StdVecVecStdOptionOptionStdPrimitiveBool, 
    pub std_vec_vec_std_option_option_std_string_string: StdVecVecStdOptionOptionStdStringString,

    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8, 
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16, 
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i128: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u128: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    pub std_option_option_std_vec_vec_std_option_option_std_string_string: StdOptionOptionStdVecVecStdOptionOptionStdStringString,

    pub generic: Generic<Doggie>,
    pub std_option_option_generic: StdOptionOptionGeneric<Doggie>,
    pub std_vec_vec_generic: StdVecVecGeneric<Doggie>,
    pub std_option_option_std_vec_vec_generic: StdOptionOptionStdVecVecGeneric<Doggie>,
    pub std_vec_vec_std_option_option_generic: StdVecVecStdOptionOptionGeneric<Doggie>,
    pub std_option_option_std_vec_vec_std_option_option_generic: StdOptionOptionStdVecVecStdOptionOptionGeneric<Doggie>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub std_string_string: StdStringString,
}

// impl std::fmt::Display for Something {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub enum SomethingField {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8,
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16,
// }
// impl error_occurence_lib::ToStdStringString for SomethingField {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed {
//     FieldsFilterIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueFieldFilter {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: SomethingField,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct SomethingOptions {
//     std_primitive_i8: std::option::Option<crate::value::Value<std::primitive::i8>>,
//     std_primitive_i16: std::option::Option<crate::value::Value<std::primitive::i16>>,
// }
// impl std::convert::From<Something> for SomethingOptions {
//     fn from(value: Something) -> Self {
//         Self {
//             std_primitive_i8: Some(crate::value::Value {
//                 value: value.std_primitive_i8.0,
//             }),
//             std_primitive_i16: Some(crate::value::Value {
//                 value: value.std_primitive_i16.0,
//             }),
//         }
//     }
// }
// impl<'de> serde::Deserialize<'de> for SomethingOptions {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         enum __Field {
//             __field0,
//             __field1,
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "field identifier")
//             }
//             fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     0u64 => serde::__private::Ok(__Field::__field0),
//                     1u64 => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     "std_primitive_i8" => serde::__private::Ok(__Field::__field0),
//                     "std_primitive_i16" => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"std_primitive_i8" => serde::__private::Ok(__Field::__field0),
//                     b"std_primitive_i16" => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//         }
//         impl<'de> serde::Deserialize<'de> for __Field {
//             #[inline]
//             fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//             where
//                 __D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//             }
//         }
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<SomethingOptions>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = SomethingOptions;
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct SomethingOptions")
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i8, std::string::String>,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"struct SomethingOptions with 2 elements",
//                         ));
//                     }
//                 };
//                 let __field1 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i16, std::string::String>,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             1usize,
//                             &"struct SomethingOptions with 2 elements",
//                         ));
//                     }
//                 };
//                 serde::__private::Ok(SomethingOptions {
//                     std_primitive_i8: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(crate::value::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_primitive_i16: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(crate::value::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//             #[inline]
//             fn visit_map<__A>(
//                 self,
//                 mut __map: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i8, std::string::String>,
//                     >,
//                 > = serde::__private::None;
//                 let mut __field1: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i16, std::string::String>,
//                     >,
//                 > = serde::__private::None;
//                 while let serde::__private::Some(__key) =
//                     serde::de::MapAccess::next_key::<__Field>(&mut __map)?
//                 {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field(
//                                         "std_primitive_i8",
//                                     ),
//                                 );
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<std::primitive::i8, std::string::String>,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         __Field::__field1 => {
//                             if serde::__private::Option::is_some(&__field1) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field(
//                                         "std_primitive_i16",
//                                     ),
//                                 );
//                             }
//                             __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<std::primitive::i16, std::string::String>,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
//                                 &mut __map,
//                             )?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => {
//                         serde::__private::de::missing_field("std_primitive_i8")?
//                     }
//                 };
//                 let __field1 = match __field1 {
//                     serde::__private::Some(__field1) => __field1,
//                     serde::__private::None => {
//                         serde::__private::de::missing_field("std_primitive_i16")?
//                     }
//                 };
//                 serde::__private::Ok(SomethingOptions {
//                     std_primitive_i8: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(crate::value::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_primitive_i16: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(crate::value::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["std_primitive_i8", "std_primitive_i16"];
//         serde::Deserializer::deserialize_struct(
//             __deserializer,
//             "SomethingOptions",
//             FIELDS,
//             __Visitor {
//                 marker: serde::__private::PhantomData::<SomethingOptions>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct SomethingWrapper(pub SomethingOptions);
// impl<'de> serde::Deserialize<'de> for SomethingWrapper {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<SomethingWrapper>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = SomethingWrapper;
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct SomethingWrapper")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(
//                 self,
//                 __e: __E,
//             ) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<SomethingOptions, std::string::String> = <Result<
//                     SomethingOptions,
//                     std::string::String,
//                 > as serde::Deserialize>::deserialize(
//                     __e
//                 )?;
//                 serde::__private::Ok(SomethingWrapper(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     Result<SomethingOptions, std::string::String>,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"tuple struct SomethingWrapper with 1 element",
//                         ));
//                     }
//                 };
//                 serde::__private::Ok(SomethingWrapper(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(
//             __deserializer,
//             "SomethingWrapper",
//             __Visitor {
//                 marker: serde::__private::PhantomData::<SomethingWrapper>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// impl GeneratePostgresqlQueryPart<SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed, ()>
//     for SomethingField
// {
//     fn generate_postgresql_query_part_from_self_vec(
//         value: &std::vec::Vec<Self>,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//         is_optional: std::primitive::bool,
//     ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed>
//     {
//         if value.is_empty() {
//             return Err(
//                 SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed::FieldsFilterIsEmpty {
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 },
//             );
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return
//                 Err(SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed
//                 :: NotUniqueFieldFilter
//                 {
//                     field : element.clone(), code_occurence :
//                     error_occurence_lib :: code_occurence! (),
//                 });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             acc.push_str(& format!
//             ("{}||", match element
//             {
//                 Self :: StdPrimitiveI8 => format!
//                 ("jsonb_build_object('std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i8') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i8') else jsonb_build_object('Err','todo this must be error message') end)"),
//                 Self :: StdPrimitiveI16 => format!
//                 ("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object('Err','todo this must be error message') end) ")
//             }));
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part = match is_optional {
//             true => format!(
//                 r#"
//                             when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then
//                                 jsonb_build_object(
//                                     'Ok',
//                                     null
//                                 )
//                         "#
//             ),
//             false => std::string::String::default(),
//         };
//         // Ok(format!(
//         //     r#"
//         //                 case 
//         //                     when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then 
//         //                         jsonb_build_object(
//         //                             'Ok',
//         //                             jsonb_build_object({acc})  
//         //                         )
//         //                     {is_optional_query_part}
//         //                     else 
//         //                         jsonb_build_object(
//         //                             'Err',
//         //                             'todo error message'
//         //                         ) 
//         //                 end
//         //             "#
//         // ))
//         Ok(format!(
//             r#"
//                         case 
//                             when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then 
//                                 jsonb_build_object(
//                                     'Ok',
//                                     {acc}  
//                                 )
//                             {is_optional_query_part}
//                             else 
//                                 jsonb_build_object(
//                                     'Err',
//                                     'todo error message'
//                                 ) 
//                         end
//                     "#
//         ))
//     }
//     fn generate_postgresql_query_part(
//         &self,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//     ) -> Result<std::string::String, ()> {
//         match self
//         {
//             Self :: StdPrimitiveI8 =>
//             Ok(format!
//             ("'std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i8') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i8') else jsonb_build_object('Err','todo this must be error message') end ")),
//             Self :: StdPrimitiveI16 =>
//             Ok(format!
//             ("'std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object('Err','todo this must be error message') end "))
//         }
//     }
// }
