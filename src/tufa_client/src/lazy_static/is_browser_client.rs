use crate::components::rc::rc_animate::util::motion::can_use_dom;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IS_BROWSER_CLIENT: bool = can_use_dom();
}
