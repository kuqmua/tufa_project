#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct ProvidersInitJsonSchema {
    pub data: Vec<String>,
}
