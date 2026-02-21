use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "entry", default)]
    pub entries: Vec<DataEl>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataEl {
    pub author: DataElAuthor,
    pub content: Option<String>,
    pub id: Option<String>,
    pub link: Option<String>,
    pub media: Option<String>,
    pub published: Option<String>,
    pub title: Option<String>,
    pub updated: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataElAuthor {
    pub name: Option<String>,
    pub uri: Option<String>,
}
