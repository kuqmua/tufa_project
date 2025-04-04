use crate::_alloc_prelude::*;
use crate::SchemaGenerator;
use crate::{JsonSchema, Schema, json_schema};
use alloc::borrow::Cow;
use either1::Either;

impl<L: JsonSchema, R: JsonSchema> JsonSchema for Either<L, R> {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        format!("Either_{}_or_{}", L::schema_name(), R::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("either::Either<{}, {}>", L::schema_id(), R::schema_id()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "anyOf": [generator.subschema_for::<L>(), generator.subschema_for::<R>()],
        })
    }
}
