use crate::_alloc_prelude::*;
use crate::SchemaGenerator;
use crate::{JsonSchema, Schema, json_schema};
use arrayvec07::{ArrayString, ArrayVec};

// Do not set maxLength on the schema as that describes length in characters, but we only
// know max length in bytes.
forward_impl!((<const CAP: usize> JsonSchema for ArrayString<CAP>) => String);

impl<T, const CAP: usize> JsonSchema for ArrayVec<T, CAP>
where
    T: JsonSchema,
{
    always_inline!();

    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        format!("Array_up_to_size_{}_of_{}", CAP, T::schema_name()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "items": generator.subschema_for::<T>(),
            "maxItems": CAP
        })
    }
}
