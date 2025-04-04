mod util;
use schemars::{JsonSchema, generate::SchemaSettings};
use util::*;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct Flat {
    f: f32,
    #[schemars(flatten)]
    e1: Enum1,
    #[schemars(flatten)]
    e2: Enum2,
    #[schemars(flatten)]
    e3: Enum3,
    #[schemars(flatten)]
    e4: Enum4,
    #[schemars(flatten)]
    e5: Enum5,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum Enum1 {
    B(bool),
    S(String),
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum Enum2 {
    U(u32),
    F(f64),
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum Enum3 {
    B2(bool),
    S2(String),
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum Enum4 {
    U2(u32),
    F2(f64),
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum Enum5 {
    B3(bool),
    S3(String),
}

#[test]
fn test_flat_schema() -> TestResult {
    test_default_generated_schema::<Flat>("enum_flatten")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(deny_unknown_fields)]
struct FlatDenyUnknownFields {
    f: f32,
    #[schemars(flatten)]
    e1: Enum1,
    #[schemars(flatten)]
    e2: Enum2,
    #[schemars(flatten)]
    e3: Enum3,
    #[schemars(flatten)]
    e4: Enum4,
    #[schemars(flatten)]
    e5: Enum5,
}

#[test]
fn test_flat_schema_duf() -> TestResult {
    test_default_generated_schema::<FlatDenyUnknownFields>("enum_flatten_duf")
}

#[test]
fn test_flat_schema_duf_draft07() -> TestResult {
    test_generated_schema::<FlatDenyUnknownFields>("enum_flatten_duf_draft07", SchemaSettings::draft07())
}
