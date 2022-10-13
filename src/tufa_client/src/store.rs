use serde::{Deserialize, Serialize};
use yewdux::prelude::{Area, Dispatch, Persistent, PersistentStore};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct YewduxStore {
    pub count: u32,
    pub username: String,
    pub password: String,
}

impl Persistent for YewduxStore {
    fn key() -> &'static str {
        "Introduction.rs"
    }
    fn area() -> yewdux::prelude::Area {
        Area::Local
    }
}

pub fn init() -> Dispatch<PersistentStore<YewduxStore>> {
    Dispatch::<PersistentStore<YewduxStore>>::new()
}
