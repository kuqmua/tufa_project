#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct Data {
    #[serde(rename = "entry", default)]
    pub entries: Vec<DataElement>,
}
#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct DataElement {
    pub id: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub author: DataElementAuthor,
    pub media: Option<String>,
    pub content: Option<String>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct DataElementAuthor {
    pub name: Option<String>,
    pub uri: Option<String>,
}
