use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "item", default)]
    pub items: Vec<DataElement>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataElement {
    pub creator: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
}
