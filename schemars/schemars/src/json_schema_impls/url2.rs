use crate::SchemaGenerator;
use crate::{JsonSchema, Schema, json_schema};
use alloc::borrow::Cow;
use url2::Url;

impl JsonSchema for Url {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Url".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "url::Url".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "format": "uri",
        })
    }
}
