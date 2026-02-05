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
    pub author: DataElementAuthor,
    pub content: Option<String>,
    pub id: Option<String>,
    pub link: Option<String>,
    pub media: Option<String>,
    pub published: Option<String>,
    pub title: Option<String>,
    pub updated: Option<String>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct DataElementAuthor {
    pub name: Option<String>,
    pub uri: Option<String>,
}
