use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    pub image: Image,
    #[serde(rename = "item", default)]
    pub items: Vec<ItemElement>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    //title
    //link
    //width
    //height
    pub url: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemElement {
    pub creator: Option<String>,
    pub description: Option<String>,
    pub guid: Option<String>,
    pub link: Option<String>,
    #[serde(rename = "pubDate", default)]
    pub pub_date: Option<String>,
    pub title: Option<String>,
}
