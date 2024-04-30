
pub trait IntoSerializeDeserializeVersion {
    fn into_serialize_deserialize_version<'a, T: serde::Serialize + serde::Deserialize<'a> + ?Sized>(&self) -> T;
}