#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct YewduxStore {
    pub count: u32,
    pub username: String,
    pub password: String,
}

impl yewdux::prelude::Persistent for YewduxStore {
    fn key() -> &'static str {
        "Introduction.rs"
    }
    fn area() -> yewdux::prelude::Area {
        yewdux::prelude::Area::Local
    }
}

pub fn init() -> yewdux::prelude::Dispatch<yewdux::prelude::PersistentStore<YewduxStore>> {
    yewdux::prelude::Dispatch::<yewdux::prelude::PersistentStore<YewduxStore>>::new()
}
