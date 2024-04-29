pub trait IntoSerializeDeserializeToStdStringString {
    fn into_serialize_deserialize_to_std_string_string(
        self,
    ) -> impl serde::Serialize + serde::Deserialize<'static> + to_std_string_string::ToStdStringString;
}