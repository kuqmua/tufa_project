use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref UUID: Mutex<i32> = Mutex::new(0);
}
