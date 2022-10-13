use crate::components::rc::rc_animate::util::motion::{can_use_dom, get_vendor_prefixes, Prefixes};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref VENDOR_PREFIXES: Prefixes = get_vendor_prefixes(can_use_dom(), HashMap::new());//todo //window()
}
