pub trait IntoSerializeDeserializeToString {
    fn into_serialize_deserialize_to_string(
        self,
    ) -> impl serde::Serialize + serde::Deserialize<'static> + std::fmt::Display;
}

impl IntoSerializeDeserializeToString for serde_json::Error {
    fn into_serialize_deserialize_to_string(
        self,
    ) -> impl serde::Serialize + serde::Deserialize<'static> + std::fmt::Display {
        self.to_string()
    }
}
