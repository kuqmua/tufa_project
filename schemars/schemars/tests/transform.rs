mod util;
use schemars::{JsonSchema, Schema, transform::RecursiveTransform};
use serde_json::{Map, Value};
use util::*;

fn capitalize_type(schema: &mut Schema) {
    if let Some(Value::String(ty)) = schema.get("type") {
        schema.insert("upperType".to_owned(), ty.to_uppercase().into());
    }
}

fn insert_property_count(schema: &mut Schema) {
    let count = schema.get("properties").and_then(Value::as_object).map_or(0, Map::len);
    schema.insert("propertyCount".to_owned(), count.into());
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(transform = RecursiveTransform(capitalize_type), transform = insert_property_count)]
struct Struct {
    value: Value,
    #[schemars(transform = insert_property_count)]
    int: i32,
}

#[test]
fn transform_struct() -> TestResult {
    test_default_generated_schema::<Struct>("transform_struct")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(transform = RecursiveTransform(capitalize_type), transform = insert_property_count)]
enum External {
    #[schemars(transform = insert_property_count)]
    Unit,
    #[schemars(transform = insert_property_count)]
    NewType(Value),
}

#[test]
fn transform_enum_external() -> TestResult {
    test_default_generated_schema::<External>("transform_enum_external")
}
