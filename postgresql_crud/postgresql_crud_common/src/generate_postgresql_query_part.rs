#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI8Json(std::primitive::i8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI16Json(std::primitive::i16);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI32Json(std::primitive::i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI64Json(std::primitive::i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI128Json(std::primitive::i128);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU8Json(std::primitive::u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU16Json(std::primitive::u16);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU32Json(std::primitive::u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU64Json(std::primitive::u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveU128Json(std::primitive::u128);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveF32Json(std::primitive::f32);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveF64Json(std::primitive::f64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveBoolJson(std::primitive::bool);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdStringStringJson(std::string::String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI8Json(std::option::Option<std::primitive::i8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI16Json(std::option::Option<std::primitive::i16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI32Json(std::option::Option<std::primitive::i32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI64Json(std::option::Option<std::primitive::i64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveI128Json(std::option::Option<std::primitive::i128>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU8Json(std::option::Option<std::primitive::u8>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU16Json(std::option::Option<std::primitive::u16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU32Json(std::option::Option<std::primitive::u32>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU64Json(std::option::Option<std::primitive::u64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveU128Json(std::option::Option<std::primitive::u128>);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveF32Json(std::option::Option<std::primitive::f32>);
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveF64Json(std::option::Option<std::primitive::f64>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdPrimitiveBoolJson(std::option::Option<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdStringStringJson(std::option::Option<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI8Json(std::vec::Vec<std::primitive::i8>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI16Json(std::vec::Vec<std::primitive::i16>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI32Json(std::vec::Vec<std::primitive::i32>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI64Json(std::vec::Vec<std::primitive::i64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveI128Json(std::vec::Vec<std::primitive::i128>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU8Json(std::vec::Vec<std::primitive::u8>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU16Json(std::vec::Vec<std::primitive::u16>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU32Json(std::vec::Vec<std::primitive::u32>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU64Json(std::vec::Vec<std::primitive::u64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveU128Json(std::vec::Vec<std::primitive::u128>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveF32Json(std::vec::Vec<std::primitive::f32>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveF64Json(std::vec::Vec<std::primitive::f64>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdPrimitiveBoolJson(std::vec::Vec<std::primitive::bool>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdStringStringJson(std::vec::Vec<std::string::String>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI8Json(std::option::Option<std::vec::Vec<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI16Json(std::option::Option<std::vec::Vec<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI32Json(std::option::Option<std::vec::Vec<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI64Json(std::option::Option<std::vec::Vec<std::primitive::i64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveI128Json(std::option::Option<std::vec::Vec<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU8Json(std::option::Option<std::vec::Vec<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU16Json(std::option::Option<std::vec::Vec<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU32Json(std::option::Option<std::vec::Vec<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU64Json(std::option::Option<std::vec::Vec<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveU128Json(std::option::Option<std::vec::Vec<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF32Json(std::option::Option<std::vec::Vec<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveF64Json(std::option::Option<std::vec::Vec<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdPrimitiveBoolJson(std::option::Option<std::vec::Vec<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdStringStringJson(std::option::Option<std::vec::Vec<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI8Json(std::vec::Vec<std::option::Option<std::primitive::i8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI16Json(std::vec::Vec<std::option::Option<std::primitive::i16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI32Json(std::vec::Vec<std::option::Option<std::primitive::i32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI64Json(std::vec::Vec<std::option::Option<std::primitive::i64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveI128Json(std::vec::Vec<std::option::Option<std::primitive::i128>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU8Json(std::vec::Vec<std::option::Option<std::primitive::u8>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU16Json(std::vec::Vec<std::option::Option<std::primitive::u16>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU32Json(std::vec::Vec<std::option::Option<std::primitive::u32>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU64Json(std::vec::Vec<std::option::Option<std::primitive::u64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveU128Json(std::vec::Vec<std::option::Option<std::primitive::u128>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF32Json(std::vec::Vec<std::option::Option<std::primitive::f32>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveF64Json(std::vec::Vec<std::option::Option<std::primitive::f64>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdPrimitiveBoolJson(std::vec::Vec<std::option::Option<std::primitive::bool>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdStringStringJson(std::vec::Vec<std::option::Option<std::string::String>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i128>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u128>>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64Json(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBoolJson(std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdStringStringJson(std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct GenericJson<T>(T);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionGenericJson<T>(std::option::Option<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecGenericJson<T>(std::vec::Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecGenericJson<T>(std::option::Option<std::vec::Vec<T>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdVecVecStdOptionOptionStdGenericJson<T>(std::vec::Vec<std::option::Option<T>>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdOptionOptionStdVecVecStdOptionOptionStdGenericJson<T>(std::option::Option<std::vec::Vec<std::option::Option<T>>>);
/////////////////////
pub trait GeneratePostgresqlQueryPart<T> {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, T>;
}
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    pub something: std::string::String,
    pub omega: std::vec::Vec<std::primitive::bool>,
    // #[json_field_name_stringified_reader] //todo for the future proc macro
    pub doggie: Doggie,
    pub cats: std::vec::Vec<Cat>,
}



// pub enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }

impl std::fmt::Display for Something {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
impl std::convert::From<Something> for SomethingOptions {
    fn from(value: Something) -> Self {
        Self {
            something: Some(value.something),
            omega: Some(value.omega),
            doggie: Some(DoggieOptions::from(value.doggie)),
            cats: Some(value.cats.into_iter().map(|element|CatOptions::from(element)).collect::<std::vec::Vec<CatOptions>>()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum SomethingReader {
    #[serde(rename(
        serialize = "something",
        deserialize = "something"
    ))]
    Something,
    #[serde(rename(
        serialize = "omega",
        deserialize = "omega"
    ))]
    Omega {
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "doggie",
        deserialize = "doggie"
    ))]
    Doggie(std::vec::Vec<DoggieReader>),
    #[serde(rename(
        serialize = "cats",
        deserialize = "cats"
    ))]
    Cats {
        reader_vec: std::vec::Vec<CatReader>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    }
}
impl error_occurence_lib::ToStdStringString for SomethingReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
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
    NotUniqueDoggieFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieReader,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueCatsFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: CatReader,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<SomethingGeneratePostgresqlQueryPartErrorNamed> for SomethingReader {
    //todo return result instead of std::string::String
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Something => Ok(format!("'something',{column_name_and_maybe_field_getter}->'something'")),
            Self::Omega {
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
                Ok(format!("'omega',(select json_agg(value) from json_array_elements((select {column_name_and_maybe_field_getter}->'omega')) with ordinality where ordinality between {start} and {end})"))
            },
            Self::Doggie(reader_vec) => Ok(format!(
                "'doggie',jsonb_build_object({})",
                {
                    if reader_vec.is_empty() {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::FieldsFilterIsEmpty {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    let mut unique_reader_vec = vec![];
                    for element in reader_vec {
                        if unique_reader_vec.contains(&element) {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::NotUniqueDoggieFieldFilter {
                                field: *element,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        else {
                            unique_reader_vec.push(&element);
                        }
                    }
                    let mut acc = reader_vec.iter().fold(std::string::String::default(), |mut acc, element| {
                        acc.push_str(&format!(
                            "{},",
                            element.generate_postgresql_query_part(&format!("{column_name_and_maybe_field_getter}->'doggie'")).unwrap()//todo return error
                        ));
                        acc
                    });
                    let _ = acc.pop();
                    acc
                }
            )),
            Self::Cats {
                reader_vec,
                limit,
                offset
            } => {
                if reader_vec.is_empty() {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::FieldsFilterIsEmpty {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                let mut unique_reader_vec = vec![];
                for element in reader_vec {
                    if unique_reader_vec.contains(&element) {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::NotUniqueCatsFieldFilter {
                            field: *element,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    else {
                        unique_reader_vec.push(&element);
                    }
                }
                let mut acc = reader_vec.iter().fold(std::string::String::default(), |mut acc, element| {
                    acc.push_str(&format!(
                        "{},",
                        element.generate_postgresql_query_part("value").unwrap()//todo return error//todo if it two inner[][] - is it correct to use value still?
                    ));
                    acc
                });
                let _ = acc.pop();
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
                Ok(format!("'cats',(select json_agg(jsonb_build_object({acc})) from json_array_elements((select sqlx_types_json_t_as_postgresql_json_not_null->'cats')) with ordinality where ordinality between {start} AND {end})"))
            }
        }
    }
}

// SELECT 
//     jsonb_build_object(
//         'cats',
//         (SELECT json_agg(jsonb_build_object('meow', value->>'meow')) 
//          FROM json_array_elements(
//                  (SELECT sqlx_types_json_t_as_postgresql_json_not_null->'cats')
//              ) WITH ORDINALITY 
//              WHERE ordinality BETWEEN 0 AND 4)
//     ) AS sqlx_types_json_t_as_postgresql_json_not_null 
// FROM jsongeneric 
// WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1;



#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct SomethingOptions {
    something: std::option::Option<std::string::String>,
    omega: std::option::Option<std::vec::Vec<bool>>,
    // #[json_field_name_stringified_reader] //todo for the future proc macro
    doggie: std::option::Option<DoggieOptions>,
    cats: std::option::Option<std::vec::Vec<CatOptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub says: std::string::String,
}
impl std::convert::From<Doggie> for DoggieOptions {
    fn from(value: Doggie) -> Self {
        Self {
            says: Some(value.says)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum DoggieReader {
    #[serde(rename(
        serialize = "says",
        deserialize = "says"
    ))]
    Says
}
impl error_occurence_lib::ToStdStringString for DoggieReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum DoggieGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<DoggieGeneratePostgresqlQueryPartErrorNamed> for DoggieReader {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Says => Ok(format!("'says',{column_name_and_maybe_field_getter}->'says'")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct DoggieOptions {
    says: std::option::Option<std::string::String>,
}

// impl DoggieOptions {
//     fn s(value: sqlx::types::JsonValue) -> Self {
//         let f: Self = serde_json::from_value(value).unwrap();

//         f
//     }
// }
// let schema = schema_for!(Something);
// println!("{}", serde_json::to_string_pretty(&schema).unwrap());


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Cat {
    pub meow: std::string::String,
    pub one: std::string::String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum CatReader {
    #[serde(rename(
        serialize = "meow",
        deserialize = "meow"
    ))]
    Meow,
    #[serde(rename(
        serialize = "one",
        deserialize = "one"
    ))]
    One
}
impl error_occurence_lib::ToStdStringString for CatReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CatGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<CatGeneratePostgresqlQueryPartErrorNamed> for CatReader {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, CatGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Meow => Ok(format!("'meow',{column_name_and_maybe_field_getter}->'meow'")),
            Self::One => Ok(format!("'one',{column_name_and_maybe_field_getter}->'one'")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct CatOptions {
    meow: std::option::Option<std::string::String>,
    one: std::option::Option<std::string::String>,
}
impl std::convert::From<Cat> for CatOptions {
    fn from(value: Cat) -> Self {
        Self {
            meow: Some(value.meow),
            one: Some(value.one),
        }
    }
}