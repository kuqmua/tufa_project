use crate::components::rc::rc_animate::util::motion::get_option_style;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STYLE: Option<bool> = get_option_style();
}
