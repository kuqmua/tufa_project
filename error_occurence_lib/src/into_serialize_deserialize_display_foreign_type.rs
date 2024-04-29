pub trait IntoSerializeDeserializeDisplayForeignType {
    fn into_serialize_deserialize_display_foreign_type(
        self,
    ) -> impl serde::Serialize + serde::Deserialize<'static> + display_foreign_type::DisplayForeignType;
}