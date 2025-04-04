use crate::SchemaGenerator;
use crate::{JsonSchema, Schema, json_schema};
use alloc::borrow::Cow;
use uuid1::Uuid;

impl JsonSchema for Uuid {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Uuid".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "uuid::Uuid".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "format": "uuid",
        })
    }
}
